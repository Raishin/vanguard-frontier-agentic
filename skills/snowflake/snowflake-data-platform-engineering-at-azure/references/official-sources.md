# Official sources

Use this reference only when you need source grounding for Snowflake service behavior or the detailed source list.

## Snowflake documentation

Use these as starting points, not as proof of the user's live Snowflake account state:
- https://docs.snowflake.com/en/user-guide/privatelink-azure
- https://docs.snowflake.com/en/sql-reference/sql/create-storage-integration
- https://docs.snowflake.com/en/user-guide/object-tagging/introduction
- https://docs.snowflake.com/en/user-guide/security-row-intro
- https://docs.snowflake.com/en/user-guide/access-history

## Grounding rule

Official documentation explains Snowflake service behavior. It does not prove the user's current account, edition, warehouse configuration, storage integration state, policy assignments, or operational state. Prefer read-only Snowflake MCP or SQL evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Service facts from official docs:
- Azure Private Link requires Business Critical edition or higher. Setting USE_PRIVATELINK_ENDPOINT=TRUE on storage integrations and external stages routes all traffic over the private endpoint. Snowpipe requires both blob storage and queue private endpoints.
- Storage integration (TYPE=EXTERNAL_STAGE STORAGE_PROVIDER=AZURE) creates a Snowflake-managed Azure service principal. After creation, DESCRIBE INTEGRATION returns AZURE_CONSENT_URL and AZURE_MULTI_TENANT_APP_NAME. STORAGE_ALLOWED_LOCATIONS limits accessible containers. One Entra ID tenant per integration. Grant Storage Blob Data Reader for load; Contributor required for unload/write.
- External stage URL always uses azure://<account>.blob.core.windows.net/<container>/<path> even for ADLS Gen2 — Snowflake resolves Gen2 via the blob endpoint.
- Object tagging: maximum 50 tags per object; tags inherit through the securable-object hierarchy unless overridden. Assigning tags requires APPLY TAG privilege.
- Dynamic data masking requires Enterprise edition. Attach masking policy to a column; use APPLY privilege for assignment. Tag-based masking auto-assigns policies via tag values.
- Row access policies require Enterprise edition. Policy body runs as policy owner, not querying role. IS_ROLE_IN_SESSION() is used for role-based row filtering. One policy per table or view.
- ACCESS_HISTORY (SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY) requires Enterprise edition. Records per-statement read and write lineage including POLICIES_REFERENCED. 365-day retention. Supports GDPR/CCPA evidence.

Review implications:
- Do not approve public storage endpoints or missing USE_PRIVATELINK_ENDPOINT without explicit Business Critical edition confirmation and private-link activation evidence.
- Do not approve masking or row-access policy deployments without APPLY privilege review and rollback plan.
- Documentation cannot prove the user's actual warehouse configuration, storage integration state, or policy assignments.
