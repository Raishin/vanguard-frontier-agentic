# IAM Permissions — Huawei Live IAM Policy Change Guard

## Minimum Read Roles (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `IAM ReadOnlyAccess` | List IAM users, groups, roles, fine-grained policies, agencies, and enterprise project bindings without mutation capability |

## Required Mutation Roles (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `IAM FullAccess` | Attach/detach policies, manage agencies, and modify fine-grained custom policies | Confirm authority level; IAM FullAccess at account level grants complete IAM control |

## Narrowing Guidance

- Use `IAM ReadOnlyAccess` for all pre-flight audits; do not use `IAM FullAccess` for read operations.
- SCP mutations require Organization-level admin credentials — confirm the principal has Organizations FullAccess in the master account.
- Agency trust relationship changes require explicit approval from a human with IAM FullAccess; treat agency grants as high-risk mutations.
- Prefer fine-grained custom policies with explicit resource and condition constraints over broad system policies.

## Anti-Patterns — Never Grant

- `FullAccess` system policy without enterprise project scoping — grants complete control of all services account-wide.
- `SecurityAdministrator` agency to untrusted cross-account principals — grants security configuration control across accounts.
- `Huawei Cloud Account Administrator` to non-root principals — equivalent to root access.
- Attaching multiple FullAccess policies to the same IAM principal — cumulative effect may be broader than intended.
- Disabling MFA for root or privileged accounts to "simplify" workflows.

## SCP-Specific Notes

- Only master account users with Organizations FullAccess can create, modify, or delete SCPs.
- SCP deny statements override all IAM Allow policies in member accounts — there is no IAM-level bypass.
- Before adding an SCP deny, enumerate all services and principals affected across ALL member accounts.
- Test SCP changes in a dedicated test member account before applying to production member accounts.

## Audit Trail

All IAM mutations (policy attachments, agency creation, SCP changes) are logged in **Cloud Trace Service (CTS)**. Ensure CTS is enabled for IAM in the target account. Query: `CTS > Cloud Trace > IAM > createPolicy / attachPolicy / createAgency`.
