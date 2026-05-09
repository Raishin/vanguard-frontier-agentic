# Workflow and output contract

Use this reference only when performing a full serverless production readiness review.

## Review domains

Check these areas before giving a recommendation:

- FC3 version confirmation (v2 vs v3 — invocation model differs)
- Cold start: runtime, initialization code size, provisioned concurrency configuration
- VPC binding: present/absent, cold start overhead accepted, private resource access requirements
- RAM role: bound role present, AccessKey in environment variables (critical finding if present), least-privilege policy
- SAE resource limits: memory and CPU limits set on all production applications
- Concurrency limits: max concurrency set for FC functions to prevent runaway invocation costs
- ARMS tracing: enabled for all production services, trace sampling rate configured
- Security group: egress rules, inbound rules, VPC isolation scope

## Safe workflow

1. **Frame the workload**
   - FC3 function or SAE application name (sanitized):
   - Runtime (for FC3):
   - FC version (v2 or v3):
   - Latency SLA requirement:
   - Private resource access requirements (RDS, Redis, internal services):
2. **Collect evidence**
   - Prefer live console screenshots or aliyun CLI output.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the configuration**
   - Is AccessKey ID/Secret present in any environment variable or function code? (critical finding)
   - Is ARMS tracing enabled — if not, how is cross-service latency diagnosed?
   - Does the function have a concurrency limit — if not, what is the blast radius of a runaway invocation?
   - Is VPC binding required but absent?
   - Are SAE application resource limits set?
4. **Recommend the smallest safe next step**
   - Prioritize blockers: AccessKey in env vars > missing VPC binding for private access > no ARMS tracing > no concurrency limit > no SAE resource limits.
   - Production deployment is blocked if AccessKey ID/Secret is present in environment variables or function code.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Serverless Production Readiness: <service name>
## Production readiness verdict
- Verdict: READY / CONDITIONALLY READY / BLOCKED
- Blockers:
- Evidence level:
## FC version and invocation model
- FC version (v2/v3):
- Invocation model:
## Cold start and provisioned concurrency
- Runtime:
- Estimated cold start duration:
- Provisioned concurrency configured:
- Cost implications accepted:
## VPC binding
- VPC binding present:
- Private resource access requirements met:
- Cold start overhead acceptable:
## RAM role and credential hygiene
- RAM role bound: PASS / FAIL
- AccessKey in environment variables: PASS / FAIL (CRITICAL if FAIL)
- Policy least-privilege assessment:
## Memory, CPU, and concurrency limits
- Memory limit set:
- CPU limit set:
- Max concurrency limit set:
## ARMS tracing and observability
- ARMS tracing enabled:
- Trace sampling rate:
- Coverage gaps:
## Security group and network access
- Inbound rules:
- Egress rules:
- VPC isolation scope:
## Recommended actions
1. <action> — priority: <critical/high/medium>, effort: <low/medium/high>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
