# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub-v2.html
- https://docs.aws.amazon.com/guardduty/latest/ug/what-is-guardduty.html
- https://docs.aws.amazon.com/inspector/latest/user/what-is-inspector.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/what-is-access-analyzer.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Security Hub unifies cloud security signals, exposure detection, automated response paths, and third-party integrations.
- IAM Access Analyzer can identify external/internal access, unused IAM access, validate policies, and generate policies from CloudTrail activity.

Sampled live evidence:
- Read-only regional availability sampling reported Security Hub and GuardDuty as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `SecurityHub+GetFindings`, `GuardDuty+ListDetectors`, and `AccessAnalyzer+ListFindings` were reported `isAvailableIn` in those regions.

Review implications:
- Hardening requires enabled-service coverage, org aggregation, finding severity/age, exceptions, IAM exposure, public access, encryption/logging gaps, and remediation ownership.
- Security tool availability does not prove detectors are enabled, findings are triaged, or controls are enforced.
