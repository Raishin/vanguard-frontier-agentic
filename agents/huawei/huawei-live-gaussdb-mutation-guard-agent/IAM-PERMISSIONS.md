# IAM Permissions — Huawei Live GaussDB Mutation Guard

## Minimum Read Roles (Discovery and Audit)

| Policy | Purpose |
|--------|---------|
| `GaussDB ReadOnlyAccess` | Enumerate GaussDB instances, backup policies, and restore points |
| `RDS ReadOnlyAccess` | Enumerate RDS instances, backup configurations, and restore windows |

## Required Mutation Roles (Live Operations)

| Policy | Purpose | Scope Guidance |
|--------|---------|---------------|
| `GaussDB FullAccess` | Perform instance deletion, spec changes, or backup policy modification | Bind scoped to target enterprise project; never grant account-wide |
| `RDS FullAccess` | Perform RDS instance deletion, spec changes, or backup policy modification | Bind scoped to target enterprise project; never grant account-wide |

## Narrowing Guidance

- Bind `GaussDB FullAccess` / `RDS FullAccess` to the IAM principal scoped to the **target enterprise project** containing the instance, not account-wide.
- For read-only pre-flight audits, `GaussDB ReadOnlyAccess` / `RDS ReadOnlyAccess` is sufficient — do not use `FullAccess` for reads.
- Use IAM agency-based access for cross-account GaussDB/RDS operations rather than direct user credentials.
- Prefer IAM user groups bound to the target enterprise project for database operators over direct per-user policies.

## Anti-Patterns — Never Grant

- `FullAccess` system policy at account level — grants complete control over all services in the account.
- `GaussDB FullAccess` / `RDS FullAccess` without enterprise project scoping — blast radius covers every database instance in the account.
- Storing connection strings or database credentials in plaintext — use temporary credentials via agency or IAM token exchange.
- Deleting instances without verified CBR backup — GaussDB/RDS deletion is permanently destructive without a restorable backup.

## Audit Trail

All GaussDB/RDS mutations (instance deletion, spec changes, backup policy modification) are logged in **Cloud Trace Service (CTS)**. Ensure CTS is enabled for the GaussDB and RDS services in the target region. Query: `CTS > Cloud Trace > GaussDB / RDS > deleteInstance / resizeInstance / deleteBackupPolicy`.
