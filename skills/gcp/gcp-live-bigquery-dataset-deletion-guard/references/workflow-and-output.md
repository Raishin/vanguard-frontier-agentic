# Workflow and Output — GCP Live BigQuery Dataset Deletion Guard

## Step-by-Step Workflow

### Phase 1: Identity and Dataset Confirmation

1. Confirm active gcloud identity and project:
   ```
   gcloud auth list
   gcloud config get-value project
   ```
2. Show full dataset details:
   ```
   bq show --format=prettyjson <PROJECT>:<DATASET>
   ```
3. List all objects in the dataset:
   ```
   bq ls --max_results=1000 <PROJECT>:<DATASET>
   ```

### Phase 2: Dataset Inventory

4. Count tables and get storage usage:
   ```
   bq query --nouse_legacy_sql \
     'SELECT table_id, row_count, size_bytes, type
      FROM `<PROJECT>.<DATASET>.__TABLES__`
      ORDER BY size_bytes DESC'
   ```
5. List all views separately (type = VIEW):
   ```
   bq ls --filter labels.<LABEL>=<VALUE> <PROJECT>:<DATASET>
   ```
6. List all routines (stored procedures, functions):
   ```
   bq ls --routines <PROJECT>:<DATASET>
   ```

### Phase 3: Downstream Dependency Audit

7. List all Data Transfer Service configurations:
   ```
   bq ls --transfer_config \
     --transfer_location=<LOCATION> \
     --project_id=<PROJECT>
   ```
8. Check scheduled queries that reference the dataset (search by destination dataset):
   ```
   bq ls --transfer_config --transfer_location=<LOCATION> \
     --project_id=<PROJECT> --filter=dataSourceIds:scheduled_query
   ```
9. Check authorized views in other datasets that reference this dataset:
   ```
   bq show --format=prettyjson <PROJECT>:<OTHER_DATASET>
   # Look for "view.accessEntries" referencing the target dataset
   ```
10. Check Dataflow job templates and Looker connections for references (manual or via Cloud Asset Inventory).

### Phase 4: Export/Backup Confirmation

11. Export critical tables to Cloud Storage before deletion:
    ```
    bq extract \
      --destination_format=PARQUET \
      --compression=SNAPPY \
      '<PROJECT>:<DATASET>.<TABLE>' \
      'gs://<BUCKET>/exports/<DATASET>/<TABLE>/*.parquet'
    ```
12. Verify export completed successfully:
    ```
    gsutil ls -l gs://<BUCKET>/exports/<DATASET>/
    ```

### Phase 5: Approval Gate

13. Present all findings: dataset inventory, dependency audit, export status.
14. Require explicit written approval with all required confirmations.
15. Do not proceed until approval is received.

### Phase 6: Execution

16. Execute the approved deletion:
    ```
    # Delete entire dataset and all contents:
    bq rm -r -f <PROJECT>:<DATASET>

    # Delete a single table:
    bq rm -f <PROJECT>:<DATASET>.<TABLE>

    # Truncate a table (remove all rows):
    bq query --nouse_legacy_sql \
      'TRUNCATE TABLE `<PROJECT>.<DATASET>.<TABLE>`'
    ```

### Phase 7: Post-Change Verification

17. Confirm dataset no longer exists:
    ```
    bq ls --project_id=<PROJECT>
    ```
18. Check audit log for the deletion:
    ```
    gcloud logging read \
      'protoPayload.methodName="google.cloud.bigquery.v2.DatasetService.DeleteDataset"' \
      --limit=5 --project=<PROJECT>
    ```
19. Monitor downstream scheduled query runs and pipeline health for 24 hours.

## Expected Output Format

```
DATASET IDENTITY
  Project:        <project-id>
  Dataset:        <dataset-id>
  Location:       <location>
  Created:        <date>
  Last Modified:  <date>

INVENTORY
  Tables:         <count> (<total-size-gb> GB)
  Views:          <count>
  Routines:       <count>
  Largest table:  <table-name> (<size-gb> GB, <rows> rows)

DOWNSTREAM DEPENDENCIES
  Scheduled queries: [NONE | <count> referencing this dataset]
  DTS jobs:          [NONE | <count>]
  Authorized views:  [NONE | <count> in other datasets/projects]
  Known pipelines:   [NONE | <list of Dataflow jobs, Looker connections>]
  BLOCKING:          [YES — active dependencies found | NO]

EXPORT/BACKUP
  Export location:   gs://<bucket>/exports/<dataset>/
  Export status:     [CONFIRMED | NOT DONE | NOT REQUIRED — justification]

APPROVAL STATUS
  Operator:       <identity>
  Approved:       [YES / NO / PENDING]
  No-recycle-bin acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason]
  OR [EXECUTED — dataset deleted at <timestamp>]
  OR [EXECUTED — table truncated: <table-name>]

ROLLBACK POSTURE
  Recovery path:  [Restore from gs://<bucket>/exports/<dataset>/]
  OR [NO BACKUP — permanent loss if deleted]

POST-CHANGE VERIFICATION
  Dataset absent: [CONFIRMED]
  Audit log:      [FOUND — insertId: <id>]
  Pipeline health: [PASS | MONITORING — check in 24h]
```
