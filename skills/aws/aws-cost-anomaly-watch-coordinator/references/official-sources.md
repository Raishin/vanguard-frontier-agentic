# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/cost-management/latest/userguide/ce-access.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/ce-enable.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/getting-started-ad.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-managing-costs.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Cost Explorer access is permission-controlled; member-account visibility also depends on management-account Cost Explorer preferences.
- Enabling Cost Explorer activates machine-learning anomaly detection alerts according to AWS Cost Management guidance, but alert coverage still depends on monitors, subscriptions, thresholds, and account scope.

Sampled live evidence:
- Read-only regional availability sampling reported `Cost Explorer+GetAnomalies` and `Cost Explorer+GetAnomalyMonitors` as `isAvailableIn` in `us-east-1`; the same filters returned `Not Found` in `us-west-2`, `eu-west-1`, and `ap-southeast-1`, so treat Cost Explorer anomaly APIs as global/home-region-style evidence, not broad regional deployment proof.

Review implications:
- Do not claim a cost spike root cause without Cost Explorer time-series evidence, anomaly monitor/subscription scope, service/account/tag attribution, and known deployment or usage-change correlation.
- Keep this role non-destructive: recommend containment, owner escalation, budget alert review, tagging fixes, and approval-gated remediation instead of stopping resources directly.
