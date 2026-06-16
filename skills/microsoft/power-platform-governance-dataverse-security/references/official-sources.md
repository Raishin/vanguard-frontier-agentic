# Official sources

Use this reference only when grounding Power Platform or Dataverse service behavior. These are starting points, not proof of the user's live tenant state.

## Microsoft Learn documentation

- https://learn.microsoft.com/power-platform/guidance/adoption/environment-strategy — Tenant environment strategy, environment groups, ALM topology, default environment hygiene
- https://learn.microsoft.com/power-platform/guidance/adoption/dlp-strategy — DLP policy strategy, tiered data policies (Productivity / Power User / Pro Dev), connector classification
- https://learn.microsoft.com/power-platform/admin/wp-security-cds — Security concepts in Dataverse: role-based security, business units, sharing, record-level and column-level security
- https://learn.microsoft.com/power-platform/admin/wp-data-loss-prevention — Data loss prevention policies: connector classification (Business / Non-Business / Blocked), tenant and environment scope
- https://learn.microsoft.com/power-platform/admin/database-security — Configuring user security in a Power Platform environment: security roles, teams, business units, user assignment
- https://learn.microsoft.com/power-platform/admin/security-roles-privileges — Security roles and privileges reference
- https://learn.microsoft.com/power-platform/admin/manage-teams — Dataverse teams: owner teams, access teams, Microsoft Entra group teams
- https://learn.microsoft.com/power-platform/admin/field-level-security — Column (field) level security profiles
- https://learn.microsoft.com/power-platform/guidance/coe/starter-kit — Center of Excellence (CoE) Starter Kit overview and governance tooling
- https://learn.microsoft.com/power-platform/guidance/adoption/secure-default-environment — Securing the default environment
- https://learn.microsoft.com/power-platform/admin/governance-considerations — Power Platform admin governance considerations

## Grounding rule

Official documentation explains Power Platform and Dataverse service behavior. It does not prove the user's current tenant, environment, DLP policy, or Dataverse role configuration. Prefer read-only Power Platform admin center evidence, repository evidence (exported policies/roles), or sanitized user-provided evidence for current-state claims. Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

## Key service facts (grounded 2026-06-16)

- DLP policies classify connectors into Business, Non-Business, and Blocked groups. Connectors across Business and Non-Business groups cannot be combined in the same app or flow — this is enforced at runtime.
- Tenant-level DLP policies apply to all environments or all-except-selected; environment-level policies stack on top. Multiple policies on the same environment fragment the connector space and should be minimized.
- The default environment cannot be deleted; securing it (blocking all blockable connectors, routing new makers via environment routing) is a Day 1 governance priority.
- Dataverse security is accumulative: a user's effective access is the union of all their directly assigned security roles plus all roles inherited from teams they belong to. The most permissive access level prevails — you cannot restrict a record for a user who already has Organization-scope read via another role.
- Column-level security (field-level security) applies only to users who already have record-level access. It adds overhead and should not be used excessively.
- Ad-hoc row sharing is harder to audit than role-based access and should be an exception, not the default pattern.
- Microsoft Entra group-backed owner teams are the recommended pattern for scalable, auditable security role assignment.
