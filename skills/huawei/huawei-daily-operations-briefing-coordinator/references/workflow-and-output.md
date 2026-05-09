# Workflow and output contract

Use this reference only when facilitating a full daily Huawei Cloud operations briefing or producing the complete standup agenda with action item tracking.

## Briefing domains

Check these areas in sequence before giving the briefing verdict:

- CBC cost delta: by Enterprise Project, day-over-day, flagged anomalies (>15%), investigation owners assigned
- AOM alert anomalies: unacknowledged HIGH and CRITICAL alerts in last 24h, owner assignments, SMN routing verified
- CCE health: pod failure counts by AZ, multi-AZ failures escalated, application vs cluster-level triage
- CES quota: utilization for ECS, EIP, GaussDB above 80%, quota increase requests submitted
- SecMaster findings: HIGH and CRITICAL finding age, owner assignment status, SLA breach identification
- LTS log spikes: error rate vs 7-day baseline, anomalous service log streams identified
- Action items: open items carried from prior day, new items generated in today's briefing, owners and due times

## Safe workflow

1. **Frame scope**
   - Date and briefing time:
   - Enterprise Projects in scope:
   - Regions in scope:
   - Prior day's open action items:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect sanitized user-provided data exports or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the briefing**
   - Are there CBC cost spikes without investigation owners?
   - Are there unacknowledged AOM HIGH or CRITICAL alerts without owners?
   - Are CCE pod failures spanning more than one AZ being treated as application issues (wrong triage)?
   - Are any CES quotas above 80% without a quota increase request in progress?
   - Are SecMaster HIGH or CRITICAL findings older than 24h without owners (SLA breach)?
   - Are any LTS log streams showing >3x baseline error rates?
   - Are any action items from the prior briefing overdue?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged recovery, validation, and rollback.
   - If the safest action is to stop and gather more evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Daily Operations Briefing: <date>
## CBC cost delta summary by Enterprise Project
## AOM alert anomaly triage
## CCE and application health summary
## CES quota utilization warnings
## SecMaster security finding triage
## LTS log error spike review
## Open action items with owners and next 24-hour risk summary
```

Each section must include an evidence level label.
