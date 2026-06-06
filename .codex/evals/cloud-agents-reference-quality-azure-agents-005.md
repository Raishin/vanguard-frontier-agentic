# Eval: cloud agents reference quality - Azure agents batch 005

Date: 2026-06-05

## Targets

- [x] azure-live-pim-jit-activation-guard-agent
- [x] azure-maestro-agent
- [x] azure-migrate-landing-zone-cutover-agent
- [x] azure-network-topology-review-agent
- [x] azure-observability-investigator-agent

## Checks

- [x] Exactly five Azure agent directories processed.
- [x] No AWS asset paths intentionally changed.
- [x] Primary agent docs remain lean and point to local references.
- [x] Each target has official-sources, safety-checklist, workflow-and-output, documentation evidence, and component operations references.
- [x] Azure documentation wording is generic and does not expose private tool labels or environment identifiers.
- [x] Documentation-based claims are separated from configured-environment evidence.
- [x] Patch versions bumped in AGENT.md frontmatter, metadata.json, and catalog/agents.json.
- [x] Generated plugin, marketplace, Kiro, and asset integrity outputs refreshed.
- [x] Narrow validation gates passed.
- [x] Full npm run validate passed with exit 0.

## Result

PASS. Batch 005 is validated and uncommitted.
