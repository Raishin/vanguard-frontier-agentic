# Workflow and Output — Huawei Live CCE Rollout Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm account ID, enterprise project, and active IAM principal.
2. Describe the target cluster to confirm name, region, and current version:
   - CCE console > Clusters > select cluster > Overview tab
   - Or: hcloud cce cluster-show `<CLUSTER_ID>`
3. List all node pools and their current versions:
   - CCE console > Clusters > select cluster > Node Pools tab

### Phase 2: Workload PDB Audit

4. List all PodDisruptionBudgets across all namespaces:
   ```
   kubectl get pdb --all-namespaces -o wide
   ```
5. Identify any PDB with `DISRUPTIONS ALLOWED: 0` — these are blocking conditions.
6. For each blocking PDB, identify the affected workload and its node pool placement.

### Phase 3: Addon Compatibility Check

7. List currently installed addons and their versions:
   - CCE console > Clusters > select cluster > Add-ons tab
8. Cross-reference addon versions against the target Kubernetes version compatibility matrix in official CCE docs.
9. Flag any addon that does not support the target cluster version — addon must be upgraded or replaced before cluster version upgrade proceeds.

### Phase 4: Node Pool Drain Verification (Scale-Down Only)

10. For scale-down operations, confirm pod reschedulability:
    ```
    kubectl get pods -o wide --all-namespaces | grep <NODE_NAME>
    ```
11. Confirm no pods have `nodeName` hard pinning (spec.nodeName set directly).
12. Confirm PVCs with `ReadWriteOnce` access mode are not pinned to the nodes being drained.

### Phase 5: Approval Gate

13. Present all evidence to the operator: cluster identity, current vs. target version, PDB findings, addon compatibility, drain posture.
14. Require explicit written approval including acknowledgment that CCE cluster version downgrades are not supported.
15. Do not proceed until approval is received.

### Phase 6: Execution

16. Execute the approved operation:
    - Node pool upgrade: CCE console > Node Pools > select pool > Upgrade Node Pool
    - Cluster version upgrade: CCE console > Clusters > select cluster > Upgrade
    - Deployment rollout:
      ```
      kubectl set image deployment/<NAME> <CONTAINER>=<NEW_IMAGE> -n <NAMESPACE>
      ```
17. Monitor the operation:
    ```
    kubectl rollout status deployment/<NAME> -n <NAMESPACE>
    ```
    Or monitor via CCE console > Clusters > Operations.

### Phase 7: Post-Change Verification

18. Confirm all nodes are Ready at the new version:
    ```
    kubectl get nodes -o wide
    ```
19. Confirm no pods are in error states:
    ```
    kubectl get pods --all-namespaces | grep -v Running | grep -v Completed
    ```
20. Re-audit PDBs to confirm disruption budgets are healthy.
21. Verify addon pods are running in kube-system:
    ```
    kubectl get pods -n kube-system
    ```
22. Check CES (Cloud Eye) for elevated error rates or latency anomalies in the 15 minutes following the upgrade.

## Expected Output Format

The agent response for a CCE rollout operation must include:

```
CLUSTER IDENTITY
  Account:         <account-id>
  Enterprise Project: <enterprise-project-name>
  Cluster:         <cluster-name>
  Region:          <region>
  Current Version: <x.y.z>
  Target Version:  <x.y.z>

NODE POOL
  Pool Name:       <pool-name>
  Current Version: <x.y.z>
  Node Count:      <N>

WORKLOAD PDB AUDIT
  Blocking PDBs:   [NONE | <list of PDB names with ALLOWED=0>]
  Non-blocking:    <count>

ADDON COMPATIBILITY
  CoreDNS:         <version> — [COMPATIBLE / INCOMPATIBLE / UPGRADE REQUIRED]
  NGINX Ingress:   <version> — [COMPATIBLE / INCOMPATIBLE / UPGRADE REQUIRED]
  Other addons:    <list>

APPROVAL STATUS
  Operator:        <identity>
  Approved:        [YES / NO / PENDING]
  Irreversibility acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

ROLLBACK POSTURE
  [NOT POSSIBLE — CCE cluster version downgrade is not supported]
  OR
  [AVAILABLE — kubectl rollout undo deployment/<NAME>]

VERIFICATION
  Nodes Ready:     <count>/<total>
  Pods Healthy:    <count>/<total>
  PDB Status:      OK
  Addons:          OK
  CES Error Rate:  <value or "not yet checked">
```
