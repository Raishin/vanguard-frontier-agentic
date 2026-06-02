# API and Edge Observability / Incident Playbook

Use this reference for API Gateway, CloudFront, WAF, Shield, or ALB edge incidents involving 4xx/5xx spikes, latency, throttling, false positives, cache poisoning, origin failures, bot traffic, or unexpected cost.

## What people get wrong

The naive story is:

> We have dashboards, so we can debug the edge.

Wrong. Edge incidents cross services. If request IDs, logs, WAF decisions, cache behavior, and origin metrics cannot be correlated, dashboards become theater.

Common bad assumptions:

- 4xx means client fault.
- 5xx means origin fault.
- WAF blocks are always attacks.
- CloudFront cache hit ratio is always good when high.
- API Gateway throttles prove abuse rather than mis-sized quotas.
- Cost spikes are separate from reliability incidents.

## Evidence to collect first

Capture the time window and compare against a clean baseline.

Minimum evidence set:

- affected hostnames, paths, methods, status codes
- CloudFront distribution and cache behavior
- API Gateway API/stage/route, if applicable
- WAF web ACL/rule/action/labels, if applicable
- origin target and origin health
- recent deployments, WAF changes, DNS/certificate changes, or cache policy changes
- customer impact and business priority

## Failure-mode map

### 4xx spike

Check:

- authorizer failures
- resource policy denies
- WAF blocks/challenges/CAPTCHA
- CORS/preflight failures
- missing routes or base-path mappings
- signed URL/cookie failures
- request size/header validation

### 5xx spike

Check:

- origin health and target group status
- API Gateway integration errors/timeouts
- Lambda/backend errors
- CloudFront origin connection attempts/timeouts
- certificate/TLS origin mismatch
- DNS/origin failover behavior

### Latency spike

Check:

- cache hit ratio and origin latency
- API Gateway integration latency vs total latency
- backend saturation
- WAF inspection overhead from expensive rules
- regional vs global path differences

### False-positive WAF incident

Check:

- rule ID/rule group/label
- sampled requests/logs
- recent managed rule updates or overrides
- path-specific exception options
- count-mode rollback

## Minimum safe incident workflow

1. State the exact time window and affected edge path.
2. Identify whether the symptom begins at viewer, edge, WAF, API Gateway, or origin.
3. Correlate metrics and logs across services.
4. Identify the smallest reversible mitigation.
5. Prefer count/allow exception scoped by path/header/method over disabling a whole managed rule group.
6. Record rollback and post-incident hardening actions.

## Verification targets

Use read-only evidence when available:

- CloudFront standard/real-time logs or distribution metrics
- API Gateway access logs and execution metrics
- WAF logs/sampled requests and rule metrics
- CloudWatch alarms and metric math dashboards
- origin target health and backend logs
- CloudTrail events for recent configuration changes
- Cost Explorer / usage evidence if request volume or logging cost spiked

## When to push back

Push back if the user asks to:

- disable the entire WAF to fix one false positive
- increase throttles without checking origin capacity
- invalidate all cache paths as a generic rollback
- ignore missing logs and “just infer” root cause
- call the incident resolved without post-mitigation metrics
