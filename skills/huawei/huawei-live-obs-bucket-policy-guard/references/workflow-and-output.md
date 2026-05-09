# Workflow and Output — Huawei Live OBS Bucket Policy Guard

## Step-by-Step Workflow

### Phase 1: Bucket Identity Confirmation

1. Confirm account ID, enterprise project, and active IAM principal.
2. Identify the target bucket: bucket name, region, and current ACL setting.
3. Identify the proposed change type: ACL mutation, policy add/modify/delete, cross-region replication, versioning change, or bucket deletion.

### Phase 2: Data Classification and MLPS Assessment

4. Determine data sensitivity: PII presence, MLPS Level 3 classification, regulated data (financial, health, government).
5. If the bucket is in a CN-* region: check whether the proposed change involves cross-border data transfer.
6. If cross-border replication is being configured: require legal basis documentation under MLPS 2.0/CSL.

### Phase 3: Object and Downstream Inventory

7. Enumerate total object count and estimate storage volume.
8. Identify downstream applications and pipelines that read from or write to this bucket.
9. Confirm whether OBS versioning is enabled (affects recoverability of any deletion actions).

### Phase 4: Blast Radius Assessment

10. For public ACL change: assess immediate exposure risk — enumerate sensitive objects.
11. For bucket policy change: identify which principals and services lose or gain access.
12. For bucket deletion: confirm no recovery path exists if versioning is disabled.
13. For cross-border replication: confirm data residency obligation is met.

### Phase 5: Approval Gate

14. Present all evidence: bucket identity, ACL/policy state, data classification, object inventory, downstream applications, MLPS finding.
15. Require explicit written approval per Required Confirmation section in SKILL.md.
16. For CN-* cross-border replication: require legal basis statement.
17. Do not proceed until approval is received.

### Phase 6: Execution

18. Apply the approved ACL, policy, or configuration change.
19. For public ACL: immediately enable CES alarm for unexpected GET request volume spike.
20. For bucket deletion: verify `empty_bucket` is completed before deletion.
21. Record the change in the OBS change log and CTS audit trail.

### Phase 7: Post-Action Verification

22. Confirm bucket ACL/policy reflects the executed change via OBS console.
23. Verify downstream application access is intact (no inadvertent access denial).
24. For public ACL change: monitor for unexpected traffic spike in first 15 minutes.
25. Confirm CTS log entry for the mutation event.

## Expected Output Format

```
BUCKET IDENTITY
  Account:              <account-id>
  Enterprise Project:   <enterprise-project>
  Bucket Name:          <bucket-name>
  Region:               <region> [CN-* / Non-CN]
  Current ACL:          <private / public-read / public-read-write>
  Versioning:           [Enabled / Disabled / Suspended]

PROPOSED CHANGE
  Type:                 <ACL mutation / policy add|modify|delete / replication / deletion>
  New ACL / Policy:     <description>

DATA CLASSIFICATION
  PII Present:          [YES / NO / UNKNOWN]
  MLPS Level 3:         [YES / NO / UNKNOWN]
  Regulated Data:       [YES — type: <type> / NO]

CROSS-BORDER ASSESSMENT (CN-* regions only)
  Source Region:        <region>
  Target Region:        <region>
  Legal Basis:          [ASSESSED: <finding> / NOT ASSESSED — BLOCKED]

OBJECT INVENTORY
  Total Objects:        <count>
  Storage Volume:       <size>
  Sensitive Objects:    <count or UNKNOWN>
  Downstream Apps:      <list>

BLAST RADIUS
  Public Exposure Risk: [HIGH — <reason> / LOW / N/A]
  Access Denial Risk:   [YES — <affected principals> / NO]
  Recovery Path:        [Versioning enabled — recoverable / NOT RECOVERABLE]

APPROVAL STATUS
  Operator:             <identity>
  Approved:             [YES / NO / PENDING]
  Cross-border legal:   [APPROVED / N/A]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

POST-ACTION VERIFICATION
  ACL/Policy applied:   [CONFIRMED / ERROR]
  Downstream access:    [INTACT / DEGRADED — <detail>]
  CES alarm set:        [YES — public ACL / N/A]
  CTS log entry:        [CONFIRMED / NOT FOUND]
```
