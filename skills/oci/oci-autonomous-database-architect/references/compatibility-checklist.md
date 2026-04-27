# Autonomous Database Compatibility Checklist

## Workload compatibility

- Confirm database workload type: Transaction Processing, Data Warehouse, JSON, APEX, AI/vector/RAG, or mixed.
- Confirm database version and feature dependencies before assuming portability.
- Validate SQL plan stability, optimizer behavior, object types, scheduler jobs, database links, external tables, network ACLs, APEX/ORDS, and client driver support.
- Check if the application assumes host-level access; Autonomous Database does not provide normal host administration.

## Network and access compatibility

- Private endpoint or public endpoint decision is explicit.
- DNS resolution path is documented for OCI, Azure, AWS, Google Cloud, on-premises, and developer clients.
- Wallet, mTLS/TLS, secret storage, rotation, and client rollout are owned.
- Ingress and egress are least-privilege and auditable.

## Operations compatibility

- RTO/RPO mapped to Autonomous Data Guard, backups, clones, refreshable clones, or application-level replication.
- Restore test evidence exists; backup existence alone is not enough.
- Scaling model is validated: ECPU/OCPU, auto scaling, storage auto scaling, and license model.
- Observability covers database metrics, audit logs, Data Safe, Operations Insights, alarms, and application SLOs.

## Multicloud compatibility

- Provider destination is named exactly: Oracle Database@Azure, Oracle Database@Google Cloud, Oracle Database@AWS, OCI, or Cloud@Customer.
- The answer distinguishes Oracle-managed infrastructure from the host cloud's native database products.
- Region availability, billing path, quota/capacity, identity integration, support workflow, and IaC provider coverage are verified rather than assumed.
