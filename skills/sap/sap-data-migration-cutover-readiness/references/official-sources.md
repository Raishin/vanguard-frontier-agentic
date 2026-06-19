# Official sources — SAP Data Migration and Cutover Readiness Review

Use this reference when grounding SAP Migration Cockpit configuration assessment, LTMC/LTMOM approach review, cutover methodology guidance, data quality validation interpretation, and go/no-go criteria evaluation.

**Evidence level**: documentation-based (SAP Help Portal, SAP Activate portal). No live-system evidence is collected by this skill.

## SAP Migration Cockpit (LTMC/LTMOM)

- SAP Migration Cockpit — overview and getting started
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/d1c4d824383f4aefba12efafb36e5a01/96612a29edb24c9aaa4399e7bce47cde.html
  source_owner: SAP SE
  topic_supported: SAP Migration Cockpit (LTMC / LTMOM) architecture; migration project setup; migration object types; staging table approach vs. direct transfer approach; scope and prerequisites
  why_needed: Primary reference for assessing whether the user's Migration Cockpit configuration and migration object scope are appropriate for their business data migration requirements
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Migration Cockpit — migration object catalog and supported business objects
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/d1c4d824383f4aefba12efafb36e5a01/22a8f01cdd7847e58ab2c2bc9b21ae34.html
  source_owner: SAP SE
  topic_supported: Supported migration objects (business partners, materials, open items, balances, purchase orders, sales orders, assets); object dependencies and sequencing; data mapping templates
  why_needed: Defines which business objects can be migrated via SAP Migration Cockpit and what sequencing dependencies exist — used to assess migration object scope completeness and sequencing risk
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Migration Cockpit — validation run and error handling
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/d1c4d824383f4aefba12efafb36e5a01/9a8a7c2e2fa64e3f857c7fa3f2f9a0e4.html
  source_owner: SAP SE
  topic_supported: Migration run phases (Read, Convert, Write); validation run (dry run) before productive run; error log interpretation; error correction and re-run approach
  why_needed: Grounds assessment of mock run completeness — specifically whether the user has completed validation runs for each migration object and resolved errors before productive migration
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA Cloud data migration

- SAP S/4HANA Cloud — data migration using Migration Cockpit
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/9b73e07de3794f5f9d6f3a69c498c8fc.html
  source_owner: SAP SE
  topic_supported: Data migration approach for S/4HANA Cloud Public Edition; cloud-specific constraints on migration object support; difference between on-premise Migration Cockpit and cloud Migration Cockpit
  why_needed: Required when assessing readiness for S/4HANA Cloud Public Edition migration — the cloud variant has a different tool interface and more restricted migration object catalog than the on-premise LTMC
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Activate cutover methodology

- SAP Activate — cutover planning and execution in the Deploy phase
  https://help.sap.com/docs/SAP_ACTIVATE/80d20672e1e74bde9f0c7f84cda1e3a6/3c3d2e2f37ab4f3bb4adf4b12b9b2f0c.html
  source_owner: SAP SE
  topic_supported: SAP Activate Deploy phase cutover deliverables: cutover plan structure, mock run sequence (mock run 1 → mock run 2 → dress rehearsal), go/no-go gate definition, hypercare plan
  why_needed: Authoritative methodology reference for assessing whether the user's cutover plan structure and mock run sequence meet SAP Activate recommended standards
  evidence_level: primary
  last_verified: 2026-06-19

## Reconciliation and financial data validation

- S/4HANA migration — financial data reconciliation and balance migration
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/5b7c2e3f8abc4b5e9a2d1c6f8e3b0d74.html
  source_owner: SAP SE
  topic_supported: Financial data migration reconciliation: G/L balance migration approach, open item migration validation, asset accounting migration cutover, reconciliation account balancing
  why_needed: Critical reference for assessing financial reconciliation strategy completeness — financial data migration is the highest-risk migration object category with zero tolerance for balance discrepancies
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and SAP Activate documentation describe migration tool capabilities, recommended cutover methodology, and validation approaches. They do not prove which migration objects the user's program has scoped, what error rates appear in the user's actual mock runs, whether the user's data quality meets the thresholds needed for go-live, or whether the user's rollback plan is technically executable in their specific landscape. Users must supply mock run reports, validation outputs, cutover plan documents, and data quality results for concrete readiness assessment.
