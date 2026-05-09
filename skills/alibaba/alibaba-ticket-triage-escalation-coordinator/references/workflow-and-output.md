# Workflow and output contract

Use this reference only when executing the full incident triage workflow, escalation coordination, or post-incident review preparation.

## Triage domains

Check these areas before giving a recommendation:

- Incident scope: affected services, regions (CN-* vs international), user impact
- Alibaba Cloud status page: CN-* (status.aliyun.com) and international (status.alibabacloud.com)
- Severity classification: P0/P1/P2/P3 based on business impact and recovery time objective
- Evidence collection: CloudMonitor metrics, SLS log samples, RDS slow query logs, ActionTrail API call history
- Support ticket: severity mapping (紧急/高/中/低), account context, and correct console
- SLA tracking: response time deadline per priority and support tier
- DingTalk war room: required participants, update cadence, and communication template

## Safe workflow

1. **Classify the incident**
   - What services are affected and in which regions (CN-* vs international)?
   - What is the business impact (revenue loss, user count, SLA breach)?
   - Assign P0/P1/P2/P3 based on impact and recovery urgency.
2. **Check platform status**
   - Open status.aliyun.com (CN-*) and status.alibabacloud.com (international).
   - If a platform incident is active, reference it in the support ticket.
3. **Collect evidence in parallel with mitigation**
   - CloudMonitor: error rate, latency, CPU/memory spikes.
   - SLS: log error patterns, slow request samples (scrubbed of PII).
   - RDS: slow query log, connection pool exhaustion metrics.
   - ActionTrail: unusual API calls in the 30-minute window before incident.
4. **File support ticket with correct severity and account context**
   - CN-* workloads: file in Alibaba Cloud CN console.
   - International workloads: file in Alibaba Cloud International console.
   - Attach scrubbed evidence; never include credentials or raw PII.
5. **Track SLA and escalate**
   - Record ticket creation timestamp.
   - If Urgent (P0): escalate to account manager if no response in 2 hours.
   - If High (P1): escalate if no response in 4 hours.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Incident Triage: <incident description>
## Executive summary
- Severity: P0 / P1 / P2 / P3
- Impact scope:
- Platform status (CN-* / international):
## Evidence collection checklist
- [ ] CloudMonitor: error rate, latency, resource utilization
- [ ] SLS: log error patterns (scrubbed)
- [ ] RDS: slow query log, connection pool
- [ ] ActionTrail: unusual API calls
## Immediate mitigation options
1. <option> — expected impact: <impact>
## Support escalation
- Ticket severity: 紧急 / 高 / 中 / 低
- Account context: CN-* / international
- Ticket creation timestamp:
- SLA deadline:
- Account manager escalation trigger:
## DingTalk war room
- War room name:
- Required participants:
- Update cadence:
## Stakeholder communication template
> [Template text]
## Post-incident review action items
1. <action> — owner: <owner>, due: <date>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
