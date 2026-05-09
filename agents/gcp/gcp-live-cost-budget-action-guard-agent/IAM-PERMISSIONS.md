# IAM Permissions — GCP Live Cost Budget Action Guard

## Minimum Read Roles (Discovery and Audit)

| Role | Purpose |
|------|---------|
| `roles/billing.viewer` | View billing account details, budget configurations, and cost data without mutation rights |
| `roles/billing.costsManager` | View and export cost data and reports; cannot modify budgets or purchase commitments |

## Required Mutation Roles (Live Operations)

| Role | Purpose | Scope Guidance |
|------|---------|---------------|
| `roles/billing.admin` | Create and modify budgets, purchase CUD commitments, and manage billing account configurations | Bind only to a dedicated financial-authority service account; require billing account owner confirmation before use |

## Narrowing Guidance

- Bind `roles/billing.admin` only to identities that have explicit financial authority — VP Engineering, CFO-delegate, or equivalent.
- For budget-only changes (no CUD purchases), `roles/billing.budgetsEditor` is a narrower alternative to `roles/billing.admin`.
- Separate read access (`roles/billing.viewer`) from mutation access (`roles/billing.admin`) — read access for engineers, mutation access for financial owners only.
- Use billing IAM conditions to restrict `billing.admin` to specific billing accounts rather than granting it across all billing accounts.
- Quota increase requests via GCP Support require `roles/serviceusage.serviceUsageAdmin` at the project level — this is narrower than billing admin.

## Anti-Patterns — Never Grant

- `roles/owner` — grants full resource and billing control; never appropriate for a budget guard agent.
- `roles/billing.admin` to CI/CD service accounts — automated systems should never have unilateral financial authority.
- `roles/billing.admin` at the organization level without a two-person approval requirement for CUD purchases.
- Quota increases without a spending-impact assessment — increased quotas can enable runaway automated spend.
- Budget threshold reductions to zero or below operational minimums without service-impact assessment.

## Audit Trail

All billing mutations (budget changes, CUD purchases) are logged in **Cloud Audit Logs** under `cloudbilling.googleapis.com`. Ensure `ADMIN_WRITE` audit log types are enabled for the billing service. Export billing audit logs to BigQuery for long-term retention and financial compliance reporting.
