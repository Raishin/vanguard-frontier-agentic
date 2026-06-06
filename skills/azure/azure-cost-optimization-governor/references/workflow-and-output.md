# Workflow and output contract for Azure Cost Optimization Governor

## Minimal safe workflow

1. Classify the request: spend visibility, budget governance, tagging, exports, rightsizing, rate optimization, or optimization approval.
2. Ground the review in Microsoft Learn through the user's configured documentation MCP.
3. Establish scope and evidence: docs only, sanitized cost sample, Advisor recommendation, export schema, or owner-confirmed action.
4. Separate visibility gaps from savings opportunities.
5. Rank opportunities: waste removal, rightsizing, scheduling, storage/log retention, reservations, savings plans, and architecture changes.
6. For each action, require owner, blast radius, rollback, and measurement method.
7. Return potential savings carefully, never as guaranteed realized savings.

## Output contract

```markdown
## Verdict
<credible governance | conditional | high-risk | docs-only advisory>

## Evidence level
- Documentation: <sources used>
- Cost evidence: <cost_sample|owner_confirmed|not sampled>

## Findings
1. <finding> — Evidence: <docs_only|cost_sample|owner_confirmed|inference>

## Optimization backlog
| Priority | Action | Evidence needed | Risk | Owner |
| --- | --- | --- | --- | --- |

## Guardrails
- <budget/export/tag/policy control>

## Safe next actions
- <least-risk next step>
```

## Pushback triggers

Push back on unowned recommendations, immediate deletion, blind commitment purchases, unsanitized billing data, savings guarantees, or optimization plans without before/after measurement.
