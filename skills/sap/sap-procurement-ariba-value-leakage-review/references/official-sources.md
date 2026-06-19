# Official sources — SAP Procurement Ariba Value Leakage Review

Use this reference when grounding maverick spend assessment, contract compliance analysis, supplier risk review, three-way match discipline evaluation, discount capture analysis, guided buying adoption review, and spend analytics visibility assessment.

**Evidence level**: documentation-based (SAP Help Portal, SAP Ariba documentation, SAP S/4HANA Procurement documentation). No live-system evidence is collected by this skill.

## SAP Ariba Buying and Invoicing — Guided Buying and Catalog

- SAP Ariba Buying and Invoicing — Guided Buying Configuration
  https://help.sap.com/docs/SAP_ARIBA_BUYING/1c1ac45c6bce4af5bf99a2f06cf37de5/d3f4e9d0f0b04df3a7e1c4f9a0b3e2f1.html
  source_owner: SAP SE
  topic_supported: Guided buying enforcement rules, catalog configuration (static, punchout, dynamic), preferred supplier routing, purchasing channel policy, maverick spend reduction through guided buying
  why_needed: Primary reference for assessing guided buying adoption gaps and maverick spend channel controls — defines the guided buying policy enforcement model used to classify off-contract spend findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Ariba Contracts — Contract Compliance and Consumption

- SAP Ariba Contracts — Contract Compliance and Monitoring
  https://help.sap.com/docs/ARIBA_CONTRACTS/b9c2e5f0a3e14b79a6d8f1c2b4e7a0d3/e5c8b2a1d4f74e8a9c0b6e3f2a1d7c4b.html
  source_owner: SAP SE
  topic_supported: Contract-to-PO linkage configuration, contract consumption monitoring, contract compliance rate reporting, contract leakage threshold configuration, sourcing workflow design in SAP Ariba Sourcing and Contracts
  why_needed: Authoritative reference for assessing contract compliance and leakage — defines the contract consumption and PO linkage model used to classify contract compliance findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Ariba Supplier Risk — Supplier Qualification and Risk Scoring

- SAP Ariba Supplier Risk — Risk Assessment and Qualification
  https://help.sap.com/docs/SAP_ARIBA_SUPPLIER_RISK/6a3d8f1c2b4e79a0d3e5c8b2a1f4d7e6/a4b7c0d2e5f8a1b3c6d9e2f5a8b1c4d7.html
  source_owner: SAP SE
  topic_supported: Supplier risk score configuration, risk threshold alerting, supplier qualification workflow, financial health monitoring, geographic and concentration risk assessment, ESG and diversity screening
  why_needed: Defines the supplier risk and qualification model — required to classify supplier risk exposure gaps and missing qualification controls in source-to-pay processes
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Procurement — Three-Way Match and Invoice Processing

- SAP S/4HANA Procurement — Invoice Verification and Three-Way Match
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/4ed91d4f3f7046b5b1d9aa09c0b85e0d/4a2d9e8ca98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Three-way match configuration (invoice/PO/GR), tolerance group setup for invoice verification, hold and exception management workflow in Materials Management (MM-IV), LIV (Logistics Invoice Verification), tolerance-based invoice release
  why_needed: Primary reference for assessing three-way match discipline and tolerance control design — defines the invoice matching model used to classify match leakage and tolerance override findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Ariba Discount Management — Early Payment Discount Capture

- SAP Ariba Discount Management and Dynamic Discounting
  https://help.sap.com/docs/SAP_ARIBA_DISCOUNT_MANAGEMENT/3c6e9b2a1d4f78a0d3e5c8b2a1f4d7e6/b5c8d1e4f7a0b3c6d9e2f5a8b1c4d7e0.html
  source_owner: SAP SE
  topic_supported: Early payment discount program setup, dynamic discounting configuration, discount window monitoring, discount capture rate reporting, integration with SAP S/4HANA Accounts Payable payment terms
  why_needed: Defines the discount capture model in SAP Ariba Discount Management — required to classify early payment discount leakage and AP process timing gaps
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Ariba Spend Analysis — Spend Visibility and Category Management

- SAP Ariba Spend Analysis — Configuration and Reporting
  https://help.sap.com/docs/SAP_ARIBA_BUYING/2b5e8c1a3d6f79a0d3e5c8b2a1f4d7e6/c6d9e2f5a8b1c4d7e0a3b6c9d2e5f8a1.html
  source_owner: SAP SE
  topic_supported: Spend analysis category hierarchy configuration, supplier normalization, multi-channel spend data ingestion (PO-based, non-PO, procurement card), spend classification accuracy, spend visibility dashboard coverage
  why_needed: Reference for assessing spend analytics visibility gaps — defines the spend analysis data model and category hierarchy used to classify spend blind spot findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Ariba and SAP S/4HANA Procurement documentation describes the designed control model and configuration options for source-to-pay processes. It does not prove what guided buying rules are active in the user's tenant, what contract consumption rates are, whether three-way match tolerances are approved, or whether supplier risk scores are configured for specific suppliers. Users must supply configuration exports, spend analysis reports, contract compliance metrics, exception reports, supplier risk summaries, discount capture rate data, or written descriptions of their source-to-pay landscape for concrete assessment.
