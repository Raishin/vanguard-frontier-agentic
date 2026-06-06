# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/privateca/latest/userguide/ca-best-practices.html
- https://docs.aws.amazon.com/privateca/latest/userguide/PCACertInstall.html
- https://docs.aws.amazon.com/privateca/latest/userguide/PcaWelcome.html
- https://docs.aws.amazon.com/acm/latest/userguide/acm-overview.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Private CA best practices include documenting CA structure and policies, minimizing root CA use, giving root CA its own account, separating administrator and issuer roles, managed revocation, CloudTrail, and blocking public CRL access.
- Installing CA certificates differs for root and subordinate CAs and depends on compatible signing algorithms and activation steps.

Sampled live evidence:
- Read-only regional availability sampling reported AWS Private CA and ACM as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `ACM PCA+DescribeCertificateAuthority` and `ACM+DescribeCertificate` were reported `isAvailableIn` in those regions.

Review implications:
- Issuer review requires CA hierarchy, policy/OID constraints, revocation path, issuer/admin separation, key protection, CloudTrail, certificate templates, renewal, and trust-store impact evidence.
- CA availability does not prove safe issuance policy or trust boundary.
