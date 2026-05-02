# 🦅 Falco Agents

<p align="center">
  <span style="font-size:3.5em">🦅</span>
</p>

Falco agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit Falco rules, macros, exceptions, and alert routing | read-only | not allowed |

## 📋 Runtime threat rules review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `falco-runtime-threat-rules-review-agent` | Review Falco rules files for macro correctness, exception blast radius, sensitive-path coverage, K8s audit webhook gaps, and SIEM output routing | read-only | — |

## 🛡️ Operating note

- Falco rule exceptions with broad `proc.name` or container name matchers create silent detection blind spots — an exception for `proc.name = java` disables all detections for every Java process in the cluster
- `k8s_audit` rules only fire if the K8s audit webhook is configured to forward to Falco; rules exist but alerts are silent without the webhook
- Custom macro overrides that shadow built-in macros (`container`, `spawned_process`, `open_write`) can silently suppress entire detection categories
- Alert outputs sent only to `stdout` with no sidekick routing are lost in high-volume pod log churn

## 📦 Install

```bash
# Install Falco runtime threat rules review agent
npx vfa-export-agents --platform claude-code --agents falco-runtime-threat-rules-review-agent --repo .

# Install all Kubernetes supply chain security agents (includes Falco)
npx vfa-export-agents --platform claude-code --role kubernetes-supply-chain-security-engineer --repo .
```
