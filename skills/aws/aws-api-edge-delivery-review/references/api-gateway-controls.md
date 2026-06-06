# API Gateway Control Guide

Use this reference when the review scope includes API Gateway REST, HTTP, or WebSocket APIs, custom domains, authorizers, throttling, quotas, stages, resource policies, or API Gateway logging.

## What people get wrong

The lazy story is:

> API Gateway is managed, so the API edge is safe by default.

Wrong. API Gateway removes server management; it does not remove API abuse, auth, quota, logging, or data-exposure risk.

Common bad assumptions:

- A public API is acceptable because the backend is private.
- JWT authorizer exists, so authorization is solved.
- Account-level throttling is enough for per-tenant abuse.
- CloudWatch metrics exist, so audit evidence exists.
- CORS is a browser concern, not a security review item.
- Stage variables, access logs, and mapping templates cannot leak sensitive data.

## Officially grounded controls

Current AWS documentation points API Gateway reviewers toward these control families:

- IAM authorization, Lambda/JWT/Cognito authorizers, and resource policies for access control.
- CloudWatch metrics/logs and access logging for observability.
- CloudTrail for API management-plane audit activity.
- AWS Config and Security Hub CSPM for configuration/security evidence.
- Throttling and quotas to limit request rates and protect downstream systems.

Treat those as required evidence categories, not optional polish.

## Non-negotiable review checks

### 1. Identify API type before giving guidance

REST API, HTTP API, and WebSocket API do not expose the same controls or operational surfaces. Do not give one-size-fits-all recommendations.

Capture:

- API type and stage
- custom domain and TLS policy
- integration type and backend
- authorizer type
- resource policy, if any
- usage plan/API key design, if any
- throttling/quota settings
- access-log format and destination

### 2. Split authentication from authorization

An authorizer answers “who is this?” or “is this token acceptable?”. It does not automatically prove per-route, per-tenant, or object-level authorization.

Look for:

- route/method authorization coverage
- claims-to-permission mapping
- tenant isolation checks
- explicit deny paths
- unauthenticated OPTIONS/CORS behavior
- bypasses through alternate stages/domains

### 3. Treat throttling as layered protection

Review all relevant layers:

- account-level throttles
- stage/method throttles
- usage plans and quotas
- WAF rate-based rules
- downstream service capacity
- retry behavior from clients and integrations

A single throttle setting is not an abuse-control strategy.

### 4. Logs must be useful and safe

Access logs should answer who/what/when/outcome without leaking secrets.

Check for:

- request ID / extended request ID
- principal or anonymized tenant identifier
- route/method/status/integration latency
- WAF decision correlation where applicable
- no bearer tokens, cookies, API keys, passwords, or PII payloads in logs
- retention and KMS controls for log groups

## Minimum safe workflow

1. Classify API type, stage, custom domain, and backend integration.
2. Enumerate all public routes and unauthenticated routes.
3. Verify authorizer, resource policy, and route authorization coverage.
4. Verify throttles, quotas, usage plans, and downstream capacity assumptions.
5. Inspect access-log settings and log redaction posture.
6. Check CloudTrail/Config/Security Hub evidence for management-plane and config drift.
7. Identify rollback: stage variable, canary, deployment, DNS, or previous deployment ID.

## Verification targets

Use read-only evidence when available:

- API Gateway API/stage/domain/authorizer/resource-policy descriptions
- route/method authorization settings
- stage access-log settings
- usage plan and API key attachment where used
- CloudWatch metrics: 4xx, 5xx, latency, integration latency, throttles
- CloudTrail events for recent API/stage/domain changes

## When to push back

Push back if the design says:

- “public for now, auth later”
- “API keys are authentication”
- “JWT means authorization is done”
- “we do not need throttling because traffic is low”
- “we log full requests for debugging”
- “rollback is redeploying main”
