# Workflow and output contract

Use this reference only when performing the full BCDR review, HA assessment, implementation guidance, or recovery readiness pass.

## Review domains

Check these areas before giving a verdict:
- RTO/RPO targets: documented targets vs. achievable with current architecture
- Cloud SQL HA: standby zone configuration, failover test evidence, cross-region read replica for DR
- Cloud Spanner: single-region vs. multi-region config, write latency trade-off acceptance, RPO for regional outage
- GKE multi-cluster: Global Load Balancer configuration, Multi-Cluster Ingress setup, cluster health across regions
- Cloud Run: regions deployed, Global HTTPS LB with health checks, failover test evidence
- Persistent Disk snapshots: scheduled snapshot policy, cross-region snapshot for DR, restoration test evidence
- Recovery tests: last tested date, result, scope of test (full DR vs. partial), runbook used
- Runbooks: existence, completeness, last updated date, owner, tested state

## Safe workflow

1. **Frame scope**
   - Project/region/environment/workload:
   - Business criticality and owner:
   - RTO target and RPO target:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What services have zone-only redundancy (not region-redundant)?
   - What RTO/RPO targets have never been validated by a recovery test?
   - What runbooks are missing or untested?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Resilience BCDR Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- RTO/RPO targets: <documented> vs. <achievable>
- Biggest gap:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Recovery test status
- Last tested: <date or UNKNOWN>
- Result: <pass/fail or UNKNOWN>
- Scope: <full DR / partial / untested>
## Residual risk
- <risk or explicit none>
```
