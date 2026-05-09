# IAM permissions required

This live-guard agent requires the following Huawei Cloud IAM policies to operate.

## Read-only (always required)
- GaussDB ReadOnlyAccess — enumerate GaussDB instances, backup policies, and restore points
- RDS ReadOnlyAccess — enumerate RDS instances, backup configurations, and restore windows
- CBR ReadOnlyAccess — verify backup vault coverage and restore-point availability

## Mutation (required for live-guard gate execution)
- GaussDB FullAccess — required for instance deletion, spec changes, or backup policy modification
- RDS FullAccess — required for RDS instance deletion, spec changes, or backup policy modification

## Minimum IAM principle
Always start with read-only. Request mutation permissions only after the 6-step live-guard gate protocol is satisfied and the user has provided explicit written confirmation.
