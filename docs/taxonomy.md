# Cloud Marketplace Taxonomy

## Providers

- `aws`
- `azure`
- `oracle`
- `oci`
- `gcp`
- `alibaba`
- `huawei`
- `ovhcloud`
- `ionos`
- `scaleway`
- `hetzner`
- `contabo`
- `kubernetes`
- `terraform`
- `multi-cloud`
- `generic`
- `marketing`

## Language and stack boards

`provider` is a cloud/platform axis. Language- or runtime-scoped boards (for
example the `.NET` board under `agents/dotnet/` and `skills/dotnet/`) are not
cloud providers and therefore do not get a `provider` enum value. They use
`provider: generic` and a shared ID prefix (`dotnet-*`) plus a dedicated
topical directory — the same pattern as the non-cloud `hr`, `qa`, `legal`, and
`marketing` boards. A language/stack faceting axis is a deferred design item;
if it is introduced, prefixed assets migrate to it without an ID change.

See `docs/language-stack-boards.md` for detailed guidance on language/stack boards,
including how to add new ones, discovery via install roles, and the trust posture
of static-review agents.

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
