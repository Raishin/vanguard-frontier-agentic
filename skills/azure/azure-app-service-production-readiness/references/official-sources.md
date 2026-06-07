# Official sources for Azure App Service Production Readiness

Use Microsoft Learn documentation through the user's configured documentation MCP to ground App Service reviews. Documentation proves service guidance; it does not prove the user's app settings, plan SKU, network routes, diagnostics, backup success, or production readiness.

## Primary Microsoft Learn sources

| Source | Review implication |
| --- | --- |
| [Architecture best practices for Azure App Service Web Apps](https://learn.microsoft.com/en-us/azure/well-architected/service-guides/app-service-web-apps) | Ground reliability, security, cost, operational excellence, scaling, health check, slots, managed identity, diagnostics, and tradeoff guidance. |
| [Reliability in Azure App Service](https://learn.microsoft.com/en-us/azure/reliability/reliability-app-service) | Use for shared responsibility, zone/region outage posture, backup, transient faults, maintenance, and SLA framing. |
| [Deploy to deployment slots](https://learn.microsoft.com/en-us/azure/app-service/deploy-staging-slots) | Use for slot strategy, swap behavior, warm-up, sticky settings, and rollback checks. |
| [Key Vault references for App Service](https://learn.microsoft.com/en-us/azure/app-service/app-service-key-vault-references) | Use for managed identity, secret reference, and network-restricted vault behavior. |
| [Monitor App Service instances with health check](https://learn.microsoft.com/en-us/azure/app-service/monitor-instances-health-check) | Use for health path, unhealthy instance removal, and readiness checks. |
| [App Service networking features](https://learn.microsoft.com/en-us/azure/app-service/networking-features) | Use for VNet integration, private endpoints, access restrictions, and outbound routing semantics. |
| [Manage backup and restore in App Service](https://learn.microsoft.com/en-us/azure/app-service/manage-backup) | Use for backup support and linked-database backup deprecation caveats. |
| [Baseline highly available zone-redundant web application](https://learn.microsoft.com/en-us/azure/architecture/web-apps/app-service/architectures/baseline-zone-redundant) | Use for production architecture, slots, package deployment, Key Vault references, private endpoints, and zone redundancy pattern. |

## Source-grounding rules

- A plan SKU recommendation from docs is not proof the current app is correctly scaled.
- A configured slot in code is not proof of safe swap. Validate sticky settings, warm-up, health, and rollback evidence.
- A private endpoint is not proof of private-only app posture. Check public access/access restrictions, DNS, reverse proxy path, and outbound dependencies.
- Backup configured is not recovery proven. Require restore test evidence.
