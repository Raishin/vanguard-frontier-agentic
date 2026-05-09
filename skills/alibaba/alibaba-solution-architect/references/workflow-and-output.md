# Workflow and output contract

Use this reference only when performing a full architecture design, product selection review, or production-readiness pass.

## Design domains

Check these areas before giving a recommendation:

- Workload type, traffic pattern, latency requirements, and data volume
- Product selection trade-offs: compute, database, container, analytics, and load balancer tiers
- Multi-region and disaster recovery topology
- Security and compliance requirements (data residency, encryption, access control)
- Cost model: on-demand vs. subscription vs. savings plans

## Safe workflow

1. **Frame requirements**
   - Workload type and traffic characteristics:
   - Performance and availability targets (SLA/RTO/RPO):
   - Data classification and compliance driver:
   - Budget constraints:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the design**
   - What single points of failure exist?
   - What product capabilities are unverified vs. documented?
   - What can create unbounded cost?
   - What evidence is missing?
4. **Recommend the smallest safe next step**
   - Prefer proof-of-concept before production commitment.
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Solution Design: <scope>
## Executive summary
- Recommendation:
- Evidence level:
- Key trade-offs:
## Workload requirements
- Confirmed:
- Assumed:
- Out of scope:
## Product selection rationale
| Layer | Selected product | Alternatives considered | Rationale |
|---|---|---|---|
## Architecture topology
- Diagram description or component list:
## Data tier recommendations
- Primary store:
- Analytics / reporting:
- Caching / CDN:
## Security and compliance considerations
- Access control:
- Encryption:
- Compliance notes:
## Cost estimation approach
- Billing model:
- Key cost drivers:
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
