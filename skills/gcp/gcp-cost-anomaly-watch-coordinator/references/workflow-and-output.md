# Workflow and output contract

Use this reference only when performing the full cost anomaly watch review, stale resource audit, or budget alert configuration assessment.

## Review domains

Check these areas before giving a verdict:
- Billing export baseline: BigQuery billing export enabled, dataset location, export schema version (standard / detailed), data freshness (1 day lag)
- BigQuery cost posture: on-demand vs. slot reservation, bytes scanned per query, partition pruning effectiveness, clustering coverage, reservation slot count vs. workload peak
- Cloud Run scaling cost: max-instances setting, scale-to-zero vs. min-instances >= 1, concurrency setting, actual instance count at traffic peak
- Stale resource drain: unattached Persistent Disks (status=READY, no attached instance), stopped GCE instances still billed for attached disks, orphaned forwarding rules, unused static external IPs
- Budget alert configuration: budget amount, alert threshold percentages (50%/90%/100%), notification channel type (email/Pub/Sub), budget action (disable billing) configured
- Remediation playbook: documented response for BigQuery spike, Cloud Run runaway, disk cleanup, GCE shutdown, contact owner list, automation scripts
- Recommender integration: active cost recommendations reviewed, estimated annual savings, highest-priority recommendations

## Safe workflow

1. **Frame scope**
   - Projects and services in scope:
   - Cost anomaly type (BigQuery / Cloud Run / stale resources / unknown):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence or sanitized billing export queries if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Is billing export to BigQuery enabled?
   - Is BigQuery using on-demand pricing without slot reservations for production workloads?
   - Is Cloud Run missing max-instances?
   - Are there unattached Persistent Disks or stopped GCE instances with attached disks?
   - Are budget alerts configured with notification channels?
   - Is there a documented remediation playbook?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged implementation, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.
   - Do NOT recommend disabling billing on production projects without explicit acknowledgment of full service disruption.

## Output contract

Return this structure:
```markdown
# GCP Cost Anomaly Watch Coordinator Review: <scope>
## Executive verdict
- Status: MONITORED / PARTIALLY MONITORED / UNMONITORED / NEEDS EVIDENCE
- Biggest cost risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Cost posture summary
- Billing export enabled: <yes / no / unknown>
- BigQuery pricing model: <slots / on-demand / mixed / unknown>
- Cloud Run max-instances: <configured / not set / unknown>
- Stale resources detected: <yes / no / unknown>
## Budget alert posture
- Budget alerts configured: <yes / no / partial>
- Notification channels: <email / Pub/Sub / none / unknown>
- Budget action (disable billing): <configured / not configured / unknown>
## Remediation playbook
- Playbook exists: <yes / no>
- Owner assigned: <yes / no / unknown>
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
