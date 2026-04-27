# Autonomous Database Deployment Options

Use this reference when choosing an Autonomous Database or Autonomous AI Database deployment pattern.

## Deployment families

| Family | Where it runs | Good fit | Watch-outs |
|---|---|---|---|
| Autonomous Database Serverless | OCI-managed shared Exadata fleet in OCI regions, and selected multicloud database-at-provider offers where available | Fast provisioning, elastic scaling, low operational overhead | Validate private endpoint, regional availability, feature parity, RTO/RPO, wallet/connectivity, and provider-specific control plane |
| Autonomous Database on Dedicated Exadata Infrastructure | Dedicated Exadata capacity with autonomous container databases and Autonomous VM Cluster concepts | Isolation, predictable fleet governance, regulated workloads, custom patch control boundaries | More capacity planning, lifecycle ownership, ACD/AVMC dependency, maintenance planning |
| Autonomous Database on Exadata Cloud@Customer | Cloud-managed Exadata in the customer's data center where available | Data residency, low-latency on-premises integration, cloud operating model | Hardware/site readiness, connectivity to OCI control plane, support demarcation, capacity lead time |
| Oracle Database@Azure | Oracle database services reachable from Azure regions through Oracle-managed interconnect/control-plane integration | Azure application proximity, Azure procurement/ops model, low-latency app-to-database path | Region availability, Azure networking/DNS/IAM integration, support split, feature availability by offer |
| Oracle Database@Google Cloud | Oracle Exadata/Autonomous database services integrated with Google Cloud regions and interfaces where available | Google Cloud application proximity, native Google Cloud operations paths, Exadata-backed Oracle databases | Validate product generation: Exadata, Exascale, Base Database, or Autonomous; verify CLI/API ownership and networking |
| Oracle Database@AWS | Oracle database services integrated with AWS where available | AWS application proximity and AWS operating model with Oracle database platform | Validate exact AWS region/offer, autonomous dedicated dependencies, network path, billing/support boundaries |

## Ruthless compatibility questions

- Is the destination actually available in the target provider region today?
- Is the chosen flavor serverless, dedicated, Exadata, Exascale, Base Database, or Cloud@Customer?
- Which control plane creates the resource, and which API/MCP can prove current state?
- What is the private connectivity path, DNS model, egress path, and client wallet lifecycle?
- What are the backup, clone, restore, Data Guard, cross-region, and cross-provider limits?
- Who owns patch windows, support tickets, capacity increases, and incident response?
- Does the app require features not available in the selected autonomous flavor?
