# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/wellarchitected/latest/framework/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/security-pillar/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/reliability-pillar/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/cost-optimization-pillar/welcome.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- The AWS Well-Architected Framework helps evaluate tradeoffs and best practices for reliable, secure, efficient, and cost-effective cloud systems.
- Security, reliability, and cost optimization pillars each provide separate guidance; a solution decision can improve one pillar while increasing risk in another.

Sampled live evidence:
- Read-only regional availability sampling reported `WellArchitected+GetWorkload` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.

Review implications:
- Architecture recommendations must state workload context, constraints, assumptions, tradeoffs, pillar impacts, validation path, and migration/rollback path.
- Well-Architected tool/API availability does not prove a workload has been reviewed or that risks are remediated.
