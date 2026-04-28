# MCP and evidence path

Use this reference only when you need to decide how to gather Cosmos DB performance evidence.

## Live-first evidence rule

1. Prefer live Azure MCP capability evidence when the active client exposes Azure tools.
2. Treat the runtime-exposed tool inventory as truth.
3. If Cosmos DB tooling is not exposed live, say so and switch to documentation-based guidance instead of pretending the namespace exists.

## Azure MCP grounding

Based on Microsoft documentation:

- Azure MCP supports a documented `cosmos` namespace in namespace mode.
- Microsoft recommends **consolidated mode** for AI agents because it reduces tool count and improves usability.
- Namespace filtering means a client may expose only a subset of Azure tools.

Implication:

- Do not assume that `cosmos` is available in the current runtime just because Microsoft documents it.
- If live MCP discovery is unclear, inspect or ask for the available Azure tool inventory before making namespace-specific claims.

## Evidence hierarchy

Use this order:

1. **live evidence** — Azure MCP output, sanitized metrics, sanitized Data Explorer screenshots, sanitized query metrics, or diagnostic-log output
2. **user-provided sanitized evidence** — redacted SDK traces, query text, request-charge samples, portal screenshots, IaC snippets, logs
3. **documentation-based** — Microsoft Learn and official Azure MCP documentation
4. **inference** — conclusions derived from patterns but not directly proven by evidence

## Performance-specific caution points

- A 429 spike does not automatically prove underprovisioning; it can also indicate skew, bursts, or bad query shape.
- Acceptable RU charge does not prove acceptable latency.
- High latency does not automatically prove server-side slowness; region distance, retries, and client configuration matter.
- Index metrics are for troubleshooting; do not turn them into always-on ritual without cause.
