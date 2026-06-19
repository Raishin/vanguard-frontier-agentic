# Workflow and output contract

Use this reference only when performing the full Microsoft Fabric / Power BI business-insights governance review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Semantic model trust: shared models, endorsement (promoted/certified), single source of truth, Build permission workflow
- Model sprawl: duplicated/competing models, reports built on personal models, orphaned models
- Security: RLS and OLS, Viewer-role behavior, Direct Lake fixed identity, DirectQuery fallback effects
- Workspace governance: roles (Admin/Member/Contributor/Viewer), separation of model vs report workspaces
- Discoverability and lineage: OneLake catalog discoverability, lineage view, dependency tracking
- Information protection: Purview sensitivity labels, DLP for Power BI, Defender for Cloud Apps, data residency, BYOK
- Capacity and oversight: Fabric capacity sizing, monitoring, certified-dataset governance

## Safe workflow

1. **Frame scope**
   - Workspace(s)/model(s) in scope and audience (executive dashboards, self-service, embedded):
   - Required outcome (metric trust / model consolidation / security / discoverability / capacity):
   - Available evidence (admin portal, lineage view, endorsement status):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer admin portal exports, lineage view, endorsement and RLS configuration, sensitivity-label coverage.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test trust and risk**
   - Which metrics come from duplicated or uncertified models (mistrust)?
   - Which reports are built on personal models instead of an endorsed shared model?
   - Which sensitive models lack RLS, or rely on RLS while exposing Admin/Member/Contributor roles?
   - Which workspace roles are broader than necessary?
   - Which models carry sensitivity labels and DLP coverage; which do not?

4. **Recommend the smallest safe action**
   - Promote a single endorsed/certified shared semantic model; separate model and report workspaces; apply RLS for Viewers.
   - Production workspace-role, RLS, sensitivity-label, and capacity changes require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# Fabric & Power BI Governance Review: <scope>
## Executive verdict
- Status: TRUSTED / TRUSTED WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
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
## Validation
- Reports or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
