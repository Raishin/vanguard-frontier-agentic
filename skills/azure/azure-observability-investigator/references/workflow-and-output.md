# Workflow and Output Contract

## Workflow

1. Classify the problem before querying anything.
   - metrics symptom,
   - logs symptom,
   - traces or dependency symptom,
   - alerting symptom,
   - configuration or coverage gap,
   - mixed incident where multiple signal types must be correlated.
2. Establish incident frame and uncertainty.
   - what broke,
   - when it started,
   - whether the issue is current or historical,
   - affected workload, region, environment, and blast radius,
   - what telemetry is expected but may be missing.
3. Gather the highest-value evidence first.
   - current or recent alerts,
   - resource and application metrics,
   - relevant Log Analytics tables,
   - Application Insights failures, dependencies, requests, exceptions, or availability signals,
   - workbook or dashboard views only after validating the underlying data source.
4. Separate evidence from correlation from inference.
   - **Evidence**: direct signal observed in metrics, logs, traces, or alert configuration.
   - **Correlation**: timing or co-occurrence that suggests a relationship.
   - **Inference**: likely explanation that is not directly proven by the collected signals.
5. Investigate alert quality, not just whether an alert fired.
   - signal type and threshold quality,
   - action-group wiring,
   - alert-processing rules that suppress, route, or mutate behavior,
   - duplicate or overlapping alerts,
   - stale alerts tied to bad dimensions, wrong scope, or wrong evaluation windows,
   - absence-of-data situations that may actually be ingestion or telemetry failure.
6. Review telemetry coverage and workspace posture.
   - whether the right resource or app emits telemetry at all,
   - whether logs, metrics, and traces land in the expected workspace or resource,
   - whether workspace sprawl is obscuring triage,
   - whether retention, table design, or access mode blocks investigation,
   - whether cross-resource or cross-workspace queries are required.
   - whether diagnostic settings were ever enabled; without them, many platform logs will never be queryable.
   - whether Application Insights is separated per workload and environment or mixed together into unusable noise.
7. Use KQL deliberately.
   - start narrow with timeframe, resource, operation, or failure boundary,
   - verify table/schema assumptions before writing fancy joins,
   - correlate requests, dependencies, exceptions, and platform logs only when the data actually exists,
   - if no query can prove the claim, say so.
8. Assess observability outputs.
   - workbooks should reflect validated source data, not decorative charts,
   - Grafana or workbook dashboards are downstream views, not primary evidence,
   - recommend query, visualization, or ownership fixes only after validating source signals.
9. End with operator actions.
   - immediate triage next steps,
   - alert tuning or suppression fixes,
   - telemetry additions or routing corrections,
   - ownership handoff if platform versus application responsibility is split,
   - residual blind spots that prevent stronger conclusions.

## Output contract

Return all of the following:

- **Issue summary**: what symptom is being investigated and its apparent blast radius.
- **Signals reviewed**: metrics, logs, traces, alerts, workbooks, dashboards, and namespaces actually used.
- **Evidence table**: direct evidence, correlated signals, and explicit inferences kept separate.
- **Likely failure domain**: application, platform configuration, alerting design, workspace design, telemetry gap, or unresolved.
- **Telemetry gaps**: what data is missing, delayed, misrouted, or too low-quality to support confident diagnosis.
- **Recommended improvements**: concrete query, alert, action-group, alert-processing-rule, workbook, or workspace changes.
- **Next diagnostic steps**: bounded follow-up actions in priority order.
- **Assumptions and unknowns**: what remains unproven.

Use a response shape like:

```text
Issue summary
- ...

Signals reviewed
- Metrics: ...
- Logs: ...
- Traces/Application Insights: ...
- Alerts and routing: ...

Evidence vs inference
- Evidence: ...
- Correlation: ...
- Inference: ...

Likely failure domain
- ...

Telemetry gaps
- ...

Recommended improvements
- ...

Next diagnostic steps
- ...

Assumptions and unknowns
- ...
```

## Eval gate

Treat the answer as failing if it does any of the following:

- claims root cause without separating evidence from inference,
- ignores ingestion delay or missing telemetry risk,
- recommends alert changes without checking signal quality, scope, or routing,
- treats workbook or dashboard visuals as proof without validating underlying data,
- skips Log Analytics or Application Insights when they are central to the ask,
- omits telemetry gaps and residual blind spots,
- returns generic Azure Monitor advice with no operator next steps.

Minimum scenarios this skill should handle:

1. noisy or duplicate alert review,
2. recent application or platform failure investigation,
3. baseline telemetry-gap assessment for an under-instrumented workload.

## Safety notes

- Do not pretend logs prove what they do not prove.
- Do not collapse correlation into causation.
- Do not assume lack of logs means lack of failure; it may mean telemetry never arrived.
- Do not recommend broad alert proliferation as a fix for poor signal design.
- Do not expose secrets, customer payloads, tokens, or sensitive query results in the response.
- Do not suggest mutating production alerting or workbook content without calling out rollback or verification steps.
- Be explicit when a conclusion is based on partial telemetry, stale dashboards, or unverified workspace assumptions.
