# Workflow and Output Contract

## Workflow

1. Identify the target scope: management group, subscription, resource group, resource, or data plane.
2. Identify principal type: user, group, service principal, managed identity, workload identity, or application.
3. Prefer built-in roles with narrow scope before custom roles.
4. Challenge dangerous defaults:
   - `Owner` for routine operations,
   - `Contributor` at subscription scope,
   - `User Access Administrator` without strong governance,
   - custom roles with wildcard actions,
   - permanent assignments where time-bound access is appropriate.
5. Check whether data-plane permissions are separate from control-plane RBAC.
6. Stress-test operational hygiene:
   - prefer group-based assignment over direct user grants,
   - prefer PIM or other time-bounded elevation for privileged roles,
   - prefer stable role IDs in automation over role-name matching,
   - challenge estates with more than a few subscription owners.

## Output

Return:

- current access summary,
- risk findings,
- least-privilege alternative,
- validation commands or portal checks,
- assumptions and missing facts.

## Security notes

Do not suggest broad tenant, management-group, or subscription access unless the user has explicitly justified the blast radius.
