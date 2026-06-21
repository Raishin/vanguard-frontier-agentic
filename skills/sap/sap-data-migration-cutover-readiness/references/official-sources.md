# Official sources — SAP Data Migration and Cutover Readiness Review

Use this reference when grounding SAP Migration Cockpit configuration assessment, LTMC/LTMOM approach review, cutover methodology guidance, data quality validation interpretation, and go/no-go criteria evaluation.

**Evidence level**: documentation-based (SAP Help Portal, SAP Activate portal). No live-system evidence is collected by this skill.

## SAP Migration Cockpit (LTMC/LTMOM)

- SAP Migration Cockpit — overview
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/migration-cockpit-overview
  source_owner: SAP SE
  topic_supported: SAP Migration Cockpit (LTMC / LTMOM) architecture; migration project setup; migration object types; staging table approach vs. direct transfer approach; scope and prerequisites
  why_needed: Primary reference for assessing whether the user's Migration Cockpit configuration and migration object scope are appropriate for their business data migration requirements
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Migration Cockpit — migration objects
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/migration-objects
  source_owner: SAP SE
  topic_supported: Supported migration objects (business partners, materials, open items, balances, purchase orders, sales orders, assets); object dependencies and sequencing; data mapping templates
  why_needed: Defines which business objects can be migrated via SAP Migration Cockpit and what sequencing dependencies exist — used to assess migration object scope completeness and sequencing risk
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Migration Cockpit — staging tables
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/staging-tables
  source_owner: SAP SE
  topic_supported: Staging table approach: staging table structure, data load into staging tables, field mapping and transformation, staging table validation
  why_needed: Defines the staging table approach used by most LTMC migration projects — required to assess whether staging data load and mapping steps are complete before productive migration
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Migration Cockpit — simulation and validation
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/simulation-and-validation
  source_owner: SAP SE
  topic_supported: Migration run phases (Read, Convert, Write); validation run (dry run / simulation) before productive run; error log interpretation; error correction and re-run approach
  why_needed: Grounds assessment of mock run completeness — specifically whether the user has completed validation runs for each migration object and resolved errors before productive migration
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA Cloud data migration

- SAP S/4HANA Cloud — data migration to SAP S/4HANA Cloud
  https://help.sap.com/docs/sap-s4hana-cloud/sap-s4hana-cloud/data-migration-to-sap-s4hana-cloud
  source_owner: SAP SE
  topic_supported: Data migration approach for S/4HANA Cloud Public Edition; cloud-specific constraints on migration object support; difference between on-premise Migration Cockpit and cloud Migration Cockpit
  why_needed: Required when assessing readiness for S/4HANA Cloud Public Edition migration — the cloud variant has a different tool interface and more restricted migration object catalog than the on-premise LTMC
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Activate cutover methodology

- SAP Activate — cutover planning
  https://help.sap.com/docs/sap-activate/sap-activate-methodology/cutover-planning
  source_owner: SAP SE
  topic_supported: SAP Activate Deploy phase cutover deliverables: cutover plan structure, mock run sequence (mock run 1 → mock run 2 → dress rehearsal), go/no-go gate definition, hypercare plan
  why_needed: Authoritative methodology reference for assessing whether the user's cutover plan structure and mock run sequence meet SAP Activate recommended standards
  evidence_level: primary
  last_verified: 2026-06-19

## Reconciliation and financial data validation

- S/4HANA Cloud — reconciliation of migrated data
  https://help.sap.com/docs/sap-s4hana-cloud/sap-s4hana-cloud/reconciliation
  source_owner: SAP SE
  topic_supported: Financial data migration reconciliation: G/L balance migration approach, open item migration validation, asset accounting migration cutover, reconciliation account balancing
  why_needed: Critical reference for assessing financial reconciliation strategy completeness — financial data migration is the highest-risk migration object category with zero tolerance for balance discrepancies
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and SAP Activate documentation describe migration tool capabilities, recommended cutover methodology, and validation approaches. They do not prove which migration objects the user's program has scoped, what error rates appear in the user's actual mock runs, whether the user's data quality meets the thresholds needed for go-live, or whether the user's rollback plan is technically executable in their specific landscape. Users must supply mock run reports, validation outputs, cutover plan documents, and data quality results for concrete readiness assessment.
