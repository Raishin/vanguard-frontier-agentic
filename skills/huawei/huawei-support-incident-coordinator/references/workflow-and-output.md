# Workflow and output contract

Use this reference only when performing a full support incident coordination, SLA enforcement review, or post-incident packaging for a Huawei Cloud support case.

## Coordination domains

Check these areas before giving a verdict:

- Incident scope: services affected, region, business impact, time of detection
- Status page: Managed Incident (MI) declared or clean; platform-owned vs user-owned root cause
- Ticket creation: correct severity assigned, region-appropriate support team targeted, evidence attached
- Evidence scrubbing: AK/SK, account IDs, customer PII, and unredacted log data removed before submission
- Escalation path: TAM notified by phone for P0, Account Manager in the loop
- SLA tracking: response timestamp logged, breach threshold monitored
- Stakeholder communication: internal stakeholders notified, communication cadence set
- Post-incident review: PIR/RCA formally requested for platform faults, SLA credit documented

## Safe workflow

1. **Frame scope**
   - Services and regions affected:
   - Business impact:
   - Time of first detection:
   - Current-state evidence:
   - Required outcome:
2. **Collect evidence**
   - Check status.huaweicloud.com immediately for declared Managed Incidents.
   - Collect CES metrics, LTS logs, AOM alert history, and application error details in parallel with mitigation.
   - Scrub all evidence of AK/SK, account IDs, customer PII, and unredacted log content before ticket attachment.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the coordination**
   - Is the severity correctly classified (Urgent/High/Normal/Low)?
   - Is the ticket routed to the correct regional support team (China vs international)?
   - Is the TAM notified by phone for P0 if no portal response within 15 minutes?
   - Is the status page showing a Managed Incident that changes the coordination approach?
   - Are all attachments scrubbed of sensitive data?
   - Is SLA breach timing documented for contractual credit claims?
   - Is PIR/RCA formally requested for platform-caused incidents?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged recovery, validation, and rollback.
   - If the safest action is to stop and gather more evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Support Incident Coordination: <scope>
## Incident scope and initial status page check
## Support ticket creation checklist and severity classification
## Evidence collection and scrubbing guidance
## TAM and Account Manager escalation path and contact protocol
## SLA tracking and follow-up cadence
## Stakeholder communication template
## Post-incident review coordination
```

Each section must include an evidence level label.
