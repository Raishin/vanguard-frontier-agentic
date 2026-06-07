# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/mgn/latest/ug/what-is-application-migration-service.html
- https://docs.aws.amazon.com/mgn/latest/ug/getting-started.html
- https://docs.aws.amazon.com/prescriptive-guidance/latest/migration-cutover-runbook/welcome.html
- https://docs.aws.amazon.com/prescriptive-guidance/latest/cutover-traffic/welcome.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- AWS Application Migration Service is an automated lift-and-shift service for migrating physical, virtual, or cloud servers to AWS.
- MGN getting-started guidance covers initialization, launch templates/settings, best practices, quotas, and scaling migration workflows.

Sampled live evidence:
- Read-only regional availability sampling reported AWS Application Migration Service as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `mgn+DescribeSourceServers` and `Migration Hub Refactor Spaces+ListApplications` were reported `isAvailableIn` in those regions.

Review implications:
- Cutover readiness requires replication health, launch template correctness, DNS/network plan, data freeze, test launch results, rollback/rollback-window plan, owner approvals, and business validation.
- Service availability does not prove source servers are replicated, tested, or ready to cut over.
