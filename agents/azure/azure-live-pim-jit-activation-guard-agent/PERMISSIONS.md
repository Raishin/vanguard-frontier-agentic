# Permissions: Azure Live PIM JIT Activation Guard

# Least-privilege RBAC guidance for PIM JIT operations

## Identity model

PIM JIT is itself the least-privilege mechanism. The operator holds only an *eligible assignment*
— not an active one. Activation is time-bounded, MFA-gated, and audit-logged natively.

Preferred order:
1. Entra ID PIM eligible assignment (not standing active)
2. Time-bound maximum activation duration: 1–4 hours for break-glass, 8 hours maximum
3. Require approval for roles with management-group or subscription scope
4. Require justification and ticket reference for all activations

## Custom role to read eligible assignments and submit own activation

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
    "/subscriptions/<SUBSCRIPTION_ID>"
  ]
}
```

Note: `roleAssignmentScheduleRequests/write` only allows a principal to activate their *own*
eligible assignment. It does not allow activating another user's role.

## Recommended PIM role settings (configure in Entra portal or Graph API)

- Maximum activation duration: 8 hours
- Require MFA on activation: **Yes**
- Require justification: **Yes**
- Require ticket information: **Yes** (link to change management system)
- Require approval for: Owner, User Access Administrator, Global Administrator
- Notification on activation: send to security team DL

## Graceful degradation (tenants without P2 license)

Without PIM, use Conditional Access + Azure AD Group membership with time-bounded
group assignment via Access Packages (Entra ID Governance) as the nearest equivalent.

## Do not assign

- Standing `Owner` at subscription scope
- Standing `User Access Administrator` (allows arbitrary role assignments)
- `Microsoft.Authorization/roleAssignments/write` to non-PIM principals

