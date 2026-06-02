# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:

### API Gateway

- API Gateway security best practices
  https://docs.aws.amazon.com/apigateway/latest/developerguide/security-best-practices.html
- API Gateway throttling and quotas
  https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-request-throttling.html

### CloudFront

- Restrict access to an Amazon S3 origin with CloudFront OAC
  https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html
- Add custom headers to origin requests
  https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/add-origin-custom-headers.html
- Require HTTPS between CloudFront and an S3 origin
  https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-https-cloudfront-to-s3-origin.html
- Use AWS WAF protections with CloudFront
  https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-awswaf.html

### AWS WAF / Shield

- How AWS WAF works
  https://docs.aws.amazon.com/waf/latest/developerguide/how-aws-waf-works.html
- AWS Managed Rules for AWS WAF
  https://docs.aws.amazon.com/waf/latest/developerguide/aws-managed-rule-groups.html
- Best practices for anti-DDoS
  https://docs.aws.amazon.com/waf/latest/developerguide/waf-anti-ddos-best-practices.html
- How AWS Shield and Shield Advanced work
  https://docs.aws.amazon.com/waf/latest/developerguide/ddos-overview.html
- Automatic application-layer DDoS mitigation with Shield Advanced
  https://docs.aws.amazon.com/waf/latest/developerguide/ddos-automatic-app-layer-response.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:

- API Gateway security best-practice guidance points reviewers to IAM, CloudWatch, CloudTrail, AWS Config, Security Hub CSPM, and JWT authorizers as API security controls.
- API Gateway throttling and quotas are separate control layers; account/stage/method throttles, usage plans, and downstream capacity must be reviewed together.
- CloudFront OAC is the current S3-origin protection path; legacy OAI exists but is not the preferred new pattern. OAC/S3 bucket policy and SSE-KMS permissions must be reviewed together.
- CloudFront custom origin headers can help origin request shaping, but AWS documents headers CloudFront cannot add and separate guidance for forwarding `Authorization`; do not treat custom headers as a complete security boundary.
- AWS WAF web ACLs use rules, rule groups, labels, WCUs, dashboards, bot analysis, and metrics; AWS Managed Rules should be tested before production blocking.
- AWS anti-DDoS guidance calls for baseline traffic patterns, CloudWatch metrics, and staging tests; Shield Standard is automatic, while Shield Advanced adds additional protections for eligible resources.

Sampled live evidence:

- Read-only regional availability sampling reported `isAvailableIn` for Amazon API Gateway, Amazon CloudFront, AWS WAF, and AWS Shield in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Read-only API availability sampling reported `WAFV2+GetWebACL`, `WAFV2+ListRuleGroups`, and `CloudWatch+DescribeAlarms` as `isAvailableIn` in those sampled regions.
- `CloudFront+GetDistribution` sampled as `isAvailableIn` in `us-east-1` and `us-west-2`, and `Not Found` in `eu-west-1` and `ap-southeast-1`; treat CloudFront API evidence as global/service-specific rather than normal regional workload proof.

Stale or missing guidance corrected:

- The prior reference set was too generic and lacked component-specific guidance for API Gateway, CloudFront, WAF/Shield, and cross-service incident triage.
- It did not call out API type differences, JWT/authZ separation, cache-key sensitivity, OAC vs OAI, WAF count-mode limits, managed-rule staging, Shield Advanced scope, or evidence correlation across edge services.
- It did not provide verification targets or pushback criteria comparable to the stronger AgentCore references.

Review implications:

- Public edge exposure is not acceptable without evidence for authN/authZ, throttling/quotas, TLS, origin protection, WAF/Shield posture, logging, alarms, and rollback.
- CloudFront distribution state, cache behavior, origin policy, WAF decisions, API Gateway stages/routes, and quotas must come from repo/live evidence, not documentation alone.
- Treat sampled live evidence as regional/API availability only; it does not prove the user's resources are configured safely.
