# Workflow and output contract

Use this reference only when performing the full review, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Connectivity requirements (on-premises, multi-cloud, internet ingress/egress)
- VPC design (global topology, subnet CIDR planning, secondary ranges for GKE)
- Shared VPC configuration (host project, service projects, subnet IAM)
- Hybrid connectivity (Cloud Interconnect type, Cloud VPN tunnels, BGP sessions)
- DNS architecture (Cloud DNS zones, Private Google Access, DNS peering)
- NAT design (Cloud NAT per region, port allocation)
- Security perimeter (Cloud Armor policies, VPC firewall rules, firewall policies, hierarchical firewall policies)

## Safe workflow

1. **Frame scope**
   - Workload/project/region/environment:
   - Connectivity requirements (on-prem bandwidth, latency SLA):
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official GCP docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What firewall rules allow overly broad access?
   - What resources have external IPs that should be private?
   - What subnets lack Private Google Access?
   - What regions lack Cloud NAT where required?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Network Architect: <scope>
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
