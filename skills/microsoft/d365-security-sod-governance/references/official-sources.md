# Official sources

Use this reference only when you need source grounding for Dynamics 365 Finance & Operations security and SoD behavior, or the detailed source list.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live environment state:

- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/sysadmin/role-based-security
- https://learn.microsoft.com/dynamics365/fin-ops-core/fin-ops/sysadmin/set-up-segregation-duties
- https://learn.microsoft.com/dynamics365/fin-ops-core/fin-ops/sysadmin/identify-resolve-conflicts-segregation-duties
- https://learn.microsoft.com/dynamics365/fin-ops-core/fin-ops/sysadmin/roles-violating-sod
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/security-strategy-product-oa
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/sysadmin/tasks/security-diagstics-task-recordings
- https://learn.microsoft.com/dynamics365/fin-ops-core/fin-ops/sysadmin/usg-security-tasks-detailed
- https://learn.microsoft.com/training/modules/plan-implement-security-finance-operations/

## Grounding rule

Official documentation explains Dynamics 365 F&O service behavior. It does not prove the user's current environment configuration, role assignments, SoD rule set, overrides, or compliance posture. Prefer read-only evidence from the environment (e.g., exported security reports, role assignment exports, conflict logs) over inference.

## Service facts (verified 2026-06-16)

Security model structure:
- The D365 Finance & Operations security model is role-based: **Roles** contain **Duties**; **Duties** contain **Privileges**; **Privileges** contain **Permissions** to entry points (menu items, forms, services, tables).
- About 100 standard security roles ship out of the box. Microsoft recommends duplicating and modifying these rather than creating fully custom roles from scratch.
- Users with no role assignment have no access. Users assigned to a role in a specific legal entity are restricted to that legal entity scope.

SoD behavior:
- SoD rules define pairs of duties that the same user or role must not hold simultaneously. Severity levels (warning/error) and mitigation descriptions are configurable per rule.
- When a role is saved or a user-role assignment is made, the system enforces existing SoD rules. Compliance must be explicitly validated after creating or modifying a rule via **System administration > Security > Segregation of duties > Validate duties and roles**.
- Conflicts appear in **Segregation of duties unresolved conflicts**. Administrators must explicitly **Deny** or **Allow** (override) each conflict, and overrides require a documented reason.
- The **Roles violating segregation of duties** view shows all roles with active violations and violation counts.

Security reports:
1. User role assignments report — all users and their assigned roles.
2. Role to user assignment report — per-role user list with restrictions.
3. Security role access report — effective permissions per role across subroles, duties, and privileges.
4. Security duty assignments report — duties per role; use to audit SoD across role combinations.

Review implications:
- Do not approve role changes from intent alone. Require role definition review, SoD rule validation output, last-conflict log, and explicit business owner sign-off.
- Documentation cannot prove the user's actual role assignments, SoD rule set, or override history.
