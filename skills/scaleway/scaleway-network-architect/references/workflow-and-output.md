# Workflow and output contract

Use this reference when performing a full Scaleway network topology review, security group audit, Load Balancer assessment, or multi-zone HA design pass.

## Review domains

Check these areas before giving a verdict:

- VPC layout: regional VPC, number of Private Networks, project isolation, inter-project routing needs
- Private Network attachment: zone-boundary consistency (instances and Kapsule nodes must be attached per zone); cross-zone gaps where L2 adjacency is required
- Security groups: inbound/outbound rule breadth, 0.0.0.0/0 inbound exposure, stateful behavior for TCP/UDP, per-instance application, missing egress restrictions
- Load Balancer: front-end protocol (HTTP/HTTPS/TCP), backend health check configuration, TLS termination, sticky sessions, zone coverage
- Placement groups: `max_availability` (soft HA, preferred) vs `enforced` (hard constraint — may block scheduling under node failure); zone-scoped application
- Flexible IPs: unassigned or stale IPs incurring cost; mobility readiness for failover
- Multi-zone resilience: coverage across fr-par-1/2/3, nl-ams-1/2, pl-waw-1/2/3; single-zone exposure; cross-zone routing gaps

## Safe workflow

1. **Frame scope**
   - Project and environment in scope:
   - Zones and regions involved:
   - Workload types (stateless instances, Kapsule nodes, managed DBs):
   - HA or compliance requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized Terraform state (`scaleway_vpc`, `scaleway_vpc_private_network`, `scaleway_instance_security_group`, `scaleway_lb`) or sanitized network diagrams.
   - Label each finding as `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
   - If network topology was not provided, state that explicitly before proceeding.
3. **Stress-test risk**
   - Which instances are exposed to 0.0.0.0/0 inbound on non-public ports?
   - Which zones lack Private Network attachment, leaving inter-service traffic on public interfaces?
   - Where does the Load Balancer health check fail silently under zone failure?
   - Which placement group policies could leave instances unschedulable under node replacement?
   - Is there any single-zone dependency that would cause an outage on zone failure?
4. **Recommend the smallest safe network improvement**
   - Prefer narrowing security group rules, adding health checks, and expanding Private Network attachment before recommending topology restructuring.
   - Flag `enforced` placement group risk explicitly before recommending it for production.
   - If the safest action is to gather network topology evidence before recommending changes, say that plainly.

## Output contract

Return this structure:

```markdown
# Scaleway Network Architecture Review: <scope>

## Topology verdict
- Status: SECURE / SECURE WITH RISKS / INSECURE / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:

## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:

## Findings
| Severity | Area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|

## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>

## Zone-boundary gaps
- <zone isolation issues or explicit none>

## Residual risk
- <risk or explicit none>
```
