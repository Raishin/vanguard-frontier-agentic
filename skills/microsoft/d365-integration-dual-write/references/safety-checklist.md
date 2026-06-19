# Safety checklist

Use this reference before any recommendation involving enabling or disabling production dual-write table maps, initial sync runs, master-data ownership changes, or rollback execution in Dynamics 365 dual-write integrations.

## Non-negotiables

- Never ask users to paste credentials, connection strings, environment URLs, tenant IDs, LCS project IDs, Dataverse connection strings, or integration key values into chat.
- Use documented artifacts or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent table map health states, error counts, sync completion percentages, master-data ownership decisions, or integration lead sign-off records.
- Require explicit human approval before recommending enabling or disabling production dual-write table maps, running initial sync, or executing rollback.
- Use current official Microsoft Learn documentation for Dynamics 365 dual-write behavior, table map operations, and error handling.
- Keep recommendations least-change, reversible, and scoped to the integration scenario in question.
- Enabling/disabling production dual-write maps and initial sync runs are live-guard gated. Always escalate to the integration lead and named data governance owner before execution.

## Stress checks

- Have all dependent table maps been identified and enabled in the correct dependency order before the primary map is enabled?
- Has a master-data owner (Finance & Operations or Dataverse) been declared per entity map before initial sync begins?
- Have all integration key fields been mapped, including lookup field expansion for bidirectional field maps?
- Has the dual-write health check been run and passed in the target environment before production map operations?
- Are error alert settings configured with appropriate thresholds, notification recipients, and auto-pause or auto-stop rules?
- Is there a documented plan for the 24-hour queue compliance window for any maps that will be paused during operations?
- Is there a rollback plan with a named owner, rollback trigger criteria, rollback execution steps (including connection reset if needed), and rollback validation?
- Has the integration lead and data governance owner signed off on master-data ownership decisions and the initial sync plan?
- Have post-initial-sync validation steps been defined to confirm record counts and key field alignment on both sides?

## Evidence labels

Use `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual table map health, error queue state, master-data ownership decisions, or rollback readiness.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Enabling a dual-write table map in the production environment
- Disabling or stopping a dual-write table map in the production environment
- Running initial sync for any table map in the production environment
- Changing master-data ownership declarations for a table map in production
- Resetting the dual-write connection between Finance & Operations and Dataverse in production
- Authorizing bulk retry or dismiss operations on the production sync error dashboard
- Signing off on integration health or master-data ownership on behalf of the data governance owner
