# Workflow and output contract

Use this reference when gating or executing a live Scaleway Kapsule cluster or node pool mutation.

## Sequence — enforce this order, do not skip steps

1. **Confirm target identity**
   - Cluster ID and region/zone must be explicitly stated and confirmed by the user.
   - If target is ambiguous or absent, stop immediately and ask. Do not proceed.
2. **Verify cluster health before any mutation**
   - API server reachable; all nodes in Ready state; no unhealthy node pools.
   - Prefer `scw k8s cluster get` and `kubectl get nodes` as live evidence.
   - If health cannot be confirmed, stop. Document-based or inferred health is insufficient.
3. **Complete PDB audit**
   - Run `kubectl get pdb --all-namespaces` and review all workload namespaces.
   - Unprotected Deployments and StatefulSets must be explicitly documented and accepted by the user before proceeding.
4. **Confirm explicit approval**
   - Require a human sign-off token, ticket reference, or written approval.
   - Do not treat a plan, a question, or a "what if" as approval.
5. **Confirm rollback plan**
   - Version upgrade: prior version noted; node pool replacement path identified if rollback requires pool recreation.
   - Node pool deletion: workload migration plan confirmed; no unschedulable pods at reduced pool count.
   - Scaling: resource headroom verified for current workloads at new pool size.
6. **Execute the bounded action**
   - Keep the command scoped to the confirmed cluster ID and zone.
   - Do not chain mutations without re-confirming health between steps.
7. **Post-mutation verification**
   - Verify cluster health, node readiness, workload pod status, and any PDB violations after the change.
   - Report final state with evidence labels.

## Output shape

Return sections in this order:

1. Target confirmation — cluster ID, region/zone, confirmed by user
2. Cluster health evidence — source and label
3. PDB audit result — namespaces checked, gaps documented
4. Approval status — token, ticket, or written sign-off
5. Rollback plan — documented and accepted
6. Proposed or executed action — scoped command
7. Post-change verification — node status, workload status, any residual risk
8. Open risks or explicit STOP with reason

Keep all command output sanitized. Do not include raw credential material, secret values, or unredacted project/organization IDs.
