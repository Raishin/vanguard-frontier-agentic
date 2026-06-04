# Azure Identity Governance Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Treating PIM enablement as proof that privileged access is governed.
- Accepting permanent administrator assignments without activation, approval, expiration, and access-review evidence.
- Creating access packages with no resource owner, stale reviewer, or never-expiring assignment policy.
- Ignoring emergency access accounts until a lockout occurs.
- Claiming tenant compliance from documentation alone.

## Officially grounded service shape

Microsoft Learn evidence says Entra ID Governance covers entitlement management, access reviews, lifecycle workflows, and PIM. The operations guide requires task owners, testing strategy, regular reviews for applications, external identities and privileged roles, emergency access accounts, and entitlement management. Least-privilege guidance points to feature-specific administrative roles and JIT role activation through PIM.

- Identity Governance is a lifecycle control set, not a one-time role cleanup.
- Access reviews apply to groups, applications, role assignments, access packages, and external identities when the right licensing and scope exist.
- PIM supports just-in-time privileged access, but role settings, approvers, MFA, activation duration, and review cadence decide whether it is safe.
- Entitlement management uses catalogs, access packages, policies, approvals, assignment duration, and review settings; each layer needs ownership.
- Emergency access accounts are intentionally exceptional and must be protected, monitored, and periodically tested.

## Non-negotiable design rules

- Inventory standing privileged assignments before praising governance maturity.
- Require owner, reviewer, cadence, action-on-denial, and expiration for each governed access path.
- Prefer eligible JIT assignments for privileged roles and narrow scope before custom exceptions.
- Separate human operator access, workload identity access, external-user access, and break-glass access.
- Label unqueried tenant state as unverified; documentation only proves product behavior.

## Minimal safe implementation flow

- Scope the tenant, administrative planes, critical roles, external access paths, and access-package catalogs.
- Collect documentation-grounded expected controls, then gather sampled current-state evidence if available.
- Compare permanent assignments, PIM settings, access review schedules, owner coverage, and expiration posture.
- Rank gaps by blast radius: Global Administrator, Privileged Role Administrator, subscription Owner/User Access Administrator, external privileged access, and unowned packages first.
- Return blockers, safe next actions, and explicit unknowns without requesting secrets or tenant identifiers in chat.

## Safe verification targets

- Role assignment inventory distinguishes active, eligible, permanent, group-based, and direct assignments.
- PIM settings show activation duration, approval/MFA requirements, and notification/audit configuration for privileged roles.
- Access reviews have owners, recurrence, scope, reviewer selection, and automatic action behavior.
- Access packages have business owners, assignment expiration, approval policy, and review settings.
- Emergency access accounts are cloud-only, monitored, excluded from risky dependencies only where justified, and tested.

## When to push back

- The user asks for a compliant verdict without role, PIM, review, and owner evidence.
- A design depends on shared permanent administrator groups.
- Reviewers are the same people whose access is being reviewed with no compensating control.
- Break-glass accounts are missing, weakly monitored, or used for routine operations.
