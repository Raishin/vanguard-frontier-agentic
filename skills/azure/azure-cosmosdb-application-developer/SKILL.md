---
name: azure-cosmosdb-application-developer
description: Use this skill for Azure Cosmos DB application development work, especially NoSQL data modeling, document structure, partition-aware access patterns, point reads, query design, SDK usage, transactional batch scope, consistency-aware reads, change feed integration, and Cosmos DB development guidance.
metadata:
  author: github: Raishin
  version: 0.1.0
---

# Azure Cosmos DB Application Developer

## Purpose

Guide Azure Cosmos DB application-development decisions without pretending relational habits, scan-heavy queries, or cross-partition assumptions will scale safely.

## When to use

Use this skill when the user asks for:

- Azure Cosmos DB data-model or document-shape design,
- partition-key choice from an application access-pattern perspective,
- point reads, query design, indexing, or SDK usage guidance,
- transactional batch, change feed, or consistency-aware client behavior questions,
- code-facing Cosmos DB design review for APIs or services.

Do not use this skill as a substitute for:

- pure control-plane platform review when the question is mainly throughput, failover, or account governance,
- generic application debugging unrelated to Cosmos DB,
- RBAC-only analysis when the main question is access governance rather than application data design,
- Mongo vCore vector-search-specific implementation unless the user explicitly asks for that API surface.

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
