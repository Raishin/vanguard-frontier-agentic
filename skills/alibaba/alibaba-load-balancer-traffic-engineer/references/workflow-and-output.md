# Workflow and output contract

Use this reference only when performing a full load balancer traffic engineering review or migration assessment.

## Review domains

Check these areas before giving a recommendation:

- LB type: CLB / ALB / NLB / GA — is the correct type selected for the workload protocol and routing needs?
- Health check: type (HTTP/HTTPS/TCP), interval, healthy/unhealthy threshold, path — are these configured correctly for the LB type?
- WAF integration: ALB-WAF binding present for regulated HTTP workloads?
- SSL/TLS: certificate binding, security policy, TLS version enforcement (1.2+ required for PCI-DSS / MLPS 2.0)
- Backend configuration: server group type, backend instance health, connection draining
- Traffic distribution: algorithm, session persistence, weights
- Cross-region GA: is there an actual latency or reliability need that GA addresses?

## Safe workflow

1. **Frame the workload**
   - Protocol: HTTP / HTTPS / TCP / UDP:
   - Compliance requirements: PCI-DSS / MLPS 2.0 / none:
   - Traffic volume and peak throughput:
   - Cross-region acceleration need (yes/no with evidence):
2. **Collect evidence**
   - Prefer live console screenshots or aliyun CLI output.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the configuration**
   - Is the LB type capable of the required routing rules?
   - Are health check intervals aggressive enough to detect backend failure quickly?
   - Is WAF enabled for public-facing HTTP(S) endpoints subject to compliance requirements?
   - Is TLS 1.0/1.1 blocked via security policy?
   - What happens to in-flight requests during backend draining?
4. **Recommend the smallest safe next step**
   - Prioritize by risk: wrong LB type > missing WAF > TLS downgrade > inadequate health checks > missing connection draining.
   - If CLB-to-ALB migration is needed, scope it as a separate planned migration task.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Load Balancer Traffic Engineering Review: <workload scope>
## Executive summary
- Configuration verdict:
- Evidence level:
- Critical findings:
## LB type selection assessment
| Workload | Current LB type | Recommended LB type | Rationale |
|---|---|---|---|
## Health check configuration
| Listener | Health check type | Interval | Threshold | Status |
|---|---|---|---|---|
## WAF integration and security posture
- WAF enabled:
- WAF rule set:
- Compliance coverage:
## Traffic distribution and backend capacity
- Algorithm:
- Session persistence:
- Backend instance count and health:
- Connection draining:
## SSL/TLS termination
- Certificate bound:
- Security policy (TLS version):
- TLS 1.0/1.1 blocked:
## Cross-region acceleration assessment
- GA in use:
- Acceleration need confirmed:
- Cost vs. latency trade-off:
## Recommended traffic engineering actions
1. <action> — priority: <critical/high/medium>, effort: <low/medium/high>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
