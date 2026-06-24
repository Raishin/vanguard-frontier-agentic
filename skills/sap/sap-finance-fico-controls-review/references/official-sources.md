# Official sources — SAP Finance FI-CO Controls Review

Use this reference when grounding document posting control assessment, validation and substitution rule review, period-end close governance analysis, Financial Close Cockpit task list evaluation, SoD classification in financial postings, parallel ledger configuration review, and intercompany control assessment.

**Evidence level**: documentation-based (SAP Help Portal, SAP S/4HANA Finance documentation). No live-system evidence is collected by this skill.

## Document posting controls and field status

- SAP S/4HANA Finance — General Ledger Accounting
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/general-ledger-accounting
  source_owner: SAP SE
  topic_supported: G/L account master data, document types, posting keys, field status groups, tolerance groups, document posting controls in SAP S/4HANA Cloud
  why_needed: Primary reference for G/L posting control configuration — defines document type and field status assignment model used to classify posting control findings
  evidence_level: primary
  last_verified: 2026-06-19

## Validations and substitutions

- Validations and Substitutions (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/validations-and-substitutions
  source_owner: SAP SE
  topic_supported: Validation rule configuration (OB28), substitution rule configuration (OBB1), callout logic, field-level rule activation, prerequisite and check logic for FI documents
  why_needed: Authoritative reference for assessing validation and substitution rule design correctness, bypass risk, and callout dependencies — required to classify validation findings
  evidence_level: primary
  last_verified: 2026-06-19

## Posting period management

- Posting Periods (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/posting-periods
  source_owner: SAP SE
  topic_supported: Fiscal year variant configuration, posting period variant assignment to company codes, posting period open/close controls (OB52), account type–level period restrictions
  why_needed: Defines the posting period management model — required to assess period open/close authorization controls and period management SoD
  evidence_level: primary
  last_verified: 2026-06-19

## Financial Close Cockpit

- Financial Closing Cockpit
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/financial-closing-cockpit
  source_owner: SAP SE
  topic_supported: Financial Close Cockpit task list design, task type definitions (manual task, program execution, workflow task), task dependency configuration, responsible user/group assignment, monitoring and escalation
  why_needed: Primary reference for evaluating FCC task list governance — defines task sequencing, dependency modeling, and responsibility assignment model used to classify close governance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Parallel ledgers and extension ledgers

- Parallel Accounting (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/parallel-ledgers
  source_owner: SAP SE
  topic_supported: Leading ledger, non-leading ledger, extension ledger configuration, ledger-specific posting rules, parallel valuation approaches (IFRS, US GAAP, local GAAP, tax ledger)
  why_needed: Authoritative source for parallel ledger design assessment — defines the ledger group, ledger assignment, and extension ledger model used to classify parallel accounting control gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Intercompany reconciliation

- Intercompany Reconciliation
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/intercompany-reconciliation
  source_owner: SAP SE
  topic_supported: Intercompany G/L clearing account assignment, intercompany reconciliation hub configuration, matching logic, intercompany posting controls in S/4HANA Cloud
  why_needed: Defines the intercompany reconciliation control model in S/4HANA — required to assess intercompany imbalance risk and elimination control gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Accrual and journal entry controls

- Accrual Management (S/4HANA On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/accrual-management
  source_owner: SAP SE
  topic_supported: Accrual object configuration, accrual posting and reversal controls, accrual engine integration with G/L, recurring entry management
  why_needed: Defines the accrual control model — required to classify manual accrual approval gaps and missing reversal controls in period-end processes
  evidence_level: primary
  last_verified: 2026-06-19

## Financial consolidation and group reporting context

- SAP S/4HANA Group Reporting — Document Splitting
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/document-splitting
  source_owner: SAP SE
  topic_supported: Intercompany matching rules, reconciliation status, intercompany difference classification, group reporting elimination context
  why_needed: Provides context for intercompany reconciliation findings at the group level — used to understand downstream impact of entity-level intercompany control gaps on group reporting
  evidence_level: supplemental
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and S/4HANA Finance documentation describe the designed control model and configuration options for FI-CO. They do not prove what validation rules are active in the user's system, what posting period variants are assigned, whether FCC task lists exist, or whether SoD exposures are present in the user's role design. Users must supply configuration exports, validation/substitution descriptions, period management summaries, Financial Close Cockpit task list exports, parallel ledger configuration descriptions, or written descriptions of their FI-CO control landscape for concrete assessment.
