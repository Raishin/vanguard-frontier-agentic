# Permissions: OCI Live IAM Policy Compartment Guard

# OCI IAM policy for IAM policy compartment guard

## Identity model preference

1. Named IAM-admin group scoped to an IAM-management compartment
2. Dual-approval for tenancy-root policy changes (separate writer and approver)
3. Never use `any-user` or `any-group` for policy management
4. Tenancy-root policy changes require separate security-team sign-off

## Verb hierarchy reference

```
inspect  = ListXxx APIs only. No resource content.
read     = GetXxx + inspect. Can see resource details.
use      = read + limited mutation (no create/terminate).
manage   = full CRUD. Always scope to compartment, never tenancy for broad resources.
```

## Audit-only policy

```
Allow group <iam-auditors> to inspect policies in tenancy
Allow group <iam-auditors> to read policies in tenancy
Allow group <iam-auditors> to inspect dynamic-groups in tenancy
Allow group <iam-auditors> to read dynamic-groups in tenancy
Allow group <iam-auditors> to inspect groups in tenancy
Allow group <iam-auditors> to read users in tenancy
```

## Policy operator (compartment-scoped write, never tenancy root)

```
Allow group <iam-operators> to manage policies in compartment <iam-compartment>
  where target.policy.name = 'iam-managed-*'
Allow group <iam-operators> to manage dynamic-groups in tenancy
  where target.dynamicGroup.name = 'iam-managed-*'
```

`dynamic-groups` are tenancy-scoped in OCI — they cannot be compartment-scoped.
This is the minimum necessary `manage` at tenancy scope. The `where` name-pattern
condition restricts which dynamic groups this role can create or modify, preventing
privilege escalation through creation of an unrestricted dynamic group.

## Tag-condition for policy name pattern restriction

```
Allow group <iam-operators> to manage policies in compartment <iam-compartment>
  where target.policy.name = 'iam-managed-*'
```

## Tenancy-root admin (third tier — break-glass only)

OCI policy-based IAM separates compartment-scoped operators from tenancy-root
admins. The tenancy-root admin is a **break-glass** identity activated only for
incidents that require touching tenancy-level policies (e.g., when an
operator-managed policy would create a cycle or escalation path).

```
Allow group <iam-tenancy-admins> to manage policies in tenancy
  where request.user.mfaTotpVerified = 'true'
Allow group <iam-tenancy-admins> to manage groups in tenancy
  where target.group.name != 'Administrators'
```

- MFA-TOTP gate enforced at policy-evaluation time (not just login).
- Cannot modify the `Administrators` group from this role — that requires the
  bootstrap tenancy admin (no automation, no service principal).
- Membership in `<iam-tenancy-admins>` should be empty by default; add only for
  the duration of an approved change window, then remove.

## Do not use

```
# FORBIDDEN
Allow any-group to manage policies in tenancy
Allow group <iam-operators> to manage policies in tenancy
Allow any-user to inspect all-resources in tenancy
```

