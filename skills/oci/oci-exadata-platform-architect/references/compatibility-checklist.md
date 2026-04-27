# Exadata Compatibility Checklist

## Database and platform compatibility

- Confirm Oracle Database version, Grid Infrastructure version, RAC requirements, CDB/PDB layout, options/packs, and database software image strategy.
- Validate application client compatibility: connection strings, SCAN, FAN/ONS, TLS, wallets, JDBC/ODP.NET versions, and failover behavior.
- Validate storage and performance assumptions with evidence such as AWR/ASH, IOPS, throughput, latency, CPU, memory, interconnect, and cell offload indicators.

## Deployment compatibility

- Identify exact platform: OCI Dedicated Infrastructure, Exadata Cloud@Customer, Exascale, Oracle Database@Azure, Oracle Database@Google Cloud, or Oracle Database@AWS.
- Confirm region/provider availability, capacity/quota, subscription/billing, support process, and IaC/API support.
- Confirm network path between applications and database: VCN, VNet, VPC, FastConnect, VPN, interconnect, DNS, and security controls.

## Operations compatibility

- Backup destination and restore process are tested, not just configured.
- Data Guard topology, switchover/failover runbook, observer/auto-failover assumptions, and RPO/RTO are documented.
- Patching and maintenance windows are aligned across GI, DB homes, VM clusters, applications, and provider control planes.
- IORM and consolidation rules exist for noisy-neighbor control.

## Multicloud compatibility

- The database may be Oracle-managed while the application lives in AWS, Azure, Google Cloud, OCI, or on-premises. Do not blur those responsibility boundaries.
- Cross-cloud latency, egress, DNS, route propagation, identity, monitoring, and incident ownership are first-class design inputs.
