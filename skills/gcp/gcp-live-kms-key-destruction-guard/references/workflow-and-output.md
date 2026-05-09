# Workflow and Output — GCP Live KMS Key Destruction Guard

## Step-by-Step Workflow

### Phase 1: Identity and Key State Confirmation

1. Confirm active gcloud identity and project:
   ```
   gcloud auth list
   gcloud config get-value project
   ```
2. List all key rings in the target location:
   ```
   gcloud kms keyrings list --location=<LOCATION> --project=<PROJECT>
   ```
3. List all key versions for the target key:
   ```
   gcloud kms keys versions list \
     --key=<KEY_NAME> \
     --keyring=<KEYRING_NAME> \
     --location=<LOCATION> \
     --project=<PROJECT>
   ```
4. Describe the specific key version to get full state details:
   ```
   gcloud kms keys versions describe <VERSION_NUMBER> \
     --key=<KEY_NAME> \
     --keyring=<KEYRING_NAME> \
     --location=<LOCATION> \
     --project=<PROJECT>
   ```

### Phase 2: CMEK Dependency Audit

5. Identify Cloud SQL instances using this key:
   ```
   gcloud sql instances list --project=<PROJECT> --format="table(name,diskEncryptionConfiguration.kmsKeyName)"
   ```
6. Identify GCS buckets using this key:
   ```
   gsutil ls -p <PROJECT> | xargs -I{} gsutil kms encryption {}
   ```
7. Identify BigQuery datasets using this key (via Cloud Asset Inventory):
   ```
   gcloud asset search-all-resources \
     --scope=projects/<PROJECT> \
     --asset-types=bigquery.googleapis.com/Dataset \
     --query="kmsKeyName:<KEY_RESOURCE_NAME>"
   ```
8. Identify Compute Engine persistent disks:
   ```
   gcloud compute disks list --project=<PROJECT> --format="table(name,diskEncryptionKey.kmsKeyName)"
   ```
9. Identify Secret Manager secrets using this key:
   ```
   gcloud secrets list --project=<PROJECT> --format="table(name,replication.userManaged.replicas.customerManagedEncryption.kmsKeyName)"
   ```

### Phase 3: Rotation vs. Destruction Assessment

10. Determine if key rotation satisfies the requirement:
    - Rotation creates a new primary key version for future encryptions; old versions remain accessible for decryption.
    - Destruction removes decryption capability for all data encrypted with the destroyed version.
11. Document the assessment: if any active resources use the target key version, destruction is blocked.

### Phase 4: Approval Gate

12. Present all findings: key version state, CMEK dependency inventory, rotation vs. destruction assessment.
13. Require explicit written approval with all required confirmations.
14. Do not proceed until approval is received.

### Phase 5: Execution

15. Schedule destruction (creates a DESTROY_SCHEDULED state; minimum 24-hour pending period):
    ```
    gcloud kms keys versions destroy <VERSION_NUMBER> \
      --key=<KEY_NAME> \
      --keyring=<KEYRING_NAME> \
      --location=<LOCATION> \
      --project=<PROJECT>
    ```
    OR cancel a pending destruction:
    ```
    gcloud kms keys versions restore <VERSION_NUMBER> \
      --key=<KEY_NAME> \
      --keyring=<KEYRING_NAME> \
      --location=<LOCATION> \
      --project=<PROJECT>
    ```

### Phase 6: Post-Action Verification

16. Confirm new key version state:
    ```
    gcloud kms keys versions describe <VERSION_NUMBER> \
      --key=<KEY_NAME> --keyring=<KEYRING_NAME> \
      --location=<LOCATION> --project=<PROJECT>
    ```
17. Check audit log for the scheduled destruction event:
    ```
    gcloud logging read \
      'protoPayload.methodName="DestroyCryptoKeyVersion"' \
      --limit=5 --project=<PROJECT>
    ```
18. Set a Cloud Monitoring alert to notify before the actual destruction date.

## Expected Output Format

```
KEY IDENTITY
  Project:        <project-id>
  Location:       <location>
  Key Ring:       <keyring-name>
  Key:            <key-name>
  Version:        <version-number>
  Current State:  [ENABLED | DISABLED | DESTROY_SCHEDULED | DESTROYED]
  Scheduled Destruction: <ISO-8601 date, or N/A>

CMEK DEPENDENCY AUDIT
  Cloud SQL:      [NONE | <list of instance names>]
  GCS Buckets:    [NONE | <list of bucket names>]
  BigQuery:       [NONE | <list of dataset names>]
  Compute Disks:  [NONE | <list of disk names>]
  Secret Manager: [NONE | <list of secret names>]
  BLOCKING:       [YES — cannot destroy while active resources exist | NO]

ROTATION VS DESTRUCTION
  Rotation sufficient: [YES — use gcloud kms keys set-primary-version | NO — destruction required]
  Reason:         <justification>

APPROVAL STATUS
  Operator:       <identity>
  Approved:       [YES / NO / PENDING]
  Irreversibility acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason]
  OR [SCHEDULED — destruction date: <date>]
  OR [RESTORED — version state: DISABLED]

ROLLBACK POSTURE
  Restore window: [Until <destruction-date> — use versions restore]
  OR [EXPIRED — destruction complete, no recovery possible]

POST-ACTION MONITORING
  Audit log entry: [FOUND — insertId: <id>]
  Alert configured: [YES | NO]
  CMEK services check: [PASS | PENDING]
```
