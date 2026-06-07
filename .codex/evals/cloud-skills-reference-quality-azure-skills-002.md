# Eval: Azure skills batch 002 AgentCore-standard quality reset

Date: 2026-06-04
Scope: exactly five Azure skills in stable sorted order:

1. `skills/azure/azure-cosmosdb-platform-operator`
2. `skills/azure/azure-cost-estimation-review`
3. `skills/azure/azure-cost-optimization-governor`
4. `skills/azure/azure-entra-id-specialist`
5. `skills/azure/azure-governance-policy-guardrails`

## Capability criteria

- Each skill has a component-specific operations guide linked from `SKILL.md`.
- Each operations guide includes: `What people get wrong`, `Officially grounded service shape`, `Non-negotiable design rules`, `Minimal safe implementation flow`, `Safe verification targets`, and `When to push back`.
- `official-sources.md` contains current Microsoft Learn URLs plus review implications and evidence-boundary rules.
- `safety-checklist.md` contains concrete service-specific risk gates, mutation boundaries, evidence labels, and minimum safe evidence.
- `workflow-and-output.md` contains a service-specific workflow and final response contract.
- `mcp-and-evidence.md` uses generic Microsoft Learn documentation-MCP language and does not expose environment-specific implementation details.

## Regression criteria

- Versions are bumped consistently to `0.1.2` in `SKILL.md`, `metadata.json`, and `catalog/skills.json` for the five changed skills.
- Skill manifest and asset integrity are regenerated after skill changes.
- Narrow validation gates pass: `validate:skill-schema`, `manifest:check`, `validate:asset-integrity`.
- Full `npm run validate` passes after generated files change.
- No AWS assets are modified.
- Batch docs avoid prohibited internal/server/profile/connector wording.

## Evidence used

Microsoft Learn documentation through the user's configured documentation MCP was used for:

- Azure Cosmos DB reliability, failover, private endpoints, partitioning, throughput, consistency, and Well-Architected guidance.
- Azure pricing calculator, pricing data sources, cost estimation, Cost Management, exports, budgets, Advisor, reservations, and savings plans.
- Microsoft Entra security defaults, Conditional Access, Entra roles/PIM, identity governance, workload identities, and emergency access guidance.
- Azure Policy overview, effects, initiatives, assignment scope, exemptions, remediation, and safe staged rollout guidance.

## Results

Pending final validation after regeneration.

## Final validation result

PASS on 2026-06-04.

- Structural AgentCore-standard grader: PASS.
- Prohibited wording grep for batch 002 docs: PASS.
- `rtk npm run manifest:write`: PASS.
- `rtk python3 tests/validate-asset-integrity.py --write`: PASS.
- `rtk npm run validate:skill-schema`: PASS.
- `rtk npm run manifest:check`: PASS.
- `rtk npm run validate:asset-integrity`: PASS.
- `rtk npm run validate`: PASS.
- AWS asset diff check: PASS, no AWS paths changed.
