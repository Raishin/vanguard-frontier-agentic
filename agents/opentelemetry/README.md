# 📡 OpenTelemetry Agents

<p align="center">
  <span style="font-size:3.5em">📡</span>
</p>

OpenTelemetry agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit Collector pipeline config — receivers, processors, exporters, memory limits | read-only | not allowed |

## 📋 Collector config review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `opentelemetry-collector-config-review-agent` | Review OTEL Collector pipeline — `memory_limiter` position, receiver exposure, exporter cardinality, no-exporter silent loss, credential handling | read-only | — |

## 🛡️ Operating note

- `memory_limiter` processor must be **first in every pipeline** — placing it later means a cardinality spike can OOM the Collector before the limiter fires
- A pipeline with no exporter configured drops all telemetry silently — verify every pipeline has at least one exporter or a `debug` fallback
- Receivers exposing gRPC/HTTP endpoints on `0.0.0.0` without authentication become ingest endpoints for any pod in the cluster
- Credentials in exporter configuration (API keys, tokens) must come from environment variables or Kubernetes Secrets — never hardcoded
- High-cardinality dimensions (e.g., unbounded `user_id`, `request_id` labels) drive metric storage costs non-linearly

## 📦 Install

```bash
# Install OpenTelemetry Collector config review agent
npx vfa-export-agents --platform claude-code --agents opentelemetry-collector-config-review-agent --repo .

# Install all Kubernetes runtime security agents (includes OTEL review)
npx vfa-export-agents --platform claude-code --role kubernetes-runtime-security-engineer --repo .
```
