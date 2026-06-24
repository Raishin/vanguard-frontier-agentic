# Workflow and output contract

Use this reference only when performing a full backup and BCDR posture review or formatting a resilience assessment.

## Review domains

Check these areas before giving a verdict:

- **Microsoft 365 Backup policy coverage**: Which workloads (Exchange Online, SharePoint, OneDrive) have active backup policies; protection unit scope; gaps in coverage (unprotected sites, mailboxes, accounts)
- **RPO alignment**: Stated business continuity RPO versus Microsoft 365 Backup restore point granularity (10-minute for recent two weeks, weekly snapshots for weeks 2–52); Exchange Online 10-minute granularity for 52 weeks
- **RTO alignment**: Stated business continuity RTO versus restore performance expectations (up to 250 protection units/hour bulk; single-site express restore 10–120 minutes depending on size)
- **Retention versus backup clarity**: Whether the organization understands the distinction between Microsoft Purview retention and Microsoft 365 Backup; whether native tools (versioning, recycle bin) are relied upon as backup substitutes
- **Ransomware recovery readiness**: Backup policy in place before attack; append-only storage protection; pre-attack restore point identification workflow; tested or documented recovery procedure
- **Backup Storage architecture**: Data residency compliance; pay-as-you-go billing awareness; 90-day offboarding grace period known; multi-admin notification configured
- **Third-party backup boundary**: Whether partner solution uses Microsoft 365 Backup Storage platform or copies to external location; implications for RTO at scale

## Safe workflow

1. **Frame scope**
   - Tenant / environment / workloads in scope:
   - Current backup policy coverage (if available):
   - Business continuity RPO and RTO requirements:
   - Compliance and regulatory data retention requirements (distinct from backup):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Microsoft 365 admin center evidence or Microsoft Graph Backup API read output for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which workloads have no active Microsoft 365 Backup policy and rely solely on native versioning or retention?
   - What is the maximum data loss window (RPO) if a ransomware event occurs today?
   - Has the restore workflow been tested against the organization's RTO target?
   - Is the distinction between retention policy and backup understood and communicated to stakeholders?
   - Does any partner backup solution use an external-copy architecture that may not meet RTO for large tenants?
   - Has the backup policy offboarding grace period and multi-admin notification feature been configured?
4. **Recommend the smallest safe action**
   - Prefer audit of existing policies before recommending in-place restores; in-place restore overwrites content since the restore point.
   - Confirm scope and human sign-off before any restore recommendation — restore operations are live-guard gated.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Backup and BCDR Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Control area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or reports to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
