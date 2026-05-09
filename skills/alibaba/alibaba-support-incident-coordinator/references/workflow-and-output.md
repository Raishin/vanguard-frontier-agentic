# Workflow and output contract

Use this reference only when executing the full support incident coordination workflow, SLA enforcement, or post-incident review preparation.

## Coordination domains

Check these areas before giving a recommendation:

- Incident scope: affected services, account context (CN-* vs international), business impact
- Status page check: status.aliyun.com (CN-*) and status.alibabacloud.com (international)
- Severity classification: Urgent (紧急)/High (高)/Normal (中)/Low (低) — P0/P1/P2/P3 mapping
- Evidence collection and scrubbing: AccessKey IDs, PII, and unredacted log data removal
- Support ticket routing: CN-* console vs international console
- SLA tracking: response time deadlines and breach documentation
- Account manager escalation: direct contact protocol for P0
- Post-incident review: RCA request eligibility and process

## Safe workflow

1. **Confirm incident scope and account context**
   - Which services are affected and in which regions (CN-* vs international)?
   - What is the business impact (revenue, user count, SLA breach)?
2. **Check status pages**
   - Open status.aliyun.com for CN-* and status.alibabacloud.com for international.
   - If a platform incident is active, note the incident number for the support ticket.
3. **Classify severity**
   - Map business impact to Urgent/High/Normal/Low.
   - Confirm the account's support tier (Enterprise/Business/Developer).
4. **Collect and scrub evidence**
   - Remove AccessKey IDs, account numbers, customer PII, and unredacted log data.
   - Collect CloudMonitor screenshots, SLS log samples (scrubbed), and error message text.
5. **File ticket in the correct console**
   - CN-* workloads: file in the Alibaba Cloud CN console (aliyun.com).
   - International workloads: file in the Alibaba Cloud International console (alibabacloud.com).
6. **Track SLA and escalate**
   - Record ticket creation timestamp.
   - For Urgent: contact account manager directly if no response in 2 hours.
   - For High: escalate at 4 hours; Normal at 8 hours; Low at 24 hours.
7. **Request post-incident review for platform faults**
   - If Alibaba Cloud platform was at fault, request RCA report via the ticket or account manager.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Support Incident Coordination: <incident description>
## Executive summary
- Severity: Urgent (紧急) / High (高) / Normal (中) / Low (低)
- Account context: CN-* / international
- Platform status:
## Status page check
- CN-* (status.aliyun.com):
- International (status.alibabacloud.com):
- Active platform incident:
## Support ticket checklist
- [ ] Severity classified correctly
- [ ] Account context confirmed (CN-* vs international)
- [ ] Evidence scrubbed (no AccessKey IDs, PII, raw credentials)
- [ ] Filed in correct console
- Ticket creation timestamp:
- SLA deadline:
## Evidence collection and scrubbing
- Evidence collected:
- PII and credential scrubbing completed:
## Account manager escalation
- Escalation trigger time:
- Contact method:
## SLA tracking
- Response deadline:
- Breach documentation:
## Stakeholder communication template
> [Template text]
## Post-incident review
- Platform fault confirmed:
- RCA request filed:
- Expected delivery:
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
