# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/wellarchitected/latest/security-pillar/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/security.html
- https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html
- https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub-v2.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- The Well-Architected Security Pillar focuses on protecting data, systems, and assets while delivering business value through risk assessments and mitigation strategies.
- Security review domains include identity and access management, detective controls, infrastructure protection, data protection, and incident response.

Sampled live evidence:
- Read-only API availability sampling reported `WellArchitected+GetWorkload` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Read-only product/API sampling also reported Security Hub and `SecurityHub+GetFindings` available in the sampled regions, but that does not prove detector coverage or finding remediation.

Review implications:
- Require IAM evidence, logging/detection coverage, network and workload protection, encryption/data-boundary controls, vulnerability posture, incident-response readiness, and exception tracking.
- Do not claim Well-Architected security readiness from questionnaire answers without live/repo evidence.
