# Official sources

Use these sources to ground service behavior before making production recommendations.

## Official documentation checked on 2026-06-05

- https://docs.oracle.com/iaas/Content/Network/Concepts/fastconnect.htm
- https://docs.oracle.com/iaas/Content/Network/Concepts/fastconnectoverview.htm
- https://docs.oracle.com/en-us/iaas/Content/Network/Concepts/routingonprem2.htm
- https://learn.microsoft.com/azure/virtual-machines/workloads/oracle/configure-azure-oci-networking
- https://learn.microsoft.com/azure/cloud-adoption-framework/ready/azure-best-practices/connectivity-to-other-providers-oci

## Sampled read-only evidence checked on 2026-06-05

- OCI API evidence through the user's configured read-only OCI MCP was used for command/API surface shape only.
- Microsoft Learn documentation through the user's configured documentation MCP was used only where Azure interconnect behavior is in scope.
- Command-help evidence confirms available list/filter surfaces; it does not prove permissions, resource existence, regional availability, capacity, quota, data correctness, traffic safety, or production readiness.

## Grounding rules

- Prefer the most specific official documentation page for the service or feature being discussed.
- If documentation and sampled API evidence appear to conflict, report the conflict and avoid stronger claims until resolved.
- Do not cite internal tool names, local environment labels, connector identifiers, or environment-specific details in committed docs.
- Do not paste sensitive identifiers or customer data into examples.
