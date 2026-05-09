# Workflow and Output — GCP Live GKE Rollout Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm GCP project ID and active gcloud account:
   ```
   gcloud config get-value project
   gcloud auth list
   ```
2. Describe the target cluster to confirm name, region, and current version:
   ```
   gcloud container clusters describe <CLUSTER> --region <REGION> --project <PROJECT>
   ```
3. List all node pools and their current versions:
   ```
   gcloud container node-pools list --cluster <CLUSTER> --region <REGION> --project <PROJECT>
   ```

### Phase 2: PDB Audit

4. List all PodDisruptionBudgets across all namespaces:
   ```
   kubectl get pdb --all-namespaces -o wide
   ```
5. Identify any PDB with `DISRUPTIONS ALLOWED: 0` — these are blocking conditions.
6. For each blocking PDB, identify the affected workload and its node pool placement.

### Phase 3: Rollout Strategy Review

7. Review surge upgrade settings on the target node pool:
   ```
   gcloud container node-pools describe <POOL> --cluster <CLUSTER> --region <REGION>
   ```
   Check `upgradeSettings.maxSurge` and `upgradeSettings.maxUnavailable`.
8. For Deployment rollouts, review rollout history:
   ```
   kubectl rollout history deployment/<NAME> -n <NAMESPACE>
   ```

### Phase 4: Approval Gate

9. Present all evidence to the operator: cluster identity, current vs. target version, PDB findings, surge settings.
10. Require explicit written approval including acknowledgment that node pool upgrades are irreversible.
11. Do not proceed until approval is received.

### Phase 5: Execution

12. Execute the approved operation:
    - Node pool upgrade:
      ```
      gcloud container node-pools upgrade <POOL> --cluster <CLUSTER> --region <REGION> --cluster-version <VERSION>
      ```
    - Control-plane upgrade:
      ```
      gcloud container clusters upgrade <CLUSTER> --master --cluster-version <VERSION> --region <REGION>
      ```
    - Deployment rollout:
      ```
      kubectl set image deployment/<NAME> <CONTAINER>=<NEW_IMAGE> -n <NAMESPACE>
      ```
13. Monitor the operation:
    ```
    gcloud container operations list --filter="status=RUNNING"
    kubectl rollout status deployment/<NAME> -n <NAMESPACE>
    ```

### Phase 6: Post-Change Verification

14. Confirm all nodes are Ready at the new version:
    ```
    kubectl get nodes -o wide
    ```
15. Confirm no pods are in error states:
    ```
    kubectl get pods --all-namespaces | grep -v Running | grep -v Completed
    ```
16. Re-audit PDBs to confirm disruption budgets are healthy.
17. Check Cloud Monitoring for elevated error rates or latency anomalies in the 15 minutes following the upgrade.

## Expected Output Format

The agent response for a GKE rollout operation must include:

```
CLUSTER IDENTITY
  Project:        <project-id>
  Cluster:        <cluster-name>
  Region:         <region>
  Current Version: <x.y.z-gke.N>
  Target Version:  <x.y.z-gke.N>

NODE POOL
  Pool Name:      <pool-name>
  Current Version: <x.y.z-gke.N>
  Surge Max:      <N>
  Max Unavailable: <N>

PDB AUDIT
  Blocking PDBs:  [NONE | <list of PDB names with ALLOWED=0>]
  Non-blocking:   <count>

APPROVAL STATUS
  Operator:       <identity>
  Approved:       [YES / NO / PENDING]
  Irreversibility acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason] OR [EXECUTING — operation ID] OR [COMPLETE]

ROLLBACK POSTURE
  [NOT POSSIBLE — node pool upgrade is one-way]
  OR
  [AVAILABLE — kubectl rollout undo deployment/<NAME>]

VERIFICATION
  Nodes Ready:    <count>/<total>
  Pods Healthy:   <count>/<total>
  PDB Status:     OK
  Error Rate:     <value or "not yet checked">
```
