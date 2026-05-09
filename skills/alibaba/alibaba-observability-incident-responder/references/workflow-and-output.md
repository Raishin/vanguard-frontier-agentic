# Workflow and output contract

Use this reference only when performing a full incident triage, observability review, or alert governance assessment.

## Observability areas to check

- CloudMonitor: alarm inventory (metric + event + site), firing alarms, silenced alarms, contact group configuration, notification channel health
- SLS: logstore index configuration, query coverage for affected services, scheduled alert rules, logstore TTL vs. forensic evidence requirement
- ARMS APM: agent coverage for affected services, distributed trace for the incident time window, service topology error propagation, SLO breach details
- Alert governance: alarm threshold justification, alarm fire rate (> 3/week = noise), duplicate or redundant alarms, escalation path gaps
- Incident timeline: first alarm fire time, acknowledge time, diagnosis duration, remediation time, recovery confirmation

## Safe workflow

1. **Frame scope** — confirm affected services, incident time window, evidence available, and explicit non-goals
2. **Collect evidence** — alarm state, SLS log query results, ARMS traces; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what is the blast radius? what is unmonitored? what is the confidence level of the root cause hypothesis?
4. **Recommend safest action** — narrow scope, staged remediation, rollback path

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Observability Incident: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
