# Workflow and Output Contract

## Safe Workflow

1. **Classify the cluster**
   - production, preproduction, dev/test, or shared platform,
   - single-region or multi-region,
   - public API server or private cluster,
   - stateless-heavy, stateful, latency-sensitive, regulated, or mixed workload.
2. **Map ownership**
   - who owns cluster lifecycle,
   - who owns workloads,
   - who owns networking, identity, secrets, and observability,
   - whether responsibilities are explicit or dangerously fuzzy.
3. **Check baseline platform shape**
   - node-pool separation for system versus user workloads,
   - cluster/network model,
   - ingress pattern,
   - IP capacity assumptions,
   - availability-zone and region posture where relevant.
4. **Challenge identity and secret assumptions**
   - workload identity versus legacy credential patterns,
   - cluster identity and kubelet identity boundaries,
   - Key Vault or external secret flow,
   - least-privilege posture and scope.
5. **Check traffic controls**
   - network policy presence and intent,
   - east-west restrictions,
   - ingress exposure,
   - egress control assumptions,
   - private dependency access and DNS implications.
6. **Check scaling realism**
   - node autoscaling,
   - workload HPA/VPA assumptions,
   - headroom policy,
   - IP exhaustion risk,
   - pod density and node sizing assumptions.
7. **Check upgrade safety**
   - supported version posture,
   - in-place versus blue-green strategy,
   - max surge / max unavailable assumptions,
   - PDB behavior,
   - undrainable-node risk,
   - rollback and validation checkpoints.
8. **Check observability and operations**
   - cluster and workload telemetry,
   - upgrade and failure visibility,
   - alert quality,
   - runbooks,
   - on-call handoff clarity,
   - recovery drill evidence.
9. **Return a go / no-go style verdict**
   - what is credible,
   - what is fragile,
   - what is missing,
   - and what must be validated before any change.

## Role-Specific Stress Checks

- Reject “AKS is managed so upgrades are easy” as shallow thinking. Managed control plane does not remove node-pool, PDB, IP, or workload-compatibility risk.
- Reject “we use Kubernetes so we are cloud-native” as meaningless unless operator discipline, upgrade cadence, and observability are proven.
- If production has no tested rollback path for upgrades, call it out as not operationally ready.
- If workloads still rely on long-lived secrets where workload identity should be used, flag it.
- If pod-to-pod traffic is effectively flat and unrestricted, flag the lateral-movement risk.
- If the cluster subnet is tight and surge or pod density assumptions are hand-wavy, flag IP exhaustion risk.
- If the platform team cannot explain system versus user node-pool separation, treat that as an operational maturity gap.
- If the cluster is treated as immutable infrastructure, require a credible replacement-cluster strategy; otherwise require an in-place upgrade strategy.

## Output Template

```markdown
# AKS Platform Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest risk:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Cluster context
- Environment:
- Region/subscription boundary:
- Cluster exposure model:
- Workload profile:
- Ownership model:

## Findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Upgrade and rollback posture
- Current strategy:
- Blocking gaps:
- Validation checkpoints:
- Rollback path:

## Security and identity posture
- Workload identity:
- Secret flow:
- Network policy / ingress / egress:

## Safe next actions
1.
2.
3.

## Open questions
- 
```

## Red Flags

- The team wants production AKS guidance but cannot state cluster version, node-pool layout, or upgrade ownership.
- The design depends on surge upgrades but there is no subnet headroom calculation.
- The plan assumes Pod Disruption Budgets help, but nobody tested whether they block drains.
- The platform still depends on static secrets even though Microsoft Entra Workload ID is the safer target model.
- The answer claims high availability without distinguishing zone resilience, regional failure, and workload replication behavior.
- The answer recommends AKS by default without testing whether the workload actually needs Kubernetes complexity.
