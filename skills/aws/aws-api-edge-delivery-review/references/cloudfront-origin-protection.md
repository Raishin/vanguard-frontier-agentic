# CloudFront Origin Protection Guide

Use this reference when the review scope includes CloudFront distributions, origins, cache behaviors, origin access control, TLS, custom headers, cache keys, origin request policies, or distribution rollback.

## What people get wrong

The naive story is:

> CloudFront is in front, so the origin is protected.

Wrong. CloudFront only protects the origin if the origin is configured to reject direct access and the cache/origin policies do not leak or amplify risk.

Common bad assumptions:

- S3 origin is private because CloudFront uses it.
- Origin access identity and origin access control are interchangeable.
- Viewer HTTPS is enough; origin HTTPS does not matter.
- Forwarding all headers/cookies/query strings is safer because it is “complete”.
- Cache invalidation is the rollback plan.
- Custom origin headers are secrets.

## Officially grounded controls

Current AWS docs ground these CloudFront controls:

- Origin Access Control for restricting S3 origins, including migration from legacy OAI and SSE-KMS permission considerations.
- HTTPS viewer/origin policies for encrypted transport.
- Custom origin headers for origin request shaping, with documented header restrictions.
- Cache behavior, cache key, and origin request policy choices that affect correctness, privacy, and cost.
- AWS WAF association for edge filtering and rule enforcement.

## Non-negotiable review checks

### 1. Prove the origin cannot be bypassed

For S3 origins, verify bucket policy allows the CloudFront distribution/OAC path and denies unintended direct access.

For custom origins, verify one or more controls:

- origin only reachable from expected networks
- secret header is not the only control unless risk accepted
- ALB/security group/origin firewall rules restrict direct access
- origin TLS certificate and hostname behavior are correct

### 2. Cache keys are security boundaries

Cache policies can leak tenant/user-specific responses if identity-bearing inputs are omitted from the cache key.

Check:

- Authorization/cookie/query/header forwarding
- whether authenticated responses are cached
- error response caching
- compression and content negotiation
- cache behavior path precedence

### 3. TLS and domain posture are part of rollback

Capture:

- viewer protocol policy
- minimum TLS protocol
- certificate scope and expiration
- alternate domain names
- DNS cutover/rollback path

### 4. Origin request controls can create data exposure

Forwarding too much can expose cookies and auth headers to origins that do not need them. Forwarding too little can break authorization or cache correctness.

## Minimum safe workflow

1. Map viewer domain -> distribution -> behavior -> origin -> backend.
2. Identify every origin and whether direct access is blocked.
3. Review cache and origin request policies for sensitive variance.
4. Verify TLS policy, certificate, and DNS ownership.
5. Verify WAF association and logging posture.
6. Define rollback: previous distribution config, DNS change, origin failover, or behavior revert.

## Verification targets

Use read-only evidence when available:

- distribution config and status
- cache behaviors and origin request/cache policies
- OAC/OAI config and S3 bucket policy
- origin protocol policy and custom headers
- WAF web ACL association
- standard or real-time log delivery
- CloudWatch metrics and CloudFront 4xx/5xx/error-rate signals

## When to push back

Push back if the design says:

- “the S3 bucket is public but hidden behind CloudFront”
- “we forward everything to avoid bugs”
- “custom header is our only origin protection”
- “we can invalidate if something goes wrong”
- “CloudFront is global so regional evidence is irrelevant” without explaining global-control-plane implications
