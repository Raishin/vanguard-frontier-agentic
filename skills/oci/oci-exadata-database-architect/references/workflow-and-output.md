# Workflow and output contract

Use this reference for full OCI Exadata Database Architect reviews.

## Workflow

1. **Classify scope** — domain, environment, owner, resource boundary, requested action, and production impact.
2. **Ground in official docs** — use official OCI documentation for current service behavior and caveats.
3. **Collect read-only evidence when available** — use OCI API evidence through the user's configured read-only OCI MCP for sampled API shape or current-state observations.
4. **Stress test the plan** — identify broad permissions, destructive actions, unsupported assumptions, missing rollback, and missing owner validation.
5. **Prioritize safe next actions** — prefer reversible discovery and staged changes before mutation.

## Output contract

Return:

1. Scoped target and evidence level
2. Verdict: ready, ready with risks, or not ready
3. Top blockers
4. Findings with severity, evidence label, recommendation, and owner
5. Safe next actions in priority order
6. Required approvals for any mutation
7. Open questions and assumptions
8. Official-source notes and sampled-evidence caveats
