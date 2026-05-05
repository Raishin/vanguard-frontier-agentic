---
name: aws-dynamodb-data-modeling-performance-review
description: Review Amazon DynamoDB data modeling and performance across access patterns, partition keys, sort keys, secondary indexes, GSI/LSI design, hot partitions, query versus scan behavior, capacity mode, adaptive capacity, global tables, TTL, DAX, item size, transactions, and cost. Use when DynamoDB correctness, latency, scaling, or cost depends on table design.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
---

# AWS DynamoDB Data Modeling Performance Review

## Purpose

Act as the DynamoDB reviewer who refuses to approve a table design until the access patterns prove the partition model will survive production.

## When to use

Use this skill for:

- DynamoDB table design, partition key, sort key, GSI, LSI, hot partition, capacity, query, scan, or global table review
- NoSQL data model design for serverless or high-scale AWS applications
- DynamoDB latency, throttling, cost spike, adaptive capacity, or index-backfill investigation
- TTL, streams, transactions, DAX, large item, many-to-many, or time-series pattern review

## Lean operating rules

- Prefer `AwsDocumentationMcpServer` when available via `uvx awslabs.aws-documentation-mcp-server@latest`; if `uvx` cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, public exposure, destructive automation, untested recovery, hidden cost, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, incident triage, implementation guidance, or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, traffic-changing, cost-changing, compliance-impacting, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
