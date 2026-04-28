# Official Sources

Load these only when needed:

- [What is Azure role-based access control (Azure RBAC)?](https://learn.microsoft.com/azure/role-based-access-control/overview) — use for role assignment fundamentals, scopes, role definitions, and security principal types.
- [Best practices for Azure RBAC](https://learn.microsoft.com/azure/role-based-access-control/best-practices) — use for least privilege, limiting privileged roles, PIM, group-based assignment, stable role IDs, and wildcard cautions.
- [Understand Azure role definitions](https://learn.microsoft.com/azure/role-based-access-control/role-definitions) — use for `Actions`, `DataActions`, assignable scopes, and control-plane versus data-plane separation.
- [Azure built-in roles](https://learn.microsoft.com/azure/role-based-access-control/built-in-roles) — use when checking whether a built-in role already fits before inventing a custom one.
- [Azure custom roles](https://learn.microsoft.com/azure/role-based-access-control/custom-roles) — use when built-ins fail and you need exact constraints on wildcarding and assignable scopes.
- [Azure roles, Microsoft Entra roles, and classic subscription administrator roles](https://learn.microsoft.com/azure/role-based-access-control/rbac-and-directory-admin-roles) — use when users are mixing Azure RBAC with Entra roles or legacy admin assumptions.
- [Azure RBAC tools for the Azure MCP Server overview](https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/azure-rbac) — use to confirm the documented `role` namespace and its actual scope of support.

## Grounded insights worth carrying into the skill

- Microsoft recommends assigning roles to groups, not directly to users, where possible.
- Microsoft recommends using Microsoft Entra PIM for privileged access rather than permanent standing privilege.
- Microsoft explicitly recommends using stable role IDs in automation because role names can change.
- Maximum of three subscription owners is Microsoft’s stated best practice; if a design needs more, it deserves scrutiny.
