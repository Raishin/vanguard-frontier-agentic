# Workflow and output contract

Use this reference only when performing a full ELB traffic engineering review or production-readiness assessment for a Huawei Cloud load balancer configuration.

## Review domains

Check these areas before giving a verdict:

- ELB type: whether Dedicated or Shared ELB is appropriate for the throughput, protocol, and security requirements
- Listener protocol and TLS policy: whether HTTPS listeners enforce TLS-1-2 or stricter, and whether HTTP-only listeners are intentional
- WAF integration: whether WAF is enabled on public-facing listeners, whether the WAF instance is in the same region, and whether the security policy is in detection or block mode
- Health check configuration: whether HTTP health checks are used for Layer 7 workloads, and whether thresholds and intervals are appropriate
- Backend Server Group routing: whether weighted routing is used for releases and whether weights reflect current intended state
- Connection draining: whether draining is enabled and whether the timeout covers the longest in-flight request
- Cross-AZ distribution: whether backend members span multiple AZs and whether sticky sessions are required

## Safe workflow

1. **Frame scope**
   - ELB instance type and listener protocols in scope:
   - Region and account context:
   - Current-state evidence:
   - Required availability and security posture:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the configuration**
   - What happens to in-flight requests during a backend rolling deployment without connection draining?
   - What traffic reaches backends when WAF is in block mode with an untuned security policy?
   - What clients are blocked if TLS-1-2-Strict is enforced without client compatibility review?
   - What backends receive production traffic if stale routing weights were not reset after a release?
   - What evidence is missing to confirm the health check validates application readiness?
4. **Recommend the smallest safe action**
   - Prefer targeted fixes, staged changes, and verification steps.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud ELB Traffic Engineering Review: <scope>
## ELB type selection rationale
## Listener protocol and TLS policy assessment
## WAF integration status and security policy review
## Health check configuration and coverage gaps
## Backend server group routing and weight configuration
## Connection draining configuration review
## Cross-AZ traffic distribution and sticky session analysis
## Prioritized traffic engineering improvements
```

Each section must include an evidence level label.
