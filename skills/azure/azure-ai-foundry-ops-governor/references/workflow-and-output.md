# Workflow and Output Contract

## Safe Workflow

1. **Classify the boundary**
   - Identify tenant, subscription, resource group, Foundry resource, and project scope.
   - Decide whether the question is resource-governance, project-operations, or connected-resource governance.

2. **Verify resource vs project fit**
   - Check whether the workload can live at project scope.
   - If a required API or capability stays at parent resource scope, do not fake project isolation; recommend separate Foundry resources where justified.

3. **Review RBAC and identity model**
   - Prefer Entra ID and least privilege.
   - Separate admin, project manager, and project user responsibilities.
   - Challenge key-based access because Microsoft documents that keys bypass RBAC restrictions.

4. **Review quota and deployment posture**
   - Check current model quotas, deployment counts, rate limits, and cross-team contention risk.
   - Distinguish experimentation quota from production quota.
   - Call out 429 risk, PTU saturation risk, and quota ownership gaps.

5. **Review network isolation and connected resources**
   - Separate inbound access, outbound access from Foundry, and outbound access from agents.
   - Check whether private endpoints, DNS, and dependent resources are part of the design.
   - Remember Storage, Key Vault, and Azure AI Search keep their own governance boundaries.

6. **Review logging and monitoring**
   - Check metrics, audit logs, request/response logs, trace logs, and diagnostic settings.
   - Call out when monitoring exists only on paper and not in routed diagnostics.

7. **Gate MCP execution safety**
   - Read-only discovery first.
   - Mutations only after target, blast radius, approval path, and rollback are explicit.
   - Default production changes to staged or nonproduction-first unless the user clearly authorizes otherwise.

8. **Return a go / no-go verdict**
   - Summarize blockers, residual risks, and the safest next actions.

## Role-Specific Stress Checks

- Are you confusing the Foundry resource governance boundary with the project development boundary?
- Are you assuming project-scoped RBAC works for APIs that still require parent resource scope?
- Are you recommending key-based auth even though it bypasses RBAC restrictions?
- Are you proposing private access without DNS, Private Link, dependent-resource, or agent-network implications?
- Are you claiming observability exists without diagnostic settings and actual destinations?
- Are you proposing MCP writes without confirming preview status, public-endpoint limitations, rollback, and environment scope?
- Are you treating quota as a one-team problem when multiple projects share the same resource-level constraints?

## Output Template

```markdown
# Foundry Operations Governance Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest risk:
- Evidence level: live evidence / documentation-based / sanitized user evidence / inference

## Scope
- Subscription / resource group:
- Foundry resource:
- Project(s):
- Environment: dev / test / prod
- Requested action:

## Boundary decision
- Resource-level controls:
- Project-level controls:
- Connected resources requiring separate governance:
- Unsupported or parent-scope-only capabilities:

## Findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Safe execution plan
1.
2.
3.

## Rollout blockers
- 

## Open questions
- 
```

## Red Flags

- One Foundry resource is being used as a catch-all for unrelated teams with no cost, quota, or access ownership.
- The design assumes projects alone provide full isolation even when required APIs operate at parent resource scope.
- The plan uses key-based auth for convenience instead of Entra ID and scoped RBAC.
- Private networking is requested, but no one owns private DNS, private endpoints, or connected-resource approvals.
- Diagnostics are called mandatory, but no Log Analytics, retention, or alert ownership exists.
- MCP mutation is proposed directly against production without a staged validation path.
