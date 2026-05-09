# Workflow and output contract

Use this reference only when performing the full proxy audit, security review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Proxy inventory: proxy names, base paths, environment deployments, revision states
- Security policies: VerifyAPIKey / OAuthV2 / JWT presence, attachment order in flow, coverage of all proxy endpoints
- Rate limiting: SpikeArrest (burst protection) + Quota (aggregate time-window) — both required
- Environment groups: hostname routing, environment mapping (dev/test/prod), shared flow dependencies
- Target servers: backend hostname configuration, TLS settings, health check configuration
- Developer portal: API catalog completeness, developer app registration workflow
- API products and quota plans: product-to-proxy bindings, quota limits per plan tier
- Analytics: API Monitoring alerts, custom reports, latency and error rate dashboards

## Safe workflow

1. **Frame scope**
   - Apigee organization/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Apigee Management API read-only evidence if available.
   - Otherwise inspect repository proxy bundles, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What proxies lack security policy coverage on all flows?
   - What rate limiting gaps allow sustained load attacks?
   - What backend endpoints are exposed via hardcoded URLs instead of target servers?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Apigee API Platform Operator: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
