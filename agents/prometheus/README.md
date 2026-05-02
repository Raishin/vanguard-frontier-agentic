# 📊 Prometheus Agents

<p align="center">
  <span style="font-size:3.5em">📊</span>
</p>

Prometheus agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit alerting rules, recording rules, cardinality, and routing configuration | read-only | not allowed |

## 📋 Alerting and cardinality review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `prometheus-alerting-cardinality-review-agent` | Review PromQL alerting rules, recording rules, label cardinality, AlertmanagerConfig routing, and inhibition rules | read-only | — |

## 🛡️ Operating note

- High-cardinality labels (unbounded `pod`, `request_id`, `user_id`) applied to metrics drive Prometheus TSDB memory and storage non-linearly — review before deployment
- Recording rules without time-range alignment produce incorrect aggregates; review `range_interval` vs `evaluation_interval` alignment
- AlertmanagerConfig with `continue: true` and no inhibition risks alert storms from correlated failures
- `absent()` alerts without `for: 5m` grace period generate false positives during rolling restarts

## 📦 Install

```bash
# Install Prometheus alerting and cardinality review agent
npx vfa-export-agents --platform claude-code --agents prometheus-alerting-cardinality-review-agent --repo .

# Install all Kubernetes observability agents
npx vfa-export-agents --platform claude-code --role kubernetes-observability-engineer --repo .
```
