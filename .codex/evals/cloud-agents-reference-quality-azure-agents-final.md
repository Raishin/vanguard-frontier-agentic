# Eval: cloud agents reference quality - Azure agents final singleton

Date: 2026-06-05

## Target

- [x] azure-waf-security-review-agent

## Checks

- [x] Remaining Azure agent singleton processed to avoid exact-five deadlock.
- [x] No AWS asset paths intentionally changed.
- [x] Primary agent docs remain lean and point to local references.
- [x] Target has official-sources, safety-checklist, workflow-and-output, documentation evidence, and component operations references.
- [x] Component operations reference follows the AgentCore section pattern.
- [x] Azure documentation wording is generic and does not expose private tool labels or environment identifiers.
- [x] Documentation-based claims are separated from configured-environment evidence.
- [x] Stale eight-principle framing removed and replaced with current Microsoft Learn-grounded security framing.
- [x] Patch version bumped in AGENT.md frontmatter, metadata.json, and catalog/agents.json.
- [x] Generated plugin, marketplace, Kiro, and asset integrity outputs refreshed.
- [x] Narrow validation gates passed.
- [x] Full npm run validate passed with exit 0.

## Result

PASS. Azure agents are now fully processed and validated, uncommitted.
