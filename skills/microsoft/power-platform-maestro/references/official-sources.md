# Official sources

Use this reference only when you need source grounding for Power Platform service behavior or the detailed source list.

## Microsoft Power Platform documentation

Use these as starting points, not as proof of the user's live Power Platform tenant state:
- https://learn.microsoft.com/power-platform/admin/admin-documentation
- https://learn.microsoft.com/power-platform/guidance/adoption/environment-strategy
- https://learn.microsoft.com/power-platform/guidance/adoption/dlp-strategy
- https://learn.microsoft.com/power-platform/admin/wp-security-cds
- https://learn.microsoft.com/power-platform/admin/database-security
- https://learn.microsoft.com/power-platform/guidance/adoption/govern-at-scale
- https://learn.microsoft.com/power-platform/admin/security-roles-privileges
- https://learn.microsoft.com/power-platform/alm/pipelines

## Grounding rule

Official documentation explains Power Platform service behavior. It does not prove the user's current tenant, environment, Dataverse schema, connector configuration, DLP policy state, or operational posture. Prefer read-only Power Platform admin center evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Service facts from official docs:
- Power Platform environment strategy requires separate dev, test, and production environments to support ALM principles and prevent maker activity from disrupting production workloads.
- DLP policies classify connectors as business data only, no business data allowed, or blocked; tenant-level policies apply across all environments unless scoped.
- Dataverse uses role-based security with business units, security roles, and field-level security; privilege grants are accumulative and cannot be selectively hidden at record level.
- Pipelines in Power Platform provide CI/CD for low-code assets, automating promotion from development through test to production with optional approval gates.
- Managed Environments provide premium governance capabilities including solution checker enforcement, sharing limits, and usage insights at scale.

Review implications:
- Maestro routing should choose the narrowest Power Platform specialist based on domain evidence: environment strategy, DLP policy, Dataverse security, ALM/pipelines, connector risk, or citizen-dev guardrails.
- Do not centralize decisions without citing the evidence source and routing rationale.
