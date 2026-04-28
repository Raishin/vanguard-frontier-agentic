# Official Sources

## References

Load these only when needed:

- [Azure landing zone overview](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/) — use for platform-versus-application landing zone boundaries and Microsoft’s recommended Azure landing zone operating model.
- [Platform landing zone implementation options](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/implementation-options) — use for choosing between IaC accelerator, Bicep, Terraform, and portal-based approaches.
- [Azure landing zone Bicep guidance](https://learn.microsoft.com/en-us/azure/architecture/landing-zones/bicep/landing-zone-bicep) — use for Bicep accelerator structure, modular delivery, and deployment-stack-aware platform automation.
- [Terraform landing zone guidance](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/ready/landing-zone/terraform-landing-zone) — use when the user is standardizing on Terraform for Azure landing zone delivery.
- [Deployment best practices for Azure App Service](https://learn.microsoft.com/en-us/azure/app-service/deploy-best-practices) — use for deployment-source/build/deploy separation, slot-safe release patterns, and production safety.
- [App Service staging slots](https://learn.microsoft.com/en-us/azure/app-service/deploy-staging-slots?view=azure-devops-2020) — use for swap-based rollout, warm-up, and rollback patterns.
- [Azure MCP Server tools inventory](https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/) — use to verify which official Azure MCP capabilities actually exist before suggesting them.
- [Azure Deploy tools for Azure MCP Server](https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/azure-deploy) — use when live deploy-oriented MCP support is relevant.
- [Bicep MCP server](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/bicep-mcp-server) — use when schema-aware Bicep authoring support is relevant and to avoid inventing deploy behavior it does not provide.
- [Bicep documentation](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/) — use for authoritative Bicep deployment, modules, scopes, and what-if references.
