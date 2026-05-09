# Workflow and output contract

Use this reference only when performing the full triage, escalation coordination, war room management, or post-incident review planning.

## Triage domains

Check these areas before classifying an incident:
- Impact scope: user-facing vs. internal, percentage of traffic affected, geographic scope
- GCP platform status: status.google.com check for active incidents or degradations in relevant services/regions
- P-level classification: P0 (complete outage, business impact), P1 (major impact, partial outage), P2 (partial impact, degraded service), P3 (minor impact, workaround available)
- Evidence trails: Cloud Monitoring dashboards, Cloud Logging queries, Cloud Trace, structured error logs
- Mitigation options: rollback, traffic rerouting, feature flag disable, capacity scaling
- Escalation triggers: support case severity, TAM contact, executive notification
- War room structure: incident commander, communications lead, mitigation engineer, evidence collector

## Safe workflow

1. **Frame scope**
   - Service(s) affected and impact radius:
   - First symptom observed and timestamp:
   - Business impact (revenue, users, SLA):
   - Escalation owner and incident commander:
2. **Collect evidence in parallel with mitigation**
   - Check status.google.com immediately — label result as live evidence.
   - Pull Cloud Monitoring dashboards for error rate, latency, saturation.
   - Query Cloud Logging for error spikes correlated with incident start time.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Classify and escalate**
   - P0: immediate war room + Severity 1 GCP support case + TAM contact.
   - P1: Severity 2 GCP support case + engineering lead notification.
   - P2: Severity 3 GCP support case + team notification.
   - P3: Severity 4 GCP support case + ticket tracking.
4. **Track SLA timers**
   - Premium Severity 1: 15-minute initial response SLA — escalate to TAM if breached.
   - Log case creation timestamp and first response timestamp.

## Output contract

Return this structure:
```markdown
# GCP Ticket Triage Escalation Coordinator: <incident title>
## Executive verdict
- Priority: P0 / P1 / P2 / P3
- Impact: <summary of affected users, services, regions>
- GCP platform status: <clean / degradation detected / incident active>
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Evidence collection checklist
- [ ] status.google.com checked
- [ ] Cloud Monitoring error rate dashboard reviewed
- [ ] Cloud Logging error spike query executed
- [ ] Cloud Trace sampled for latency distribution
## Mitigation options
1. <option> — risk: <risk>, rollback: <rollback>
## Escalation path
- GCP support case: Severity <1/2/3/4>, case created at: <timestamp>
- TAM contact required: <yes/no> — trigger: <SLA breach / P0 default>
- SLA timer: <start time> — response due by: <deadline>
## War room and stakeholders
- Incident commander: <owner>
- Communications lead: <owner>
- Next stakeholder update: <time>
## Post-incident review
- PIR due: <5 business days from resolution for P0/P1>
- Action items: <TBD at resolution>
## Residual risk
- <risk or explicit none>
```
