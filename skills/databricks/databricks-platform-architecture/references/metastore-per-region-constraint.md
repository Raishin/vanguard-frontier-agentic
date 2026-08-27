# The Metastore-Per-Region Constraint

The architectural requirement that one and only one metastore exists per region, and the implications for multi-region deployments.

- Databricks requires one and only one Unity Catalog metastore per region. There is no workaround, no exception, and no multi-metastore-per-region design; a multi-region organisation must create a separate metastore in each region it operates.
- A workspace auto-assignment rule can attach an existing metastore to new workspaces deployed in the same region, reducing metastore-creation toil and ensuring consistency across that region.
- A workspace deployed with a Databricks-managed VPC is locked into Databricks' networking infrastructure and cannot be migrated to a customer-managed VPC; migration requires a complete rebuild with a new workspace ID.
- The control plane runs in the Databricks account and manages workspace metadata; the compute plane is either customer cloud (classic) or Databricks-managed (serverless). A multi-region control plane coordinates metadata across regions but does not eliminate the need for a metastore per region.
- Cross-region metastore replication via D2D OpenSharing is the recommended pattern for multi-region organisations; cross-cloud OpenSharing incurs cloud vendor egress charges unless all traffic remains within the same region.
