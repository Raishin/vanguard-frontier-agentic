# Workflow and output contract

Use this reference only when performing the full LB traffic engineering review, type selection analysis, or rolling deploy safety assessment.

## Review domains

Check these areas before giving a verdict:
- LB type selection: protocol requirements, client IP preservation, multi-region need, Cloud Armor requirement, internal vs. external
- Health check configuration: check type (HTTP/HTTPS/TCP), check interval, timeout, healthy threshold, unhealthy threshold, port and path match
- Cloud Armor posture: policy attached to backend service, rule priority, rate-based rules, adaptive protection enabled, WAF rule sets
- Traffic distribution: backend service balancing mode (RATE/UTILIZATION/CONNECTION), capacity scaler, failover policy
- SSL/TLS configuration: certificate type (Google-managed / self-managed / Certificate Manager), SSL policy TLS version, cipher suites
- Connection draining: draining timeout vs. max request duration, backend update sequence, traffic shift validation
- Rolling deploy safety: instance group update strategy, surge/max unavailable settings, health check propagation lag

## Safe workflow

1. **Frame scope**
   - LB type and target workload:
   - Protocol and traffic pattern:
   - Multi-region or single-region:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Is the LB type correct for the protocol and security requirements?
   - Are health checks configured correctly and matching the backend protocol?
   - Is Cloud Armor attached for internet-facing HTTP(S) workloads?
   - Is the connection draining timeout long enough for in-flight requests?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged traffic shift, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Load Balancer Traffic Engineering Review: <scope>
## Executive verdict
- Status: PRODUCTION READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- LB type: <type>
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Health check assessment
- Check type and protocol match: <correct / mismatch / unknown>
- Unhealthy threshold: <value or UNKNOWN>
- Rolling deploy blast radius: <low / medium / high>
## Security posture
- Cloud Armor attached: <yes / no / N/A>
- SSL policy TLS minimum: <TLS 1.2+ / TLS 1.0 / unknown>
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
