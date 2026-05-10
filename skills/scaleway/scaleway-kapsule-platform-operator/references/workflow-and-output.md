# Workflow and output contract

Use this reference when performing a full Kapsule cluster readiness assessment, node pool design review, CNI selection analysis, or version upgrade planning pass.

## Review domains

Check these areas before giving a verdict:

- CNI selection: Cilium (recommended for NetworkPolicy enforcement), Calico, or Kilo (WireGuard overlay for multi-cloud) — **immutable after cluster creation; flag prominently**
- Control plane: Kubernetes version currency (flag if more than two minor versions behind), upgrade path, **no downgrade path after upgrade**
- Node pools: zone binding, pool count, autoscaling bounds, instance type per pool, multi-pool multi-zone coverage
- Placement groups: `max_availability` (soft HA, preferred for production) vs `enforced` (hard constraint — may block pod scheduling)
- PodDisruptionBudgets: coverage across all workload namespaces; unprotected Deployments and StatefulSets are a gap
- Workload scheduling: affinity and anti-affinity rules, node selectors, taint/toleration consistency across pools

## Safe workflow

1. **Frame scope**
   - Cluster environment (production / staging / development):
   - Workload types (stateless, stateful, batch):
   - Compliance or uptime requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized Terraform `scaleway_k8s_cluster` / `scaleway_k8s_pool` resource definitions or sanitized `kubectl` / `scw k8s` CLI output.
   - Label each finding as `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
   - If cluster details were not provided, state that explicitly before proceeding.
3. **Stress-test risk**
   - Is the CNI choice locked in — and does it match the workload's NetworkPolicy requirements?
   - Is the control-plane version within two minor versions of current Kubernetes release?
   - Are all zones covered with node pools sized for failover headroom?
   - Which namespaces lack PDB coverage?
   - Is `enforced` placement group in use — and could it cause unschedulable pods under node failure?
4. **Recommend the smallest safe improvement**
   - Prefer adding PDB coverage, expanding node pools, and adjusting autoscaling before recommending cluster recreation.
   - Treat CNI immutability and version irreversibility as hard constraints — never recommend a path that silently ignores them.
   - If the safest action is to gather cluster state before recommending changes, say that plainly.

## Output contract

Return this structure:

```markdown
# Scaleway Kapsule Platform Readiness: <scope>

## Readiness verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
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

## Irreversibility warnings
- CNI: <confirmed or unknown>
- Control-plane version: <confirmed or unknown>

## Residual risk
- <risk or explicit none>
```
