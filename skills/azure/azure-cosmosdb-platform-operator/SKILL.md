---
name: azure-cosmosdb-platform-operator
description: Use this skill for Azure Cosmos DB platform operations and design review, especially accounts, databases, containers, partition-key design, throughput and RU posture, consistency choices, indexing, throttling, multi-region replication, private connectivity, and Cosmos DB MCP-guided discovery.
metadata:
  author: github: Raishin
  version: 0.1.0
---

# Azure Cosmos DB Platform Operator

## Purpose

Review and guide Azure Cosmos DB platform posture without hand-wavy assumptions about partitioning, consistency, throughput, or multi-region behavior.

## When to use

Use this skill when the user asks for:

- Azure Cosmos DB account, database, or container operational review,
- partition-key, consistency, throughput, or indexing decisions,
- RU cost, throttling, hot partition, or multi-region tradeoff analysis,
- private endpoint, failover, or platform control-plane posture questions,
- Cosmos DB MCP-backed discovery or evidence gathering.

Do not use this skill as a substitute for:

- application code implementation details unless they directly affect platform posture,
- generic landing-zone or network architecture when Cosmos DB is incidental,
- RBAC-only analysis when the real problem is identity governance rather than database platform design,
- vector-search-specific Mongo vCore implementation unless the user explicitly asks for that API surface.

## Lean operating rules

- Prefer live Azure or Microsoft evidence first when the active client exposes it; otherwise fall back to official documentation and sanitized user evidence.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad scope, vague partition keys, and RU-blind advice.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [MCP and evidence path](references/mcp-and-evidence.md) — use when choosing live Azure evidence, confirming Microsoft MCP capability, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying stress checks, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed Microsoft documentation list or source notes.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main design or operational risks,
- the safest next actions,
- the assumptions or blockers that prevent stronger conclusions.
