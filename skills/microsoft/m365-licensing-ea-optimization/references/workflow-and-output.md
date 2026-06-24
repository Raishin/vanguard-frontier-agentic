# Workflow and output contract

Use this reference only when performing a full licensing posture review or formatting a cost optimization assessment.

## Review domains

Check these areas before giving a verdict:

- **License assignment hygiene**: Unassigned licenses, stale accounts with active licenses, manual versus group-based assignment coverage, usage location compliance for all licensed users
- **Group-based licensing structure**: Security group design for license assignment, nested group limitations, dependency and conflict resolution, audit log coverage for group-based licensing changes
- **SKU fit analysis**: E3 versus E5 capability gap for the user population, F1/F3 appropriateness for Firstline Worker scenarios, add-on necessity versus base SKU inclusion, Microsoft Entra ID P1 versus P2 requirement
- **Over- and under-assignment**: Users with E5 where E3 suffices, missing add-ons for users who need a specific capability, add-on purchased when base SKU already includes it
- **True-up planning**: EA annual true-up preparation, license count trend, seat growth and reduction planning, reconciliation of purchased versus assigned counts
- **Contract type awareness**: EA, CSP, or MCA contract characteristics in context (advisory only — no pricing commitments); volume licensing admin center usage for VL contracts
- **License governance**: License Administrator role scoping, audit log coverage for license assignments, reporting cadence via Microsoft 365 admin center and Microsoft Graph

## Safe workflow

1. **Frame scope**
   - Tenant / environment / current SKU mix (if available):
   - Assignment method (manual, group-based, PowerShell):
   - Contract type and renewal context (advisory context only):
   - Compliance and regulatory licensing requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Microsoft 365 admin center evidence or Microsoft Graph license API read output for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which users have licenses assigned but have not signed in for 90+ days?
   - Are any license assignments manual-only with no group-based automation, creating de-provisioning gaps?
   - Are nested groups used for license assignment, creating silent gaps in coverage?
   - Which add-ons are assigned to users whose base SKU already includes those capabilities?
   - Is the E3-versus-E5 decision based on documented capability requirements or historical default?
   - Does the EA true-up timeline align with current headcount trend to avoid surprise overage?
4. **Recommend the smallest safe action**
   - Prefer audit and reporting before removing licenses — confirm inactivity before de-provisioning.
   - Never recommend license removal for active users without confirming service dependency.
   - Never make or imply purchase commitments or savings guarantees — advisory only.
   - Group-based licensing changes in production are live-guard gated; escalate to a human administrator.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Licensing and EA Optimization Review: <scope>
## Executive verdict
- Status: OPTIMIZED / OPTIMIZATION OPPORTUNITIES IDENTIFIED / NEEDS EVIDENCE
- Biggest risk or opportunity:
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
