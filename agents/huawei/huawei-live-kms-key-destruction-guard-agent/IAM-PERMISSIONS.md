# IAM permissions required

This live-guard agent requires the following Huawei Cloud IAM policies to operate.

## Read-only (always required)
- DEW ReadOnlyAccess — enumerate KMS keys, key metadata, and pending deletion status
- RDS ReadOnlyAccess — identify DBSS-encrypted RDS/GaussDB instances relying on the key
- OBS Viewer — identify server-side encrypted buckets relying on the key
- CSMS ReadOnlyAccess — enumerate CSMS secrets encrypted by the key

## Mutation (required for live-guard gate execution)
- DEW FullAccess — required to schedule key deletion, cancel pending deletion, or disable/enable keys

## Minimum IAM principle
Always start with read-only. Request mutation permissions only after the 6-step live-guard gate protocol is satisfied and the user has provided explicit written confirmation.
