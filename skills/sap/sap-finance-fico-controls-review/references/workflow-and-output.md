# Workflow and output contract — SAP Finance FI-CO Controls Review

Use this reference for all control finding classification, risk level assignment, remediation path selection, and output formatting.

## FI-CO control finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `document-posting-controls` | `over-permissive-document-type` | Document type allows posting to account classes that should be restricted for the given business process (e.g., vendor document type usable for G/L-only postings) |
| `document-posting-controls` | `tolerance-group-gap` | User or G/L account tolerance group permits posting differences or payment differences above the financially material threshold |
| `document-posting-controls` | `field-status-suppressed-required` | Field status group suppresses a required control field (cost center, profit center, business area, tax code) for an account category where it is a control requirement |
| `validations-substitutions` | `substitution-silent-overwrite` | Substitution rule silently overwrites a key control field (cost center, profit center, tax code, business area) without user notification or explicit approval step |
| `validations-substitutions` | `validation-inactive-or-missing` | Validation rule that should prevent an invalid account assignment combination is inactive, missing, or has an incomplete callout chain |
| `validations-substitutions` | `substitution-bypass-path` | A substitution rule can be triggered to bypass a validation that would otherwise block a posting, creating a control circumvention path |
| `period-management` | `period-open-close-sod-gap` | The same user or role can both open/close posting periods and post financial documents in the same company code — SoD gap in period management |
| `period-management` | `unrestricted-prior-period-posting` | Posting periods for prior fiscal periods are open without documented business justification or compensating approval control |
| `period-management` | `missing-period-variant-assignment` | A company code has no posting period variant assigned, or shares a posting period variant with a company code in a different control boundary |
| `financial-close-cockpit` | `task-no-owner` | FCC task list task has no responsible user or group assigned — close process has no accountable party for the task |
| `financial-close-cockpit` | `missing-task-dependency` | FCC task list allows tasks to be completed in any order, including a downstream task being marked complete before its upstream prerequisite is done |
| `financial-close-cockpit` | `no-escalation-path` | FCC task list has no blocking dependency or notification configured for overdue tasks — close timeline risk has no governance response |
| `sod-financial-postings` | `create-approve-pay-combined` | User or role has combined authority to create financial documents, approve payment runs, and release payments within the same company code |
| `sod-financial-postings` | `post-and-reverse-combined` | User or role can both post original financial documents and reverse them — creates undetected error or fraud path |
| `sod-financial-postings` | `cross-company-code-sod` | User has posting authority in multiple company codes that are in separate control boundaries, enabling artificial intercompany transactions |
| `parallel-ledgers` | `unexplained-ledger-difference` | Difference between leading ledger and non-leading ledger (local GAAP, IFRS, tax) balance is not explained by a documented reconciliation control |
| `parallel-ledgers` | `extension-ledger-misuse` | Extension ledger is used to post adjustments that should be in the base ledger, creating a hidden override of approved accounting entries |
| `intercompany` | `imbalanced-ic-posting` | Intercompany G/L postings can be created without a corresponding offsetting entry in the partner company code or without routing through the reconciliation hub |
| `intercompany` | `clearing-account-not-reconciled` | Intercompany clearing accounts carry open items that are not matched and reconciled within the expected period — unreconciled intercompany balances at period close |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | SoD in financial postings enabling fraud (create + approve + pay + reverse); uncontrolled period management SoD; financial statement misstatement risk at material threshold |
| `high` | Validation bypass or silent substitution of key control field; parallel ledger difference without reconciliation control; imbalanced intercompany posting path; unrestricted prior-period posting without compensating control |
| `medium` | FCC governance gap (missing owner, missing dependency, no escalation); tolerance group above internal threshold; field status suppression of a recommended control field |
| `low` | Best practice deviation in document type design; minor field status gap in a low-risk account category; cosmetic FCC task list structure improvement |

## Remediation path decision tree

For each finding:

1. **Is this a critical SoD exposure in financial postings (create + approve + pay + reverse combined)?**
   - Yes → `critical`. Immediately escalate to the audit team and GRC team. Do not approve or defer without documented compensating control and approver sign-off. State this explicitly in the response. Role redesign is required.
   - No → continue.

2. **Is this a validation bypass or silent substitution overwrite of a key control field?**
   - Yes → `high`. Review the substitution rule logic and callout chain. Remove the overwrite for the affected field or add an explicit user notification step. Engage the FI configuration team to correct the rule in a non-production system first.
   - No → continue.

3. **Is this a period management SoD gap (same user opens periods and posts documents)?**
   - Yes → `high`. Separate the S_PERIOD_OPEN authorization (or equivalent in S/4HANA) from the F_BKPF_BUK posting authorization. Assign period management authority to a dedicated finance operations user or the Controller role only.
   - No → continue.

4. **Is this a parallel ledger difference without a reconciliation control?**
   - Yes → `high`. Document the expected differences (e.g., IFRS vs. local GAAP revaluation, tax depreciation delta). Implement a ledger reconciliation report as a scheduled FCC task. Flag any unexplained residual difference for audit review.
   - No → continue.

5. **Is this an intercompany imbalance or unmatched clearing account?**
   - Yes → `high`. Enforce intercompany reconciliation hub routing for all intercompany G/L postings. Add a period-end FCC task requiring reconciliation hub match rate of 100% (or documented exception) before period close sign-off.
   - No → continue.

6. **Is this a Financial Close Cockpit governance gap?**
   - Yes → `medium`. Assign a responsible user or group to each unowned task. Add dependency links between prerequisite and downstream tasks. Configure blocking logic or notification for overdue tasks.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — configuration exports, validation/substitution descriptions, period variant summaries, FCC task list exports, parallel ledger configuration notes, intercompany reconciliation summaries, role lists, or user descriptions.
2. **Classify each finding** by FI-CO control domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag critical SoD exposures in financial postings** immediately — escalate before any other remediation discussion.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical SoD exposures first; then high validation/substitution bypass and period management gaps; then high parallel ledger and intercompany findings; then medium FCC and tolerance gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. FI-CO control domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected configuration object, transaction code, company code, role or user (if provided), and specific control gap
5. Recommended remediation action (role redesign, validation rule correction, tolerance group tightening, period variant SoD separation, FCC task owner assignment, parallel ledger reconciliation control, intercompany hub routing enforcement, etc.)
6. FI-CO control posture after remediation
7. Escalation notice for any critical SoD exposure in financial postings — explicit statement that this requires audit team and GRC team sign-off before proceeding
8. Prioritized remediation sequence
9. Confirmation that no financial documents were posted, reversed, or modified in this review
