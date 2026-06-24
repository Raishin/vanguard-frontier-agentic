# Workflow and output contract

Use this reference only when performing the full governance review, structured findings report, environment strategy assessment, or DLP/Dataverse security implementation guidance.

## Review domains

Check these areas before giving a verdict:

- **Environment strategy**: default environment hygiene, environment count and purpose, environment groups and rules, ALM topology (Dev/Test/Prod), maker provisioning process, and shared vs. isolated environment decisions
- **DLP policy posture**: coverage gaps (environments not covered), connector classification correctness, custom connector governance, HTTP connector exposure, policy layering complexity, and exception/escalation process
- **Dataverse security roles**: privilege scope (User / Business Unit / Parent-Child / Organization) per table, wildcard or overly broad role assignments, System Administrator minimization, and role-to-job-function alignment
- **Business unit design**: hierarchy depth, cross-business-unit data access patterns, hierarchy vs. position hierarchy use, and misaligned BU structure vs. org structure
- **Team design**: owner teams vs. access teams, Microsoft Entra group-backed teams vs. manual membership, and security role assignment via teams vs. direct assignment
- **Sharing posture**: ad-hoc row sharing volume, access team template appropriateness, and sharing with broad groups
- **Column security**: field-level security profile coverage on PII or sensitive columns, profile assignment scope, and overhead risk
- **CoE alignment**: CoE Starter Kit deployment, environment request process, DLP change request workflow, and maker governance signals

## Safe workflow

1. **Frame scope**
   - Tenant name (sanitized) / environment context:
   - Business criticality and data classification:
   - Compliance drivers (industry, regulation):
   - Required outcome:
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported DLP policy JSON, security role definitions, environment inventory, or Power Platform admin center screenshots as repo/user evidence.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What connectors can exfiltrate data to uncontrolled endpoints?
   - What Dataverse privileges can expose data beyond intended scope?
   - What environment gaps allow makers to bypass DLP?
   - What sharing or column security gaps leave sensitive tables exposed?
   - What evidence is missing that prevents a stronger conclusion?

4. **Recommend the smallest safe action**
   - Prefer targeted DLP policy edits, role cloning with reduced scope, and staged rollout over broad changes.
   - Production DLP changes require live-guard approval, blast-radius assessment, and rollback plan.
   - If the safest action is to gather evidence first, say that plainly.

## Output contract

Return this structure:

```markdown
# Power Platform Governance & Dataverse Security Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or admin center views:
- Expected result:
## Residual risk
- <risk or explicit none>
```
