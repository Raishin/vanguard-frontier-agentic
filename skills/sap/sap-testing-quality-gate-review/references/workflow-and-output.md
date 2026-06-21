# Workflow and output contract — SAP Testing and Quality Gate Review

Use this reference for all quality gate assessment, risk tier classification, defect governance evaluation, and output formatting.

## Assessment dimension taxonomy

| Dimension | Description |
|-----------|-------------|
| `test-scope-risk-prioritization` | Coverage of business processes across test types; risk tier assignment rationale; high-risk process test case density |
| `cloud-alm-test-management` | Test plan structure per phase; test case library completeness; defect integration; execution tracking |
| `automation-coverage` | Tricentis/CBTA automation coverage percentage; automation maintenance status; regression suite readiness for target release |
| `regression-strategy` | Regression scope definition; regression trigger conditions; transport-to-regression gating; baseline maintenance |
| `test-data-management` | Test data availability per phase; production data masking and anonymization; data refresh cadence |
| `defect-management` | Severity and priority classification consistency; defect-to-transport traceability; backlog governance; exit criteria enforcement |
| `entry-exit-criteria` | Measurability of criteria; formal documentation; named sign-off authority per phase |

## Test phase sequence

| Phase | Typical SAP Activate stage | Key gate concerns |
|-------|--------------------------|-------------------|
| `unit-testing` | Realize (early) | Individual configuration object correctness; developer-owned; pass rate tracking |
| `string-testing` | Realize (mid) | End-to-end process strand coverage for integrated configuration; cross-object dependencies |
| `integration-testing` | Realize (late) | Cross-system interface testing; integration scenario coverage; error handling verification |
| `regression-testing` | Realize and Deploy | Baseline coverage for unaffected processes; automation coverage; transport-triggered execution |
| `uat` | Deploy | Business user sign-off; risk-based scope; measurable exit criteria; formal phase acceptance |
| `performance-testing` | Deploy (if in scope) | Volume and peak load testing; response time thresholds; infrastructure sizing validation |

## Risk tier model

| Tier | Assignment criteria | Required test coverage |
|------|--------------------|-----------------------|
| `tier-1-critical` | Financial posting processes, payroll, regulatory reporting, cross-system integration on critical path | Full test case coverage; automation required for regression; UAT sign-off mandatory |
| `tier-2-high` | Core order-to-cash, procure-to-pay, inventory management, manufacturing execution | High test case density; automation recommended; UAT coverage expected |
| `tier-3-medium` | Supporting processes, reporting, non-critical interfaces | Moderate test case coverage; manual regression acceptable; UAT sampling |
| `tier-4-low` | Configuration-only changes, static master data, informational reports | Smoke test or focused check; no UAT required unless stakeholder-requested |

## Quality gate status values

Each gate is assessed as: `PASS` / `PARTIAL` / `FAIL` / `NOT-YET-ASSESSED`

| Gate | Minimum pass criteria | Common failure modes |
|------|-----------------------|---------------------|
| `test-scope-coverage` | All tier-1 and tier-2 processes have test cases; risk tier assignment documented | Missing tier-1 process in test scope; no documented rationale for risk tier assignment |
| `cloud-alm-test-management` | Test plans exist per phase; test cases linked to processes; defect records created from failures; execution progress tracked | Test cases exist in spreadsheets not Cloud ALM; defects not linked to test cases; no execution tracking |
| `automation-coverage` | Minimum 60% of regression test cases automated for tier-1 and tier-2 processes; scripts verified against target release | Automation scripts written for prior release and not updated; coverage percentage not quantified |
| `regression-gate` | Regression scope defined; regression triggered by transport release cycle; results reviewed before production transport approval | Regression run ad hoc with no defined scope; no transport-to-regression gating |
| `test-data-availability` | Test data exists at representative volume and variety for each phase; masking applied to any production data in lower environments | Test data too limited for integration testing; unmasked production data confirmed in lower environment |
| `defect-governance` | All open defects reviewed and classified before phase exit; critical and high defects resolved or formally risk-accepted; backlog prioritized | Unreviewed defect backlog at phase exit boundary; critical defects deferred without risk acceptance documentation |
| `exit-criteria-enforced` | Measurable exit criteria formally documented per phase; named sign-off authority; criteria enforced at gate | Exit criteria subjective or absent; sign-off authority not named; criteria not formally approved by project governance |

## Automation coverage maturity model

| Maturity level | Automation coverage | Characteristics |
|----------------|--------------------|--------------------|
| `no-automation` | 0% | All regression is manual; high regression execution time; significant regression cost per cycle |
| `initial` | < 30% | Partial automation; coverage concentrated on a subset of processes; incomplete regression without manual complement |
| `developing` | 30–60% | Meaningful automation coverage for tier-1 processes; gaps remain for tier-2 and integration scenarios |
| `regression-ready` | 60–80% | Sufficient automation to execute a complete regression cycle with targeted manual complement |
| `optimized` | > 80% | Comprehensive automation coverage; regression cycle is fast and reliable; manual testing focused on exploratory and UAT |

## Defect governance criteria

A governed defect backlog at phase exit must include, for each open defect:

- Severity and priority classification (critical / high / medium / low)
- Resolution status (open / in-progress / resolved / deferred with accepted risk)
- Responsible transport (where defect is attributable to a specific change)
- Target resolution date or accepted-risk record with approver name
- Phase exit impact assessment (blocking / non-blocking with mitigation)

## Workflow

1. **Identify program scope** — confirm SAP system types, transformation phase, and test types in scope (unit, string, integration, regression, UAT, performance).
2. **Assess test scope and risk prioritization** — confirm risk tier assignment and test case coverage allocation per tier.
3. **Review Cloud ALM test management configuration** — assess test plan structure, test case library, defect integration, and execution tracking.
4. **Evaluate automation coverage** — determine maturity level and confirm script currency against target release.
5. **Assess regression strategy** — confirm regression scope definition, trigger conditions, and transport gating.
6. **Review test data management** — confirm data availability, masking status, and refresh cadence.
7. **Evaluate defect governance** — assess classification consistency, backlog governance, and exit criteria enforcement.
8. **Validate entry and exit criteria** — confirm measurability, documentation, and named sign-off authority per phase.
9. **Assign overall quality gate posture** — `gates-met` (all gates PASS) / `conditional` (gaps with documented mitigation and accepted risk) / `gates-not-met` (one or more gates FAIL or quality-gate-blocking gap).
10. **Return output** per the output contract below.

## Output contract

Return:

1. Program scope and test phase sequence in scope
2. Evidence label per dimension (documentation-based / user-provided evidence / inference)
3. Quality gate assessment table: gate name, status (PASS/PARTIAL/FAIL/NOT-YET-ASSESSED), gap description
4. Risk tier coverage summary: tier-1 and tier-2 process test case coverage completeness
5. Automation maturity level and regression readiness assessment
6. Defect backlog governance assessment: open critical and high defects, resolution status, blocking classification
7. Overall quality gate posture: gates-met / conditional / gates-not-met
8. Risk level per gate (quality-gate-blocking / high / medium / low)
9. Prioritized gap remediation recommendations with SAP Cloud ALM or SAP Activate reference
10. Escalation trigger if unmasked production data, missing test execution evidence, or unreviewed critical defects require immediate action
11. Explicit advisory boundary reminder: this review does not execute tests, trigger automation, or authorize phase sign-off
