# Official Sources

Load these only when needed:

- [What is Azure role-based access control (Azure RBAC)?](https://learn.microsoft.com/azure/role-based-access-control/overview) — use for the basic role-assignment model and scope hierarchy.
- [Azure built-in roles](https://learn.microsoft.com/azure/role-based-access-control/built-in-roles) — use as the first stop before proposing any custom role.
- [Understand Azure role definitions](https://learn.microsoft.com/azure/role-based-access-control/role-definitions) — use for control-plane versus data-plane actions and how role permissions are actually expressed.
- [Azure custom roles](https://learn.microsoft.com/azure/role-based-access-control/custom-roles) — use when built-ins do not fit and you need exact constraints on `Actions`, `DataActions`, wildcarding, and assignable scope.
- [Best practices for Azure RBAC](https://learn.microsoft.com/azure/role-based-access-control/best-practices) — use for least privilege, privileged role avoidance, and automation hygiene.
- [Assign Azure roles using Azure CLI](https://learn.microsoft.com/azure/role-based-access-control/role-assignments-cli) — use when the answer must include the permission needed to create role assignments.
- [Azure RBAC tools for the Azure MCP Server overview](https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/azure-rbac) — use to confirm the documented `role` namespace rather than assuming arbitrary RBAC tooling exists.

## Grounded insights worth carrying into the skill

- If a custom role uses `DataActions`, Microsoft documents that it cannot be assigned at management-group scope.
- Microsoft recommends specifying `Actions` and `DataActions` explicitly instead of using `*` wildcards in custom roles.
- Role names can change; role IDs are the safer automation anchor.
- Control plane and data plane are separate authorization paths. A “close enough” role often is not close enough if the wrong plane is involved.
