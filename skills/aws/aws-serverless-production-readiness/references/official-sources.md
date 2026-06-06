# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/lambda/latest/dg/durable-execution-sdk-retries.html
- https://docs.aws.amazon.com/lambda/latest/dg/governance-observability.html
- https://docs.aws.amazon.com/lambda/latest/dg/best-practices.html
- https://docs.aws.amazon.com/serverless/latest/devguide/serverless-samples.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Lambda durable function retry guidance covers step retries, invocation retries, backend retries, exponential backoff, CloudWatch monitoring, and retry best practices.
- Lambda governance/observability guidance covers visibility into configurations, compliance, function boundaries through Security Hub CSPM, dashboards, tagging, and owner outreach.

Sampled live evidence:
- Read-only regional availability sampling reported Lambda, API Gateway, and Step Functions as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `Lambda+GetFunction` and `SFN+DescribeStateMachine` were reported `isAvailableIn` in those regions.

Review implications:
- Production readiness requires concurrency, timeout/memory, retry/DLQ/destination behavior, idempotency, observability, IAM, secrets, deployment/rollback, cost guardrails, and failure-mode tests.
- Serverless service availability does not prove workload readiness or correct event-source semantics.
