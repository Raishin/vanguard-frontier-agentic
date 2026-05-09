---
name: gcp-live-cloud-run-traffic-migration-guard
description: Gate Cloud Run traffic percentage migrations, min-instances changes, and revision deletions against revision health verification and rollback posture assessment. Migrating 100% traffic to a broken revision causes complete service unavailability with no automatic rollback — this guard enforces health checks, gradual canary splits, and explicit approval before any production traffic change is executed.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: delivery
---

# GCP Live Cloud Run Traffic Migration Guard

## Purpose

Act as the guarded live GCP operator for gcp-live-cloud-run-traffic-migration-guard work. Gate every Cloud Run traffic migration, min-instances change, and revision deletion with revision health verification and a documented rollback plan. Cloud Run has no automatic rollback — every traffic migration must be treated as a production outage risk until the target revision is verified healthy.

## When to Use

Use this skill when:

- A Cloud Run traffic percentage is being migrated to a new revision (canary, gradual, or full cut-over)
- A revision is being deleted and the rollback implications need to be assessed
- Min-instances settings are being changed for a production service
- An immediate rollback of a broken traffic migration is needed
- A traffic split between two revisions needs to be configured or modified
- An operator needs to audit the current revision inventory and health before any change

## When NOT to Use

Do not use this skill when:

- The target is a development or staging Cloud Run service with no production traffic
- The task is deploying a new revision without changing traffic allocation (safe — new revisions default to 0% traffic)
- The task is read-only service inspection with no mutation intent
- The task involves GKE, App Engine, or other non-Cloud Run compute

## Pre-Flight Checklist

Before executing any Cloud Run traffic mutation, verify all of the following:

1. **Service identity confirmed** — run `gcloud run services describe <SERVICE> --region=<REGION> --project=<PROJECT>` and confirm the service name, region, and current traffic allocation match the intended target.
2. **Revision inventory captured** — list all revisions with their traffic allocations and creation timestamps.
3. **Target revision health assessed** — confirm the target revision has an acceptable error rate (typically < 1%) and latency p99 below the SLO threshold. If the revision is new, check container startup logs for errors.
4. **Current traffic split documented** — record the exact current traffic percentages for all revisions before any change; this is the rollback target if needed.
5. **Min-instances impact assessed** — if changing min-instances, document the cost impact and cold-start behavior change before proceeding.
6. **Revision deletion safety confirmed** — if deleting a revision, confirm it holds no traffic allocation and is not the only known-good revision available for rollback.
7. **Canary strategy agreed** — for untested revisions, confirm the operator agrees to start at ≤10% traffic rather than an immediate 100% cut-over.

## Required Confirmation

The operator must explicitly state all of the following before any traffic mutation is executed:

- "I confirm the service is `<SERVICE_NAME>` in project `<PROJECT_ID>`, region `<REGION>`."
- "I confirm the target revision is `<REVISION_NAME>` and it has passed health checks."
- "I understand there is no automatic rollback — if the migration fails, I must manually re-split traffic."
- "I approve this traffic migration to `<X>%` for revision `<REVISION_NAME>`."
- For revision deletion: "I confirm revision `<REVISION_NAME>` holds no traffic and is not needed for rollback, and I approve its deletion."

## Execution Steps

1. Capture current traffic split and revision health metrics.
2. Confirm active principal has `roles/run.developer` for the target service.
3. Present the planned migration, revision health findings, and rollback target to the operator for explicit approval.
4. Execute the mutation:
   - Gradual migration (recommended): `gcloud run services update-traffic <SERVICE> --region=<REGION> --to-revisions=<REVISION>=<PERCENT> --project=<PROJECT>`
   - Full cut-over to latest: `gcloud run services update-traffic <SERVICE> --region=<REGION> --to-latest --project=<PROJECT>`
   - Update min-instances: `gcloud run services update <SERVICE> --region=<REGION> --min-instances=<N> --project=<PROJECT>`
   - Delete revision: `gcloud run revisions delete <REVISION> --region=<REGION> --project=<PROJECT>`
5. Monitor the service for errors and latency in the 5-15 minutes following the traffic change.

## Rollback Procedure

- **Traffic migration rollback** (always possible while the prior revision still exists): Re-split traffic back to the previous revision:
  ```
  gcloud run services update-traffic <SERVICE> --region=<REGION> \
    --to-revisions=<PRIOR_REVISION>=100 --project=<PROJECT>
  ```
- **If prior revision was deleted**: Deploy a new revision from the last-known-good container image and migrate traffic to it.
- **Min-instances rollback**: Revert to previous min-instances value with the same update command.
- After rollback, investigate the failed revision's startup logs and error metrics before attempting another migration.

## Post-Change Verification

1. Run `gcloud run services describe <SERVICE> --region=<REGION>` — confirm traffic allocation shows the expected percentages.
2. Check Cloud Run request logs in Cloud Logging for error rates in the 15 minutes following migration:
   ```
   gcloud logging read \
     'resource.type="cloud_run_revision" AND resource.labels.service_name="<SERVICE>"' \
     --limit=50 --project=<PROJECT>
   ```
3. Check Cloud Monitoring for the service's error rate and latency p99 metrics.
4. For min-instances changes, confirm new instance count is reflected in the Cloud Run console.
5. Confirm the rollback revision still exists (has not been deleted) and is available if needed.

## Response Shape

1. Service and region identity confirmation
2. Current revision inventory and traffic splits
3. Target revision health (error rate, latency p99)
4. Min-instances and concurrency settings
5. Approval status
6. Proposed or executed traffic migration
7. Post-migration health check and rollback verification
