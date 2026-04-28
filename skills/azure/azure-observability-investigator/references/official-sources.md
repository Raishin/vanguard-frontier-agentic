# Official Sources

Load these only when needed:

- [Monitor Azure resources with Azure Monitor](https://learn.microsoft.com/azure/azure-monitor/platform/monitor-azure-resource) — use for the basic monitoring surface: Activity Log, Alerts, Metrics, Diagnostic settings, and Logs.
- [Introduction to Application Insights - OpenTelemetry observability](https://learn.microsoft.com/azure/azure-monitor/app/app-insights-overview) — use for Application Insights investigation views, telemetry paths, and OpenTelemetry-oriented instrumentation expectations.
- [Architecture strategies for designing a monitoring system](https://learn.microsoft.com/azure/well-architected/operational-excellence/observability) — use for the broader observability-system architecture, including Azure Monitor, Log Analytics, Network Watcher, and operational design.
- [Architecture best practices for Log Analytics](https://learn.microsoft.com/azure/well-architected/service-guides/azure-log-analytics) — use for workspace design, reliability, retention, and operational-excellence tradeoffs.
- [Architecture best practices for Application Insights](https://learn.microsoft.com/azure/well-architected/service-guides/application-insights) — use for environment separation, workspace dependency, and alerting or instrumentation quality.
- [Azure Monitor alerts overview](https://learn.microsoft.com/azure/azure-monitor/alerts/alerts-overview) — use for alert types, routing, and action-group behavior.
- [Azure MCP Server tools inventory](https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/) — use to verify `monitor`, `applicationinsights`, `workbooks`, `grafana`, or other documented namespaces before naming them.

## Grounded insights worth carrying into the skill

- Resource logs do not become queryable just because a service exists; Microsoft explicitly states you need diagnostic settings to route them.
- Application Insights is part of Azure Monitor and depends on its underlying workspace and routing design; do not treat it as a standalone magic box.
- Microsoft recommends one Application Insights resource per workload per environment to avoid mixed telemetry and investigation confusion.
- Dashboards and workbooks are downstream views. If the underlying logs and metrics are missing or misrouted, the pretty dashboard proves nothing.
