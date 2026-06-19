# Workflow and output contract

Use this reference only when performing a full Microsoft Purview compliance posture review, DLP gap assessment, eDiscovery readiness review, or formatting the final review output.

## Review domains

Check these areas before giving a verdict:

- **Sensitivity labels**: Label taxonomy design (public, general, confidential, highly confidential), mandatory labeling, auto-labeling policies, label inheritance for Copilot, encryption and access control scope, label policy publishing coverage
- **Data Loss Prevention**: Policy scope and locations (Exchange, SharePoint, OneDrive, Teams, Endpoints, Copilot), sensitive information type accuracy and confidence levels, DLP rule actions (block, audit, warn, user override), Endpoint DLP device onboarding coverage, Adaptive Protection integration with Insider Risk risk levels
- **Data lifecycle and retention**: Retention policy coverage for regulated content types, retention label application (manual, auto-apply, default), records management and regulatory records, preservation locks, disposition review workflows
- **Insider Risk Management**: Policy template selection, risk indicator configuration, privacy controls (pseudonymization), Adaptive Protection enablement, escalation path to eDiscovery, pay-as-you-go licensing consideration
- **eDiscovery and legal hold**: Active litigation hold completeness (all custodians, all data sources), legal hold notification workflows, review set readiness, KQL search accuracy, custodian release procedures
- **Audit (Premium)**: Audit log retention configuration (365-day baseline, 10-year for E5 Compliance), intelligent insights coverage, forensic investigation readiness
- **DSPM for AI**: Data risk assessment results for SharePoint and OneDrive oversharing, sensitive data exposure in Copilot and third-party AI apps, recommended actions status

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier (Microsoft 365 E3, E5, or Microsoft Purview suite):
   - Compliance driver or regulation (GDPR, HIPAA, financial records, legal hold):
   - Data classification maturity (no labels, partial, full label taxonomy deployed):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Microsoft Purview compliance portal evidence or Graph API read output for current-state claims when available.
   - Otherwise inspect repository configuration files, exported policy JSON, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What sensitive content is not covered by a sensitivity label or DLP policy?
   - What DLP policy is in audit/test mode and has never been enforced?
   - What regulated content type lacks a retention policy or retention label?
   - What eDiscovery hold does not cover all relevant custodians or data sources?
   - What Insider Risk policy template is missing for high-risk scenarios (departing users, data leakage)?
   - What Microsoft 365 Copilot or AI app interaction is exposing unlabeled or over-shared sensitive data?
   - What rollback path exists if a new DLP policy blocks legitimate business workflows?
4. **Recommend the smallest safe action**
   - Prefer report/audit mode for new DLP policies before enforcement, staged rollout (pilot group), and access reviews before revoking sharing.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# Purview Data Security and Compliance Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Control area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or reports to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
