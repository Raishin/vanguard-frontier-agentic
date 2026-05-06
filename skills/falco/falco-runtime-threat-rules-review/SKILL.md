---
name: falco-runtime-threat-rules-review
description: Use this skill when reviewing Falco rules files, falco.yaml configuration, or runtime security posture for a Kubernetes workload. Trigger when a user provides Falco rules YAML, asks whether their Falco setup covers a specific threat, questions rule exception scope, or wants to validate that Falco alert output reaches their SIEM or incident response pipeline.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# Falco Runtime Threat Rules Review

## Purpose
This skill reviews Falco runtime security rules and configuration for correctness, coverage gaps, and operational safety. Falco is a CNCF kernel-level threat detection tool; a misconfigured exception or a silently unconfigured audit webhook means real attacks produce zero alerts. The review catches macro composition errors, overly broad exceptions, missing sensitive-path rules, K8s audit webhook gaps, and alert output routing failures before attackers can exploit them.

## Lean operating rules
- Treat any rule exception that whitelists an entire process name family (`proc.name in (java, python, node, sh, bash)`) for a sensitive syscall category as HIGH — this creates a full detection blind spot for those runtimes.
- Treat any rule exception that uses `container.name in (my-app)` without an explicit syscall scope as HIGH — it disables all Falco detection for that container.
- Treat the absence of rules covering `/proc/*/mem` access, `/etc/shadow` reads, and `/var/run/secrets` mounts as HIGH — these are high-signal kernel-level indicators of container escape and credential theft.
- Treat K8s audit rules present in the ruleset but no K8s audit webhook configured in the API server as HIGH — the rules exist but never fire because audit events are never delivered.
- Treat Falco output routed only to stdout with no log aggregation or Falco sidekick configured as HIGH — alerts are silently lost unless a logging pipeline captures stdout from the Falco pod.
- Flag rules with priority set uniformly to EMERGENCY or CRITICAL for non-critical conditions as MEDIUM — miscalibrated priorities cause alert fatigue and operators begin ignoring or disabling Falco.
- Flag macro composition that uses negation (`not`) without referencing container context macros — bare process-name rules fire on the host as well as in containers.
- Do not recommend disabling or commenting out default Falco rules without stating the specific workload justification and residual risk.
- Label all findings with evidence basis: rule text provided, documentation-based, or inference from missing config.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Macro and rule composition correctness findings
- Exception scope assessment (process name, container name, syscall scope)
- Sensitive-path coverage gaps (/proc/*/mem, /etc/shadow, /var/run/secrets)
- K8s audit webhook connectivity assessment
- Alert output channel findings (sidekick, gRPC, stdout-only risk)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
