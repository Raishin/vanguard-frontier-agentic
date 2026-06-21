# Workflow and output contract — SAP Security HR Legal Escalation Protocol

Use this reference for all trigger classification, evidence assembly, cross-function handoff sequencing, decision rights application, and output formatting.

## Trigger classification table

| Trigger class | Primary signal | Activating agent | HR involvement | Legal involvement |
|---|---|---|---|---|
| `identity-misuse` | Anomalous access pattern, bulk extraction, unexpected location | sap-security-iam-grc-sod-reviewer-agent | Required for employment-status confirmation | Required if criminal or employment action may follow |
| `privileged-access-anomaly` | Firefighter/EAM access outside approved scope; SAP\_ALL outside change window | sap-security-iam-grc-sod-reviewer-agent | Optional (required if HR data was accessed) | Optional (required if disciplinary action follows) |
| `critical-sod-violation` | Unmitigated critical SoD conflict involving active employee | sap-security-iam-grc-sod-reviewer-agent | Required (employment status + role confirmation) | Required (regulatory and employment-law review) |
| `insider-risk` | DLP or SIEM exfiltration indicator against HR, payroll, or financial data | sap-security-iam-grc-sod-reviewer-agent + sap-successfactors-hr-process-risk-agent | Required (data sensitivity classification) | Required (legal hold decision) |
| `hr-sensitive-access` | Access to SuccessFactors compensation, performance, or succession data outside authorized scope | sap-successfactors-hr-process-risk-agent | Required (owns data sensitivity classification) | Conditional (if personal data breach threshold met) |
| `jml-gap` | Post-termination access; mover retaining previous role; joiner access beyond matrix | sap-successfactors-hr-process-risk-agent | Required (lifecycle event confirmation) | Conditional (if post-termination access was used) |
| `fraud-sensitive-access` | Single-user execution of incompatible financial transactions | sap-security-iam-grc-sod-reviewer-agent | Required (employment context) | Required (potential fraud investigation) |

## Protocol workflow

### Phase 1 — Triage (SAP Security lead)

1. Classify the trigger condition using the table above.
2. Identify which participating agents are activated.
3. Confirm which evidence items are available and which are outstanding.
4. Issue evidence assembly requests to the responsible function for each missing item.
5. Apply the redaction policy before any cross-function share.

### Phase 2 — Evidence assembly (multi-function)

1. `sap-security-iam-grc-sod-reviewer-agent` assembles identity evidence, access logs, GRC conflict reports, and role assignment state.
2. `sap-successfactors-hr-process-risk-agent` assembles HR lifecycle state and provides redacted summary to Security per redaction policy.
3. Legal is notified if trigger classification indicates a legal dimension; Legal confirms whether a legal hold is required.

### Phase 3 — Cross-function review

1. All functions review assembled evidence within their domain authority.
2. `sap-security-iam-grc-sod-reviewer-agent` classifies SoD conflicts by risk level.
3. `sap-successfactors-hr-process-risk-agent` classifies HR data sensitivity and confirms whether HR lifecycle events were processed correctly.
4. Decision rights table is applied — each pending decision is assigned to the correct authority.

### Phase 4 — Action proposal

1. Proposed actions are documented with the required approvals per the approval requirements section of the SKILL.md.
2. Irreversible-action gate is evaluated for each proposed action.
3. Actions requiring `sap-role-assignment-guarded-operator-agent` are queued with approval documentation attached — the operator agent does not execute without confirmed approvals.

### Phase 5 — Audit package assembly

1. All evidence, decision records, approval records, redaction logs, and action outcomes are consolidated into the audit package.
2. Audit package is made available to internal audit and, where required by legal hold, to Legal.
3. Residual risk is assessed and documented.

## Output contract

Return, in order:

1. **Trigger classification**: Which trigger class(es) apply; which participating agents are activated.
2. **Evidence inventory**: Present items, missing items, and responsible function for each missing item.
3. **Redaction confirmation**: Explicit statement that HR-sensitive data has been identified and the redaction policy status (applied / pending / not applicable).
4. **Decision rights map**: For each pending decision, primary authority, secondary approvals required, and current status (pending / approved / deferred).
5. **Irreversible-action gate**: Whether any irreversible actions are pending; approval status for each; whether gate is cleared or blocked.
6. **SoD conflict summary** (if applicable): Conflict ID or description, risk level, mitigation status, and escalation notice if unmitigated critical.
7. **HR data sensitivity assessment** (if applicable): Data categories accessed or at risk; classification; redaction applied.
8. **Escalation notice**: If an unmitigated critical SoD conflict, insider-risk signal, or legal-hold trigger is present, explicit notice must appear before any other recommendation, naming the escalation owners and required action.
9. **Audit package status**: Populated items and outstanding items.
10. **Next step**: Single next action with named responsible owner.
