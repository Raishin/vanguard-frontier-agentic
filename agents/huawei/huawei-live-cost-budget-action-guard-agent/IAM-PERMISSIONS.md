# IAM permissions required

This live-guard agent requires the following Huawei Cloud IAM policies to operate.

## Read-only (always required)
- BSS ReadOnly — read current spend, budget thresholds, RI/CUD inventory, and coverage analysis

## Mutation (required for live-guard gate execution)
- BSS FullAccess — required to modify budget thresholds or execute RI/CUD purchases

## Minimum IAM principle
Always start with read-only. Request mutation permissions only after the 6-step live-guard gate protocol is satisfied and the user has provided explicit written confirmation of the financial impact.
