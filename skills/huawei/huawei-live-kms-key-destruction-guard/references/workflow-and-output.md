# Workflow and Output — Huawei Live KMS Key Destruction Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm account ID, enterprise project, and active IAM principal.
2. Describe the target key: key ID, alias, type, and current status.
3. Confirm the proposed pending-deletion window length.

### Phase 2: Encrypted Resource Enumeration

4. Enumerate OBS buckets using this key for SSE (server-side encryption):
   - OBS console > Buckets > Encryption tab — check for CMK binding
5. Enumerate ECS instances with disk encryption using this key:
   - EVS console > Volumes > check encryption key ID
6. Enumerate GaussDB/RDS instances with CMK encryption using this key.
7. Enumerate CSMS secrets encrypted by this key:
   - DEW console > Secrets > filter by encryption key
8. Enumerate DBSS-protected databases using this key.
9. Confirm all enumerated resources have been migrated to a replacement key or decommissioned.

### Phase 3: MLPS Classification Assessment

10. For each enumerated resource, determine MLPS Level 3 classification.
11. If any resource is MLPS Level 3: data destruction triggers 24-hour mandatory incident report obligation.

### Phase 4: Approval Gate

12. Present all evidence: key identity, window length, enumerated resources, migration/decommission status, MLPS obligation.
13. Require explicit written approval per Required Confirmation section.
14. Do not proceed until approval is received.

### Phase 5: Execution

15. Schedule key for deletion via DEW console:
    - DEW console > Keys > select key > Schedule Deletion > set window days
16. Record the exact deletion date (current date + window days).
17. Document the cancel-pending-deletion deadline in the change ticket.

### Phase 6: Post-Action Verification

18. Confirm key status shows `Pending Deletion` with correct deletion date.
19. Test sample encrypted resource access within the window.
20. If MLPS Level 3: initiate incident report within 24 hours.

## Expected Output Format

```
KEY IDENTITY
  Account:            <account-id>
  Enterprise Project: <enterprise-project>
  Key ID:             <key-id>
  Key Alias:          <alias>
  Current Status:     <status>
  Pending Window:     <days>
  Deletion Date:      <calculated date>

ENCRYPTED RESOURCE INVENTORY
  OBS SSE buckets:    <count> — [ALL MIGRATED / ACTIVE DEPENDENCY]
  ECS disk volumes:   <count> — [ALL MIGRATED / ACTIVE DEPENDENCY]
  GaussDB/RDS (CMK):  <count> — [ALL MIGRATED / ACTIVE DEPENDENCY]
  CSMS secrets:       <count> — [ALL MIGRATED / ACTIVE DEPENDENCY]
  DBSS databases:     <count> — [ALL MIGRATED / ACTIVE DEPENDENCY]

MLPS CLASSIFICATION
  Level:              [MLPS Level 3 / Not classified / Unknown]
  Incident obligation: [YES — 24h report required / NO]

ROLLBACK WINDOW
  Cancel deadline:    <deletion date - 1 day>
  Cancel procedure:   DEW console > Keys > Cancel Pending Deletion

APPROVAL STATUS
  Operator:           <identity>
  Approved:           [YES / NO / PENDING]
  MLPS acknowledged:  [YES / NO / N/A]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

VERIFICATION
  Key status:         Pending Deletion
  Deletion date:      <confirmed date>
  Sample resource:    [ACCESSIBLE / ERROR]
  Incident filed:     [YES / NO / N/A]
```
