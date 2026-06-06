# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/drift-aware-change-sets.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/event-detail-stack-drift-detection-change.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/best-practices.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-policy.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- CloudFormation drift-aware change sets support three-way comparison and `REVERT_DRIFT` for supported resources.
- CloudFormation drift-detection status change events expose drift status, drifted resource counts, operation status, and client request tokens as event evidence.

Sampled live evidence:
- Read-only regional availability sampling reported AWS CloudFormation and AWS Config as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `CloudFormation+CreateChangeSet`, `CloudFormation+DetectStackDrift`, and `CloudFormation+ValidateTemplate` were reported `isAvailableIn` in those regions.

Review implications:
- IaC safety review must inspect planned diff, replacement/delete risk, IAM/security policy impact, network exposure, data loss, drift, rollback, and approval evidence.
- A valid template or available change-set API does not prove the proposed change is safe.
