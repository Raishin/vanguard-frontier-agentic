# IAM Permissions — GCP Live BigQuery Dataset Deletion Guard

## Minimum Read Roles (Discovery and Audit)

| Role | Purpose |
|------|---------|
| `roles/bigquery.dataViewer` | Read dataset metadata, table schemas, and row counts without mutation rights |
| `roles/bigquery.metadataViewer` | View dataset and table metadata, IAM policies, and authorized view configurations |

## Required Mutation Roles (Live Operations)

| Role | Purpose | Scope Guidance |
|------|---------|---------------|
| `roles/bigquery.dataOwner` | Delete datasets, drop tables, truncate tables, and manage authorized views | Bind at the dataset level where possible; avoid project-level binding unless multiple datasets are in scope |

## Narrowing Guidance

- Prefer binding `roles/bigquery.dataOwner` at the **dataset level** rather than project level — this limits blast radius to the specific dataset being managed.
- For table truncation only, `roles/bigquery.dataEditor` is a narrower alternative that does not include dataset deletion rights.
- Use BigQuery column-level security and row-level security policies to further restrict what data can be accessed or modified.
- Implement a two-person integrity requirement for dataset deletion by requiring a secondary approval from a data steward or governance owner.
- Enable the `constraints/bigquery.disableByoidProjectCreation` and related org policies to prevent unauthorized cross-project data exfiltration.

## Anti-Patterns — Never Grant

- `roles/owner` — grants billing, IAM, and full resource control; never appropriate for a dataset deletion guard.
- `roles/bigquery.dataOwner` at the organization or folder level — blast radius is every BigQuery dataset in the hierarchy.
- `roles/bigquery.admin` to CI/CD service accounts without deletion intent locked to specific datasets.
- Granting deletion rights without a corresponding export/backup policy — irreversible deletion without a backup path violates data governance.
- Authorized view changes that grant cross-project data access without a documented data sharing agreement.

## Audit Trail

All BigQuery data lifecycle operations (dataset deletion, table deletion, truncation) are logged in **Cloud Audit Logs** under `bigquery.googleapis.com`. Ensure `DATA_WRITE` and `DATA_READ` audit log types are enabled. Export BigQuery audit logs to a separate, protected BigQuery dataset or Cloud Storage bucket for compliance and forensic retention.
