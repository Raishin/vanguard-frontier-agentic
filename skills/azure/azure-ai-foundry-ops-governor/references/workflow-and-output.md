# Workflow and output contract for Azure AI Foundry Ops Governor

## Minimal safe workflow

1. Classify the request: architecture review, RBAC review, quota planning, network isolation, observability, or mutation approval.
2. Load Microsoft Learn evidence through the user's configured documentation MCP. Use official sources before drawing conclusions.
3. Define the scope: Foundry resource, project, connected resource, region, deployment type, and environment. If any are unknown, label the review as partial.
4. Build the evidence table: documentation-based claims, sampled read-only evidence if available, sanitized user evidence, and explicit unknowns.
5. Stress test the design against separation of concerns, least privilege, quota, regional support, network isolation, secret flow, diagnostics, and rollback.
6. Produce a verdict with blockers, safe next actions, and open questions.
7. For any mutation request, require explicit user approval after showing blast radius and rollback.

## Output contract

```markdown
## Verdict
<go | conditional-go | no-go | docs-only advisory>

## Evidence level
- Documentation: <sources used>
- Configured-environment evidence: <sampled / not sampled>
- User-supplied evidence: <present / absent>

## Findings
1. <risk or confirmed control> — Evidence: <docs_only|sampled_read_only|user_supplied|inference>

## Blockers
- <missing proof that prevents stronger conclusion>

## Safe next actions
- <least-privilege, reversible action>

## Open questions
- <only questions required to reduce risk>
```

## When to push back

Push back on requests that ask you to approve production readiness with no current-state evidence, grant broad roles, bypass private networking review, skip quota verification, hide tool mutations, or treat a single project as a full isolation boundary.
