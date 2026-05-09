# Workflow and output contract

Use this reference only when performing a full incident response or observability review.

## Observability areas to check

- CES alarms: active alarms, alarm history, threshold configuration, resource coverage gaps, silenced alarms
- LTS: log group retention periods (MLPS: 180-day minimum), active SQL queries, scheduled alert coverage, login audit completeness
- AOM: topology map health, critical path identification, alert aggregation rules, missing agent coverage
- APM: trace completeness, service dependency map, error rate per service, latency distribution
- SMN: topic inventory, subscriber health, bound alarm count per topic, recent delivery failures

## Incident response workflow

1. **Triage CES alarms** — identify active and recently fired alarms; prioritize by severity.
2. **LTS log triage** — query LTS for error patterns in the 30 minutes before alarm onset.
3. **AOM topology** — identify affected services and their upstream/downstream dependencies.
4. **APM trace** — pull distributed traces for failing requests to identify the root-cause service.
5. **Root cause** — correlate metric, log, topology, and trace evidence into a single root cause statement.
6. **Remediation** — recommend narrowest safe action with rollback.

## Safe workflow

1. **Frame scope** — confirm incident timeline, affected services, and non-goals
2. **Collect evidence** — prefer live CES/LTS/APM data; label all evidence types
3. **Stress-test** — blast radius, missing coverage, SMN routing gaps
4. **Recommend safest action** — staged remediation with alert coverage restoration

## Output contract

Return this structure:

```markdown
# Huawei Cloud Incident Response: <scope>
## Scope and evidence level
## CES alarm inventory and active alarms
## LTS log findings
## AOM topology blast-radius
## APM trace root cause
## SMN notification routing health
## Recommended remediation
## Open questions
```

Each section must include an evidence level label.
