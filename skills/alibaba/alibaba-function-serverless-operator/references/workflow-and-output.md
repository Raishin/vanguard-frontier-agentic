# Workflow and output contract

Use this reference only when performing a full serverless operations review, platform migration assessment, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Workload type classification and platform fit
- Function Compute trigger configuration, timeout, and concurrency limits
- SAE application scaling rules and MSE service registration
- EDAS application health and framework version support
- Cold start frequency and provisioned instance configuration
- Cost model: invocation-based vs. CU-based vs. instance-based

## Safe workflow

1. **Frame scope**
   - Workload type (event-driven / web app / microservice):
   - Traffic pattern (steady / bursty / scheduled):
   - Latency requirements:
   - Compliance requirements:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live FC/SAE/EDAS console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What workloads have cold start latency issues?
   - What functions lack authentication on internet-facing triggers?
   - What auto-scaling configurations have no upper concurrency limit?
   - What EDAS framework versions are end-of-support?
4. **Recommend the smallest safe action**
   - Prefer canary deployment before full rollout.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Serverless Review: <scope>
## Executive verdict
- Status: HEALTHY / ATTENTION NEEDED / ACTION REQUIRED
- Biggest risk:
- Evidence level:
## Workload type classification
| Workload | Type | Current platform | Recommended platform | Rationale |
|---|---|---|---|---|
## Platform selection rationale
- FC use cases:
- SAE use cases:
- EDAS use cases:
## Function and app health
- Health summary:
- Error rates:
## Auto-scaling configuration
- Current config:
- Findings:
## Cold start assessment
- Affected workloads:
- Mitigation in place:
## Recommendations
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Open risks
- <risk or explicit none>
```
