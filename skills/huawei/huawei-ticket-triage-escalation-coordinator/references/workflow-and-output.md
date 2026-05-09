# Workflow and output contract

Use this reference only when performing a full incident triage, escalation coordination, or post-incident review for a Huawei Cloud operational event.

## Triage domains

Check these areas before giving a verdict:

- Incident classification: P0/P1/P2/P3 with impact scope (services, users, regions affected)
- Status page: declared Managed Incident (MI) or clean
- Evidence: CES metrics, LTS logs, AOM alerts, CCE pod events collected and labeled
- Mitigation: immediate options available (failover, rollback, scaling, restart)
- Escalation path: support ticket created with correct severity, TAM notified if P0
- Stakeholder communication: war room formed, internal stakeholders notified
- Post-incident: PIR/RCA requested if platform fault confirmed

## Safe workflow

1. **Frame scope**
   - Incident type and observed symptoms:
   - Services and regions affected:
   - Current-state evidence:
   - Business impact:
   - Time of first detection:
2. **Collect evidence**
   - Check status.huaweicloud.com immediately.
   - Collect CES metrics, LTS logs, AOM alert history, and CCE pod events in parallel with mitigation.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the triage**
   - Is this a platform fault (MI declared) or user-side issue?
   - Is the support ticket created with the correct severity and complete evidence?
   - Is the TAM notified by phone if P0 and no response within 15 minutes?
   - Is AOM alert routing verified end-to-end (SMN notification)?
   - Is all evidence scrubbed of AK/SK, PII, and unredacted log data before ticket attachment?
   - What evidence is missing to confirm root cause?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged recovery, validation, and rollback.
   - If the safest action is to stop and gather more evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Incident Triage: <scope>
## Incident classification (P0/P1/P2/P3) and impact scope
## Huawei Cloud status page check result
## Evidence collection checklist (CES metrics, LTS logs, CCE events)
## Immediate mitigation options
## Huawei Cloud Premium Support escalation path and SLA tracking
## War room and stakeholder communication plan
## Post-incident review action items
```

Each section must include an evidence level label.
