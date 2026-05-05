---
name: aws-api-edge-delivery-review
description: Review AWS API and edge delivery posture across API Gateway, CloudFront, AWS WAF, Shield, ALB, custom domains, TLS policies, authentication, authorization, throttling, quotas, caching, origin protection, logging, and abuse controls. Use when public APIs, web entry points, or edge delivery can affect security and availability.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.2"
  updated: "2026-05-05"
  category: networking
---

# AWS API Edge Delivery Review

## Purpose

Act as the AWS API/edge reviewer who assumes every public endpoint without throttling, auth, WAF, origin protection, and logs is an incident waiting for traffic.

## When to use

Use this skill for:

- API Gateway REST/HTTP/WebSocket API, CloudFront distribution, WAF web ACL, Shield, ALB ingress, or public endpoint review
- throttling, quotas, auth/JWT/IAM/Lambda authorizer, custom domain, TLS, logging, caching, or CORS questions
- origin access control, S3 origin protection, WAF managed rules, bot/abuse protection, or DDoS posture
- API/edge incident involving 4xx/5xx spikes, latency, cache poisoning, blocked traffic, or unexpected cost

## Lean operating rules

- Prefer `AwsDocumentationMcpServer` when available via `uvx awslabs.aws-documentation-mcp-server@latest`; if `uvx` cannot run in the current environment, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to repository evidence, sanitized user evidence, official AWS documentation, Context7, and read-only AWS CLI evidence when available.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, public exposure, destructive automation, untested recovery, hidden cost, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, incident triage, implementation guidance, or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, traffic-changing, cost-changing, compliance-impacting, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding AWS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
