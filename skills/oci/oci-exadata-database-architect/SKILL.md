---
name: oci-exadata-database-architect
description: Design, review, migrate, and operate Oracle Exadata Database Service across OCI Dedicated Infrastructure, Exascale Infrastructure, Cloud@Customer, and Oracle Database multicloud destinations including Azure, Google Cloud, and AWS, with official-doc grounding.
metadata:
  author: github: Raishin
---

# OCI Exadata Database Architect

## Purpose

Use this skill when the user asks for Exadata architecture, migration, sizing,
Terraform/IaC, compatibility, performance, HA/DR, or operational review across:

- Exadata Database Service on Dedicated Infrastructure
- Exadata Database Service on Exascale Infrastructure
- Exadata Cloud@Customer
- Oracle Database@Azure Exadata-style deployments
- Oracle Database@Google Cloud Exadata deployments
- Oracle Database@AWS Exadata deployments
- Hybrid designs where apps run in AWS/GCP/Azure and Exadata remains in OCI or
  on customer premises

This is a heavy platform skill. Exadata mistakes are expensive and hard to
undo. Be ruthless about missing capacity, licensing, network, HA, backup, and
operational assumptions.

## Source-Grounding Contract

Before giving advice that affects deployment, sizing, licensing, networking,
database version, storage, VM cluster shape, backup, Data Guard, or multicloud
placement:

1. Query official Oracle documentation, preferably through Context7 using:
   - library: `/websites/oracle_en-us_iaas_content`
   - focus: Exadata Database Service, Exadata Cloud@Customer, Exadata
     Infrastructure, Cloud VM Cluster, DB Home, CDB/PDB, backup, Data Guard,
     Oracle Database@Azure, Oracle Database@Google Cloud, Oracle Database@AWS,
     quotas, and limits.
2. If Terraform is involved, verify exact resources in the active provider:
   - OCI resources such as `oci_database_cloud_exadata_infrastructure`,
     `oci_database_cloud_vm_cluster`, `oci_database_db_home`, and
     `oci_database_database`
   - Google Cloud Oracle Database resources for Database@Google Cloud Exadata
   - Azure Oracle Database resources or AzAPI for Database@Azure
   - AWS-specific Oracle Database@AWS resources only after current docs confirm
     availability
3. Label every major claim:
   - `Verified from official docs`
   - `Verified from live environment`
   - `Inference`
   - `Unknown / needs confirmation`

## First Questions to Answer

Never start with a shape. Start with constraints.

1. **Workload profile**
   - OLTP, DSS/analytics, mixed, batch-heavy, consolidation, RAC-sensitive
   - Peak CPU, memory, IOPS, throughput, latency, redo rate, sessions
   - Current AWR/ASH evidence, not just human estimates
2. **Database estate**
   - Number of CDBs and PDBs
   - Database versions and patch levels
   - RAC, Data Guard, GoldenGate, TDE, RMAN, partitioning, in-memory,
     advanced compression, spatial, XML DB, Java-in-DB, external tables
   - Current Exadata features in use: Smart Scan, storage indexes, HCC,
     IORM, Smart Flash Cache, RoCE/RDMA assumptions
3. **Destination**
   - OCI Dedicated Infrastructure
   - OCI Exascale Infrastructure
   - Exadata Cloud@Customer
   - Oracle Database@Azure
   - Oracle Database@Google Cloud
   - Oracle Database@AWS
   - Hybrid private connectivity
4. **Availability target**
   - Single availability domain, multi-AD, cross-region standby, Cloud@Customer
     paired site
   - RTO/RPO and switchover/failover expectations
5. **Operational ownership**
   - Who patches GI, DB homes, CDBs/PDBs, client drivers, network, backups,
     Data Guard, monitoring, and security?
   - Who can open Oracle SRs and approve maintenance windows?

## Deployment Option Matrix

Use this as the starting point, then verify current docs and service limits.

