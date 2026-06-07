# WAF, Shield, and Abuse Controls Guide

Use this reference when the review scope includes AWS WAF web ACLs, managed rules, custom rules, rate-based rules, Bot Control, Shield Advanced, Firewall Manager, labels, logging, or abuse/DDoS posture.

## What people get wrong

The lazy story is:

> We attached WAF, so abuse is handled.

Wrong. A web ACL can be ineffective, too broad, too expensive, noisy, or dangerous if rules are untested or exclusions are careless.

Common bad assumptions:

- Managed rule groups can be enabled straight to block in production.
- Count mode means protected.
- Rate-based rules work without baseline traffic knowledge.
- Bot Control labels are automatically safe to block on.
- WAF logs are optional because CloudWatch metrics exist.
- Shield Advanced means no app-layer DDoS planning is needed.

## Officially grounded controls

Current AWS docs ground these WAF/Shield controls:

- Web ACLs, rules, rule groups, labels, WCUs, dashboards, and bot analysis.
- AWS Managed Rules with explicit guidance to test rule groups before production deployment.
- Rate, CAPTCHA/challenge, SQLi/XSS-style request inspection, and label-based handling.
- Anti-DDoS best practices: establish traffic baselines, monitor CloudWatch metrics, and test in staging.
- Shield Standard is automatically included; Shield Advanced adds additional DDoS protections for eligible resources.

## Non-negotiable review checks

### 1. Identify scope and attachment

Capture whether the web ACL is for CloudFront/global or regional resources such as ALB/API Gateway/AppSync.

Verify:

- associated resource(s)
- default action
- rule priority order
- managed/custom rule groups
- WCU headroom
- count/block/challenge/CAPTCHA actions
- labels and label match rules

### 2. Test managed rules before blocking

Require count-mode observation or staged rollout for new or changed managed rule groups unless there is an active incident requiring emergency block.

### 3. Rate limits need baselines

Rate-based rules should be tied to:

- normal request-rate baseline
- attack threshold
- aggregation key
- exception list
- false-positive rollback
- CloudWatch/WAF-log validation

### 4. Logging is evidence, not decoration

WAF logs are needed to explain blocked/allowed traffic, labels, rule matches, false positives, and cost/noise tradeoffs.

## Minimum safe workflow

1. Map protected resources and web ACL scope.
2. Inspect default action and rule priority.
3. Review managed rule groups, custom rules, labels, WCU, and overrides.
4. Check rate-based rules against baseline traffic.
5. Verify logging, metrics, dashboards, and alarm coverage.
6. Define false-positive rollback and emergency block procedure.
7. If Shield Advanced is claimed, verify protected resources and response/escalation path.

## Verification targets

Use read-only evidence when available:

- WAFv2 web ACL, rules, rule groups, and logging configuration
- CloudFront/API Gateway/ALB association
- WAF sampled requests and log destinations
- CloudWatch metrics: allowed, blocked, counted, challenged, CAPTCHA, rule matches
- Shield Advanced protection status where claimed
- Firewall Manager policy if multi-account governance is in scope

## When to push back

Push back if the design says:

- “turn on every managed rule in block mode”
- “we do not need WAF logs”
- “Count mode means protected”
- “one global rate limit fits all endpoints”
- “allowlist our office IPs broadly”
- “Shield Advanced means WAF tuning is unnecessary”
