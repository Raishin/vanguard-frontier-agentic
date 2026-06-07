# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/awssupport/latest/user/trusted-advisor.html
- https://docs.aws.amazon.com/organizations/latest/userguide/services-that-can-integrate-ta.html
- https://docs.aws.amazon.com/health/latest/ug/what-is-aws-health.html
- https://docs.aws.amazon.com/systems-manager-incidents/latest/userguide/what-is-incident-manager.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Trusted Advisor inspects AWS environments and can recommend cost, performance, availability, security, and service-limit improvements depending on support plan and feature access.
- Trusted Advisor can integrate with AWS Organizations for delegated-administrator visibility across member accounts.

Sampled live evidence:
- Read-only regional availability sampling reported `Support+DescribeCases` as `isAvailableIn` in `us-east-1`, `us-west-2`, and `eu-west-1`, and `Not Found` in `ap-southeast-1`.
- `Health+DescribeEvents` was `isAvailableIn` in `us-east-1` and `Not Found` in the other sampled regions, consistent with account/global-style evidence rather than a normal regional workload API.

Review implications:
- Triage must capture severity, customer/business impact, affected accounts/regions/services, evidence source, owner, escalation target, support entitlement, and next update time.
- Do not claim an AWS Support or Health state unless the relevant account/source was queried or the user provided sanitized evidence.
