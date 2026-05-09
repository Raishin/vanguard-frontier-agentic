# Workflow and Output — GCP Live Cloud Run Traffic Migration Guard

## Step-by-Step Workflow

### Phase 1: Identity and Service Confirmation

1. Confirm active gcloud identity and project:
   ```
   gcloud auth list
   gcloud config get-value project
   ```
2. Describe the target Cloud Run service:
   ```
   gcloud run services describe <SERVICE_NAME> \
     --region=<REGION> \
     --project=<PROJECT> \
     --format=json
   ```
3. Confirm service name, region, URL, and current traffic allocation from the output.

### Phase 2: Revision Inventory and Health Assessment

4. List all revisions for the service:
   ```
   gcloud run revisions list \
     --service=<SERVICE_NAME> \
     --region=<REGION> \
     --project=<PROJECT> \
     --format="table(metadata.name,status.conditions[0].status,metadata.creationTimestamp,spec.containers[0].image)"
   ```
5. Get traffic split for the service:
   ```
   gcloud run services describe <SERVICE_NAME> \
     --region=<REGION> --project=<PROJECT> \
     --format="value(spec.traffic)"
   ```
6. Check Cloud Monitoring for the target revision's error rate and latency p99:
   ```
   # Via gcloud CLI metrics query or Cloud Console > Cloud Run > <service> > Metrics
   # Key metrics: run.googleapis.com/request_count (filter by response_code_class!=2xx)
   # run.googleapis.com/request_latencies (p99)
   ```
7. Check container startup logs for errors in the target revision:
   ```
   gcloud logging read \
     'resource.type="cloud_run_revision" AND resource.labels.service_name="<SERVICE>" AND resource.labels.revision_name="<REVISION>"' \
     --limit=20 --project=<PROJECT>
   ```

### Phase 3: Min-Instances Review (if applicable)

8. View current min-instances setting:
   ```
   gcloud run services describe <SERVICE_NAME> \
     --region=<REGION> --project=<PROJECT> \
     --format="value(spec.template.metadata.annotations['autoscaling.knative.dev/minScale'])"
   ```
9. Estimate cost impact: min-instances × instance idle cost per hour × 730 hours/month.

### Phase 4: Approval Gate

10. Present all findings: current revision inventory, traffic splits, target revision health, rollback target.
11. Require explicit written approval with all required confirmations.
12. Do not proceed until approval is received.

### Phase 5: Execution

13. Execute the approved migration:
    ```
    # Split to canary (recommended first step):
    gcloud run services update-traffic <SERVICE_NAME> \
      --region=<REGION> \
      --to-revisions=<NEW_REVISION>=10,<OLD_REVISION>=90 \
      --project=<PROJECT>

    # After canary validation, full cut-over:
    gcloud run services update-traffic <SERVICE_NAME> \
      --region=<REGION> \
      --to-revisions=<NEW_REVISION>=100 \
      --project=<PROJECT>

    # OR migrate to latest directly (skip canary only if previously tested):
    gcloud run services update-traffic <SERVICE_NAME> \
      --region=<REGION> --to-latest \
      --project=<PROJECT>
    ```

### Phase 6: Post-Migration Health Check

14. Confirm new traffic allocation:
    ```
    gcloud run services describe <SERVICE_NAME> \
      --region=<REGION> --format="value(spec.traffic)"
    ```
15. Monitor error rate and latency for 15 minutes in Cloud Monitoring.
16. Confirm the rollback revision still exists and holds a traffic allocation or is available.

### Rollback (if needed)

17. Immediately re-split traffic to the prior known-good revision:
    ```
    gcloud run services update-traffic <SERVICE_NAME> \
      --region=<REGION> \
      --to-revisions=<PRIOR_REVISION>=100 \
      --project=<PROJECT>
    ```

## Expected Output Format

```
SERVICE IDENTITY
  Project:        <project-id>
  Service:        <service-name>
  Region:         <region>
  URL:            https://<service-hash>-<region>.run.app

CURRENT TRAFFIC SPLIT
  <revision-name-1>: <percent>% (ACTIVE)
  <revision-name-2>: <percent>% (STABLE)

REVISION HEALTH — TARGET
  Revision:       <target-revision-name>
  Created:        <date>
  Image:          <image-uri>
  Error Rate:     <pct>% (last 30m)
  Latency p99:    <ms>ms (last 30m)
  Health:         [PASS | WARN | FAIL — reason]

MIN-INSTANCES
  Current:        <N>
  Proposed:       <N>
  Cost delta:     ~$<amount>/month

APPROVAL STATUS
  Operator:       <identity>
  Approved:       [YES / NO / PENDING]
  Canary strategy: [YES — starting at <pct>% | FULL — acknowledged risk]
  No-auto-rollback acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason]
  OR [EXECUTED — traffic split: <revision>=<pct>%, <revision>=<pct>%]
  OR [EXECUTED — revision <name> deleted]

ROLLBACK POSTURE
  Rollback revision: <prior-revision-name>
  Still exists:    [YES | NO — deleted, redeploy required]
  Rollback command: gcloud run services update-traffic ... --to-revisions=<prior>=100

POST-MIGRATION
  Error rate:     <pct>% (15m post-change)
  Latency p99:    <ms>ms (15m post-change)
  Health:         [PASS | DEGRADED | ROLLED BACK]
  Audit log:      [FOUND — insertId: <id>]
```
