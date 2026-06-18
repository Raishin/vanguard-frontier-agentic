# Official sources

Use this reference only when you need source grounding for Databricks Lakehouse engineering or Azure service behavior, or the detailed source list.

## Databricks and Azure documentation

Use these as starting points, not as proof of the user's live workspace state:
- https://docs.databricks.com/en/lakehouse/index.html
- https://docs.databricks.com/en/connect/storage/azure-storage.html
- https://learn.microsoft.com/en-us/azure/databricks/connect/storage/tutorial-azure-storage
- https://docs.databricks.com/en/clusters/cluster-config-best-practices.html

## Grounding rule

Official documentation explains Databricks and Azure service behavior. It does not prove the user's current workspace, cluster configuration, storage credential state, or operational environment. Prefer read-only workspace MCP or CLI evidence, repository evidence (Terraform/IaC), or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Service facts from official docs:

**Medallion architecture:** Bronze (raw ingestion), Silver (cleaned/conformed), Gold (business-level aggregations). Delta Lake underpins all layers with ACID transactions, schema enforcement, and time travel.

**ADLS Gen2 access:** Unity Catalog manages storage access via external locations and storage credentials. Storage credentials reference an Access Connector (Microsoft.Databricks/accessConnectors) with a system-assigned or user-assigned managed identity. ADLS Gen2 accounts must have hierarchical namespace (HNS) enabled; flat namespace accounts are not supported for Unity Catalog external locations.

**Access Connector:** The preferred managed identity model. The Access Connector's managed identity is granted Storage Blob Data Contributor (or Reader) on the ADLS Gen2 container. Direct service principal credential injection into clusters is discouraged.

**Credential passthrough DEPRECATED:** Azure Active Directory (now Microsoft Entra ID) credential passthrough is deprecated as of Databricks Runtime 15.0 and will be removed. Migrate to Unity Catalog storage credentials and external locations.

**Cluster access modes:** Dedicated (formerly Single User) clusters are Unity Catalog-compatible. Standard (formerly Shared) clusters support Unity Catalog in DBR 13.3 LTS+. No Isolation Shared clusters do not support Unity Catalog. Cluster policies (Premium plan) enforce access mode compliance.

**AKV-backed secret scopes:** Secret values stored in Azure Key Vault are read-only from Databricks; Databricks cannot write back to AKV. Use Vault access policy model (not RBAC model) when the workspace requires AKV-backed scopes.

**Network isolation:** VNet injection places the Databricks control plane and data plane in the customer VNet. Private Link further isolates the workspace front-end and back-end. Both are required for high-compliance environments.

**Certification reference:** DP-750 (Azure Databricks Data Engineer Associate) covers Lakehouse architecture and Unity Catalog integration fundamentals.

Review implications:
- Do not approve credential passthrough patterns; require migration timeline and Unity Catalog external location design.
- Require cluster access mode evidence and policy enforcement before approving production cluster configuration.
- Documentation cannot prove the user's actual cluster state, storage credentials, or VNet configuration.
