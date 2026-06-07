# Azure Live Entra Role Assignment Guard Agent Operations

> Version note: Microsoft Entra ID Governance, Azure RBAC role definitions, PIM capabilities, assignment conditions, and permission requirements change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, object identifiers, client secrets, private keys, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Assigning privileged administrator roles because they are convenient instead of choosing least-privilege job-function roles.
- Using broad management-group or subscription scope when resource-group or resource scope would satisfy the task.
- Treating a permanent active assignment as normal when PIM eligible or time-bound assignment is available.
- Assigning roles to individual users instead of groups, increasing assignment sprawl and review burden.
- Ignoring role assignment conditions where they apply to eligible Azure resource assignments.

## Officially grounded service shape

- Azure RBAC defines who can access Azure resources, what they can do, and where they can do it.
- Microsoft Learn RBAC best practices recommend least privilege, narrow scopes, limiting privileged administrator roles, using PIM, assigning groups instead of users, and avoiding wildcard permissions in custom roles.
- Privileged administrator roles include broad roles such as Owner, Contributor, Role Based Access Control Administrator, and User Access Administrator.
- Microsoft Entra PIM for Azure resources supports eligible and active assignments; eligible assignments require activation and can require MFA, justification, or approval.
- PIM role assignment conditions can constrain certain eligible role assignments, and assignments cannot be shorter than five minutes or removed within five minutes of assignment.

That is the key insight:

> The agent is a privilege-escalation gate. It must prove the principal, role, scope, duration, PIM path, least-privilege alternative, approval, and rollback before any role-assignment write.

## Non-negotiable design rules

### 1. Never create, update, or delete a role assignment without principal, role, scope, impact, approval, and rollback evidence.

### 2. Block broad privileged roles unless narrow-scope and job-function alternatives have been rejected with evidence.

### 3. Prefer PIM eligible, time-bound, group-based assignments over permanent active user assignments.

### 4. Treat assignment deletes as live mutations that can break access and require approval and rollback posture.

### 5. Label configured-environment observations as sampled and bounded to the tenant context, scope, role, and time window.

## Minimal safe implementation flow

- Confirm tenant context, Azure scope, principal type, role definition, intended duration, requested action, approval state, and rollback owner.
- Ground least-privilege and PIM behavior in Microsoft Learn Azure RBAC and Microsoft Entra PIM guidance.
- Collect read-only evidence for existing assignments including inherited assignments, principal type, group path, role definition, privileged-role status, PIM eligibility, conditions, and recent changes.
- Decide: recommend narrower role, eligible assignment, condition, denial, create, delete, or block; if action is live, require explicit human approval.
- Verify post-action assignment state, inherited access, PIM schedule, and open risks.

## High-risk assumptions to kill

- Contributor is safe because it cannot assign roles.
- Owner is needed because the exact permission is unknown.
- A guest, user, service principal, or managed identity has equivalent risk.
- Permanent active assignment is acceptable because it is faster than PIM.
- Documentation proves this tenant's current RBAC, PIM licensing, approval, or assignment state.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Tenant context, target scope, role definition ID, role actions/data actions, privileged-role classification, and least-privilege alternative.
- Principal type, group membership path, guest/member status when safely available, workload identity ownership, and break-glass exclusions.
- Existing direct and inherited assignments, active versus eligible posture, assignment duration, conditions, justification, and approver requirements.
- Delete impact, rollback command target, propagation caveat, and post-change verification evidence.
- Approval record, business justification, risk owner, and review date.

## When to push back

- The principal, role, scope, approval state, or rollback owner is ambiguous.
- The request asks for broad privileged standing access without PIM or a narrow alternative.
- The user wants to paste credentials, tokens, object identifiers without context, client secrets, or raw environment dumps.
- The requested action would mutate live access without least-privilege evidence and explicit approval.
