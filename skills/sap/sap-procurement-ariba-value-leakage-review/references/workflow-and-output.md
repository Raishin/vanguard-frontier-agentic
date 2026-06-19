# Workflow and output contract — SAP Procurement Ariba Value Leakage Review

Use this reference for all value leakage finding classification, risk level assignment, remediation path selection, and output formatting.

## Procurement value leakage finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `maverick-spend` | `off-contract-channel-active` | Spend in a category with active negotiated contracts is being placed through non-contract channels (free-text PO, non-preferred supplier, purchase card) without policy enforcement blocking or escalation |
| `maverick-spend` | `guided-buying-not-enforced` | SAP Ariba guided buying is not configured to enforce preferred supplier routing or catalog-first purchasing for in-scope categories — users can bypass without system-level friction |
| `maverick-spend` | `catalog-coverage-gap` | Approved supplier catalog content is absent or outdated for a high-spend category — users default to free-text requisitions because guided buying offers no viable catalog option |
| `contract-compliance` | `low-contract-consumption` | Contract consumption rate is materially below committed volumes for a negotiated agreement — savings from the negotiated rate are not being realized |
| `contract-compliance` | `po-without-contract-linkage` | Purchase orders in an in-scope spend category are being created without reference to an active contract — contract bypass without system enforcement or approval |
| `contract-compliance` | `contract-leakage-threshold-missing` | No contract leakage threshold or compliance alert is configured — low consumption rates are not surfaced to category managers or procurement leadership until contract expiry |
| `supplier-risk` | `unqualified-supplier-with-spend` | A supplier with active purchase orders or invoices has not completed the required qualification workflow in SAP Ariba Supplier Management — unassessed risk with active spend exposure |
| `supplier-risk` | `risk-alert-threshold-not-configured` | Supplier risk scoring thresholds are not configured or are set too broadly — material changes in supplier financial health, geographic risk, or sanctions status do not generate alerts |
| `supplier-risk` | `concentration-risk-not-monitored` | Spend concentration in a single supplier or geographic region above a material threshold is not tracked or reported — single-point-of-failure risk in the supply base |
| `three-way-match` | `tolerance-override-without-approval` | Invoice tolerance override (quantity or price) is configured to release invoices automatically without requiring documented approver authorization — payment can be released on a match failure without human review |
| `three-way-match` | `exception-hold-aging` | Invoice holds from match exceptions are aging beyond the target resolution window — aged holds create payment delay risk (penalty) and mask systematic match failure patterns |
| `three-way-match` | `gr-not-required-for-payment` | Invoice payment can be released in SAP S/4HANA before a goods receipt (GR) is posted — three-way match control is incomplete for goods-based procurement |
| `discount-capture` | `discount-window-expiry` | Early payment discount windows are expiring without capture due to AP processing delays — measurable cost of foregone discount income |
| `discount-capture` | `dynamic-discounting-not-active` | SAP Ariba Discount Management or dynamic discounting is not configured for eligible high-spend suppliers — available discount capture mechanism is unused |
| `discount-capture` | `discount-monitoring-gap` | No dashboard or report monitors available discount opportunities against capture rates — discount leakage is invisible to AP management |
| `guided-buying` | `bypass-rate-above-threshold` | Guided buying bypass rate exceeds 20% for in-scope categories — policy is not being enforced or guided buying user experience is insufficient to drive adoption |
| `spend-analytics` | `non-po-spend-excluded` | Non-PO spend (invoices without a purchase order reference) is not included in spend analysis — a material spend channel is invisible to category management |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Three-way match bypass without approval enabling payment to unauthorized or fraudulent invoices; payment to a supplier with active sanctions screening alert |
| `high` | Maverick spend above materiality threshold in a category with negotiated contracts; contract PO linkage bypass without enforcement; unqualified supplier with active spend; tolerance override without approver authorization; discount capture failure at material scale |
| `medium` | Guided buying bypass rate above 20%; catalog coverage gap driving free-text spend; contract consumption below target without alert; exception hold aging beyond target; concentration risk not monitored |
| `low` | Best practice deviation in sourcing workflow design; minor catalog update lag; spend analytics category hierarchy alignment gap |

## Remediation path decision tree

For each finding:

1. **Is this a three-way match bypass without approver authorization releasing payment?**
   - Yes → `critical`. Immediately escalate to the audit team and AP controls team. Suspend the tolerance override configuration pending review. Require documented approver authorization for all exception invoice releases. Do not proceed with other recommendations until this is escalated.
   - No → continue.

2. **Is this a payment to a supplier with an active sanctions or disqualification alert?**
   - Yes → `critical`. Immediately escalate to the compliance and legal team. Place a payment block on affected invoices pending compliance review. Do not proceed without compliance team authorization.
   - No → continue.

3. **Is this maverick spend above a material threshold in a category with active contracts?**
   - Yes → `high`. Enable guided buying enforcement rule to require catalog or contract selection for the affected category. Review and resolve catalog coverage gaps. Report spend bypass to category managers monthly until spend channel discipline is restored.
   - No → continue.

4. **Is this a contract compliance leakage finding (low consumption or PO-without-contract)?**
   - Yes → `high`. Configure contract-to-PO linkage enforcement in SAP Ariba Buying or SAP S/4HANA Procurement. Activate contract leakage threshold alerts in SAP Ariba Contracts. Engage category managers to address low consumption root cause (contract terms, supplier performance, internal process).
   - No → continue.

5. **Is this a supplier risk gap with active spend exposure?**
   - Yes → `high`. Require supplier qualification completion before new POs can be issued to the affected supplier. Configure risk score threshold alerts in SAP Ariba Supplier Risk for the affected supplier tier. Review concentration exposure and document contingency supplier options.
   - No → continue.

6. **Is this a discount capture failure at material scale?**
   - Yes → `high`. Diagnose AP processing cycle time vs. discount window. Configure discount monitoring dashboard in SAP Ariba Discount Management. Activate dynamic discounting for eligible suppliers. Set AP SLA target for invoice processing within the discount window.
   - No → classify as `medium` or `low` and provide guidance.

## Workflow

1. **Receive artifacts** — spend analysis exports, contract compliance reports, guided buying adoption metrics, three-way match exception reports, supplier risk summaries, discount capture rate data, invoice aging reports, or written descriptions of the source-to-pay landscape.
2. **Classify each finding** by procurement leakage domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical findings** immediately — three-way match bypass without approval and payments to sanctioned suppliers must be escalated before any other remediation discussion.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical match bypass and sanctions issues first; then high maverick spend, contract compliance, supplier risk, tolerance override, and discount capture gaps; then medium guided buying and catalog findings; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Value leakage domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected spend category, supplier or contract (if provided), SAP Ariba module or S/4HANA configuration object, and specific leakage or control gap
5. Quantified leakage estimate where data is available (spend volume at risk, discount window value, contract shortfall vs. committed volume)
6. Recommended remediation action (guided buying enforcement, contract linkage activation, supplier qualification requirement, tolerance approval workflow, discount monitoring setup, spend analytics channel inclusion, etc.)
7. Procurement value leakage posture after remediation
8. Escalation notice for any critical three-way match bypass or sanctions payment finding — explicit statement that this requires audit team and compliance team sign-off before proceeding
9. Prioritized remediation sequence
10. Confirmation that no purchase orders were created, no invoices were approved, no payments were released, and no supplier records were modified in this review
