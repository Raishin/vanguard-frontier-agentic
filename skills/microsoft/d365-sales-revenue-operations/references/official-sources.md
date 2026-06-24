# Official sources

Use this reference only when you need source grounding for Dynamics 365 Sales revenue operations, pipeline management, forecasting, or sales accelerator behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live environment state:

- https://learn.microsoft.com/dynamics365/sales/opportunity-management-overview
- https://learn.microsoft.com/dynamics365/sales/project-accurate-revenue-sales-forecasting
- https://learn.microsoft.com/dynamics365/sales/configure-forecast
- https://learn.microsoft.com/dynamics365/sales/enable-configure-sales-accelerator
- https://learn.microsoft.com/dynamics365/sales/configure-predictive-opportunity-scoring
- https://learn.microsoft.com/dynamics365/sales/use-opportunity-pipeline-view
- https://learn.microsoft.com/dynamics365/sales/configure-predictive-lead-scoring
- https://learn.microsoft.com/dynamics365/sales/create-and-activate-a-sequence
- https://learn.microsoft.com/dynamics365/sales/overview
- https://learn.microsoft.com/dynamics365/sales/digital-selling-sales-accelerator

## Grounding rule

Official documentation explains Dynamics 365 Sales service behavior. It does not prove the user's current pipeline state, forecast configuration, sales process design, or CRM data quality. Prefer exported pipeline views, forecast snapshots, and sales reports as evidence over inference.

## Service facts (verified 2026-06-16)

Pipeline and opportunity management:
- Dynamics 365 Sales uses a role-based opportunity model: leads qualify into opportunities, which progress through configurable sales stages with business process flows.
- The pipeline view provides a Kanban-style board for managing opportunities by stage, with drag-and-drop stage progression and deal metrics per stage.
- Predictive opportunity scoring uses AI to score opportunities 0–100 on conversion likelihood, based on historical patterns and CRM activity signals.
- The Sales Opportunity Agent (preview in 2026 wave 1) provides AI-generated research summaries and recommended next actions for each opportunity.

Forecasting:
- Dynamics 365 Sales forecasting aggregates opportunity revenue by forecast category (Pipeline, Best Case, Committed, Won, Lost) across a configurable hierarchy (user, territory, or custom).
- Forecast columns map to opportunity fields; adjustments by sellers and managers are tracked separately from system rollup.
- Premium forecasting adds AI-based predictive forecast columns that augment seller-submitted values with machine-learned estimates.
- Forecasts are most accurate when opportunities have current close dates, realistic probabilities, and correctly assigned forecast categories.

Sales accelerator:
- The sales accelerator provides a prioritized work list of leads and opportunities with AI-driven suggestions for the next best action.
- Sequences define ordered activity steps (email, phone call, task) that guide sellers through a repeatable sales motion for a given scenario.
- Assignment rules automatically route leads and opportunities to sellers based on configurable criteria.
- Sales Enterprise license includes 1,500 sequence-connected records per month; higher volumes require Sales Premium.

CRM data hygiene:
- Stale opportunities (no activity for 30+ days, close dates in the past, probability not updated) are the primary driver of pipeline mistrust and forecast inaccuracy.
- Duplicate detection rules in Dynamics 365 can be configured to flag duplicate leads and contacts on creation or import.
- Data enrichment integrations (LinkedIn Sales Navigator, ZoomInfo) can supplement missing contact and account data.