| Option | Best Fit | Watchouts |
|---|---|---|
| Exadata Database Service on Dedicated Infrastructure in OCI | Large Oracle estates needing RAC, Exadata performance, and strong isolation | Capacity commitment, VM cluster design, maintenance windows, network segmentation, backup/DR complexity |
| Exadata Database Service on Exascale Infrastructure in OCI | Elastic Exadata-style consumption where available | Verify region/service support and exact operational differences from dedicated infrastructure |
| Exadata Cloud@Customer | On-prem data residency/latency with Oracle-managed Exadata cloud service | Data center readiness, networking, control-plane connectivity, hardware lifecycle, responsibility split |
| Oracle Database@Azure | Azure applications requiring Oracle DB close to Azure network, billing, governance | Verify regions, Azure RBAC, Oracle subscription link, VNet/subnet requirements, available Exadata services |
| Oracle Database@Google Cloud | Google Cloud applications requiring Oracle Exadata in Google Cloud networking/consumption model | Verify Google region, ODB network/subnet, service limits, Google provider support, OCI-backed resource mapping |
| Oracle Database@AWS | AWS applications requiring Oracle-managed Exadata close to AWS | Verify current service availability, region, backup/recovery prerequisites, and AWS/OCI role model before designing |
| Hybrid app cloud + Exadata in OCI | Safer when native Database@cloud service is unavailable or immature | Latency, egress, private connectivity, DNS, failover routing, two-cloud operations |

## Compatibility Checklist

### Database Compatibility

- Source and target Oracle Database versions and RU/RUR patch levels.
- CDB/PDB architecture and maximum PDB counts.
- RAC dependencies and services configuration.
- ASM, Data Guard, Active Data Guard, GoldenGate, RMAN, TDE wallet/keystore.
- Exadata-specific behavior:
  - Smart Scan eligibility
  - Hybrid Columnar Compression
  - Storage indexes
  - IORM/resource management
  - Flash cache assumptions
  - Direct path read/write behavior
  - Offload efficiency
- Unsupported or restricted OS-level access assumptions in managed cloud
  service.
- DB home creation, image selection, out-of-place patching, and rollback model.

### Application Compatibility

- Connection routing through SCAN/listeners/services.
- FAN/FCF, Application Continuity, Transaction Guard, and connection pool
  behavior during maintenance/failover.
- Client driver versions and TLS/cert expectations.
- Latency sensitivity if app tier is in AWS/GCP/Azure while Exadata is in OCI
  or on-premises.
- Batch window impact from changed I/O and parallelism behavior.

### Operational Compatibility

- Existing DBA runbooks versus Oracle-managed operations.
- Backup destination:
  - Object Storage
  - Autonomous Recovery Service
  - Recovery Appliance where supported
- Restore test and clone process.
- Maintenance window control and patch cadence.
- Observability:
  - OCI Monitoring
  - Database Management
  - Operations Insights
  - Enterprise Manager if retained
  - Cloud-provider-native monitoring for Database@Azure/GCP/AWS wrappers
- Audit, Data Safe, Cloud Guard, Security Zones, Vault/KMS, and logging.

### Licensing and Commercial Compatibility

- BYOL versus license-included.
- Database options actually used versus licensed.
- Multicloud billing/subscription linkage.
- Support ownership and escalation path.
- Egress, private connectivity, backup storage, standby, clone, and test
  environment cost.

## Multicloud-Specific Guardrails

### Azure Destination

For Oracle Database@Azure:

- Verify Azure resource provider, API version, and supported regions.
- Validate Azure custom roles/RBAC for DB Systems, Exadata infrastructure,
  network anchors, resource anchors, and related resources.
- Confirm OCI policies for multicloud network links, Exadata infrastructure,
  VM clusters, DB homes, databases, backups, and work requests.
- Validate VNet/subnet sizing, DNS, NSGs, route tables, private endpoints, and
  any subnet delegation requirements.
- Confirm whether management is done through Azure-native resources, OCI APIs,
  or both.

