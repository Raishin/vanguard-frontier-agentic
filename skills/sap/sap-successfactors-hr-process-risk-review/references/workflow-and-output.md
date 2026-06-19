# Workflow and output contract — SAP SuccessFactors HR Process Risk Review

Use this reference for all finding classification, risk level assignment, remediation path selection, and output formatting.

## HR governance domain taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `rbp` | `sensitive-field-over-permission` | Permission role grants visibility to compensation, national ID, bank details, home address, or health data beyond the documented business need |
| `rbp` | `broad-target-population` | Target population scope exposes sensitive HR data to non-HR users, managers outside the relevant org unit, or system users without justification |
| `rbp` | `missing-field-level-restriction` | Employee Central sensitive data fields are accessible without field-level permission control, relying only on UI suppression |
| `rbp` | `direct-user-permission-group-assignment` | User assigned directly to a permission group rather than via a dynamic role or position-based rule, bypassing lifecycle governance |
| `org-position` | `unenforced-position-approval` | Position creation or reclassification workflow not enforced — positions can be created without headcount plan approval |
| `org-position` | `broken-reporting-line` | Reporting line in Employee Central does not match the approved org structure, with no audit trail for the discrepancy |
| `org-position` | `headcount-plan-mismatch` | Filled or open positions do not match the approved headcount plan — over- or under-headcount not flagged |
| `hire-to-retire` | `incomplete-onboarding-workflow` | Hire event action steps are missing required triggers (e.g., IT provisioning, equipment request, policy acknowledgment) |
| `hire-to-retire` | `rehire-duplicate-not-detected` | Rehire process does not check for duplicate employee records, risking concurrent active records for the same individual |
| `hire-to-retire` | `incomplete-termination-action-steps` | Termination event does not trigger access revocation, final pay, or equipment return within the required SLA |
| `hire-to-retire` | `missing-data-archiving-policy` | No data retention or archiving policy is configured for terminated employee records in Employee Central |
| `payroll-integration` | `unresolved-replication-error` | Failed replication event for a payroll-relevant field (pay grade, work schedule, bank details) is unacknowledged and unresolved |
| `payroll-integration` | `missing-reconciliation-control` | No reconciliation process exists to validate that Employee Central payroll-relevant field changes are correctly propagated to the payroll processor |
| `payroll-integration` | `over-broad-integration-field-mapping` | Integration Center mapping replicates more fields to the payroll processor than required — excess personal data transferred without justification |
| `data-privacy` | `pii-field-unclassified` | Employee Central fields containing personal data are not classified by sensitivity level in the data privacy impact assessment |
| `data-privacy` | `missing-erasure-workflow` | No tested erasure workflow exists for EU employee personal data in Employee Central — GDPR right-to-erasure compliance at risk |
| `data-privacy` | `cross-border-transfer-undocumented` | Personal data replicated from Employee Central to a payroll processor or integration target in a different country without documented transfer basis |
| `data-privacy` | `consent-management-absent` | No consent management configuration for optional personal data processing in Employee Central for the applicable jurisdiction |
| `jml-lifecycle` | `termination-access-delay` | Terminated employee retains active SuccessFactors access beyond the defined deprovisioning SLA |
| `jml-lifecycle` | `joiner-access-delay` | New joiner does not receive appropriate RBP access on the contracted start date |
| `jml-lifecycle` | `mover-rbp-not-updated` | Internal transfer or promotion does not trigger an RBP update within the required window, leaving the employee with their previous role's access |
| `jml-lifecycle` | `no-access-certification` | No periodic access certification campaign covers SuccessFactors RBP assignments for HR data access |

## Risk classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Terminated employee retaining active access; confirmed PII exposure (unauthorized access to national ID, bank details, health data, or compensation); unresolved payroll replication error directly affecting pay calculations |
| `high` | Sensitive HR field over-permission without business justification; JML SLA breach (joiner or mover); missing GDPR erasure workflow for EU data; unresolved payroll replication error for payroll-relevant fields; broad target population exposing compensation or health data to non-HR users |
| `medium` | Governance gap without immediate exposure risk: missing access certification, broken reporting line without confirmed fraud risk, incomplete onboarding workflow, undocumented cross-border transfer basis, missing data archiving policy |
| `low` | Best practice deviation: direct user permission group assignment without lifecycle impact, missing field-level PII classification for non-sensitive fields, UI-only restriction without policy control |

## Remediation path decision tree

For each finding:

1. **Is this a terminated employee retaining active SuccessFactors access?**
   - Yes → `critical`. Immediately deactivate the user in SuccessFactors and revoke all RBP assignments. Escalate to HR and IT security. Do not defer deprovisioning. State this explicitly.
   - No → continue.

2. **Is this confirmed unauthorized access to compensation, health data, national ID, or bank details?**
   - Yes → `critical`. Escalate to HR leadership and legal or data protection team before any further analysis or remediation. Do not attempt to assess the scope of exposure without legal guidance.
   - No → continue.

3. **Is this a sensitive HR field over-permission without justification, or a missing GDPR erasure workflow?**
   - Yes → `high`. Redesign the permission role to restrict sensitive field visibility to the minimum required population. For missing erasure workflow: implement the Employee Central data erasure capability and test with a dry run before production use.
   - No → continue.

4. **Is this an unresolved payroll replication error for a payroll-relevant field?**
   - Yes → `high`. Escalate to the payroll processing team immediately. Validate that the payroll processor holds the correct value. If the error affects bank details or pay grade, treat as `critical` and escalate HR and payroll leadership.
   - No → continue.

5. **Is this a governance gap without confirmed exposure (missing certification, broken reporting line, undocumented transfer)?**
   - Yes → `medium`. Initiate an access certification campaign to validate RBP assignments. Correct reporting line discrepancies with HR approval. Document the cross-border transfer basis per applicable regulation.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — RBP configuration exports, permission role lists, org chart descriptions, integration mapping documentation, data privacy impact assessment summaries, or written process descriptions. Redirect immediately if raw PII is supplied.
2. **Classify each finding** by HR governance domain and finding class above.
3. **Assign risk level** per risk classification table (critical / high / medium / low).
4. **Flag critical findings immediately** — terminated access retention, confirmed PII exposure, and critical payroll errors must be escalated before other remediation is discussed.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — critical JML and PII findings first; then high RBP and payroll findings; then medium governance gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. HR governance domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. RBP or JML detail (if applicable): permission role, permission group, target population, affected employee population, or integration field
5. Recommended remediation action (RBP redesign, target population narrowing, termination action step addition, payroll error escalation, GDPR erasure workflow implementation, access certification campaign, etc.)
6. HR compliance posture after remediation
7. Escalation notice for critical PII or JML findings — explicit statement that HR leadership and legal must be engaged before proceeding
8. Prioritized remediation sequence
