# Official Sources

Load these only when needed:

- [Network topology and connectivity](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/network-topology-and-connectivity) — use for the landing-zone design-area framing and connectivity-management-group intent.
- [Hub-spoke network topology in Azure](https://learn.microsoft.com/azure/architecture/networking/architecture/hub-spoke) — use for hub-spoke recommendations, non-transitive peering, spoke-to-spoke patterns, and gateway transit.
- [Azure Private Link in a hub-and-spoke network](https://learn.microsoft.com/azure/architecture/networking/guide/private-link-hub-spoke-network) — use for hub-versus-spoke private endpoint placement, `/32` route propagation, DNS requirements, and on-premises implications.
- [Private Link and DNS integration at scale](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/azure-best-practices/private-link-and-dns-integration-at-scale) — use when DNS and private endpoint resolution are the real bottleneck.
- [Integrate Azure services with virtual networks for network isolation](https://learn.microsoft.com/azure/virtual-network/vnet-integration-for-azure-services) — use when the user is confusing Private Link with other integration patterns.
- [Azure landing zone design areas and conceptual architecture](https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-areas) — use when topology choices are coupled to broader landing-zone organization.
- [Azure MCP Server tools inventory](https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/) — use to verify `group`, `subscription`, `monitor`, or other documented namespaces before naming them.

## Grounded insights worth carrying into the skill

- Virtual network peering is non-transitive. Any answer that assumes transitivity is broken.
- Private endpoints in spokes inject `/32` routes that propagate across peerings and VPN or ExpressRoute paths; if you ignore that, your topology advice is shallow.
- Private endpoint success depends on deliberate DNS zone linkage and resolution path, not just resource placement.
- In Virtual WAN, private endpoints belong in connected spoke virtual networks, not in the hub itself.
