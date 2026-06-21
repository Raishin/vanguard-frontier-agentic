# Workflow and output contract — SAP Order-to-Cash Review

Use this reference for all OTC finding classification, risk level assignment, remediation path selection, and output formatting.

## OTC finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `order-management` | `incompletion-procedure-gap` | Incompletion log (order incompletion procedure) does not cover a required field (ship-to party, requested delivery date, plant, price condition) — orders with missing data can proceed to fulfillment and cause downstream exceptions |
| `order-management` | `document-type-over-permissive` | Sales order document type allows partner function combinations or organizational assignments that should be restricted for the given business process |
| `pricing` | `override-without-approval` | Manual price override on a pricing condition type can be performed by the sales order entry user without a documented approval workflow — unauthorized discounting is possible without audit trail |
| `pricing` | `zero-price-not-blocked` | Sales orders with a net value of zero (or below a minimum) can be saved and fulfilled without a system block or approval requirement — enables fraudulent or error-driven revenue leakage |
| `pricing` | `pricing-error-not-blocked` | Pricing errors (condition type missing, access sequence not finding a valid condition record) result in an error indicator on the sales order but do not block the order from proceeding to delivery and billing — orders with pricing errors can be invoiced at zero or incorrect values |
| `credit-management` | `credit-block-release-sod-gap` | The user or role that entered the sales order can also release the credit block on the same order — SoD gap in the credit risk control model |
| `credit-management` | `credit-limit-override-uncontrolled` | Credit limits can be increased or set to a high threshold without a credit management approval workflow — credit exposure is not governed |
| `credit-management` | `no-dynamic-credit-check` | Credit check is static (exposure vs. limit at order creation) rather than dynamic (updated with deliveries, billing, and open items) — credit exposure between order creation and invoice posting is not monitored |
| `billing` | `unbilled-delivery-aging` | Goods issue has been posted but the delivery has not been invoiced within the standard billing cycle window — revenue recognition is delayed and represents a revenue completeness gap |
| `billing` | `billing-plan-milestone-slippage` | Milestone billing plan dates are passing without billing being triggered — periodic or project revenue is being deferred beyond contractual and accounting policy timelines |
| `billing` | `sd-fi-account-determination-gap` | SD–FI account determination (VKOA) is missing or incorrect for a revenue or deferred revenue account type — billing documents are posting to incorrect G/L accounts |
| `order-blocks` | `aged-credit-block` | Credit-blocked orders are aging beyond the standard resolution target (typically 24–48 hours) without release or cancellation — material revenue at risk and customer experience impact |
| `order-blocks` | `block-release-without-credit-review` | Credit blocks are being released by the sales team without a documented credit management review — credit control is bypassed |
| `order-blocks` | `incompletion-block-pattern` | A pattern of incompletion blocks in a specific order type or sales organization indicates a systemic order entry quality issue, not a case-by-case data problem |
| `fulfillment-exceptions` | `atp-check-not-configured` | Availability-to-promise (ATP) check is not configured for a material type or plant that requires it — orders for unavailable stock are confirmed without a realistic ship date |
| `fulfillment-exceptions` | `uncontrolled-delivery-split` | Delivery split logic creates multiple delivery documents for a single sales order line without customer consent configuration — adds fulfillment cost and customer experience risk |
| `fulfillment-exceptions` | `gi-posting-timing-gap` | Goods issue is not posted at the actual time of physical goods leaving the warehouse — revenue recognition and inventory accuracy are misaligned |
| `dso-drivers` | `billing-cycle-timing-gap` | Billing is triggered significantly after goods issue rather than immediately — billing cycle timing is adding preventable days to DSO |
| `dso-drivers` | `dispute-volume-unresolved` | SAP Dispute Management has a high volume of aged open dispute cases — unresolved disputes are converting to deductions and DSO elevation |
| `dso-drivers` | `dunning-without-escalation` | Dunning procedure has insufficient levels or escalation amounts to prompt payment — late-paying customers receive dunning notices without meaningful consequence escalation |
| `revenue-completeness` | `unapplied-cash-aging` | Customer payments are sitting in the clearing account unapplied beyond the standard cash application window — DSO is overstated and customer account balances are inaccurate |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Sales orders processed at zero net value without block or approval (fraud enablement); credit block release SoD gap with material revenue or credit exposure; billing document posting to incorrect revenue accounts causing financial statement misstatement at a material level |
| `high` | Manual pricing override without approval workflow; credit limit override without governance; unbilled delivery aging above materiality threshold; billing plan milestone slippage creating IFRS 15 / ASC 606 risk; no dynamic credit check with material order book exposure; aged credit blocks representing significant revenue at risk |
| `medium` | Incompletion procedure gap causing fulfillment exceptions; ATP check gap for material-heavy order types; aged dispute cases creating deductions risk; billing cycle timing gap adding preventable DSO days; dunning without escalation for persistent late payers |
| `low` | Best practice deviation in document type design or partner function configuration; minor delivery split configuration refinement; cosmetic billing plan date alignment |

