# Workflow And Output

Diagnostic sequence and output contract for platform-reliability review and incident-evidence assessment.

## Workflow

1. Establish the scope of the reliability review: compute, jobs, pipelines, disaster recovery, or incident investigation.
2. For job/pipeline reliability: check timeout configuration and verify it applies per retry, not globally; confirm retries are configured only for non-continuous jobs.
3. For incident preservation: identify which system tables are available (GA/PUBLIC PREVIEW status), their schema, and retention windows; confirm a log-preservation plan exists for beyond-60-day analysis.
4. For disaster recovery: confirm managed DR is in place or assessed; if DIY, flag as not-recommended; verify failover test schedule (quarterly) and rollback readiness.
5. For quota headroom: identify current consumption from `system.billing.usage` or logs and calculate margin to limits; flag any workload approaching a limit.
6. Gather cluster-policy configuration and verify constraints are consistent (no conflicts between fixed, forbidden, allowlist); flag any malformed policies.
7. Check instance-pool configuration: identify minimum-idle instances and confirm the standing-cost impact is acknowledged and budgeted.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the reliability scope (compute / jobs / pipelines / disaster recovery / incident evidence).
- Timeout/retry/dependency findings for jobs/pipelines, with specific configurations and per-retry semantics clarified.
- System-table availability findings: GA/PUBLIC PREVIEW status, schema details, and retention windows.
- Disaster-recovery findings: managed-DR assessment, RPO/RTO targets, and failover-test schedule.
- Severity-labelled findings (critical / high / medium / low), each with evidence basis.
- Safe next actions, incident-preservation recommendations, and any required confirmations (RTO target, failover test date, current incident timeline, minimum-idle cost approval).
