# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized YAML or JSON snippets (no real hostnames, no auth tokens, no kubeconfig inline):
- Falco rules file(s) (`falco_rules.yaml`, custom rules YAML)
- `falco.yaml` (main Falco configuration — output channels, driver type, grpc settings)
- K8s API server audit policy (`audit-policy.yaml`) and webhook configuration (`audit-webhook-config.yaml`), if K8s audit rules are present
- Falco sidekick configuration, if deployed
- Optional: output of `falco --list` or `falcoctl rules list` showing loaded rules

If the user provides only a partial set, note which sections are absent and scope findings accordingly.

### Step 2 — Macro composition audit

Review every macro definition and its use in rules.

Check for:
- Macros used in negation context (`not is_container`) that do not also scope by `container.id != host`
- Macros that reference process names without syscall scope (e.g., a macro that matches `proc.name = bash` without specifying which syscalls it applies to)
- Inheritance chains where a child macro overrides a parent silently (Falco macro override via `override: true` or duplicate macro name)

Example macro composition risk:
```yaml
# RISKY — this macro matches bash on host AND in containers
# If used in a NOT clause, it exempts bash everywhere
- macro: bash_shell
  condition: proc.name = bash

# CORRECT — scope to container context
- macro: bash_in_container
  condition: proc.name = bash and container.id != host
```

Flag any macro that, when used in a negation, could suppress host-level detection as MEDIUM.

### Step 3 — Rule priority calibration audit

Review the `priority` field on all custom rules.

Falco priority ladder (highest to lowest):
`EMERGENCY` → `ALERT` → `CRITICAL` → `ERROR` → `WARNING` → `NOTICE` → `INFORMATIONAL` → `DEBUG`

Check for:
- All custom rules set to `CRITICAL` or `EMERGENCY` regardless of actual threat severity → MEDIUM (alert fatigue)
- Rules covering expected or semi-expected behavior (e.g., a CI/CD pipeline running `kubectl exec`) set to `CRITICAL` → MEDIUM
- Rules covering genuine high-severity threats (container escape attempts, `/proc/*/mem` access) set to `WARNING` or lower → HIGH (under-detection)

Recommended calibration:
```yaml
# Container escape attempt — should be CRITICAL
- rule: Read sensitive memory path
  desc: Detects direct /proc/PID/mem access indicative of memory scraping
  condition: open_read and fd.name startswith /proc and fd.name contains /mem
  output: "Sensitive memory read (proc=%proc.name pid=%proc.pid file=%fd.name)"
  priority: CRITICAL
  tags: [container, process, mitre_credential_access]

# Expected CI noise — should be NOTICE or lower
- rule: Kubectl exec in CI namespace
  condition: spawned_process and proc.name = kubectl and k8s.ns.name = ci
  output: "kubectl exec in CI (pod=%k8s.pod.name)"
  priority: NOTICE
```

### Step 4 — Exception scope audit

Review every `exceptions:` block on every rule.

**4a. Process name exceptions**
```yaml
# HIGH — whitelists all Java processes from shell spawn detection
- rule: Spawned shell from non-shell binary
  exceptions:
    - name: java_apps
      fields: [proc.pname]
      comps: [pmatch]
      values:
        - [java]
```
Any exception that matches a broad process family (`java`, `python`, `node`, `ruby`, `sh`, `bash`) for a sensitive syscall or spawn category completely blinds Falco to attacks running inside those runtimes.

**4b. Container name exceptions**
```yaml
# HIGH — disables ALL Falco detection for this container
- rule: Write below binary dir
  exceptions:
    - name: my_app_exception
      fields: [container.name]
      comps: [=]
      values:
        - [my-privileged-app]
```
Container-name exceptions applied at the rule level disable only that rule for that container. But if the same pattern is repeated across multiple rules, the cumulative effect is full detection blindness for that container.

**4c. Correct narrow exception pattern**
```yaml
# CORRECT — scopes exception to specific image + specific writable path
- rule: Write below binary dir
  exceptions:
    - name: my_app_installer
      fields: [container.image.repository, fd.directory]
      comps: [=, =]
      values:
        - [my-org/my-app, /usr/local/bin/app-plugins]
```

Flag any exception where `fields` contains only `proc.name` or `container.name` without additional syscall or path scope as HIGH.

### Step 5 — Sensitive path coverage audit

Verify that rules exist (custom or inherited from the default ruleset) for:

| Threat | Expected rule condition |
|--------|------------------------|
| Container memory scraping | `fd.name startswith /proc` and `fd.name contains /mem` |
| Shadow file access | `fd.name = /etc/shadow` or `fd.name = /etc/gshadow` |
| K8s service account token read | `fd.name startswith /var/run/secrets/kubernetes.io` |
| Privileged container write to host path | `container.privileged = true` and `fd.name startswith /host` |
| Binary directory write | `fd.directory in (/bin, /usr/bin, /usr/local/bin, /sbin)` |

If any of these are absent and not covered by a loaded default ruleset, flag as HIGH.

Check whether `falco_rules.yaml` references `- rule: ...` with `override: replace` that silently removes a default rule for one of the above categories.

### Step 6 — Kubernetes audit rules audit

Detect whether K8s audit rules are present in the ruleset:
```yaml
# K8s audit rules require k8s_audit macro
- rule: K8s Secret Get or List
  condition: k8s_audit and ka.verb in (get, list, watch) and ka.target.resource = secrets
  priority: WARNING
```

If K8s audit rules exist, check:
- Whether `falco.yaml` has a `webserver` section configured (Falco embedded audit webhook listener)
- Whether the K8s API server has an audit webhook pointing to Falco (`--audit-webhook-config-file`)
- Whether the audit policy includes `resources: [secrets, configmaps]` at a minimum

```yaml
# Required in falco.yaml for K8s audit
webserver:
  enabled: true
  listen_port: 8765
  k8s_audit_endpoint: /k8s-audit
  ssl_enabled: false
```

If K8s audit rules are present but no webhook is configured or no audit policy is provided, flag as HIGH — the rules are dead weight.

### Step 7 — Alert output channel audit

Review `falco.yaml` `output` section and any sidekick deployment:

**7a. stdout-only output**
```yaml
# RISKY — alerts go to pod stdout only
stdout_output:
  enabled: true
file_output:
  enabled: false
grpc_output:
  enabled: false
```
If only stdout is enabled and no log aggregation (Fluentd, Fluent Bit, Loki) is confirmed to be scraping the Falco pod, all alerts are silently lost when the pod restarts or the log buffer rolls over. Flag as HIGH.

**7b. Falco sidekick**
Falco sidekick is the recommended integration bridge (Slack, PagerDuty, Splunk, OpsGenie, SIEM webhooks):
```yaml
# Correct — gRPC to sidekick
grpc_output:
  enabled: true
grpc:
  enabled: true
  bind_address: "unix:///var/run/falco/falco.sock"
```
Verify sidekick is deployed as a Deployment (not a DaemonSet sidecar) and has a live output target configured.

**7c. Output throttling**
```yaml
# Check for rate limiting that drops high-volume events
outputs:
  rate: 1
  max_burst: 1000
```
Very low `rate` values with small `max_burst` can silently throttle alerts during an active incident. Flag `rate < 10` combined with `max_burst < 100` as MEDIUM.

### Step 8 — Driver type compatibility audit

Identify the configured driver (`ebpf`, `module`, `modern_ebpf`) from `falco.yaml` or deployment manifests.

- `modern_ebpf` (CO-RE) requires kernel 5.8+; check whether the node kernel version is compatible
- Managed K8s (GKE Autopilot, EKS Fargate) restricts kernel module loading; eBPF or modern_ebpf is required
- Some syscalls are not available on all drivers — verify critical syscall coverage against `falco --list`

Flag driver/kernel incompatibility as HIGH if it means syscalls used in critical rules are not captured.

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence summary: pass / needs work / critical issues found>

## Evidence level
<live evidence | user-provided sanitized config | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>
...

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never recommend adding broad process-name exceptions (`proc.name in (java, python, node)`) — this creates detection blind spots that attackers can exploit by running malicious code inside a whitelisted runtime.
- Never recommend disabling the default Falco ruleset (`rules_file: []`) without a complete custom ruleset replacement.
- Treat any exception that uses `container.name` as the sole discriminator across multiple rules as cumulative HIGH — the container effectively runs undetected.
- Do not recommend stdout-only output as production-ready without confirming a log aggregation pipeline scrapes the Falco pod and forwards to a SIEM or alerting system.
- Flag the absence of alerting on Falco's own health (`falco_events_total`, dropped events counter) — a crashing or throttled Falco pod goes unnoticed without self-monitoring.
