---
name: sap-data-migration-cutover-readiness
description: Review data migration and cutover READINESS for an SAP S/4HANA transformation. Assesses SAP Migration Cockpit approach, data quality and validation gate completeness, mock run results, cutover plan structure, rollback and fallback viability, reconciliation strategy, and go/no-go criteria. Advisory only — never executes migration, cutover, or any live data movement. Use when assessing whether a program is ready to proceed to cutover, not when executing one.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: delivery
  lifecycle: experimental
---

# SAP Data Migration and Cutover Readiness Review

## Purpose

Assess whether an SAP S/4HANA transformation program is ready to proceed to data migration execution and system cutover. Review the SAP Migration Cockpit (LTMC/LTMOM) approach and migration object scope, evaluate data quality and validation gate completeness, interpret mock run results and open issues, assess cutover plan structure and sequencing, evaluate rollback and fallback plan viability, review reconciliation strategy and sign-off process, and validate that go/no-go criteria are formally defined and measurable. This is an advisory readiness review. It does not execute, trigger, or assist in executing any data migration, cutover action, or live system change.

## When to use

Use this skill when the user asks to:

- assess whether the data migration approach (SAP Migration Cockpit, LTMC, LTMOM, or third-party ETL) is correctly scoped and configured for their S/4HANA transformation,
- review data quality results from extraction, validation, or mock migration runs and classify open data quality issues by severity,
- evaluate the completeness of cutover plan documentation: task list, sequencing, duration estimates, responsible parties, and dependency mapping,
- assess whether mock run results (mock run 1, mock run 2, dress rehearsal) indicate readiness or outstanding risk for go-live,
- review rollback and fallback procedures: technical feasibility, activation triggers, decision authority, and time-to-rollback estimates,
- validate that reconciliation checks (financial totals, open items, inventory balances, master data record counts) are defined and will be executed at cutover,
- assess whether go/no-go criteria are formally documented with measurable thresholds and a named decision authority,
- identify data migration readiness gaps that would represent unacceptable risk for proceeding to production cutover.

## When not to use

- When the user wants to execute a data migration or cutover — this skill does not assist with execution. A separate guarded live-execution agent would be required for that purpose.
- When the user needs a transformation strategy assessment (brownfield vs greenfield) — use `sap-s4hana-transformation-architecture-review`.
- When the user needs custom code remediation review — use `sap-custom-code-remediation-review`.
- When the user needs live transport management — use `sap-guarded-transport-import`.
- When live system discovery is needed to enumerate migration objects or open cutover tasks — use `sap-live-readonly-landscape-discovery` first.

## Advisory-only boundary — explicitly stated

**This skill does not execute, trigger, or assist in executing any data migration, cutover, or live system change.** It does not:
- connect to any SAP system, LTMC/LTMOM session, or Migration Cockpit tenant,
- trigger migration runs, transformation jobs, staging table loads, or cutover scripts,
- approve, authorize, or initiate go/no-go decisions for live cutovers,
- access production data, financial records, or business partner master data.

All readiness assessment is based on documentation, mock run reports, cutover plan documents, and data quality validation outputs supplied by the user. Live cutover execution is a risk-critical mutating operation that requires a separate, explicitly guarded execution skill with approval gates and rollback controls.

## Lean operating rules

- Readiness is binary per gate. Each readiness dimension (data quality, mock run completion, cutover plan, rollback, reconciliation, go/no-go criteria) is either sufficiently complete or not. Partial completion is classified as a gap, not as partial readiness.
- Mock run evidence is required before assessing cutover readiness. A program cannot be assessed as cutover-ready without at least one completed mock run with documented results. Dress rehearsal results carry the most weight.
- Data quality thresholds must be explicit. "Data quality is good" is not a sufficient readiness indicator. Thresholds must be defined (e.g., error rate < 1% for master data; 0% error tolerance for financial open items) and the last validation run must have met or exceeded those thresholds.
- Rollback must be time-bounded. A rollback plan that says "restore from backup if needed" without a documented time-to-rollback estimate and activation threshold is not a credible rollback plan.
- Go/no-go criteria must have a named decision authority. Go/no-go criteria documented but with no identified decision authority are not enforceable criteria.
- Do not fabricate mock run statistics or data quality metrics. Only classify findings the user has provided from their actual mock run reports or validation outputs.
- Evidence from official SAP sources takes precedence over memory or training data.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP Migration Cockpit docs, SAP Activate cutover methodology, or SAP Help Portal
- `user-provided evidence` — mock run reports, data quality validation outputs, cutover plan documents, go/no-go checklists, or reconciliation templates provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled

## Live-environment rules

**This skill does not touch live systems.** There is no credential, API call, RFC connection, LTMC session, or SAP system access in this skill's execution path. Users must supply mock run reports, cutover plan documents, data quality outputs, and written descriptions of their migration approach and readiness status for this skill to review.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — readiness gate taxonomy, assessment dimensions, cutover plan evaluation criteria, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, common readiness assessment mistakes, advisory boundary enforcement, when to push back.
- [Official sources](references/official-sources.md) — SAP Migration Cockpit, LTMC/LTMOM, SAP Activate cutover methodology, data validation and reconciliation guidance.

## Response minimum

Return, at minimum:

- **Problem classification**: migration approach (SAP Migration Cockpit / LTMC / LTMOM / third-party ETL), migration object scope, and current readiness phase (pre-mock / mock run 1 complete / mock run 2 complete / dress rehearsal complete).
- **Evidence used**: documentation-based / user-provided evidence / inference.
- **Risk level**: cutover-blocking / high-risk / medium-risk / low-risk per readiness gate assessed.
- **Recommended action**: gap remediation recommendation per readiness dimension, grounded in SAP Activate cutover methodology or Migration Cockpit guidance.
- **Refusal / escalation triggers**: refuse to assess execution readiness if no mock run results are available; escalate to live system discovery if migration object inventory is unknown; do not provide execution guidance under any circumstances.
- **Business impact**: go-live date risk, data integrity risk in production, rollback window implications, financial close impact.
- **Next verification step**: which readiness gate or mock run evidence must be provided before the program can be assessed as cutover-ready.
