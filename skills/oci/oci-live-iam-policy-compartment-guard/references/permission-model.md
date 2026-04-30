# Permission Model: OCI Live IAM Policy Compartment Guard

## OCI verb hierarchy

```
inspect  = ListXxx APIs only. No resource content.
read     = GetXxx + inspect. Can see resource details.
use      = read + limited mutation (no create/terminate).
manage   = full CRUD. Always scope to compartment, never tenancy for broad resources.
```

## 3-tier separation

| Tier | Group | Scope | Activation |
|------|-------|-------|-----------|
| Auditor | `<iam-auditors>` | tenancy (read-only) | Standing |
| Operator | `<iam-operators>` | compartment + name pattern | Standing (restricted) |
| Tenancy-root admin | `<iam-tenancy-admins>` | tenancy | Break-glass only, MFA-TOTP gated |

## Audit-only policy

```
Allow group <iam-auditors> to inspect policies in tenancy
Allow group <iam-auditors> to read policies in tenancy
Allow group <iam-auditors> to inspect dynamic-groups in tenancy
Allow group <iam-auditors> to read dynamic-groups in tenancy
Allow group <iam-auditors> to inspect groups in tenancy
Allow group <iam-auditors> to read users in tenancy
```

## Policy operator (compartment-scoped, name-pattern restricted)

```
Allow group <iam-operators> to manage policies in compartment <iam-compartment>
  where target.policy.name = /iam-managed-*/
Allow group <iam-operators> to manage dynamic-groups in tenancy
  where target.dynamicGroup.name = /iam-managed-*/
```

`dynamic-groups` are tenancy-scoped in OCI — compartment scope is not supported. The
`where target.dynamicGroup.name = /iam-managed-*/` name-pattern condition prevents
privilege escalation through creation of an unrestricted dynamic group.

**Critical syntax**: OCI IAM uses **forward-slash regex** `/pattern*/`, **not** quoted strings,
for wildcard matching. `= 'iam-managed-*'` would only match the literal string
`iam-managed-*` (one specific name with a literal asterisk) — the operator could
create any other dynamic group and bypass the guard entirely. Always use `/.../`
slashes for pattern conditions. Reference: Oracle policy conditions docs at
`https://docs.oracle.com/en-us/iaas/Content/Identity/policysyntax/conditions.htm`.

## Tenancy-root admin (break-glass only, MFA-TOTP gated)

```
Allow group <iam-tenancy-admins> to manage policies in tenancy
  where request.user.mfaTotpVerified = 'true'
Allow group <iam-tenancy-admins> to manage groups in tenancy
  where target.group.name != 'Administrators'
```

- MFA-TOTP gate is enforced at policy-evaluation time, not just login.
- Cannot modify the `Administrators` group — requires the bootstrap tenancy admin.
- Membership must be empty by default; add only during an approved change window.

## Do not use

```
# FORBIDDEN
Allow any-group to manage policies in tenancy
Allow group <iam-operators> to manage policies in tenancy
Allow any-user to inspect all-resources in tenancy
```
