# Azure Role Selection Operations

> Version note: Azure service behavior and tooling change over time. Verify exact command syntax, permissions, and feature availability against Microsoft Learn documentation through the user's configured documentation MCP before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Starting from Owner or Contributor instead of required actions.
- Ignoring the difference between control-plane Actions and data-plane DataActions.
- Using a custom role when a narrower built-in role already fits.
- Assigning at subscription scope when resource or resource-group scope is enough.
- Using wildcard custom-role permissions that can expand with future provider operations.

## Officially grounded service shape

- Microsoft Learn evidence says Azure built-in roles expose Actions, NotActions, DataActions, and NotDataActions so reviewers can compare required permissions against role definitions.
- Role selection guidance says start with job-function roles, review service categories, choose the most restrictive role, and create a custom role only when no suitable built-in role exists.
- Control-plane authorization is handled through Azure Resource Manager, while data-plane authorization is handled by a resource provider or Azure Resource Manager depending on service behavior.
- Custom roles can include data actions, but roles with DataActions cannot be assigned at management group scope. Microsoft recommends explicit permissions instead of wildcards.

Documentation evidence proves documented Azure service behavior. It does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.

## Non-negotiable design rules

- Start from required operations, target resource, principal type, and duration.
- Separate management-plane and data-plane access before selecting a role.
- Prefer built-in job-function roles and narrow scope before custom roles.
- Use custom roles only with explicit Actions/DataActions and documented assignable scopes.
- Require a removal, expiry, or review path for the assignment.

## Minimal safe implementation flow

- Capture principal, target resource, operations, data access, environment, and duration.
- Map operations to resource provider actions and service built-in roles.
- Choose the narrowest role and scope; add data-plane role separately if required.
- If no built-in role fits, design an explicit custom role with narrow assignable scopes.
- Return selected role, scope, evidence level, risks, validation checks, and cleanup path.

## Safe verification targets

- Role definition contains only needed Actions/DataActions for the task.
- Assignment scope is no broader than the named resource set.
- Privileged administrator roles are avoided or explicitly justified.
- Custom role avoids wildcard permissions and has bounded assignable scopes.
- Access can be tested and removed without broad collateral impact.

## When to push back

- The user asks for Owner/Contributor because it is faster.
- The request mixes management-plane and data-plane access without naming both.
- The custom role depends on wildcard permissions.
- The target scope is broader than the requested task.
