# Workflow and Output — Alibaba Cloud Live KMS Key Mutation Guard

## Step-by-Step Workflow

### Phase 1: Key Identity and State Confirmation

1. Describe the target key to confirm identity and current state:
   ```
   aliyun kms DescribeKey --KeyId <KEY_ID>
   ```
2. Confirm region and account ID match the intended target.
3. Note the key state: `Enabled`, `Disabled`, or `PendingDeletion`.

### Phase 2: CMK Dependency Audit

4. Identify OSS buckets using SSE-KMS with this key ID (check bucket encryption config in OSS console or via API).
5. Identify ECS disks encrypted with this key:
   - Query ECS disk list filtered by KMS key ID in the console or via:
     ```
     aliyun ecs DescribeDisks --Encrypted true --KMSKeyId <KEY_ID>
     ```
6. Identify RDS instances with TDE using this key (check RDS console TDE settings).
7. Identify PolarDB clusters with TDE using this key (check PolarDB console TDE settings).
8. Identify SSM secrets encrypted with this key:
   ```
   aliyun kms ListSecrets
   ```
   Filter results for secrets using the target key ID.

### Phase 3: Disable vs. Delete Assessment

9. If any CMK dependencies are found: recommend **disable** first, not deletion.
10. If the operator proceeds to deletion: confirm all dependent data has been migrated or is confirmed non-critical.
11. Confirm the deletion window (7–30 days, default 30 days).

### Phase 4: Approval Gate

12. Present all evidence to the operator: key ID, current state, dependency audit results, disable vs. delete recommendation, deletion window.
13. Require explicit written approval including acknowledgment that deletion is permanent.
14. Do not proceed until approval is received.

### Phase 5: Execution

15. Execute the approved operation:
    - Disable key:
      ```
      aliyun kms DisableKey --KeyId <KEY_ID>
      ```
    - Schedule deletion:
      ```
      aliyun kms ScheduleKeyDeletion --KeyId <KEY_ID> --PendingWindowInDays 30
      ```
    - Cancel deletion (if reversing):
      ```
      aliyun kms CancelKeyDeletion --KeyId <KEY_ID>
      ```
    - Re-enable key:
      ```
      aliyun kms EnableKey --KeyId <KEY_ID>
      ```

### Phase 6: Post-Action Verification

16. Confirm new key state:
    ```
    aliyun kms DescribeKey --KeyId <KEY_ID>
    ```
17. Check ActionTrail for the mutation event.
18. Monitor CloudMonitor for application-level errors indicating CMK access failures.

## Expected Output Format

The agent response for a KMS key mutation operation must include:

```
KEY IDENTITY
  Key ID:         <key-id>
  Key Alias:      <alias or NONE>
  Region:         <region>
  Account ID:     <account-id>
  Current State:  <Enabled / Disabled / PendingDeletion>

CMK DEPENDENCY AUDIT
  OSS SSE-KMS Buckets:    [NONE | <bucket names>]
  ECS Encrypted Disks:    [NONE | <disk IDs>]
  RDS TDE Instances:      [NONE | <instance IDs>]
  PolarDB TDE Clusters:   [NONE | <cluster IDs>]
  SSM Secrets:            [NONE | <secret names>]

DISABLE vs. DELETE
  Recommendation:   [DISABLE FIRST | PROCEED TO DELETION]
  Reason:           <dependency status or operator decision>

DELETION WINDOW
  Window:           <N days (if deletion selected)>
  Deletion Date:    <date or N/A>

APPROVAL STATUS
  Operator:         <identity>
  Approved:         [YES / NO / PENDING]
  Permanent loss acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

ROLLBACK POSTURE
  [REVERSIBLE — re-enable key or cancel deletion before <date>]
  OR
  [NOT REVERSIBLE — deletion window expired; data permanently inaccessible]

VERIFICATION
  Key State Post-Action:  <Enabled / Disabled / PendingDeletion>
  ActionTrail Event:      [logged / not yet confirmed]
  Application Errors:     <none detected / <description>>
```
