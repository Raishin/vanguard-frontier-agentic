# Workflow and output contract

Use this reference only when performing a full tenant governance review or formatting a governance posture assessment.

## Review domains

Check these areas before giving a verdict:

- **Admin role inventory**: Global Administrator count and justification, task-specific role assignments, stale or orphaned admin accounts, role audit via Microsoft 365 admin center or Microsoft Graph
- **Least-privilege compliance**: Each admin role justified against the principle of least privilege; Global Administrator reserved for emergency scenarios; workload-specific admin roles used for day-to-day operations
- **GDAP and delegated admin**: Active GDAP relationships scoped by task and time; legacy DAP relationships identified and flagged for migration; partner access limited to necessary roles only
- **Microsoft Secure Score governance**: Current score baseline, top governance-relevant improvement actions, score trend over time, ownership of improvement action implementation
- **Message Center governance**: Monitoring workflow for planned changes; CAB or change review process for major updates; communication and rollout tracking for feature changes
- **Org-wide settings**: Tenant-level defaults for sharing, external access, Teams meeting policies, and cross-workload settings; change control history; settings aligned to data classification and compliance requirements
- **Multi-workload policy coordination**: Policy consistency across Exchange Online, SharePoint, Teams, and Microsoft Entra ID; inheritance and conflict detection; policy documentation

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier:
   - Admin role count and inventory (if available):
   - Active partner/GDAP relationships (if available):
   - Compliance and regulatory drivers:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Microsoft 365 admin center evidence or Microsoft Graph read output for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - How many Global Administrator accounts exist and are any used for day-to-day tasks?
   - Which partner GDAP or DAP relationships grant more access than needed for the stated task scope?
   - Which Message Center advisory notices have been missed or not acted on?
   - Which org-wide settings were changed without a documented change control record?
   - Which Secure Score governance improvement actions remain unaddressed and at what risk score?
   - What cross-workload policy inconsistency creates a gap in data protection or compliance posture?
4. **Recommend the smallest safe action**
   - Prefer audit and reporting mode before role removal; confirm account ownership before deactivating admin roles.
   - Propose GDAP relationship scope reduction in partnership with the affected partner, not unilaterally.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Tenant Governance Review: <scope>
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
