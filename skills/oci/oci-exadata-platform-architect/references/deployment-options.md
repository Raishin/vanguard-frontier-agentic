# Exadata Deployment Options

Use this reference when choosing an Exadata Database Service deployment pattern.

## Deployment families

| Family | Where it runs | Good fit | Watch-outs |
|---|---|---|---|
| Exadata Database Service on Dedicated Infrastructure | OCI region on Oracle-managed dedicated Exadata infrastructure | Highest Oracle database performance in OCI, RAC, consolidation, predictable database platform | Capacity planning, VM cluster design, network design, patching windows, backup/DR, licensing |
| Exadata Database Service on Exascale Infrastructure | OCI and selected multicloud offers where available | Elastic Exadata-style consumption and simplified capacity model | Feature availability and operational semantics vary; verify current service docs and limits |
| Exadata Cloud@Customer | Customer data center with Oracle-managed Exadata cloud infrastructure | Data residency, low latency to on-premises apps, cloud subscription model | Site readiness, physical capacity lead time, OCI control-plane connectivity, support demarcation |
| Oracle Database@Azure | Oracle Exadata Database Service and Autonomous database services integrated with Azure regions where available | Azure app proximity and Azure operating model | Azure region availability, VNet/DNS/private connectivity, billing/support split, IaC path |
| Oracle Database@Google Cloud | Exadata Database Service integrated with Google Cloud regions and Google Cloud interfaces where available | Google Cloud app proximity, Google Cloud networking/security consumption path | Exact offer matters: Dedicated, Exascale, Base Database, or Autonomous; verify Google/Oracle API responsibilities |
| Oracle Database@AWS | Oracle database services integrated with AWS where available | AWS app proximity and AWS operating model | Region/offer maturity, VPC/DNS/private connectivity, capacity, support, and Terraform/provider coverage |

## Non-negotiable design checks

- VM cluster sizing: OCPU/ECPU equivalent, memory, local storage, DATA/RECO split, DB node count, and spare capacity.
- Database topology: CDB/PDB layout, RAC services, Data Guard, backup destination, TDE/key ownership.
- Network: client subnet, backup subnet, NSGs/security lists, SCAN/listener ports, DNS, routing, and cross-cloud latency.
- Operations: maintenance windows, patch sequencing, node lifecycle, IORM, monitoring, incident owner, and support route.
- Migration: source platform, endian/version compatibility, downtime budget, backup/restore, Data Pump, GoldenGate, ZDM, or app-level migration path.
