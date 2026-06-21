# Workflow and output contract — SAP Data Migration and Cutover Readiness Review

Use this reference for all readiness gate assessment, cutover plan evaluation, and output formatting.

## Migration approach taxonomy

| Approach | Description | Readiness considerations |
|----------|-----------|------------------------|
| `sap-migration-cockpit-staging` | LTMC/LTMOM staging table approach: data extracted to staging tables, transformed, validated in simulation run before productive run | Requires completed validation run per migration object; error rate thresholds must be defined and met |
| `sap-migration-cockpit-direct` | Direct transfer approach (on-premise to S/4HANA on same system) | Available only for specific brownfield scenarios; fewer migration objects supported |
| `sap-migration-cockpit-cloud` | Cloud variant for S/4HANA Cloud Public Edition: reduced migration object catalog, SaaS-delivered tool | Cloud-specific object catalog limitations must be assessed against scope |
| `third-party-etl` | Customer or partner ETL tooling (e.g., Syniti, SNP, or custom extraction/load) outside SAP Migration Cockpit | Requires proof of SAP certification or explicit customer acceptance of unsupported tool risk |

## Readiness gate taxonomy

Each gate is assessed as: `PASS` / `PARTIAL` / `FAIL` / `NOT-YET-ASSESSED`

| Gate | Minimum pass criteria | Common failure modes |
|------|----------------------|---------------------|
| `migration-object-scope` | All business objects required for go-live are in scope with confirmed Migration Cockpit or ETL support | Missing migration object types; financial objects not scoped; master data dependencies not sequenced |
| `data-quality-thresholds` | Error rate thresholds are explicitly defined per object type; last validation run results are below threshold | No defined thresholds; threshold met for master data but not transactional data; error rate not measured |
| `mock-run-completion` | At minimum mock run 1 completed with documented results; dress rehearsal ideally complete before go-live assessment | No mock runs completed; mock runs completed but results not formally documented; open errors unresolved |
| `cutover-plan-structure` | Cutover plan includes: task list with owner and duration, sequencing with dependencies, critical path identified, system downtime window defined | Generic task list with no durations; no critical path; no defined downtime window |
| `rollback-plan-viability` | Rollback procedure documented with: activation threshold, time-to-rollback estimate, named decision authority, technical steps | Rollback is "restore from backup" with no time estimate; no named authority to trigger rollback |
| `reconciliation-strategy` | Reconciliation checks defined for: financial totals (G/L balances, open items), master data record counts, inventory quantities; sign-off process defined | No financial reconciliation checks; reconciliation checks defined but no sign-off authority named |
| `go-no-go-criteria` | Go/no-go criteria are measurable (specific thresholds), named decision authority identified, criteria formally documented | Criteria exist but are subjective ("quality is acceptable"); no named decision authority; criteria not formally approved |
| `hypercare-plan` | Hypercare period defined with: support coverage, escalation path, rollback activation window, and business process monitoring | No hypercare plan; hypercare period too short for financial close or period-end processing |

## Mock run maturity model

| Stage | Mock run | Evidence required | Readiness signal |
|-------|---------|-----------------|-----------------|
| Pre-mock | Not yet started | None | Not ready for any cutover assessment |
| Mock run 1 | First end-to-end trial with production-representative data | Migration log with error counts per object; identified gaps | Initial risk surface; remediation plan required |
| Mock run 2 | Second trial after mock run 1 gaps resolved | Updated migration log showing error reduction; duration improvement | Remediation effectiveness confirmed; cutover duration more reliable |
| Dress rehearsal | Final cutover simulation including all cutover tasks in sequence | Full cutover task log with actual duration; all reconciliation checks executed | Highest confidence readiness signal; remaining gaps are go/no-go blockers |

## Cutover plan assessment criteria

A complete cutover plan must include for each task:

- Task description and owner (named individual, not just role)
- Duration estimate (in hours/minutes)
- Predecessor and successor dependencies
- Go/no-go decision point (which tasks are critical path vs. parallel)
- System downtime window confirmed with business stakeholders
- Escalation contact if task is delayed beyond buffer

Assess the user's cutover plan against these criteria. Flag missing elements as readiness gaps.

## Rollback plan viability criteria

A viable rollback plan must include:

- Activation threshold: objective criteria that trigger rollback (e.g., "if reconciliation error rate > 0.5% on financial open items after productive run")
- Time-to-rollback: estimated duration from trigger decision to system available for legacy operation
- Named decision authority: who can authorize rollback (named individual + backup)
- Technical steps: what happens technically to restore legacy system availability (point-in-time restore, database fallback, transport reversal)
- Business impact of rollback: what business processes must restart; what data entered during production downtime window is lost

## Workflow

1. **Identify migration approach** — confirm tooling: Migration Cockpit (staging/direct/cloud), third-party ETL, or hybrid.
2. **Assess migration object scope** — confirm all required business objects are scoped and sequencing dependencies are documented.
3. **Evaluate data quality gates** — confirm thresholds are defined and last validation run results meet or exceed thresholds.
4. **Review mock run maturity** — confirm mock run stage (pre-mock / mock run 1 / mock run 2 / dress rehearsal) and assess documented results.
5. **Evaluate cutover plan structure** — assess task list completeness, duration estimates, critical path, and downtime window.
6. **Assess rollback plan viability** — check against viability criteria above.
7. **Review reconciliation strategy** — confirm financial, master data, and inventory reconciliation checks are defined with sign-off process.
8. **Validate go/no-go criteria** — confirm criteria are measurable, formally documented, and have named decision authority.
9. **Assign overall readiness posture** — `cutover-ready` (all gates PASS) / `conditional` (minor gaps with mitigation) / `not-ready` (one or more gates FAIL or cutover-blocking gap).
10. **Return output** per the output contract below.

## Output contract

Return:

1. Migration approach classification and current readiness phase (mock run stage)
2. Evidence label per dimension (documentation-based / user-provided evidence / inference)
3. Readiness gate assessment table: gate name, status (PASS/PARTIAL/FAIL/NOT-YET-ASSESSED), gap description
4. Overall readiness posture: cutover-ready / conditional / not-ready
5. Risk level per gate (cutover-blocking / high / medium / low)
6. Prioritized gap remediation recommendations with SAP Activate or Migration Cockpit reference
7. Escalation trigger if live system data or additional mock run evidence is needed before readiness can be assessed
8. Explicit advisory boundary reminder: this review does not authorize, execute, or assist in executing any migration or cutover action
