# Workflow and output contract

Use this reference only when performing a full network design, connectivity review, or production-readiness pass.

## Design domains

Check these areas before giving a recommendation:

- VPC CIDR planning, subnet segmentation, and availability zone distribution
- Security group rules and Network ACL baselines
- CEN transit router configuration and bandwidth package sizing
- Load balancer type selection and listener configuration
- Express Connect or Smart Access Gateway for private/branch connectivity
- Monitoring and flow log coverage

## Safe workflow

1. **Frame requirements**
   - Connectivity endpoints (regions, accounts, on-premises, branches):
   - Throughput, latency, and availability targets:
   - Compliance and data residency constraints:
   - Existing topology constraints:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live VPC/CEN console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the design**
   - What single points of failure exist in the routing path?
   - What CEN bandwidth limits or quotas apply?
   - What security group rules are overly permissive?
   - What evidence is missing?
4. **Recommend the smallest safe next step**
   - Prefer staged rollout and rollback plan before production traffic cutover.
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Network Design: <scope>
## Executive summary
- Recommendation:
- Evidence level:
- Key trade-offs:
## Connectivity requirements
- Confirmed:
- Assumed:
- Out of scope:
## VPC topology recommendation
- Region(s) and VPC CIDR allocation:
- Subnet segmentation:
- Availability zone distribution:
## CEN design
- Transit router configuration:
- Bandwidth package sizing:
- Cross-account peering:
## Load balancer selection rationale
| Endpoint | Selected LB type | Rationale |
|---|---|---|
## Express Connect / Smart Access Gateway design
- Private circuit or SD-WAN approach:
- Bandwidth and redundancy:
## Security group and ACL review
- Baseline rules:
- Findings:
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
