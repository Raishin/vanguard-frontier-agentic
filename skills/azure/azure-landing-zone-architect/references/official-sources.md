# Official Sources

Load these only when needed:

- [What is an Azure landing zone?](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/) — use for platform versus application landing zones and the reference-architecture baseline.
- [Azure landing zone design areas and conceptual architecture](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-areas) — use for the design-area map and the dependency between resource organization, networking, governance, management, and automation.
- [Azure landing zone design principles](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-principles) — use for policy-driven governance, Azure-native alignment, and avoiding application-agnostic hierarchy mistakes.
- [Deploy Azure landing zones](https://learn.microsoft.com/azure/architecture/landing-zones/landing-zone-deploy) — use for platform landing zone deployment approaches and application landing zone patterns.
- [Platform landing zone vs. application landing zones](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/#platform-landing-zone-vs-application-landing-zones) — use when teams are blurring shared platform services with workload-local ownership.
- [Tailor the Azure landing zone architecture to meet requirements](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/tailoring-alz) — use when the user wants to deviate from the reference architecture without pretending there is one canonical hierarchy.
- [Ready your Azure environment for workloads](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/) — use for the baseline expectation that management, governance, security, and monitoring apply across subscriptions.
- [Azure MCP Server tools inventory](https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/) — use to verify `cloudarchitect`, `policy`, `group`, `subscription`, `role`, or `wellarchitectedframework` before naming them.

## Grounded insights worth carrying into the skill

- Microsoft’s landing-zone guidance is explicitly modular and should be tailored; a single canned hierarchy is usually a sign of lazy thinking.
- Platform landing zones and application landing zones are different operating boundaries; mixing them casually creates ownership and governance confusion.
- Azure AI workloads do not require a separate “AI landing zone” by default; Microsoft says they should usually fit inside normal application landing zones governed by the same design areas.
- A landing zone is not complete if management, governance, monitoring, and recovery posture are still deferred.