### Google Cloud Destination

For Oracle Database@Google Cloud:

- Verify project, region, enabled APIs, ODB network, ODB subnet, service
  limits, and linked OCI tenancy.
- Expect OCI OCIDs behind Google Cloud resources; design inventory and drift
  checks accordingly.
- Verify whether creating Exadata infrastructure, cloud VM cluster, DB home,
  and database requires both Google provider resources and OCI provider
  resources.
- Validate private networking, DNS, firewall rules, and client test VM/subnet.

### AWS Destination

For Oracle Database@AWS:

- Verify current Oracle docs for exact service scope, region, and resource
  model before naming resources or promising features.
- Confirm backup/recovery prerequisites, including any OCI policy for
  assigned subscriptions and Recovery Service.
- Do not extrapolate from Azure/GCP examples.
- If service scope is unclear, present two designs:
  1. Native Oracle Database@AWS if current docs prove it supports the workload.
  2. OCI Exadata with AWS private connectivity as the fallback.

## Sizing Discipline

Do not size Exadata from "number of cores" alone. Demand evidence:

- AWR reports from peak and normal periods.
- CPU by database and service.
- IOPS, MB/s, redo generation, log file sync, DB file sequential/scattered read.
- Top SQL, execution plans, parallel query, temp usage.
- Storage growth and retention.
- Backup and restore throughput.
- Consolidation interference risks.
- HA/DR overhead and standby capacity.

If evidence is missing, say the design is provisional and list the exact data
needed. Guessing Exadata size is not architecture; it is gambling with a large
invoice.

## IaC Review Rules

For Terraform or other IaC:

1. Reject committed passwords, TDE wallet passwords, SSH private keys, and API
   tokens.
2. Validate compartment/project/subscription/account mapping.
3. Validate Exadata infrastructure, VM cluster, DB home, CDB/PDB dependencies.
4. Validate backup settings, deletion behavior, and final-backup expectations.
5. Validate maintenance windows and patching options.
6. Validate license model and database edition.
7. Validate network:
   - VCN/VNet/VPC/VPC-network
   - client subnet
   - backup subnet if relevant
   - NSGs/security lists/firewalls
   - DNS and SCAN/listener resolution
8. Validate service limits before assuming capacity.
9. For multicloud, check both the wrapper cloud resource and OCI backing
   resource. One can drift from the other.

## Output Format

Use this format for serious Exadata answers:

```markdown
# Exadata Recommendation

## Bottom Line
[Direct recommendation. Include what not to do.]

## Source Status
- Official docs checked:
- Live environment checked:
- Unknowns:

## Workload Evidence
- AWR/ASH evidence:
- Capacity evidence:
- Missing evidence:

## Destination Fit
- Recommended destination:
- Rejected destinations and why:
- Multicloud implications:

## Architecture
- Exadata infrastructure:
- VM clusters:
- DB homes:
- CDB/PDB layout:
- Network:
- Backup/DR:
- Observability:
- Security:
- Cost/licensing:

## Migration Plan
1. Assessment
2. Landing zone/network
3. Provisioning
4. Migration rehearsal
5. Performance validation
6. Cutover
7. Fallback

## Validation Plan
- Pre-deploy:
- Deploy:
- Post-deploy:
- DR test:

## Ruthless Risks
- [Assumptions most likely to fail.]
```

## Red Flags

Stop and challenge the user if you see:

- No AWR/ASH evidence for sizing.
- "Lift and shift to Exadata" without RAC/service/failover review.
- No Data Guard/RPO/RTO decision.
- No restore test.
- App in one cloud, Exadata in another, and no latency/egress analysis.
- Cloud@Customer chosen without data center/network/control-plane readiness.
- Multicloud destination assumed to support the same Exadata features as OCI.
- Provider examples copied with passwords or broad IAM/RBAC roles.
- No patch/maintenance ownership model.
- No fallback plan for cutover.

