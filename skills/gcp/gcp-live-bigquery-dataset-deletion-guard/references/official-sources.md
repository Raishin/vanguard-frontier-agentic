# Official Sources — GCP Live BigQuery Dataset Deletion Guard

Authoritative GCP documentation for BigQuery dataset management, table operations, and authorized views.

## Core References

- **Managing Tables** — https://cloud.google.com/bigquery/docs/managing-tables
  How to create, copy, move, update, and delete BigQuery tables; table expiration policies and their interaction with dataset deletion.

- **Managing Datasets** — https://cloud.google.com/bigquery/docs/datasets
  Dataset creation, listing, updating access controls, and deletion procedures; behavior when deleting a dataset with existing tables.

- **Authorized Views** — https://cloud.google.com/bigquery/docs/authorized-views
  How to create, modify, and remove authorized views; impact of dataset deletion on views in other projects.

- **BigQuery Data Transfer Service** — https://cloud.google.com/bigquery/docs/dts-introduction
  Overview of scheduled data transfer jobs that may target datasets being considered for deletion.

- **Scheduled Queries** — https://cloud.google.com/bigquery/docs/scheduling-queries
  How to list and manage scheduled queries; impact of target dataset deletion on scheduled query runs.

- **BigQuery Snapshots** — https://cloud.google.com/bigquery/docs/table-snapshots-intro
  Creating table snapshots as a pre-deletion backup mechanism; restoration from snapshots.

- **BigQuery Audit Logging** — https://cloud.google.com/bigquery/docs/reference/auditlogs
  What BigQuery operations are captured in Cloud Audit Logs and how to query for dataset and table lifecycle events.

- **Exporting Table Data** — https://cloud.google.com/bigquery/docs/exporting-data
  How to export BigQuery tables to Cloud Storage in Avro, Parquet, CSV, or JSON format before deletion.
