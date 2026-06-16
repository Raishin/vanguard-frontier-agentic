# Workflow and output contract

Use this reference only when performing the full identity posture review, Conditional Access baseline gap assessment, or formatting the final review.

## Review domains

Check these areas before giving a verdict:

- **MFA coverage**: Admin MFA (phishing-resistant), all-user MFA, legacy authentication block, security defaults vs. Conditional Access tradeoffs
- **Conditional Access baseline**: Common policies (admin MFA, all-user MFA, legacy block, device compliance, app protection), named location inventory, session controls, authentication strengths
- **Risk-based policies**: Sign-in risk and user risk policies (requires Identity Protection / Entra ID P2), self-remediation flows
- **Privileged Identity Management**: Standing vs. eligible role assignments, activation requirements (MFA, approval, justification, time limit), PIM Discovery and Insights, access review cadence for privileged roles
- **Least-privilege role assignments**: Global Administrator count and alternatives, role delegation by task, administrative units, custom roles
- **Microsoft Entra ID Governance**: Access reviews for groups, apps, privileged roles, and guest users; entitlement management expiration; lifecycle workflows
- **Guest and external identity**: B2B collaboration settings, guest access reviews, cross-tenant access policies, external user lifecycle
- **Break-glass accounts**: Emergency access accounts, excluded from CA policies, monitored via alerts, reviewed regularly

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier (Entra ID P1 or P2):
   - Administrator count and role inventory:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Microsoft Entra admin evidence or Graph API read output for current-state claims when available.
   - Otherwise inspect repository IaC/config (Bicep, Terraform, JSON), sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What identity path can an attacker use to escalate to Global Administrator?
   - What MFA or Conditional Access gap allows password spray or phishing to succeed?
   - What standing privileged assignment widens blast radius?
   - What guest or external identity has stale access?
   - What compliance or audit evidence is missing?
   - What rollback or validation path is unproven for CA changes?
4. **Recommend the smallest safe action**
   - Prefer report mode for new CA policies, staged rollout (pilot group), PIM eligible before active, and access reviews before removal.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Identity Zero Trust Review: <scope>
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
