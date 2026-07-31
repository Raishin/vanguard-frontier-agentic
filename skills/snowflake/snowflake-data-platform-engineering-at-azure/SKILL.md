---
name: snowflake-data-platform-engineering-at-azure
description: Design and review Snowflake data platform engineering on Azure, covering warehouse sizing and cost governance, Azure Private Link requirements, storage integration with ADLS Gen2 and Azure Blob, Snowpipe automation, object tagging, dynamic data masking, row access policies, and ACCESS_HISTORY lineage for GDPR and CCPA compliance.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: data
---

# Snowflake Data Platform Engineering at Azure

## Purpose

Act as the Snowflake data platform engineer who treats every ungoverned warehouse, over-permissive storage integration, unmasked sensitive column, and missing access lineage record as a future compliance incident until proven otherwise.

## When to use

Use this skill for:

- Virtual warehouse sizing, auto-suspend/resume configuration, and cost governance
- Azure Private Link setup (Business Critical edition requirement) and USE_PRIVATELINK_ENDPOINT
- Storage integration with Azure Blob / ADLS Gen2, service principal trust, and RBAC assignments
- External stage creation with azure:// blob endpoint, Snowpipe automation with blob + queue private endpoints
- Object tagging design (inheritance, 50-tag-per-object limit), dynamic data masking (Enterprise), and tag-based masking policies
- Row access policies (Enterprise; IS_ROLE_IN_SESSION()), ACCESS_HISTORY for read/write lineage and GDPR/CCPA audit

## Lean operating rules

- Prefer official Snowflake documentation through the user's configured documentation MCP for Snowflake service behavior. Use per-skill facts and sampled live evidence in `references/official-sources.md`; when the user has configured read-only Snowflake access, use exposed read-only tools for current-state evidence instead of guessing.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge oversized warehouses, missing auto-suspend, public storage endpoints, ungoverned masking, missing row filters, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.
- Static review only — never execute SQL against a live Snowflake account. Production warehouse, storage integration, masking policy, or row access policy changes are live-guard gated; escalate.

## Snowflake data platform key facts

- **Virtual warehouses**: right-size to workload; enable AUTO_SUSPEND (60–300 s) and AUTO_RESUME; use separate warehouses per workload tier to isolate cost and contention.
- **Azure Private Link**: requires Business Critical edition (or higher). Set USE_PRIVATELINK_ENDPOINT = TRUE on storage integrations and external stages to route traffic over the private endpoint. Snowpipe needs both blob storage and queue private endpoints.
- **Storage integration**: CREATE STORAGE INTEGRATION TYPE=EXTERNAL_STAGE STORAGE_PROVIDER=AZURE creates a Snowflake-managed Azure service principal. After creation, retrieve AZURE_CONSENT_URL and AZURE_MULTI_TENANT_APP_NAME from DESCRIBE INTEGRATION. STORAGE_ALLOWED_LOCATIONS restricts allowed containers. One Entra ID tenant per integration. Grant Storage Blob Data Reader for load-only; Contributor is required for unload (write) scenarios.
- **External stage URL**: always use the azure:// blob endpoint (azure://<account>.blob.core.windows.net/<container>/<path>) even for ADLS Gen2 accounts. Snowflake resolves Gen2 via the blob endpoint.
- **Snowpipe on Azure**: automation requires an Azure Event Grid subscription on the storage account sending blob-created events to a storage queue; reference the queue in NOTIFICATION INTEGRATION TYPE=QUEUE NOTIFICATION_PROVIDER=AZURE_STORAGE_QUEUE.
- **Object tagging**: tags are schema-level objects; assign to tables, views, columns, warehouses, and other objects. Maximum 50 tags per object. Tag values are inherited by descendant objects in the hierarchy unless overridden. Requires APPLY TAG privilege on the tag object.
- **Dynamic data masking**: Enterprise edition required. Masking policy is a schema-level object; attach to a column with ALTER TABLE … ALTER COLUMN SET MASKING POLICY. Use APPLY privilege to assign policies. Tag-based masking allows automatic policy assignment via tag values.
- **Row access policies**: Enterprise edition required. Policy body evaluates as policy owner's role (not querying role). Use IS_ROLE_IN_SESSION() to allow access based on the active role at query time. Attach with ALTER TABLE … ADD ROW ACCESS POLICY. One policy per table/view.
- **ACCESS_HISTORY**: Enterprise edition required. Query SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY for per-statement read and write lineage. Includes POLICIES_REFERENCED column showing which masking/row-access policies were evaluated. 365-day retention. Supports GDPR/CCPA data-access audit evidence.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, implementation guidance, or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before warehouse resizing, storage integration changes, masking policy deployment, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding Snowflake service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
