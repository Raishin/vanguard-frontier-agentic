# Official sources

Use this reference only when you need source grounding for Microsoft Fabric / Power BI security, governance, endorsement, or RLS behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live workspace, model, or security configuration:

- https://learn.microsoft.com/fabric/governance/governance-compliance-overview — Fabric governance: DLP detection/audit logs, securing items in a workspace (workspace roles), data-level controls (table/row/column), Purview Audit, compliance certifications. Supports workspace governance and information-protection steps.
- https://learn.microsoft.com/fabric/security/service-admin-row-level-security — Row-level security (RLS): define roles/rules in Power BI Desktop, publish, add members, "Test as role"; RLS only restricts Viewer-role users; Direct Lake RLS support and DirectQuery fallback notes. Supports the security review step and the "RLS only restricts Viewers" caution.
- https://learn.microsoft.com/power-bi/guidance/powerbi-implementation-planning-usage-scenario-managed-self-service-bi — Managed self-service BI: shared semantic model in a dedicated workspace, endorsement (certified/promoted), discoverability, OneLake catalog, Build permission workflow, publish reports to separate workspaces so RLS applies to Viewers. Supports the semantic-model trust and reuse steps.
- https://learn.microsoft.com/power-bi/guidance/fabric-adoption-roadmap-system-oversight — System oversight: Purview DLP for Power BI, Defender for Cloud Apps, data residency, encryption keys (BYOK). Supports the information-protection and oversight steps.
- https://learn.microsoft.com/fabric/security/security-overview — Fabric security: access via apps without Viewer SQL access, sensitivity labels from Purview Information Protection following data across items and exports. Supports the protect-data step.

## Grounding rule

Official documentation explains Fabric/Power BI behavior. It does not prove the user's actual model inventory, endorsement status, RLS configuration, workspace role assignments, or capacity. Prefer admin portal exports, lineage view, and sanitized user-provided evidence for current-state claims. Remember RLS does not restrict Admin/Member/Contributor roles — never present it as protection for those roles.
