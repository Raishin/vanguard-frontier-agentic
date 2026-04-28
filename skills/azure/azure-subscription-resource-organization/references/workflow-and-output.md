# Workflow and Output Contract

## Safe Workflow

1. **Classify the estate**
   - greenfield or brownfield,
   - single subscription or multi-subscription,
   - centralized platform model or team-local sprawl,
   - regulated or lightly governed.
2. **Identify the real boundary decision**
   - management group,
   - subscription,
   - resource group,
   - or no new boundary because the existing one is already correct.
3. **Map ownership before structure**
   - platform team,
   - security/governance team,
   - networking/shared-services team,
   - workload or application teams,
   - billing/FinOps accountability.
4. **Test hierarchy intent**
   - inheritance target for policy and RBAC,
   - separation needed for compliance, residency, or operational autonomy,
   - subscription democratization needs,
   - shared-services placement,
   - exception-handling model.
5. **Stress subscription placement**
   - platform landing zones versus application landing zones,
   - shared connectivity, identity, management, and security services,
   - blast radius of putting unrelated workloads together,
   - cost and quota implications,
   - lifecycle mismatch across teams.
6. **Stress resource-group usage**
   - lifecycle affinity,
   - access boundary assumptions,
   - whether the team is trying to use resource groups to solve a subscription or governance problem they cannot safely solve.
7. **Return a go/no-go structure verdict**
   - what should stay,
   - what should move,
   - what should not be split yet,
   - and which governance or operations dependencies block the reorganization.

## Role-Specific Stress Checks

- Reject “one big subscription is simpler” when multiple teams, environments, workloads, or control boundaries clearly differ.
- Reject “just mirror the org chart” as a management-group strategy. Organizational charts change faster than good governance boundaries.
- Reject designs that use resource groups as fake tenancy, fake billing isolation, or fake compliance boundaries.
- Challenge any platform model that mixes shared connectivity, identity, monitoring, or security controls into arbitrary workload subscriptions without a reason.
- Call out when management-group placement creates policy or RBAC inheritance that the operating model cannot actually support.
- Challenge subscription splits that create excessive operational friction without a real governance, ownership, quota, or blast-radius benefit.
- Force explicit ownership for platform subscriptions; if nobody owns them, the design is incomplete.

## Output Template

```markdown
# Azure Resource Organization Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Primary boundary decision:
- Biggest organizational risk:
- Evidence level: live evidence / documentation-based / user-provided sanitized evidence / inference

## Current or proposed structure
- Tenant/platform context:
- Management-group shape:
- Subscription model:
- Resource-group pattern:
- Platform owners:
- Workload owners:

## Findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Boundary decisions
| Decision | Best boundary | Why | Governance impact | Operations impact |
|---|---|---|---|---|

## Safe next actions
1.
2.
3.

## Assumptions and unknowns
- 
```

## Red Flags

- The request asks for management-group hierarchy advice without naming who owns governance, policy, or shared services.
- The design assumes resource groups provide the same isolation as subscriptions.
- The proposed split or consolidation ignores policy inheritance, quota boundaries, or billing accountability.
- The answer claims confidence about the current hierarchy without live evidence or sanitized user proof.
- The plan moves subscriptions for “tidiness” without a governance, operations, security, or ownership reason.
