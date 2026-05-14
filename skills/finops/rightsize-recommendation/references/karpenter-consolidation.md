# Karpenter Consolidation Eligibility

## What consolidation does

Karpenter consolidation (enabled via `consolidationPolicy: WhenUnderutilized` or `WhenEmpty` in a NodePool) automatically removes underutilized nodes and reschedules their pods to better-packed nodes. This reduces idle node cost without requiring manual intervention.

For a pod to be a candidate for consolidation, Karpenter must be able to disrupt it safely and reschedule it onto a smaller or fewer nodes.

## Eligibility conditions

A pod is consolidation-eligible when all five conditions are met:

### 1. No blocking PodDisruptionBudget

A PodDisruptionBudget (PDB) blocks consolidation if it would prevent Karpenter from evicting the pod at the time of consolidation.

Blocking PDB configurations:
- `maxUnavailable: 0` — zero replicas can be disrupted; Karpenter cannot evict any pod.
- `minAvailable` equal to the current replica count — effectively blocks all evictions.
- `maxUnavailable: 0%` — same as `maxUnavailable: 0` in percentage form.

Non-blocking PDB configurations:
- `maxUnavailable: 1` or higher — at least one replica can be disrupted.
- `minAvailable` less than the current replica count — some replicas can be disrupted.

To check for blocking PDBs: the caller should inspect `kubectl get pdb -n <namespace>` output and confirm whether any PDB covers the workload.

### 2. No consolidation-blocking pod anti-affinity

Pod anti-affinity rules that spread replicas across nodes by `kubernetes.io/hostname` (host anti-affinity) can prevent Karpenter from co-locating pods that are currently on separate nodes.

If every replica of a Deployment has a `requiredDuringSchedulingIgnoredDuringExecution` anti-affinity rule against other replicas on the same host, Karpenter cannot pack them together, blocking node consolidation.

`preferredDuringSchedulingIgnoredDuringExecution` anti-affinity is soft and does not block consolidation.

### 3. No topologySpreadConstraints with WhenUnsatisfiable: DoNotSchedule

`topologySpreadConstraints` with `whenUnsatisfiable: DoNotSchedule` and a spread domain of `kubernetes.io/hostname` prevents Karpenter from scheduling multiple pods onto the same node, blocking consolidation of node pairs.

`whenUnsatisfiable: ScheduleAnyway` is soft and does not block consolidation.

### 4. No local storage (hostPath or local PV)

Pods that use `hostPath` volumes or PersistentVolumes backed by a `local` StorageClass are pinned to the node where the volume exists. Karpenter cannot reschedule these pods to a different node without data loss.

Pods using network-attached storage (EBS, Azure Disk, GCE PD, OCI Block Volume) are not pinned and are eligible for consolidation — the volume can be re-attached to the new node.

Note: EBS volumes in AWS have an availability-zone constraint; consolidation targets must be in the same AZ as the volume.

### 5. No nodeName selector pinning

A pod with `spec.nodeName` set to a specific node name is pinned to that node and cannot be rescheduled. Karpenter will not consolidate the host node as long as pinned pods remain on it.

Similarly, a `nodeSelector` that matches only one specific node (e.g., `kubernetes.io/hostname: <exact-node-name>`) effectively pins the pod.

## Summary eligibility table

| Condition | Eligible when | Blocks consolidation when |
|---|---|---|
| PodDisruptionBudget | No PDB, or PDB with `maxUnavailable >= 1` | PDB with `maxUnavailable: 0` or `minAvailable == replica count` |
| Pod anti-affinity | No anti-affinity, or `preferredDuring...` only | `requiredDuring...` host anti-affinity between replicas |
| TopologySpreadConstraints | No constraints, or `whenUnsatisfiable: ScheduleAnyway` | `whenUnsatisfiable: DoNotSchedule` on hostname topology |
| Storage | Network-attached PV only | `hostPath` volume or `local` StorageClass PV |
| Node selector | No `nodeName` and no single-node `nodeSelector` | `spec.nodeName` set or `nodeSelector` matching one node |

## Consolidation policy modes

| Policy | Behavior |
|---|---|
| `WhenEmpty` | Consolidates only fully empty nodes (all pods evicted or completed). Conservative. |
| `WhenUnderutilized` | Consolidates nodes when doing so would reduce total node count without violating pod scheduling constraints. Aggressive. |

For FinOps purposes, `WhenUnderutilized` provides the greater cost reduction but requires that pods pass all five eligibility conditions above.

## Disruption budget in Karpenter v0.33+

Karpenter v0.33 introduced a `disruption.budgets` field in the NodePool spec that limits the rate of node replacement across all consolidation actions. A low disruption budget (e.g., `nodes: "10%"`) slows consolidation speed but reduces service disruption risk.

The disruption budget does not affect eligibility; it affects when Karpenter will act on an eligible pod.