## Remediation path decision tree

For each finding:

1. **Is this a zero-price sales order path without a block, or a billing document posting to an incorrect revenue account at a material level?**
   - Yes → `critical`. Immediately escalate to the audit and finance team. Implement a minimum value check on the incompletion procedure or a pricing completeness check blocking zero-net-value orders. Correct the SD–FI account determination for the affected billing document types in a non-production system first.
   - No → continue.

2. **Is this a credit block SoD gap where the sales order creator can also release the credit block?**
   - Yes → `critical`. Escalate to the GRC and audit team. Redesign the credit block release authorization to require a separate credit management role distinct from the sales order entry role. Document the compensating control if immediate role redesign is not possible.
   - No → continue.

3. **Is this a pricing override without an approval workflow?**
   - Yes → `high`. Implement a pricing override approval workflow in SAP S/4HANA. Require a secondary authorization (sales manager or pricing team) for manual overrides to net price condition types. Configure audit logging for all pricing overrides above a defined threshold.
   - No → continue.

4. **Is this an unbilled delivery aging or billing plan milestone slippage at a material scale?**
   - Yes → `high`. Implement a daily unbilled delivery aging report with escalation to the billing team for items beyond the billing cycle target. Review billing plan milestone dates for alignment with contract terms. Configure the billing due list to automatically propose billing for overdue milestones.
   - No → continue.

5. **Is this a DSO elevation driven by dispute volume, dunning gaps, or cash application delays?**
   - Yes → `high`. Diagnose the dominant DSO driver (dispute volume, cash application aging, dunning escalation gap). Configure SAP Dispute Management for the primary dispute reason codes. Add dunning escalation levels for persistent late payers. Establish a cash application SLA for remittance matching within one business day of payment receipt.
   - No → classify as `medium` or `low` and provide guidance.

## Workflow

1. **Receive artifacts** — order block aging reports, billing schedule adherence reports, pricing procedure documentation, credit management configuration summaries, DSO analytics, dispute management reports, dunning procedure documentation, unbilled delivery reports, fulfillment exception reports, or written descriptions of the OTC landscape.
2. **Classify each finding** by OTC process domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical findings** immediately — zero-price sales order path, revenue account determination errors at material scale, and credit block SoD gaps must be escalated before other remediation.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical revenue integrity and SoD findings first; then high pricing override, credit limit governance, unbilled delivery, and billing plan slippage gaps; then medium incompletion, ATP, dispute, and dunning findings; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. OTC process domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected sales order type, customer account, billing document type, credit control area, or OTC configuration object (if provided), and specific control or process gap
5. Quantified revenue or DSO impact estimate where data is available (unbilled delivery value, billing plan slippage amount, DSO days attributable to identified driver, credit exposure at risk)
6. Recommended remediation action (pricing override approval workflow, credit block SoD separation, unbilled delivery aging control, billing plan monitoring, SD–FI account determination correction, incompletion procedure extension, ATP check activation, dispute management configuration, dunning escalation addition, cash application SLA, etc.)
7. OTC control posture after remediation
8. Escalation notice for any critical zero-price revenue path or material revenue account misposting — explicit statement that this requires audit and finance team review before any further OTC transactions are processed
9. Prioritized remediation sequence
10. Confirmation that no sales orders were created or modified, no order blocks were released, no billing documents were posted, no credit limits were changed, and no customer payments were modified in this review
