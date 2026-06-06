# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, subscription, RBAC, quota, migration project, network, telemetry, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/azure-monitor/fundamentals/overview
- https://learn.microsoft.com/azure/azure-monitor/fundamentals/best-practices-operation
- https://learn.microsoft.com/azure/azure-monitor/alerts/alerts-overview
- https://learn.microsoft.com/azure/azure-monitor/alerts/action-groups
- https://learn.microsoft.com/azure/azure-monitor/alerts/alerts-processing-rules
- https://learn.microsoft.com/azure/azure-monitor/logs/log-analytics-overview
- https://learn.microsoft.com/azure/azure-monitor/logs/workspace-design
- https://learn.microsoft.com/azure/azure-monitor/app/app-insights-overview
- https://learn.microsoft.com/azure/azure-monitor/visualize/workbooks-overview
- https://learn.microsoft.com/azure/managed-grafana/how-to-use-azure-monitor-alerts

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says Azure Monitor collects, analyzes, and acts on logs, metrics, traces, and events across cloud and hybrid environments. Log Analytics workspaces store log and trace data queried with KQL, Azure Monitor workspaces store Prometheus/OpenTelemetry metrics, Application Insights provides application performance monitoring, and alerts/action groups support proactive response. Workbooks and Grafana visualize signals, but alerting should use Azure Monitor native alerts for Azure Monitor services.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
