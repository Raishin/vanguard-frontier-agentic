# Workflow and Output Contract

## Safe Workflow

1. **Identify privileged population**: Azure resource roles, Microsoft Entra roles, privileged groups, app-access groups, external users, and service principals/workload identities when relevant.
2. **Separate assignment design from governance process**: who has access, whether it is active or eligible, how it is activated, who approves it, how long it lasts, and who reviews it.
3. **Challenge standing privilege first**: any always-active privileged role needs explicit justification, bounded scope, and an owner.
4. **Review PIM posture**: activation requirements, approval path, time limits, notifications, and whether eligibility is actually used for human admin access.
5. **Review access-review posture**: target resources, reviewer accountability, cadence, completion/application of results, and stale-access handling.
6. **Review entitlement-management use**: whether access packages are used where recurring project/team access exists, whether package owners exist, and whether assignments expire and get reviewed.
7. **Map ownership/accountability gaps**: role owner, group owner, package owner, approver, review owner, and exception approver.
8. **Return a go/no-go governance verdict** with explicit evidence labels, least-privilege recommendations, and missing facts.

## Role-Specific Stress Checks

- PIM does not fix bad scope design. Eligible `Owner` at subscription scope can still be reckless.
- Access reviews that never apply removals are theater, not governance.
- Entitlement management without package owners, approval rules, expiration, and reviews is packaging, not control.
- Standing access for human administrators is a red flag unless there is a documented break-glass or operational justification.
- Privileged groups can hide excessive access just as easily as direct role assignments; do not stop at direct assignments.
- “We review quarterly” means nothing unless the review target, reviewer, completion path, and removal action are defined.
- Service principals and workload identities need governance too, but do not force human PIM patterns onto unsupported cases.

## Output Template

```markdown
# Azure Identity Governance Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest governance gap:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Tenant or hierarchy boundary:
- Privileged population reviewed:
- Requested outcome:
- Review owner:

## Current privilege model
| Area | Current state | Risk |
|---|---|---|
| Standing vs eligible access |  |  |
| PIM posture |  |  |
| Access reviews |  |  |
| Entitlement management |  |  |
| Ownership/accountability |  |  |

## Findings
| Finding | Severity | Evidence | Why it matters | Recommendation | Owner |
|---|---|---|---|---|---|

## Least-privilege governance pattern
- Human privileged access:
- Workload or service access:
- Review cadence:
- Approval model:
- Expiration model:

## Safe next actions
1.
2.
3.

## Open questions
- 
```

## Red Flags

- Permanent `Owner`, `Contributor`, `User Access Administrator`, or high-privilege Entra role assignments for normal operator work.
- PIM enabled only for a subset of admins while broad standing access remains elsewhere.
- Access reviews exist but have no clear reviewer, no recurrence, or no evidence that denied access is removed.
- Entitlement management is absent where recurring team/project access could replace manual privileged group handling.
- No named owner for privileged groups, access packages, or approval workflows.
- Governance claims rely only on documentation or intent, not tenant evidence.
