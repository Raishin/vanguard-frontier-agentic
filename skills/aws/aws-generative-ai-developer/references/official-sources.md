# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/bedrock/latest/userguide/what-is-bedrock.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/GenAI-observability.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Amazon Bedrock provides managed access to foundation models and related application features; model availability, feature maturity, and regional coverage can vary.
- CloudWatch generative AI observability can trace prompts, track token latency, and monitor AgentCore agents, but observability must be enabled and scoped to the application.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon Bedrock as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.

Review implications:
- Require model/region selection, guardrails, prompt/version management, RAG source boundaries, IAM/KMS controls, logging, cost/token limits, evaluation criteria, and rollback/fallback behavior.
- Do not infer deployed agent/model safety from Bedrock service availability; inspect app configuration, prompts, tools, data sources, and telemetry.
