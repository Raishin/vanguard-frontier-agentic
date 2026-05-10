# Official sources

Use this reference only when grounding IONOS DCD topology concepts, resource behavior, or IaC definitions.

## IONOS Cloud DCD and compute documentation

Use these as starting points, not as proof of the user's live DCD configuration:

- https://docs.ionos.com/cloud/compute-engine/data-center-designer — DCD graphical interface, topology concepts, and resource lifecycle
- https://docs.ionos.com/cloud/compute-engine/virtual-servers — server sizing, NIC configuration, and volume attachment behavior
- https://docs.ionos.com/cloud/network/lans — private LAN design, segmentation, and IP assignment
- https://registry.terraform.io/providers/ionos-cloud/ionoscloud/latest/docs/resources/datacenter — IaC datacenter resource definition and supported arguments

## Grounding rule

Official IONOS documentation describes DCD capabilities and resource behavior. It does not prove the user's current topology, resource layout, NIC assignments, firewall rules, or live configuration state. Prefer a DCD export or user-provided sanitized IaC for current-state claims. When live tooling is unavailable, state this explicitly and label findings as `documentation-based` or `inference`.
