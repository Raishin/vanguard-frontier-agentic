# Azure Entra and RBAC Role Assignment Operations

Use this reference for current, source-grounded service behavior and the hard live-operation gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Assigning Owner because Contributor failed without diagnosing the missing permission.
- Granting broad subscription or management-group scope when resource-group or resource scope is enough.
- Creating permanent assignment when PIM eligible access satisfies the need.
- Skipping principal-type checks for guests, service principals, and groups.
- Assuming deletion instantly revokes every cached token.

## Officially grounded service shape

Microsoft Learn evidence says Azure RBAC grants who can access Azure resources, what they can do, and where. Best practices require least privilege, narrow scope, limiting privileged administrator roles, assigning to groups where manageable, and using PIM for just-in-time access. Privileged role assignments such as Owner, Contributor, and User Access Administrator are powerful and can be monitored with alerts; role assignment changes can take time to propagate.

- Azure RBAC assignment combines principal, role definition, and scope.
- Privileged administrator roles create broad blast radius and should be minimized.
- PIM can provide time-bound access for Azure resource roles.
- Alerts can detect privileged role assignment events at subscription scope.
- Propagation and token caching mean assignment or deletion may not be immediately observed.

## Non-negotiable design rules

- Confirm tenant, subscription, management group/resource scope, principal, role, and active caller before write.
- Prefer built-in job-function roles and narrow scopes before privileged administrator roles.
- Require PIM alternative analysis for privileged or temporary need.
- Classify principal type and external/guest risk before approval.
- Provide rollback delete command but state propagation caveats.

## Minimal safe implementation flow

- Scope requested role assignment or deletion and business justification.
- Collect read-only evidence: principal details, existing assignments, target scope, role definition, and PIM eligibility.
- Classify risk by role power, scope breadth, principal type, duration, and blast radius.
- Gate mutation on explicit approval and rollback plan.
- Verify assignment or deletion, alerts/audit trail, and expected propagation window.

## Safe verification targets

- Role is the least privileged role that meets the task.
- Scope is the narrowest practical scope.
- Principal type and owner are known and not an unapproved guest or stale service principal.
- PIM eligible assignment was considered for privileged or temporary access.
- Privileged assignment monitoring or audit evidence exists.

## When to push back

- The assignee identity or scope is ambiguous.
- The request wants permanent privileged access for convenience.
- Guest or broad group assignment lacks documented exception.
- The user demands immediate revocation proof despite propagation limitations.
