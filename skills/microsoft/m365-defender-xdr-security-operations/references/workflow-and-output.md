# Workflow and output contract

Use this reference only when performing a full Microsoft Defender XDR SecOps posture review, incident triage assessment, advanced hunting gap analysis, or formatting the final review output.

## Review domains

Check these areas before giving a verdict:

- **Incident queue**: Severity distribution and triage coverage, incident assignment workflow, SLA for high-severity incident response, unified queue coverage across all Defender signal sources
- **Alert correlation**: Cross-product signal correlation across Defender for Endpoint, Defender for Office 365, Defender for Identity, and Defender for Cloud Apps; manual alert correlation gaps; false-positive rate and suppression rules
- **Advanced hunting**: KQL query library coverage for key threat scenarios, custom detection rule deployment, schema table coverage, FileProfile() and threat intelligence enrichment, guided mode vs. advanced mode usage
- **Automated investigation and response (AIR)**: Device group automation level (Full vs. Semi vs. None), Action Center pending action backlog, AIR false-positive rate, approval workflow for semi-automated groups
- **Automatic attack disruption**: Disruption signal review cadence, post-disruption entity review (isolated devices, disabled accounts), disruption reversal workflow
- **Containment and response runbooks**: Device isolation procedures, user account disable procedures, file and URL block procedures, incident closure criteria, escalation paths to SecOps owner
- **Defender signal sources**: Defender for Endpoint onboarding coverage, Defender for Office 365 policy configuration, Defender for Identity sensor coverage on domain controllers, Defender for Cloud Apps connected apps inventory
- **Microsoft Sentinel integration**: Workspace onboarding status, analytics rule coverage and tuning, playbook automation for common response actions, SIEM-XDR unified incident queue configuration

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier (Microsoft 365 E5, Microsoft 365 Defender, or standalone):
   - SOC maturity level and analyst headcount:
   - Key threat scenarios in scope (ransomware, BEC, insider, supply chain):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Defender XDR portal evidence, Graph Security API read output, or Sentinel workspace query results for current-state claims when available.
   - Otherwise inspect repository configuration files, exported policy JSON, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What active incident or alert in the queue has not been triaged within SLA?
   - What Defender signal source has onboarding or coverage gaps leaving devices, identities, or cloud apps unmonitored?
   - What advanced hunting scenario has no KQL query or custom detection rule covering it?
   - What AIR device group is set to None or Semi, creating a manual approval bottleneck?
   - What containment runbook is missing, untested, or lacks a defined escalation path to the SecOps owner?
   - What Sentinel analytics rule is generating excessive false positives, suppressing analyst attention to real threats?
   - What post-disruption review cadence exists for automatically isolated devices or disabled accounts?
4. **Recommend the smallest safe action**
   - Prefer advisory and runbook review mode for containment recommendations; never recommend live execution without SecOps owner approval.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# Defender XDR SecOps Review: <scope>
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
