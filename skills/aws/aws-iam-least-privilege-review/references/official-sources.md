# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-custom-policy-checks.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/getting-started-reduce-permissions.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- IAM best practices call for least-privilege permissions, federation and temporary credentials, MFA, Access Analyzer policy generation/validation, removal of unused permissions, policy conditions, and cross-account/public-access analysis.
- IAM policy documents grant permissions through identity-based, resource-based, and other policy types; least privilege requires narrowing actions, resources, conditions, and principals.

Sampled live evidence:
- Read-only regional availability sampling reported IAM as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled API `AccessAnalyzer+ValidatePolicy` was reported `isAvailableIn` in those regions; `IAM+GetPolicy` was `isAvailableIn` in `us-east-1` and `us-west-2`, and `Not Found` in `eu-west-1` and `ap-southeast-1`, so treat IAM as global/service-specific availability evidence rather than regional resource proof.

Review implications:
- Do not approve broad IAM from intent alone. Require action/resource/condition scope, Access Analyzer findings, last-accessed or CloudTrail evidence, boundary/SCP context, and break-glass exception handling.
- Documentation cannot prove the user's actual principals, policies, or account guardrails.
