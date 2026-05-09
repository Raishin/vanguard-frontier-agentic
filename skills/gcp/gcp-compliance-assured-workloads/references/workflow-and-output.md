# Workflow and output contract

Use this reference only when performing the full compliance review, evidence package assembly, implementation guidance, or audit preparation pass.

## Review domains

Check these areas before giving a verdict:
- Assured Workloads folder: framework (FedRAMP High/Moderate/HIPAA/PCI-DSS/ITAR/IL4/IL5), folder ID, compliance program state
- Service usage: all GCP services in use vs. authorized services list for the applicable framework
- HIPAA BAA: BAA-covered services vs. services touching PHI data path
- ITAR: personnel access restriction to US persons, Assured Workloads ITAR config status
- Data residency: resource locations vs. allowed regions for the framework
- Audit logs: Data Access audit log types enabled (admin activity / data read / data write) per service
- SCC compliance dashboard: finding count by severity, unresolved findings, compliance score
- Asset Inventory: org policy violations, unauthorized resource types, resource change history

## Safe workflow

1. **Frame scope**
   - Org/folder/project and compliance framework:
   - Business criticality and owner:
   - Data classification (PHI/CUI/PCI/ITAR):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API/SCC read-only evidence if available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What services are in use but not authorized for the compliance framework?
   - What Data Access audit log types are missing?
   - What SCC findings are unresolved?
   - What evidence is missing for the compliance evidence package?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Compliance Assured Workloads: <scope>
## Executive verdict
- Status: COMPLIANT / COMPLIANT WITH GAPS / NON-COMPLIANT / NEEDS EVIDENCE
- Framework:
- Biggest gap:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Evidence package gaps
- <gaps or explicit none>
## Residual risk
- <risk or explicit none>
```
