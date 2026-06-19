# Safety checklist

Use this reference before any recommendation involving production forecast configuration changes, bulk opportunity updates, sales-process modifications, or CRM data operations in Dynamics 365 Sales.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, client secrets, certificates, or customer personally identifiable information into chat.
- Use exported pipeline reports, forecast snapshots, or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent opportunity counts, forecast numbers, quota values, conversion rates, or live environment state.
- Require explicit human approval before recommending any production forecast configuration change, bulk opportunity update, sales-process modification, or assignment rule change.
- Use current official Microsoft Learn documentation for Dynamics 365 Sales service behavior.
- Keep remediation scoped, reversible, and explicit about rollback paths.
- Production forecast configuration and sales-process changes are live-guard gated. Always escalate to a qualified Dynamics 365 Sales administrator with environment access before execution.

## Stress checks

- What opportunity data changes could corrupt an in-progress forecast period's committed numbers?
- What bulk pipeline updates could permanently delete activity history or stage history?
- What sequence changes could interrupt active seller engagements mid-sequence?
- What forecast column or category reconfigurations would invalidate historical trend comparisons?
- What assignment rule changes could orphan leads or opportunities with no assigned seller?
- What quota changes during an active forecast period would invalidate attainment tracking?

## Evidence labels

Use `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live pipeline state, forecast configuration, or CRM data quality.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Modifying forecast configuration (columns, hierarchy, date ranges) in a production environment
- Bulk-updating opportunity stage, close date, probability, or forecast category in production
- Activating, deactivating, or modifying sequences connected to live seller work lists
- Changing assignment rules that route production leads and opportunities
- Deleting or merging records (opportunities, leads, accounts, contacts) in production
- Modifying sales business process flows that govern stage progression in production
