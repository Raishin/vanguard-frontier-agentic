# Cloud Marketplace Taxonomy

## Providers

- `aws`
- `azure`
- `oracle`
- `oci`
- `gcp`
- `kubernetes`
- `terraform`
- `multi-cloud`
- `generic`

## Asset types

- `skill`: workflow instructions for a recurring task.
- `agent`: role/persona definition with responsibilities and review behavior.
- `rule`: harness-specific operating guidance.
- `mcp-reference`: catalog entry for an MCP server or MCP setup path.

## Harnesses

- `codex`
- `copilot`
- `claude-code`
- `cursor`
- `gemini`
- `kiro`
- `other`

## Skill categories

Each `SKILL.md` may declare `metadata.category` for marketplace filtering. Categories are coarse and intentionally non-exhaustive — assign the single best fit, not multiple. New categories require a documented rationale and a schema update.

| Category | Scope |
|----------|-------|
| `security` | IAM, posture, secrets, KMS, identity, policy, RBAC, runtime threat, supply chain |
| `networking` | Service mesh, network policy, ingress, segmentation, DNS, private endpoints |
| `platform` | Cluster ops, compute, container platforms, storage, lifecycle automation |
| `data` | Databases, data modeling, query performance, replication, migration |
| `finops` | Cost, anomaly detection, budget, chargeback, optimization |
| `ai` | Generative AI, agents, model platforms, knowledge bases, guardrails |
| `delivery` | CI/CD, release, GitOps, progressive delivery, deployment guards |
| `observability` | Metrics, logs, traces, alerting, SLO, telemetry pipelines |
| `compliance` | Audit, evidence, governance, regulatory mapping |
| `resilience` | Backup, DR, BCDR, restore validation, recovery posture |

## Skill lifecycle

`metadata.lifecycle` declares stability:

- `experimental` — interface unstable, expect breaking changes
- `beta` — externally usable, breaking changes signalled
- `stable` — default for shipped skills; backwards-compatible changes only
- `deprecated` — scheduled for removal; replacement documented

Absence implies `stable`.

## Skill updated date

`metadata.updated` is an ISO 8601 date (`YYYY-MM-DD`) capturing the last meaningful change. "Meaningful" means substantive content, behavior, or contract changes — not whitespace, badge bumps, or transitive metadata churn. Refresh on each substantive edit.

## Trust levels

- **Official**: published by the cloud/provider or official project owner.
- **Community**: public third-party source with clear maintainer and license.
- **Original**: created for this repository.
- **Adapted**: derived from another source and license-reviewed.

Do not blur these categories. A community MCP server that targets AWS is not an official AWS MCP server.
