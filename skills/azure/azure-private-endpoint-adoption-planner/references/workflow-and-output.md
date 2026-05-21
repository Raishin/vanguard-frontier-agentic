# Workflow and Output Contract

## Safe Workflow

1. **Classify the adoption scope**
   - target Azure service or services,
   - single workload or shared platform service,
   - one VNet, hub-spoke, Virtual WAN-adjacent, or hybrid environment,
   - single subscription or multi-subscription.
2. **Identify the real consumers**
   - which workloads need access,
   - whether consumers are concentrated in one spoke or many,
   - whether on-premises or cross-region consumers exist,
   - whether the service is platform-shared or workload-specific.
3. **Choose the placement decision deliberately**
   - hub placement when many spokes need the same service and shared governance is intentional,
   - workload-local placement when access should stay narrow and workload-owned,
   - reject centralization-by-habit when it increases blast radius without a clear operational benefit.
4. **Map DNS dependencies before approving anything**
   - private DNS zone required or not,
   - which VNets must link to which zones,
   - whether custom DNS or Azure DNS Private Resolver changes the path,
   - whether multi-network designs risk DNS override or split-resolution confusion.
5. **Map routing and access implications**
   - private endpoints inject interface-level reachability, not generic service exposure,
   - check whether access depends on peering, hybrid reachability, or centralized inspection paths,
   - explicitly call out `/32` route implications where Microsoft guidance says they matter,
   - verify that security controls still allow the intended flows.
6. **Assess centralized versus workload-local trade-offs**
   - central hub endpoint simplifies shared consumption but couples unrelated workloads,
   - local endpoint improves least privilege but increases endpoint count, DNS linking, and operational repetition,
   - Azure Monitor Private Link needs extra caution because shared DNS can create tenant-wide surprises.
7. **Return a bounded rollout and validation plan**
   - nonproduction-first,
   - DNS validation before workload cutover,
   - access-path validation from each consumer network,
   - rollback path for zone-link, endpoint, and route-adjacent changes.

## Role-Specific Stress Checks

- If the design says "put every private endpoint in the hub," challenge it. That is often laziness disguised as architecture.
- If the design says "put every private endpoint in each workload spoke," challenge the duplication, DNS sprawl, and operating cost.
- If the answer does not explain who owns private DNS zones and VNet links, it is incomplete.
- If the design assumes peering alone solves name resolution, it is wrong.
- If Azure Monitor Private Link is involved, check for AMPLS and shared-DNS side effects before blessing the design.
- If on-premises consumers exist, check resolver and forwarding design explicitly.
- If the rollout changes private access but ignores rollback of DNS links or endpoint cutover, it is not safe.

## Output Template

```markdown
# Azure Private Endpoint Adoption Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Primary decision: HUB / SPOKE / MIXED
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Target service(s):
- Consumer network(s):
- Subscription / resource-group boundary:
- Shared or workload-specific:
- Requested action:

## Placement recommendation
- Recommended pattern:
- Why:
- Rejected alternative:
- Trade-off accepted:

## DNS requirements
- Private DNS zone(s):
- Required VNet links:
- Custom DNS / resolver dependency:
- Failure mode if omitted:

## Routing and security implications
- Reachability path:
- `/32` route or path caveat:
- Peering / hybrid dependency:
- Access-control impact:

## Validation plan
1.
2.
3.

## Open questions
-
```

## Red Flags

- The request asks for private endpoints but does not name the consuming networks.
- The plan centralizes endpoints without naming the DNS-zone owner.
- The design assumes private DNS "just works" across peered or hybrid networks.
- The recommendation ignores `/32` route behavior or access-path consequences.
- Azure Monitor private link is proposed with multiple DNS-sharing networks and no AMPLS design review.
- The rollout assumes production cutover before DNS validation from each consumer path.
