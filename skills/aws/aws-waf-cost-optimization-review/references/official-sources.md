# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/wellarchitected/latest/cost-optimization-pillar/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/cost-optimization.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/cost_cloud_financial_management.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/cost_manage_demand_resources.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- The Well-Architected Cost Optimization Pillar focuses on avoiding unnecessary cost while meeting business outcomes.
- Cost optimization review areas include cloud financial management, expenditure awareness, cost-effective resources, demand/resource management, and continuous optimization.

Sampled live evidence:
- Read-only API availability sampling reported `WellArchitected+GetWorkload` as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Cost-review evidence still requires workload, billing, tagging, utilization, and commitment data from the user's account; Well-Architected API availability alone does not prove review status.

Review implications:
- Require spend owner, cost allocation/tags, unit economics, utilization, rightsizing, commitment coverage, idle resources, and risk of reliability/security regression before recommending savings.
- Do not treat Well-Architected answers or tool access as proof that cost risks are remediated.
