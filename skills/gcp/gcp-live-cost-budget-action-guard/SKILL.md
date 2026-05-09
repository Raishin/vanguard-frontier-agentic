---
name: gcp-live-cost-budget-action-guard
description: Gate Cloud Billing budget threshold changes, committed-use discount (CUD) purchases, and quota increase requests with explicit financial-authority approval. CUD contracts are 1-3 year financial commitments that cannot be cancelled — this guard ensures every billing action is backed by spend-impact assessment, budget inventory review, and confirmed financial authority before execution.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: finops
---

# GCP Live Cost Budget Action Guard

## Purpose

Act as the guarded live GCP operator for gcp-live-cost-budget-action-guard work. Gate every Cloud Billing budget change, CUD purchase, and quota increase with a full financial-impact assessment and explicit financial-authority approval. Treat CUD contracts as the highest-risk financial action — they are multi-year, non-cancellable commitments.

## When to Use

Use this skill when:

- A Cloud Billing budget alert threshold needs to be increased or decreased
- A committed-use discount (CUD) contract purchase is being requested
- A service quota increase is being submitted to GCP Support
- An operator needs to audit current budgets, alert thresholds, and CUD commitments
- A budget anomaly or unexpected billing spike needs investigation and remediation
- Programmatic budget actions (Cloud Functions triggered by budget alerts) need to be reviewed

## When NOT to Use

Do not use this skill when:

- The task is a read-only cost report review with no billing mutation intent
- The task involves resource rightsizing or scheduling that does not touch billing account settings
- The task is unrelated to GCP billing, quotas, or committed-use discounts
- The task involves AWS Cost Explorer, Azure Cost Management, or other non-GCP billing systems

## Pre-Flight Checklist

Before executing any billing or quota mutation, verify all of the following:

1. **Billing account identity confirmed** — run `gcloud billing accounts list` and confirm the billing account ID and display name match the intended target.
2. **Active principal confirmed** — run `gcloud auth list` and confirm the identity has `roles/billing.admin` (for mutations) or `roles/billing.viewer` (for reads).
3. **Current budget inventory captured** — list all budgets for the billing account and document current thresholds and alert rules before any change.
4. **CUD commitment inventory captured** — list all active committed-use discounts, their resource types, commitment amounts, and expiry dates.
5. **Quota usage vs. limits assessed** — for quota increase requests, document current usage, current limit, and the requested new limit; estimate the maximum spend impact at the requested limit.
6. **Financial authority confirmed** — the approver must have explicit financial authority (billing account owner, VP Engineering, CFO-delegate, or equivalent); document their identity and approval.
7. **Service suspension risk assessed** — for budget threshold reductions, confirm the new threshold is above the minimum operational spend to avoid unexpected service suspension.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the billing account is `<BILLING_ACCOUNT_ID>` and the project is `<PROJECT_ID>`."
- "I have reviewed the current budget inventory and the proposed change is `<SPECIFIC_CHANGE>`."
- "I have financial authority for this change (or I confirm that `<APPROVER_NAME>` has provided written approval)."
- "I approve this billing/quota action."
- For CUD purchases: "I understand this is a `<1/3>`-year non-cancellable commitment of approximately `<$AMOUNT>` and I approve the purchase."

## Execution Steps

1. Capture pre-change billing state: budget list, CUD inventory, quota usage.
2. Confirm active principal has `roles/billing.admin` (for billing changes) or the appropriate project-level role (for quota requests).
3. Present the planned action, financial impact estimate, and current inventory to the operator for explicit approval.
4. Execute the mutation:
   - Update budget: Use Cloud Console or Billing API (no direct gcloud budget mutation CLI; use `gcloud billing budgets update` if available or Billing API).
   - Purchase CUD: Via Cloud Console > Committed Use Discounts or via `gcloud compute commitments create`.
   - Request quota increase: Via GCP Console > IAM & Admin > Quotas, or `gcloud alpha services quota update` (where available).
5. Confirm the action is reflected in the billing account or quota system.

## Rollback Procedure

- **Budget threshold change** (reversible): Update the budget back to the previous threshold using the same method.
- **CUD commitment purchase** (NOT reversible): CUD contracts cannot be cancelled. If a CUD was purchased in error, open a GCP Support P1 case immediately — Google may be able to reverse within a very short window, but this is not guaranteed.
- **Quota increase** (reversible): Quota increases can be reversed by submitting a quota decrease request. However, any spend incurred while the quota was elevated cannot be reversed.
- For budget alert reductions that caused service suspension: restore the threshold and check for suspended services in Cloud Console.

## Post-Change Verification

1. List budgets to confirm the change is reflected: verify via Cloud Billing console or API.
2. Confirm Cloud Monitoring budget alert notifications are still configured correctly.
3. For CUD purchases: confirm the new commitment appears in the CUD inventory with the correct resource type and term.
4. For quota increases: verify the new limit is visible via `gcloud compute project-info describe` or the Quotas page in Cloud Console.
5. Check Cloud Audit Logs for the billing action: `gcloud logging read 'resource.type="billing_account"' --limit=5`.

## Response Shape

1. Billing account and project identity confirmation
2. Current budget inventory and alert thresholds
3. CUD commitment inventory and expiry
4. Quota usage vs. limits for affected services
5. Financial authority approval status
6. Proposed or executed financial action
7. Post-change alert and monitoring confirmation
