# Azure RBAC Review Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Giving Owner because the exact job-function role was not checked.
- Assigning broad roles at subscription or management-group scope for convenience.
- Creating wildcard custom roles that silently inherit future permissions.
- Assigning roles directly to users instead of groups or managed identities.
- Treating RBAC as static and ignoring PIM, reviews, and privileged assignment governance.

## Officially grounded service shape

- Microsoft Learn evidence says Azure RBAC defines who can access Azure resources, what actions they can perform, and where they can perform them.
- Best practices emphasize least privilege, narrow scope, limiting subscription owners, limiting privileged administrator roles, PIM for time-bound access, assigning roles to groups, using unique role IDs in automation, and avoiding wildcard custom-role permissions.
- Privileged administrator roles require special scrutiny; where role assignment delegation is needed, conditions can constrain what assignees may grant.
- Built-in job-function roles should be preferred before custom roles, and custom roles should specify explicit Actions and DataActions.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, incident state, or production readiness.

## Non-negotiable design rules

- Start from task-required actions and scope, not from a desired role name.
- Prefer built-in job-function roles and narrow scopes before custom roles or privileged administrator roles.
- Use groups or managed identities instead of direct user assignments where operationally possible.
- Use PIM or time-bound access for elevated privileges.
- Reject wildcard custom roles unless there is a documented, exceptional justification and compensating review.

## Minimal safe implementation flow

- Scope principal, resource, required actions, data-plane needs, assignment duration, and approval path.
- Inventory existing assignments and inherited assignments at management group, subscription, resource group, and resource scopes.
- Compare required actions to built-in roles, then custom role only if necessary.
- Assess privileged role, PIM, condition, group assignment, and review requirements.
- Return least-privilege recommendation, risks, rollback/removal plan, and verification query targets.

## High-risk assumptions to kill

- Owner or Contributor is not a default troubleshooting role; job-function built-in roles and narrower scopes must be checked first.
- A role assignment at management-group or subscription scope multiplies blast radius even if the principal only needs one resource.
- Direct user assignments and standing privileged access are operational debt unless there is a documented exception.
- Custom roles with wildcards can silently gain future permissions and are not least privilege.
- RBAC evidence is incomplete unless inherited assignments, data-plane permissions, PIM eligibility, conditions, and review cadence are considered.

## Safe command/code verification targets

- Inventory direct and inherited assignments for the principal at management group, subscription, resource group, and resource scopes.
- Compare required management-plane and data-plane actions against built-in job-function roles before custom role design.
- Check privileged administrator roles, assignment conditions, PIM/time-bound controls, group-based assignment, and access review evidence.
- Verify custom role definitions use explicit Actions/DataActions and stable role IDs for automation.
- Provide removal/expiry verification so access can be cleanly revoked after the task.

## Safe verification targets

- Chosen role grants only required management-plane and data-plane actions.
- Assignment scope is the narrowest workable scope.
- Privileged administrator roles are absent or explicitly justified with PIM/time-bound controls.
- Custom roles avoid wildcards and use explicit Actions/DataActions.
- Role assignment can be removed or expires cleanly after the task.

## When to push back

- The user asks for Owner without proving why narrower roles fail.
- The request grants permissions to an individual user for a standing operational pattern.
- The custom role uses wildcard permissions.
- The target scope is broader than the resource set named in the task.
