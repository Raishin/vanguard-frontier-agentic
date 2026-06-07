# Eval: cloud agents reference quality - Azure agents batch 006

Date: 2026-06-05

## Targets

- [x] azure-platform-automation-devops-agent
- [x] azure-private-endpoint-adoption-planner-agent
- [x] azure-rbac-review-agent
- [x] azure-resilience-bcdr-review-agent
- [x] azure-resource-health-incident-triage-agent

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

PASS. Batch 006 is validated and uncommitted.
