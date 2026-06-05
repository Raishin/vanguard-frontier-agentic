# Azure Observability Investigation Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Calling the first correlated symptom the root cause.
- Ignoring missing telemetry or sampling gaps.
- Creating noisy alerts without action owner, severity, and suppression plan.
- Querying the wrong workspace, time range, or resource scope.
- Using dashboards as proof that alerting and incident response work.

## Officially grounded service shape

Microsoft Learn evidence says Azure Monitor collects, analyzes, and acts on logs, metrics, traces, and events across cloud and hybrid environments. Log Analytics workspaces store log and trace data queried with KQL, Azure Monitor workspaces store Prometheus/OpenTelemetry metrics, Application Insights provides application performance monitoring, and alerts/action groups support proactive response. Workbooks and Grafana visualize signals, but alerting should use Azure Monitor native alerts for Azure Monitor services.

- Azure Monitor is the unified observability service for metrics, logs, traces, and events.
- Metrics, logs, traces, activity logs, resource health, and service health answer different questions.
- Log Analytics uses KQL and workspace scope; workspace design affects access, cost, retention, and query correctness.
- Application Insights supports OpenTelemetry-based app monitoring, dependency maps, live metrics, failures, performance, and availability.
- Alerts need action groups, routing, severity, suppression/processing rules, and ownership.

## Non-negotiable design rules

- Separate observation, hypothesis, evidence, and inference.
- State time range, scope, query, signal source, and sampling caveat for every finding.
- Prefer narrow diagnostic changes before broad alert rewrites.
- Validate alert routing and action groups, not just alert rule existence.
- Protect sensitive telemetry and do not paste secrets or customer data from logs.

## Minimal safe implementation flow

- Scope incident/resource/workload, time window, user impact, and available telemetry.
- Inventory metrics, logs, traces, alerts, action groups, dashboards, diagnostic settings, and workspace coverage.
- Run focused KQL/metric checks or request sampled evidence where tools are unavailable.
- Correlate signals and classify root cause, contributing factor, symptom, or unknown.
- Return findings, confidence, telemetry gaps, next diagnostics, and safe remediation.

## High-risk assumptions to kill

- The first correlated alert is not the root cause; root cause needs time-bounded, scoped evidence across metrics, logs, traces, events, and changes.
- A dashboard is not proof that telemetry, alerting, action routing, or incident response works.
- Missing telemetry is a finding. Do not invent certainty when diagnostic settings, workspace coverage, sampling, or retention are incomplete.
- Log Analytics workspaces and Azure Monitor workspaces are different stores with different query models; querying the wrong store invalidates conclusions.
- Alert changes can hide real incidents; suppression, thresholds, and action group changes require ownership and rollback.

## Safe command/code verification targets

- Record resource scope, workspace, time range, query text, signal source, sampling status, and data freshness for every finding.
- Verify diagnostic settings, data collection rules, workspace retention, Application Insights/OpenTelemetry coverage, and alert/action group routes.
- Separate observations, hypotheses, inferred causes, confirmed causes, and telemetry gaps in the final output.
- Test alert routing or action groups with safe evidence before claiming response readiness.
- Redact secrets, customer identifiers, and sensitive payloads from logs before storing or sharing evidence.

## Safe verification targets

- Diagnostic settings send required platform logs and metrics to the intended destination.
- Workspace scope, retention, access model, and query performance fit the investigation.
- Application Insights or OpenTelemetry captures requests, dependencies, exceptions, traces, availability, and sampling status where relevant.
- Alerts have owner, severity, threshold rationale, action group route, and noise controls.
- Workbooks/Grafana dashboards reflect authoritative signals and are not the only evidence.

## When to push back

- Telemetry is missing for the claimed root cause.
- The time window or resource scope is ambiguous.
- Alert changes would silence production incidents without owner approval.
- Logs contain secrets or customer data that should be redacted before sharing.
