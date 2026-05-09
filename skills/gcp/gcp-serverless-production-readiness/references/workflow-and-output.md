# Workflow and output contract

Use this reference only when performing the full serverless production readiness review, security audit, or traffic splitting safety assessment.

## Review domains

Check these areas before giving a verdict:
- Cold start and min-instances: min-instances value, startup latency SLA, startup probe configuration, CPU boost on startup
- Memory and CPU: memory allocation vs. workload peak, CPU allocation mode (always-on vs. request-only), CPU throttling during idle
- Concurrency: concurrency limit vs. workload type (stateful/CPU-bound/I/O-bound), max-instances throttling
- VPC connector: connector existence, egress setting (all-traffic vs. private-ranges-only), throughput tier, private IP vs. Auth Proxy for Cloud SQL
- Secret hygiene: Secret Manager volume mount vs. env var reference vs. raw secret in env var, secret version pinning, IAM accessor binding
- CMEK: CMEK key configured, key ring region alignment with service region, key rotation policy
- Traffic splitting: revision traffic split percentages, canary testing procedure, rollback revision availability, rollback procedure documented
- Authentication: --allow-unauthenticated vs. IAM-authenticated, ingress setting (all / internal / internal-and-cloud-load-balancing)
- Service account: identity bound to service, roles assigned, least privilege verification

## Safe workflow

1. **Frame scope**
   - Service name and runtime (Cloud Run / Cloud Functions gen2):
   - Workload type (latency-sensitive / batch / stateful / CPU-bound):
   - Private resource dependencies (Cloud SQL / Memorystore / internal GKE):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Are secrets stored in environment variables instead of Secret Manager?
   - Is min-instances=0 on a latency-sensitive workload?
   - Is concurrency too high for a stateful or CPU-bound workload?
   - Is a VPC connector missing for private resource access?
   - Is a rollback plan documented for traffic splits?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Serverless Production Readiness Review: <scope>
## Executive verdict
- Status: PRODUCTION READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest blocker:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Configuration summary
- min-instances: <value or UNKNOWN>
- concurrency: <value or UNKNOWN>
- VPC connector: <attached / missing / not required>
- Secrets in env vars: <none detected / detected / unknown>
- Authentication: <IAM / unauthenticated / unknown>
## Traffic splitting posture
- Current split: <percentages or UNKNOWN>
- Rollback revision available: <yes / no / unknown>
- Rollback procedure documented: <yes / no>
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
