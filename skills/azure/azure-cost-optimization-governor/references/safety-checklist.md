# Safety checklist for Azure Cost Optimization Governor

## Non-negotiable gates

- Never ask for invoices, billing account identifiers, enrollment identifiers, tenant identifiers, subscription identifiers, customer names, or raw exported cost data containing sensitive business details.
- Do not recommend deleting, resizing, stopping, or changing commitments without owner, impact, rollback, and utilization evidence.
- Do not purchase or recommend purchase of reservations or savings plans until waste is removed and baseline usage is stable.
- Treat budgets as alerts/automation triggers, not guaranteed hard stops.
- Require explicit approval before budget automation, resource shutdown, rightsizing, commitment purchase, export creation, tag enforcement, or policy remediation.

## High-risk assumptions to kill

- "Advisor savings are additive." Recommendation datasets can overlap.
- "Potential savings equal realized savings." Realized savings require implementation and measurement.
- "Budgets prevent overruns." They notify and can trigger automation; spend can still accrue.
- "Commitments fix waste." Discounts reduce rates, not unused resources.
- "Tags are enough." Tags must be enforced, inherited where appropriate, and used by reporting owners.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `cost_sample`: sanitized Cost Management/Advisor/export evidence reviewed.
- `owner_confirmed`: resource or cost center owner confirmed action scope.
- `savings_validated`: before/after actuals prove realized savings.
- `mutation_ready`: approval, impact, rollback, and monitoring are documented.

## Minimum safe evidence

- Scope, billing model, management group/subscription/resource-group boundaries, and reporting owners.
- Budget thresholds, alert recipients, action groups, and response runbook.
- Cost analysis views, exports, dimensions, tags, and chargeback model.
- Advisor recommendations with overlap analysis and owner review.
- Utilization history before rightsizing or commitment decisions.
