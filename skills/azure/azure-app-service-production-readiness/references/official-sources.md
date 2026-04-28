# Official Sources

## References

Load these only when needed:

- [Architecture best practices for Azure App Service (Web Apps)](https://learn.microsoft.com/en-us/azure/well-architected/service-guides/app-service-web-apps) — use for pillar-based production design tradeoffs and shared-responsibility framing.
- [Deployment best practices](https://learn.microsoft.com/en-us/azure/app-service/deploy-best-practices) — use for source/build/deploy separation, slot-first rollout, and anti-pattern detection.
- [Set up staging environments in Azure App Service](https://learn.microsoft.com/en-us/azure/app-service/deploy-staging-slots) — use for slot behavior, swap boundaries, and rollback realism.
- [Best practices for Azure App Service](https://learn.microsoft.com/en-us/azure/app-service/app-service-best-practices) — use for runtime, scaling, cert, and diagnostics guidance.
- [Scale up an app in Azure App Service](https://learn.microsoft.com/en-us/azure/app-service/manage-scale-up) — use for plan-tier capability questions and scale-up tradeoffs.
- [Enable virtual network integration](https://learn.microsoft.com/en-us/azure/app-service/configure-vnet-integration-enable) — use for outbound private reachability requirements and subnet constraints.
- [Manage App Service virtual network integration routing](https://learn.microsoft.com/en-us/azure/app-service/configure-vnet-integration-routing) — use when outbound routing, image pulls, backup, content share, or managed-identity pathing matters.
- [Use private endpoints for Azure App Service apps](https://learn.microsoft.com/en-us/azure/app-service/overview-private-endpoint) — use for inbound private access, subnet separation, and public exposure reduction.
- [App Service access restrictions](https://learn.microsoft.com/en-us/azure/app-service/overview-access-restrictions) — use for public-endpoint filtering and to avoid confusing it with private-endpoint controls.
- [Use Key Vault references as app settings](https://learn.microsoft.com/en-us/azure/app-service/app-service-key-vault-references) — use for managed-identity-based secret retrieval, slot-setting guidance, and rotation caveats.
- [Monitor App Service instances by using Health check](https://learn.microsoft.com/en-us/azure/app-service/monitor-instances-health-check) — use for health endpoint expectations, unhealthy-instance behavior, and swap implications.
- [Back up and restore your app in Azure App Service](https://learn.microsoft.com/en-us/azure/app-service/manage-backup) — use for tier-dependent backup and restore constraints.
- [Configure App Service plans for zone redundancy](https://learn.microsoft.com/en-us/azure/app-service/configure-zone-redundancy) — use for zone support checks and minimum-instance expectations.
- [Reliability in Azure App Service](https://learn.microsoft.com/en-us/azure/reliability/reliability-app-service) — use for SLA/reliability framing, shared responsibility, and recovery expectations.
- [Azure MCP Server tools inventory](https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/) — use to verify official Azure MCP namespaces before naming them.
- [Azure MCP Server tools for Azure App Service](https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/azure-app-service) — use to confirm the actual App Service MCP operations and their limits.
