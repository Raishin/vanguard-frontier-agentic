# Azure Observability Investigator operations

Version note: refreshed 2026-06-05 from Microsoft Learn documentation through the user's configured documentation MCP. Documentation-based evidence does not prove any user's deployed Azure state.

## What people get wrong

An alert is a symptom, not root cause. Missing telemetry is evidence too; label it instead of guessing.

## Officially grounded service shape

Azure Monitor unifies metrics, logs, traces, events, alerting, and application telemetry across cloud and hybrid resources. Alerts combine resource, signal, condition, and action routing. That is the key insight: investigation quality depends on signal coverage and query evidence, not alert volume.

## Non-negotiable design rules

1. Separate observed facts, sampled evidence, documentation-based behavior, and inference.
2. Do not claim root cause without corroborating telemetry.
3. Treat missing diagnostic settings, workspace gaps, and action group routing gaps as blockers or open risks.
4. Scope KQL queries narrowly and avoid exposing sensitive payloads.
5. Verify alert rule condition, statefulness, target resource, action group, and suppression behavior before recommending alert changes.

## Minimal safe implementation flow

1. Classify symptom, impact window, affected resources, and telemetry sources.
2. Ground Azure Monitor and alert behavior in Microsoft Learn.
3. Collect or request only sanitized sampled evidence: metrics, logs, traces, alert history, and config snippets.
4. Identify telemetry gaps before forming a root-cause hypothesis.
5. Return verdict, evidence level, likely causes, blockers, next probes, and open questions.

## High-risk assumptions to kill

- The loudest alert is the root cause.
- No logs means no problem.
- Application Insights alone covers infrastructure, network, and identity dependencies.
- Changing alert thresholds fixes incident response.
- Documentation proves the user's diagnostics are enabled.

## Safe command/code verification targets

- Metric trends, log query windows, trace correlation, deployment events, activity logs, and alert history.
- Diagnostic settings, workspace routing, retention, table availability, and sampling configuration.
- Alert rules, dimensions, evaluation frequency, statefulness, processing rules, and action groups.

## When to push back

- Telemetry coverage is too thin to support a root-cause claim.
- The requested KQL would expose sensitive data.
- Alert changes are proposed without action group and suppression review.
- The incident window or affected scope is undefined.
