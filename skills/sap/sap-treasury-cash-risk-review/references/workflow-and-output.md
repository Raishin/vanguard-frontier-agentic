# Workflow and output contract — SAP Treasury Cash Risk Review

Use this reference for all treasury control finding classification, risk level assignment, remediation path selection, and output formatting.

## Treasury control finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `cash-position-liquidity` | `memo-record-uncontrolled` | Memo records (planned cash items) can be created or modified without authorization controls, distorting the cash position view and enabling fraudulent cash concentration decisions |
| `cash-position-liquidity` | `bank-statement-processing-gap` | Electronic bank statement processing is not automated or is partially automated, creating a manual data entry risk and delaying cash position accuracy |
| `cash-position-liquidity` | `liquidity-hierarchy-incomplete` | Liquidity item hierarchy does not cover all relevant cash flow sources (SD billing, MM invoice, payroll, intercompany), creating blind spots in the liquidity forecast |
| `bank-account-management` | `bam-four-eyes-gap` | Bank account master data can be created or modified without a documented four-eyes (dual-control) approval workflow in SAP Bank Account Management |
| `bank-account-management` | `unauthorized-signatory-change` | Bank account signatories can be added, modified, or removed without a separate authorization control distinct from the bank account maintenance role |
| `bank-account-management` | `shadow-bank-account` | Bank account records exist in the system without a corresponding active BAM lifecycle governance entry — potential for untracked external payment destinations |
| `in-house-cash-payment-factory` | `aggregation-single-approver` | Payment factory aggregation and external release can be authorized by a single approver without a second independent reviewer at the aggregation stage |
| `in-house-cash-payment-factory` | `ihc-intercompany-posting-gap` | In-house cash intercompany postings can be created without a corresponding offsetting internal account entry — creates hidden intercompany imbalance in the in-house bank |
| `in-house-cash-payment-factory` | `netting-authorization-gap` | Netting proposals can be created, modified, and settled by the same user without a separate settlement authorization control |
| `financial-instruments` | `instrument-type-misconfiguration` | Financial instrument type configuration does not match the entity's stated hedging or investment strategy — creates a mismatch between TRM position reporting and actual risk exposure |
| `financial-instruments` | `settlement-single-approver` | Financial instrument settlement can be authorized by the same user who created the transaction — no dual-control separation between deal entry and settlement release |
| `hedge-exposure-management` | `hedge-documentation-incomplete` | Hedge designation documentation is missing required elements under IFRS 9 or IAS 39: risk component identification, hedging relationship description, effectiveness testing methodology, or hedge accounting start date |
| `hedge-exposure-management` | `effectiveness-testing-gap` | Effectiveness testing is not configured or is not being run at required intervals — hedge relationship may not qualify for hedge accounting treatment |
| `hedge-exposure-management` | `exposure-determination-mismatch` | Exposure determination source configuration does not match the actual source of the hedged exposure (e.g., FX exposure derived from SD orders but actual exposure is from intercompany loans) |
| `payment-fraud-controls` | `single-user-payment-release` | A single user can create, approve, and release an external payment run without a second authorized approver — critical fraud control gap |
| `payment-fraud-controls` | `bank-channel-security-gap` | Host-to-bank file transmission lacks integrity controls (digital signature, file hash verification, or SFTP with certificate authentication) — payment file could be intercepted or modified |
| `payment-fraud-controls` | `payment-status-monitoring-gap` | No automated payment status monitoring or exception alerting is configured — rejected or returned payments may not be detected in time to prevent reuse of fraudulent payment instructions |
| `treasury-sod` | `create-approve-release-combined` | User or role has combined authority to create financial instruments or payment requests, approve them, and release settlement or payment — full SoD bypass in treasury operations |
| `treasury-sod` | `bam-payment-release-combined` | User or role has combined authority to manage bank account master data (TR_BANKI, TR_BANKA) and release external payments (F_PAYR_BUK, F_REGU_BUK) — enables unauthorized payment destination manipulation followed by payment release |
| `treasury-sod` | `position-memo-record-combined` | User can modify cash position memo records and also release payment requests — enables manipulation of the cash position view to mask unauthorized outflows |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Single-user external payment release authority (create + approve + release without dual-control); combined BAM write authority and payment release authority enabling payment destination fraud; unauthorized bank account creation path |
| `high` | BAM four-eyes approval gap; hedge documentation deficiency creating derecognition or restatement risk under IFRS 9 or IAS 39; settlement dual-control gap in financial instrument processing; payment factory aggregation single-approver; unauthorized signatory modification; bank channel integrity control gap |
| `medium` | Cash position accuracy gap from memo record governance weakness; liquidity hierarchy incomplete for material cash flow source; effectiveness testing configuration gap; netting authorization gap; payment status monitoring gap; exposure determination source mismatch |
| `low` | Best practice deviation in instrument type design; minor liquidity hierarchy hierarchy gap for an immaterial cash flow category; reporting configuration deviation |

