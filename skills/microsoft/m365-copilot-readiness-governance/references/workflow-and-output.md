# Workflow and output contract

Use this reference only when performing the full readiness assessment, generating a remediation plan, incident triage, or formatting the final review.

## Review domains

Check these areas before giving a verdict:

- **Layer 1 — Data protection**: Sensitivity label coverage, DLP policy scope, DSPM for AI data risk assessment, oversharing controls (RSS, RCD, DAG reports), Microsoft Purview Compliance Manager AI regulations
- **Layer 2 — Identity and access**: MFA enforcement, Conditional Access baseline, access reviews, Microsoft Entra ID P1/P2 licensing scope
- **Layer 3 — App protection**: Intune app protection policies, approved client apps, MAM without enrollment
- **Layer 4 — Device management**: Intune enrollment, device compliance policies, Defender for Endpoint integration
- **Layer 5 — Threat protection**: Defender for Office 365, EOP, Defender XDR integration, audit log enabled
- **Layer 6 — Secure Teams collaboration**: Teams sharing settings, guest access controls, channel lifecycle, external access policies
- **Layer 7 — User permissions to data**: JEA/JIT, EEEU removal, site access reviews, site ownership confirmed, inactive site remediation

Also check:
- Microsoft Graph permission scope for any connectors, plugins, or extensibility scenarios
- Copilot extensibility: Microsoft 365 Copilot Studio agents, Graph connectors, third-party connectors — each must have scoped permissions reviewed before enablement

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier:
   - Business criticality and data sensitivity classification:
   - Copilot license rollout size and target population:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Microsoft 365 Admin Center or Graph API read evidence for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Microsoft docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk per layer**
   - What data can Copilot surface that users should not see?
   - What stale permissions or EEEU grants amplify oversharing blast radius?
   - What connectors or plugins have unscoped Graph permissions?
   - What compliance or audit evidence is missing?
   - What prevents rollback if Copilot is paused post-enablement?
4. **Recommend the smallest safe action**
   - Prefer narrow scope (pilot group), staged rollout, SAM interim controls, and rollback playbook.
   - If the safest action is to stop and complete the baseline first, say that plainly and refuse enablement.

## Output contract

Return this structure:

```markdown
# M365 Copilot Readiness Governance Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Zero Trust layer findings
| Layer | Status | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or reports to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
