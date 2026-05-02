# Official Sources

Load these only when needed:

- [Azure RBAC overview](https://learn.microsoft.com/en-us/azure/role-based-access-control/overview) — use for role assignment model, scope hierarchy (management group → subscription → resource group → resource), and security principal types.
- [Best practices for Azure RBAC](https://learn.microsoft.com/en-us/azure/role-based-access-control/best-practices) — use for least privilege, group-based assignment, PIM preference, limiting Owner and UAA, and stable role ID usage.
- [Azure built-in roles](https://learn.microsoft.com/en-us/azure/role-based-access-control/built-in-roles) — use when checking whether a narrow built-in role satisfies the requirement before recommending Contributor or Owner.
- [Alert on privileged role assignments](https://learn.microsoft.com/en-us/azure/role-based-access-control/role-assignments-alert) — use for the Kusto query pattern to detect Owner / Contributor / UAA assignment events in Activity Log.
- [Entra ID PIM overview](https://learn.microsoft.com/en-us/entra/id-governance/privileged-identity-management/pim-configure) — use when the permanent assignment request should instead use PIM eligible assignment with JIT activation.
- [az role assignment CLI reference](https://learn.microsoft.com/en-us/cli/azure/role/assignment) — use for exact `az role assignment create`, `list`, `delete` syntax and parameter options.
- [Understand role assignments](https://learn.microsoft.com/en-us/azure/role-based-access-control/role-assignments) — use for role assignment object structure (name, roleDefinitionId, principalId, principalType, scope, condition).

## Grounded insights worth carrying into the skill

- The Azure RBAC API version for role assignments is `2022-04-01` (`Microsoft.Authorization/roleAssignments`).
- Dangerous role definition IDs (stable — never rename): Owner `8e3af657-a8ff-443c-a75c-2fe8c4bcb635`, Contributor `b24988ac-6180-42a0-ab88-20f7382dd24c`, User Access Administrator `18d7d88d-d35e-4fb5-a5c3-7773c20a72d9`.
- A permanent role assignment at subscription scope granted to a Guest user is one of the most common post-breach persistence techniques in Azure tenants — always block without explicit CISO-level sign-off.
- Azure AD token caching means a deleted assignment may still be honored for up to 5 minutes after deletion; do not declare rollback complete immediately.
- `Microsoft.Authorization/roleAssignments/write` at subscription scope is the permission that enables all downstream privilege escalation — any principal with it can assign themselves Owner.
- Prefer `az role assignment list --include-inherited` to find assignments at parent scopes that affect the target resource.
- Microsoft recommends group-based role assignment over direct user assignment to simplify access reviews and offboarding.
