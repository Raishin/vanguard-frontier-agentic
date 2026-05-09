# Workflow and Output — Alibaba Cloud Live ACK Rollout Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm the active RAM principal and its policy scope.
2. Describe the target cluster to confirm cluster ID, type, region, and current version via the ACK console or API.
3. List all node pools and their current versions:
   ```
   aliyun cs GET /clusters/<CLUSTER_ID>/nodepools
   ```

### Phase 2: PDB Audit

4. List all PodDisruptionBudgets across all namespaces:
   ```
   kubectl get pdb --all-namespaces -o wide
   ```
5. Identify any PDB with `DISRUPTIONS ALLOWED: 0` — these are blocking conditions.
6. For each blocking PDB, identify the affected workload and its node pool placement.

### Phase 3: Rollout Strategy Review

7. For node pool scale-in: confirm drain posture — identify any pods with local storage or no eviction toleration.
8. For Deployment rollouts, review rollout history:
   ```
   kubectl rollout history deployment/<NAME> -n <NAMESPACE>
   ```
9. For cluster version upgrades: confirm the target version is available and document that the operation is irreversible.

### Phase 4: Approval Gate

10. Present all evidence to the operator: cluster identity, type, current vs. target version, PDB findings, drain posture.
11. Require explicit written approval including acknowledgment that cluster version upgrades are irreversible.
12. Do not proceed until approval is received.

### Phase 5: Execution

13. Execute the approved operation:
    - Node pool scale via console or:
      ```
      aliyun cs POST /clusters/<CLUSTER_ID>/nodepools/<NODEPOOL_ID>/scale
      ```
    - Cluster version upgrade via console or ACK API.
    - Deployment rollout:
      ```
      kubectl set image deployment/<NAME> <CONTAINER>=<IMAGE> -n <NAMESPACE>
      ```
14. Monitor the operation:
    ```
    kubectl rollout status deployment/<NAME> -n <NAMESPACE>
    aliyun cs GET /clusters/<CLUSTER_ID>/tasks/<TASK_ID>
    ```

### Phase 6: Post-Change Verification

15. Confirm all nodes are Ready at the new version:
    ```
    kubectl get nodes -o wide
    ```
16. Confirm no pods are in error states:
    ```
    kubectl get pods --all-namespaces | grep -v Running | grep -v Completed
    ```
17. Re-audit PDBs to confirm disruption budgets are healthy.
18. Check CloudMonitor for elevated error rates or latency anomalies in the 15 minutes following the change.

## Expected Output Format

The agent response for an ACK rollout operation must include:

```
CLUSTER IDENTITY
  Cluster ID:     <cluster-id>
  Cluster Name:   <cluster-name>
  Cluster Type:   <managed / dedicated / serverless>
  Region:         <region>
  Current Version: <x.y.z>
  Target Version:  <x.y.z>

NODE POOL
  Pool ID:        <nodepool-id>
  Pool Name:      <nodepool-name>
  Current Version: <x.y.z>
  Node Count:     <N>

PDB AUDIT
  Blocking PDBs:  [NONE | <list of PDB names with ALLOWED=0>]
  Non-blocking:   <count>

APPROVAL STATUS
  Operator:       <RAM principal>
  Approved:       [YES / NO / PENDING]
  Irreversibility acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason] OR [EXECUTING — task ID] OR [COMPLETE]

ROLLBACK POSTURE
  [NOT POSSIBLE — cluster version upgrade is one-way]
  OR
  [AVAILABLE — kubectl rollout undo deployment/<NAME>]

VERIFICATION
  Nodes Ready:    <count>/<total>
  Pods Healthy:   <count>/<total>
  PDB Status:     OK
  Error Rate:     <value or "not yet checked">
```
