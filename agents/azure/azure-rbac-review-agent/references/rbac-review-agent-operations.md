# Azure RBAC Review operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

RBAC risk is usually scope plus privilege plus permanence. A role that looks reasonable at resource scope can be dangerous at subscription or management-group scope.

## Officially grounded service shape

Microsoft guidance defines Azure RBAC as who can access which resources, what actions they can perform, and where they can perform them. Best practices emphasize least privilege, narrow scopes, limited subscription owners, group assignments, built-in roles before custom roles, and PIM for privileged exposure. That is the key insight: role review is a scope review.

## Non-negotiable design rules

1. Prefer built-in job-function roles before custom roles or privileged administrator roles.
2. Assign at the narrowest scope that satisfies the task.
3. Avoid Owner, User Access Administrator, and broad Contributor unless explicitly justified.
4. Prefer group-based assignments over direct user assignments.
5. Treat custom roles as high-risk until actions, dataActions, notActions, assignableScopes, and owners are reviewed.

## Minimal safe implementation flow

1. Classify principal type, role, scope, duration, and business function.
2. Ground Azure RBAC behavior in Microsoft Learn.
3. Review inherited scope, privileged role status, custom role permissions, direct user grants, and stale access.
4. Identify least-privilege replacement or PIM/time-bound path.
5. Return risk verdict, blockers, and safe remediation sequence.

## High-risk assumptions to kill

- Contributor is safe because it is not Owner.
- Subscription scope is acceptable for convenience.
- Custom roles are safer because they are custom.
- Direct user assignments are easier and therefore acceptable.

## Safe command/code verification targets

- Role definition actions, dataActions, notActions, assignableScopes, and privileged administrator status.
- Role assignment principal, scope, inheritance, condition, duration, and owner.
- Access review, PIM, group membership, and break-glass exception evidence.

## When to push back

- Broad scope is justified only by convenience.
- A privileged role is permanent with no PIM or review evidence.
- A custom role includes wildcard permissions without evidence.
- The requester wants unsanitized identity dumps.