## Remediation path decision tree

For each finding:

1. **Is this single-user external payment release authority (create + approve + release combined)?**
   - Yes → `critical`. Immediately escalate to the treasury, audit, and GRC team. Do not authorize further external payments until dual-control is implemented and verified. Implement two-person payment release in SAP Bank Communication Manager and verify authorization object separation between payment creation (F_BKPF_BUK) and payment release (F_PAYR_BUK, F_REGU_BUK). State this explicitly.
   - No → continue.

2. **Is this combined BAM write authority and payment release authority (TR_BANKI/TR_BANKA + F_PAYR_BUK)?**
   - Yes → `critical`. Escalate immediately. Separate bank account management authority from payment release authority at the role and authorization object level. This combination enables a user to create a fraudulent bank account and then release a payment to it.
   - No → continue.

3. **Is this a BAM four-eyes approval gap or unauthorized signatory modification?**
   - Yes → `high`. Implement four-eyes approval workflow in SAP Bank Account Management for account creation, modification, and signatory changes. Assign separate approver roles distinct from the bank account maintenance role.
   - No → continue.

4. **Is this a hedge documentation deficiency or effectiveness testing gap?**
   - Yes → `high`. Identify all active hedge relationships with incomplete documentation. Engage the treasury accounting team to complete IFRS 9 or IAS 39 hedge designation documentation. Configure effectiveness testing at the required intervals. Document the risk component identification, hedging relationship, and start date for each relationship. Flag for audit review.
   - No → continue.

5. **Is this a payment factory aggregation single-approver or financial instrument settlement dual-control gap?**
   - Yes → `high`. Add a second, independent approval step at the aggregation or settlement stage. Verify that the second approver role is separate from the deal entry or payment request creation role.
   - No → continue.

6. **Is this a cash position accuracy or liquidity hierarchy gap?**
   - Yes → `medium`. Map all material cash flow sources to the liquidity item hierarchy. Add memo record authorization controls. Automate bank statement processing where manual entry is currently used.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — authorization object exports, bank account master data reports, BAM configuration summaries, payment run logs, in-house cash configuration notes, hedge documentation exports, financial instrument type summaries, bank statement processing descriptions, or written descriptions of the treasury landscape.
2. **Classify each finding** by treasury control domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical payment fraud findings immediately** — escalate before any other remediation discussion.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical payment fraud and BAM findings first; then high hedge documentation and instrument settlement gaps; then high payment factory and bank channel findings; then medium cash position and liquidity gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Treasury control domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected configuration object, transaction code or Fiori app, company code or treasury area, role or authorization object (if provided), and specific control gap
5. Recommended remediation action (dual-control payment release implementation, BAM four-eyes approval workflow, hedge documentation completion, instrument settlement dual-control, payment factory second approver, bank channel integrity control, treasury SoD role separation, memo record authorization tightening, etc.)
6. Treasury control posture after remediation
7. Escalation notice for any critical payment fraud finding or combined BAM/payment-release authority — explicit statement that this requires treasury, audit, and GRC team sign-off before proceeding
8. Prioritized remediation sequence
9. Confirmation that no payments, trades, hedges, settlements, or bank account changes were initiated, approved, or released in this review
