# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/apigateway/latest/developerguide/security-best-practices.html
- https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-request-throttling.html
- https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html
- https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-awswaf.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer AWS managed MCP read-only evidence through the user's configured read-only AWS profile, read-only AWS CLI evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- API Gateway security best-practice guidance explicitly covers IAM, CloudWatch, CloudTrail, AWS Config, Security Hub CSPM, and JWT authorizers as security controls for APIs.
- CloudFront supports origin access control for S3 origins and can associate AWS WAF web ACLs with distributions for edge filtering.

Sampled live evidence:
- Read-only regional availability sampling reported `isAvailableIn` for Amazon API Gateway, Amazon CloudFront, and AWS WAF in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.

Review implications:
- Do not treat public edge exposure as safe without authN/authZ, throttling, TLS policy, WAF/origin protection where applicable, logging, and rollback evidence.
- CloudFront distribution state, quotas, WAF rule behavior, cache keys, and origin policy must come from live evidence or repo configuration, not docs alone.
