# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/bedrock/latest/userguide/security-best-practice-agents.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-injection.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-how.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer AWS managed MCP read-only evidence through the user's configured read-only AWS profile, read-only AWS CLI evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- Bedrock prompt-injection guidance says Guardrails can detect prompt attacks, but customers still need application controls such as input validation, guardrail association, and safer prompt design.
- Bedrock CloudTrail guidance covers Bedrock API activity; logging proves calls occurred, not whether an agent decision was safe.

Sampled live evidence:
- Read-only regional availability sampling reported `isAvailableIn` for Amazon Bedrock in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.

Review implications:
- Guardrails and prompt-injection controls reduce risk; they do not prove a specific agent, action group, knowledge base, or tool integration is safe.
- Require evidence for IAM scope, data-source boundaries, logging, model/tool invocation paths, memory retention, PII handling, and kill-switch behavior before production readiness claims.
