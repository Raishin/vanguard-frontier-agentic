---
name: alibaba-live-cost-budget-action-guard
description: Gate live financial authority actions — budget threshold changes, Savings Plan purchases, and Reserved Instance commitments. These are committed spend or can trigger immediate service suspension.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: finops
---

# Alibaba Cloud Live Cost Budget Action Guard

## Purpose

Act as the guarded live Alibaba Cloud operator for alibaba-live-cost-budget-action-guard work. Gate every budget threshold change, Savings Plan purchase, and Reserved Instance commitment with a full financial-impact assessment and explicit financial-authority approval. Treat Savings Plan and RI purchases as the highest-risk financial actions — they are non-refundable committed spend contracts.

## When to Use

Use this skill when:

- A Cost Manager budget alert threshold needs to be increased or decreased
- A Savings Plan purchase (compute-based or ECS-instance) is being requested
- A Reserved Instance commitment (zone or regional, 1-year or 3-year) is being submitted
- An operator needs to audit current budgets, alert thresholds, and active commitments
- A budget anomaly or unexpected billing spike needs investigation and remediation

## When NOT to Use

Do not use this skill when:

- The task is a read-only cost report review with no billing mutation intent
- The task involves resource rightsizing or scheduling that does not touch billing account settings
- The task is unrelated to Alibaba Cloud billing, Savings Plans, or Reserved Instances

## Key Risk Facts

- **Savings Plans** are non-refundable committed spend contracts (hourly commitment × term). A compute-based Savings Plan requires an hourly CNY/USD commitment for 1 or 3 years — there is no cancellation.
- **Reserved Instances** lock capacity for 1 or 3 years. Partial/all-upfront payment options exist. RI purchases cannot be refunded.
- **Budget threshold reduction below current spend** triggers immediate service suspension in Alibaba Cloud BSS. Services are suspended within minutes of the billing threshold being breached. Restoration requires manual re-enabling of suspended services and can cause data loss in stateful workloads.
- **All three actions** require the 6-step live-guard gate: (1) identity confirm, (2) current inventory capture, (3) blast radius assessment, (4) financial authority confirm, (5) explicit written approval, (6) post-action verification.

## Pre-Flight Checklist

Before executing any billing mutation, verify all of the following:

1. **Account identity confirmed** — confirm the BSS account ID, billing account name, and the operator's RAM identity.
2. **Current budget inventory captured** — list all active budgets, current thresholds, alert notification targets, and actual spend vs. threshold ratio before any change.
3. **Commitment inventory captured** — list all active Savings Plans (commitment amount, remaining term) and Reserved Instances (instance type, scope, expiry) before any new purchase.
4. **Utilization verified** — for Savings Plans and RIs: confirm current utilization rate of existing commitments before purchasing additional. Purchasing new commitments when existing ones are underutilized compounds waste.
5. **Blast radius assessed** — for budget threshold reduction: calculate the gap between the proposed threshold and current spend. Confirm the new threshold is above the minimum operational spend. Document which services are at risk of suspension.
6. **Financial authority confirmed** — the approver must have explicit financial authority (BSS account administrator, VP Engineering, CFO delegate, or equivalent). Document their identity and written approval.

## Required Confirmation

The operator must explicitly state all of the following before any mutation is executed:

- "I confirm the BSS account is `<ACCOUNT_ID>` and the operator identity is `<RAM_IDENTITY>`."
- "I have reviewed the current budget and commitment inventory and the proposed change is `<SPECIFIC_CHANGE>`."
- "I have verified the current utilization of existing commitments is `<RATE>%`."
- "I have assessed the blast radius: `<DESCRIPTION>`."
- "I have financial authority for this change (or I confirm that `<APPROVER_NAME>` has provided written approval)."
- "I approve this billing/commitment action."
- For Savings Plan purchase: "I understand this is a `<1/3>`-year non-refundable commitment of approximately `<AMOUNT>` per hour and I approve the purchase."
- For RI purchase: "I understand this is a `<1/3>`-year non-refundable commitment for `<INSTANCE_TYPE>` in `<REGION/ZONE>` and I approve the purchase."

## Execution Steps

1. Capture pre-change billing state: budget list, Savings Plan inventory, RI inventory.
2. Confirm active operator has BSS administrator permissions.
3. Present the planned action, financial impact estimate, and current inventory to the operator for explicit approval.
4. Execute the mutation via the Alibaba Cloud BSS Console or API.
5. Confirm the action is reflected in BSS and document the confirmation ID.

## Rollback Procedure

- **Budget threshold change** (reversible): Update the budget back to the previous threshold immediately via BSS Console or API. Re-enable any suspended services.
- **Savings Plan purchase** (NOT reversible): Savings Plans cannot be cancelled or refunded. If purchased in error, open an Alibaba Cloud Support P1 ticket — refund is not guaranteed and subject to policy.
- **Reserved Instance purchase** (NOT reversible): RI purchases are non-refundable. Contact Alibaba Cloud Support immediately if purchased in error.
- For service suspension from budget threshold breach: restore threshold above current spend immediately; manually re-enable suspended services; assess for data loss in stateful workloads.

## Post-Change Verification

1. Confirm budget changes are reflected in BSS Console.
2. Confirm Savings Plan or RI purchases appear in the commitment inventory.
3. Verify no services were inadvertently suspended.
4. Check billing alerts are active and notification targets are correct.

## Response Shape

1. BSS account and operator identity confirmed
2. Current budget inventory and spend vs. threshold ratio
3. Commitment inventory (Savings Plans, RIs) and utilization rates
4. Blast radius assessment for budget threshold changes
5. Financial authority approval status
6. Execution confirmation
7. Post-action verification results
