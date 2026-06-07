# Permission Model: Azure Live PIM JIT Activation Guard

## Identity model

PIM JIT is itself the least-privilege mechanism. The operator holds only an *eligible assignment*
— not a standing active one. Activation is time-bounded, MFA-gated, and audit-logged natively.

Preferred model:
1. Entra ID PIM eligible assignment (never standing active)
2. Maximum activation duration: 1–4 hours for break-glass; 8 hours absolute maximum
3. Require approval for roles with management-group or subscription scope
4. Require justification and ticket reference for all activations

## Custom role — read eligible assignments and submit own activation

```json
{
  "Name": "PIM JIT Activation Operator",
  "IsCustom": true,
  "Description": "Read PIM eligible assignments and submit own activation requests.",
  "Actions": [
    "Microsoft.Authorization/roleEligibilitySchedules/read",
    "Microsoft.Authorization/roleEligibilityScheduleRequests/read",
    "Microsoft.Authorization/roleAssignmentSchedules/read",
    "Microsoft.Authorization/roleAssignmentScheduleRequests/write",
    "Microsoft.Authorization/roleAssignments/read"
  ],
  "NotActions": [],
  "AssignableScopes": [
    "$APPROVED_AZURE_SCOPE"
  ]
}
```

`roleAssignmentScheduleRequests/write` only allows a principal to activate their **own**
eligible assignment. It does not allow activating another user's role.

## Recommended PIM settings (configure in Entra portal or Graph API)

- Maximum activation duration: 8 hours
- Require MFA on activation: **Yes**
- Require justification: **Yes**
- Require ticket information: **Yes** (link to change management system)
- Require approval for: Owner, User Access Administrator, Global Administrator
- Notification on activation: send to security team distribution list

## Graceful degradation (tenants without P2 license)

Use Conditional Access + time-bounded Entra ID Group membership via Access Packages
(Entra ID Governance) as the nearest equivalent to PIM eligible assignment.

## Do not assign

- Standing `Owner` at subscription scope
- Standing `User Access Administrator` (allows arbitrary role assignments)
- `Microsoft.Authorization/roleAssignments/write` to non-PIM principals

Use exact resource scopes from approved change records; do not paste raw subscription identifiers into chat.
