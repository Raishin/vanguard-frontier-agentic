# MCP and Evidence Path

## Evidence path

Prefer evidence in this order:

1. Azure Monitor fundamentals and analysis guidance:
   - https://learn.microsoft.com/en-us/azure/azure-monitor/overview
   - https://learn.microsoft.com/en-us/azure/azure-monitor/best-practices-analysis
2. Alerts and notification flow:
   - https://learn.microsoft.com/en-us/azure/azure-monitor/alerts/alerts-overview
   - https://learn.microsoft.com/en-us/azure/azure-monitor/alerts/action-groups
   - https://learn.microsoft.com/en-us/azure/azure-monitor/alerts/alerts-processing-rules
3. Log Analytics and query posture:
   - https://learn.microsoft.com/en-us/azure/azure-monitor/logs/log-analytics-workspace-overview
   - https://learn.microsoft.com/en-us/azure/azure-monitor/logs/workspace-design
   - https://learn.microsoft.com/en-us/azure/azure-monitor/logs/get-started-queries
4. Application Insights and application telemetry:
   - https://learn.microsoft.com/en-us/azure/azure-monitor/app/app-insights-overview
   - https://learn.microsoft.com/en-us/azure/well-architected/service-guides/application-insights
5. Visualization and reporting:
   - https://learn.microsoft.com/en-us/azure/azure-monitor/visualize/workbooks-overview
   - https://learn.microsoft.com/en-us/azure/azure-monitor/visualize/visualize-grafana-overview
6. Azure MCP discovery and monitor tools, when supported in the client:
   - https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/
   - https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/azure-monitor
   - https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/application-insights

Only mention Azure MCP namespaces when they are actually useful to the task. Based on the repo spec, relevant namespaces can include:

- `monitor` for logs and metrics
- `applicationinsights` for Application Insights resource discovery, not as a substitute for full Azure Monitor analysis
- `kusto` only if Azure Data Explorer is actually part of the observability stack
- `workbooks`
- `grafana`

Use live MCP evidence to reduce guesswork. Do not pretend a namespace is available if the client does not expose it.
