# Permission Model: Azure Live Entra Role Assignment Guard

## Risk classification by role

| Role | Risk | Reason |
|---|---|---|
| Owner | Critical | Full resource control + can reassign access |
| User Access Administrator | Critical | Can assign any role to any principal at scope |
| Contributor | High | Full resource read/write, no access management |
| Global Administrator | Critical | Tenant-wide Entra ID control, bypasses RBAC |
| Privileged Role Administrator | Critical | Can assign Entra directory roles including Global Admin |
| Application Administrator | High | Can create service principals and grant Graph API permissions |
| Custom roles with `*/write` | High | Broad mutation rights — review assignable scopes |
| Reader | Low | Read-only — acceptable for most principals |
| Narrow built-in roles | Low | e.g. Storage Blob Data Reader, Key Vault Secrets User |

## Risk classification by scope

| Scope | Risk |
|---|---|
| Management group | Critical — affects all child subscriptions and resource groups |
| Subscription | High — affects all resources in the subscription |
| Resource group | Medium — contained to group members |
| Individual resource | Low — minimal blast radius |

## Risk classification by principal type

| Principal type | Risk | Notes |
|---|---|---|
| Guest user (`userType: Guest`) | Critical | External identity, not governed by corporate IdP; highest breach risk |
| Member user | Medium | Internal — verify employment status and team ownership |
| Service principal (application) | High | Non-human identity; verify application ownership and client secret rotation policy |
| Managed identity (system-assigned) | Low-Medium | Scoped to a resource lifecycle; verify the resource owner |
| Managed identity (user-assigned) | Medium | Shared across resources; verify all attached resources |
| Group | Medium | Verify group membership is actively governed; avoid open groups |

## Least-privilege guidance

1. **Prefer PIM eligible assignments over permanent.** If the role is needed periodically, PIM with time-bounded activation + MFA + justification is always the correct approach.
2. **Prefer narrow built-in roles over Contributor/Owner.** Azure has 200+ built-in roles; check whether a service-specific role (e.g. `Monitoring Contributor`, `Key Vault Secrets Officer`) satisfies the requirement.
3. **Prefer resource-group scope over subscription scope.** Subscription scope is justified only for infrastructure, platform, or governance roles.
4. **Prefer group-based assignment over direct user assignment.** Groups enable consistent access reviews and offboarding.

## Minimum caller permissions for role assignment operations

```json
{
  "Name": "Role Assignment Operator (Guarded)",
  "IsCustom": true,
  "Description": "Read role assignments and create new ones at resource-group or lower scope only.",
  "Actions": [
    "Microsoft.Authorization/roleAssignments/read",
    "Microsoft.Authorization/roleAssignments/write",
    "Microsoft.Authorization/roleAssignments/delete",
    "Microsoft.Authorization/roleDefinitions/read"
  ],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>"
  ]
}
```

Restrict `AssignableScopes` to resource-group scope for operators who should not assign at subscription level.

## Dangerous combinations — always block

- Owner at management-group scope assigned to a Guest principal
- User Access Administrator at subscription scope (allows re-elevating to Owner)
- Any Entra directory role (Global Admin, Privileged Role Admin) assigned outside of PIM
- Service principal with Owner and no owner/contact defined in application registration
