# Workflow and Output — GCP Live IAM Policy Change Guard

## Step-by-Step Workflow

### Phase 1: Identity and Hierarchy Confirmation

1. Confirm active gcloud identity:
   ```
   gcloud auth list
   gcloud config get-value account
   gcloud config get-value project
   ```
2. Identify the resource hierarchy level (org/folder/project) and confirm the target resource ID:
   ```
   gcloud organizations list
   gcloud resource-manager folders list --organization=<ORG_ID>
   gcloud projects describe <PROJECT_ID>
   ```

### Phase 2: Current Policy Capture

3. Get the current IAM policy for the target resource:
   ```
   # For project:
   gcloud projects get-iam-policy <PROJECT_ID> --format=json
   # For folder:
   gcloud resource-manager folders get-iam-policy <FOLDER_ID> --format=json
   # For organization:
   gcloud organizations get-iam-policy <ORG_ID> --format=json
   ```
4. Save the current policy as a pre-change baseline for diff comparison.

### Phase 3: Blast-Radius Assessment

5. For org-level changes, enumerate all child folders:
   ```
   gcloud resource-manager folders list --organization=<ORG_ID>
   ```
6. Enumerate all projects in affected folders:
   ```
   gcloud projects list --filter="parent.id=<FOLDER_ID>"
   ```
7. Assess which resources will inherit the new binding and document the scope.

### Phase 4: Service Account Key Audit (if applicable)

8. List existing keys for the service account:
   ```
   gcloud iam service-accounts keys list --iam-account=<SA_EMAIL> --project=<PROJECT>
   ```
9. Check for any system-managed keys vs. user-managed keys.

### Phase 5: Approval Gate

10. Present all evidence: current policy, proposed change, blast radius, SA key inventory.
11. Require explicit written approval with all required confirmations.
12. Do not proceed until approval is received.

### Phase 6: Execution

13. Execute the approved mutation:
    ```
    # Add binding:
    gcloud projects add-iam-policy-binding <PROJECT> \
      --member="<MEMBER>" --role="<ROLE>"

    # Remove binding:
    gcloud projects remove-iam-policy-binding <PROJECT> \
      --member="<MEMBER>" --role="<ROLE>"

    # Create SA key:
    gcloud iam service-accounts keys create <OUTPUT_FILE> \
      --iam-account=<SA_EMAIL>
    ```

### Phase 7: Post-Change Verification

14. Get the updated policy and diff against the pre-change baseline:
    ```
    gcloud projects get-iam-policy <PROJECT_ID> --format=json
    ```
15. Query Cloud Audit Logs for the change record:
    ```
    gcloud logging read \
      'protoPayload.methodName="SetIamPolicy"' \
      --limit=5 --project=<PROJECT>
    ```
16. Test effective permissions for the affected principal.

## Expected Output Format

```
RESOURCE IDENTITY
  Type:           [organization | folder | project]
  ID:             <resource-id>
  Display Name:   <name>

ACTIVE PRINCIPAL
  Account:        <email>
  Role Available: [YES / NO]

CURRENT POLICY SNAPSHOT
  Binding count:  <N>
  Notable roles:  <comma-separated list of sensitive roles present>

PROPOSED CHANGE
  Action:         [ADD | REMOVE]
  Member:         <principal-email>
  Role:           <role-id>
  Blast Radius:   [Org-wide: N folders, M projects | Project-scoped]

SA KEY AUDIT (if applicable)
  Existing keys:  <count>
  Oldest key age: <days>
  Recommendation: [Use Workload Identity | Proceed with justification]

APPROVAL STATUS
  Operator:       <identity>
  Approved:       [YES / NO / PENDING]
  Authority level: [project-admin | security-lead | CISO]

ACTION
  [BLOCKED — reason] OR [EXECUTED — policy etag: <etag>]

ROLLBACK POSTURE
  Reversal command: [gcloud ... remove-iam-policy-binding ...]
  OR [NOT REVERSIBLE — SA key created; revoke with keys delete if needed]

VERIFICATION
  Policy diff:    [CONFIRMED — only intended change present]
  Audit log:      [FOUND — insertId: <id>]
  Access test:    [PASSED | NOT TESTED]
```
