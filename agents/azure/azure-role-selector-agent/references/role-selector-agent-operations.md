# Azure Role Selector operations

> Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state. Do not paste secrets, identifiers, billing exports, or customer data into commands or files.

## What people get wrong

The lazy answer is Owner or Contributor. That is not role selection; it is permission dumping.

## Officially grounded service shape

Microsoft guidance says Azure RBAC defines who can access resources, what actions they can perform, and where they can perform them. Role selection starts with built-in job-function roles, minimal permissions, and the narrowest scope; privileged administrator roles are exceptional. That is the key insight: the role is only half the answer; scope and permanence decide the blast radius.

## Non-negotiable design rules

### 1. Start with required actions, not favorite roles.
### 2. Prefer built-in job-function roles before privileged administrator roles or custom roles.
### 3. Choose the narrowest scope that still covers the task.
### 4. Treat Owner, Contributor, User Access Administrator, and RBAC Administrator as high-risk until justified.
### 5. Never ask for unsanitized tenant, subscription, principal, or customer identity exports.

## Minimal safe implementation flow

1. Classify resource type, required actions, principal type, and duration.
2. Map actions to the smallest built-in role that fits.
3. Select resource, resource-group, subscription, or management-group scope deliberately.
4. Flag privileged administrator roles, direct user assignments, and custom-role needs.
5. Return recommended role, scope, rationale, blockers, and safer alternatives.

## High-risk assumptions to kill

- Contributor is a harmless default.
- A broad scope is acceptable because it is faster.
- A custom role is automatically safer than a built-in role.
- Role assignment permission means the assignment should be made.

## Safe command/code verification targets

- Required management-plane and data-plane actions.
- Built-in role definition and whether it is a job-function or privileged administrator role.
- Scope inheritance, existing assignment overlap, group membership, PIM or time-bound path, and access review evidence.

## When to push back

- The requested role is broader than the required actions.
- Scope is subscription or management-group only for convenience.
- The principal is a user when a group or managed identity is appropriate.
- The request needs privilege assignment but lacks approval and review evidence.
