---
name: gcp-live-bigquery-dataset-deletion-guard
description: Gate BigQuery dataset deletion, table truncation, and authorized view changes against a full downstream dependency audit and export confirmation. Dataset deletion is immediate and permanent with no recycle bin — this guard ensures no dataset is deleted without enumerating all tables, scheduled queries, Data Transfer jobs, Looker connections, and Dataflow pipelines that depend on it.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: data
---

# GCP Live BigQuery Dataset Deletion Guard

## Purpose

Act as the guarded live GCP operator for gcp-live-bigquery-dataset-deletion-guard work. Gate every BigQuery dataset deletion, table truncation, and authorized view change with a complete downstream dependency audit and export confirmation. Dataset deletion in BigQuery is immediate and permanent — there is no recycle bin, and all downstream pipelines break the moment the dataset is gone.

## When to Use

Use this skill when:

- A BigQuery dataset is being deleted or a request to delete one is received
- A BigQuery table is being truncated (all rows deleted, schema preserved)
- An authorized view configuration is being changed or removed
- A scheduled query referencing the dataset needs to be decommissioned first
- A Data Transfer Service job is being removed alongside its target dataset
- An operator needs to enumerate all resources and dependencies before a BigQuery decommissioning

## When NOT to Use

Do not use this skill when:

- The task is creating a new dataset or table with no existing data at risk
- The task is a read-only schema inspection or data quality audit with no deletion intent
- The task involves Cloud Storage, Cloud SQL, or other non-BigQuery data stores
- The task is modifying query logic or views without deleting any underlying data

## Pre-Flight Checklist

Before executing any BigQuery deletion or truncation, verify all of the following:

1. **Dataset identity confirmed** — run `bq show --format=prettyjson <PROJECT>:<DATASET>` and confirm the dataset name, project, and location match the intended target.
2. **Dataset inventory captured** — run `bq ls <PROJECT>:<DATASET>` to enumerate all tables, views, and external tables. Document the table count and total bytes stored.
3. **Scheduled queries audited** — run `bq ls --transfer_config --transfer_location=<LOCATION> --project_id=<PROJECT>` to list all scheduled queries and Data Transfer jobs that reference this dataset.
4. **Authorized views audited** — check all authorized views that grant access to this dataset; identify any cross-project views that will break.
5. **Downstream pipeline dependencies confirmed** — check Dataflow jobs, Looker/Looker Studio connections, and any application code that references the dataset for active dependencies.
6. **Export/backup confirmed** — for production datasets, confirm that a full export to Cloud Storage (Avro, Parquet, or JSON) has been completed and verified before deletion is authorized.
7. **Data retention policy reviewed** — confirm there are no legal hold, compliance retention, or regulatory requirements that prevent deletion.

## Required Confirmation

The operator must explicitly state all of the following before any deletion is executed:

- "I confirm the target is dataset `<DATASET_ID>` in project `<PROJECT_ID>`, location `<LOCATION>`."
- "I have reviewed the dataset inventory: `<N>` tables, `<N>` views, approximately `<X>` bytes."
- "I have audited downstream dependencies and confirmed all scheduled queries and DTS jobs have been decommissioned or will tolerate this deletion."
- "I confirm that an export/backup has been completed (or that no backup is required with documented justification)."
- "I approve this deletion."

## Execution Steps

1. Capture full dataset inventory and downstream dependency audit.
2. Confirm active principal has `roles/bigquery.dataOwner` for the target dataset.
3. Present the dataset inventory, dependency findings, and export status to the operator for explicit approval.
4. Execute the mutation:
   - Delete dataset (with all contents): `bq rm -r -f <PROJECT>:<DATASET>`
   - Delete a single table: `bq rm -f <PROJECT>:<DATASET>.<TABLE>`
   - Truncate a table (delete all rows): `bq query --nouse_legacy_sql 'TRUNCATE TABLE \`<PROJECT>.<DATASET>.<TABLE>\`'`
   - Update authorized views: `bq update --source <JSON_FILE> <PROJECT>:<DATASET>`
5. Confirm the deletion is reflected with `bq ls <PROJECT>:` (dataset should not appear).

## Rollback Procedure

- **Dataset deletion** (NOT reversible): BigQuery dataset deletion is immediate and permanent. There is no recycle bin. If deletion was accidental, the only recovery path is restoring from a Cloud Storage export.
- **Table deletion** (partially recoverable): If the table had a default expiration set, it may appear in the BigQuery `__TABLES__` metadata with an expiration timestamp — but this does not allow recovery after deletion.
- **Table truncation** (NOT reversible without backup): Truncation removes all rows. Recovery requires restoring from an export or a snapshot.
- For accidental deletion, immediately open a GCP Support case and provide the dataset resource path and deletion timestamp from Cloud Audit Logs.

## Post-Change Verification

1. Run `bq ls <PROJECT>:` — confirm the deleted dataset no longer appears.
2. Check Cloud Audit Logs for the deletion event: `gcloud logging read 'protoPayload.methodName="google.cloud.bigquery.v2.DatasetService.DeleteDataset"' --limit=5 --project=<PROJECT>`.
3. Monitor scheduled query runs in the 24 hours following deletion — confirm no pipeline failures are attributed to the deleted dataset.
4. Check Dataflow job logs and Looker connection health for any errors referencing the deleted dataset.

## Response Shape

1. Project and dataset identity confirmation
2. Dataset inventory (tables, views, routines, bytes stored)
3. Downstream dependency audit (scheduled queries, DTS jobs, authorized views)
4. Export/backup confirmation before deletion
5. Approval status
6. Executed deletion or truncation action
7. Post-change pipeline impact verification
