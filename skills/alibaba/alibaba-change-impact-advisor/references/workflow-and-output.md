# Workflow and output contract

Use this reference only when performing a full change impact analysis, blast radius assessment, or safe change sequencing review.

## Analysis domains

Check these areas before giving a recommendation:

- Change scope: target resources, account context (CN-* vs international), and change window
- Resource Directory OU membership and all affected member accounts
- RAM policy cascade: cross-account STS AssumeRole callers via ActionTrail
- VPC peering topology: full mesh map, non-transitive connectivity implications
- CEN route table changes: global propagation scope and staging validation status
- SLB backend pool: live traffic impact, connection drain plan, blue/green readiness
- RDS connection pool: restart requirements, connection drain, client reconnect logic
- Rollback plan: point-in-time restore, policy version revert, route table rollback

## Safe workflow

1. **Frame the change**
   - Change type and target resources:
   - Account context (CN-* vs international):
   - Planned change window:
   - Explicit rollback plan:
2. **Collect evidence**
   - Prefer live console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the blast radius**
   - Which Resource Directory OUs and member accounts are affected?
   - Which cross-account RAM roles and STS callers depend on the changed policy?
   - Which VPC peering connections or CEN attachments are in the blast radius?
   - Which SLB listeners and backend pools serve live traffic through the affected resources?
   - What evidence is missing?
4. **Recommend the smallest safe change sequence**
   - Prefer staged rollout over atomic change.
   - If the safest action is to gather more evidence, say that plainly.
   - Require explicit approval gate before each irreversible step.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Change Impact Analysis: <change description>
## Executive summary
- Change verdict (proceed / hold / reject):
- Evidence level:
- Blast radius summary:
## Change description
- Target resources:
- Account context:
- Change window:
## Resource Directory OU scope
- Affected OUs:
- Affected member accounts:
- Approval gate required:
## RAM policy cascade
- Modified policies:
- Cross-account STS callers affected:
- Downstream services at risk:
## VPC/CEN network topology impact
- VPC peering connections in blast radius:
- CEN route table propagation scope:
- Staging validation status:
## Application dependency impact
- SLB backend pool changes:
- RDS connection pool disruption:
- Connection drain plan:
## Safe change sequence
1. <step> — gate: <approval requirement>
## Rollback plan
- Rollback trigger:
- Rollback steps:
- Estimated rollback time:
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
