# Official sources — SAP Treasury Cash Risk Review

Use this reference when grounding cash position and liquidity control assessment, bank account management governance review, in-house cash and payment factory dual-control evaluation, financial instrument configuration review, hedge and exposure management control analysis, payment fraud control assessment, and treasury SoD classification.

**Evidence level**: documentation-based (SAP Help Portal, SAP S/4HANA Treasury and Risk Management documentation, SAP Cash Management documentation). No live-system evidence is collected by this skill.

## Cash Management and liquidity planning

- SAP S/4HANA Cash Management (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/cash-management
  source_owner: SAP SE
  topic_supported: Cash position structure, liquidity item hierarchy, memo record management, bank statement processing (electronic bank statement, CAMT, BAI2, MT940), cash concentration, and short-term liquidity forecasting in SAP S/4HANA Cloud
  why_needed: Primary reference for assessing cash position accuracy controls, liquidity hierarchy design completeness, and bank statement processing automation — required to classify cash visibility and memo record governance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Bank Account Management (BAM)

- SAP Bank Account Management (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/bank-account-management
  source_owner: SAP SE
  topic_supported: Bank account master data management, bank account lifecycle (open, modify, close), signatory management, bank account authorization objects (TR_BANKI, TR_BANKA), four-eyes approval workflow for account maintenance in SAP S/4HANA On-Premise
  why_needed: Authoritative reference for assessing bank account governance completeness — defines the BAM approval workflow and authorization object model used to classify unauthorized bank account creation and modification findings
  evidence_level: primary
  last_verified: 2026-06-19

## In-house cash and payment factory

- In-House Cash (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/in-house-cash
  source_owner: SAP SE
  topic_supported: In-house bank configuration, internal account structure, payment request routing, netting and settlement controls, payment factory aggregation design, dual-control configuration in in-house cash payment processing
  why_needed: Defines the in-house cash and payment factory control model — required to assess dual-control gaps in payment aggregation and release, and to classify in-house bank authorization SoD findings
  evidence_level: primary
  last_verified: 2026-06-19

## Treasury and Risk Management — financial instruments

- SAP Treasury and Risk Management (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/treasury-and-risk-management
  source_owner: SAP SE
  topic_supported: Financial instrument type configuration (OTC derivatives, bonds, interest rate instruments, FX forwards, commodity derivatives), transaction flow design, position management, settlement processing controls, and TRM authorization object model
  why_needed: Primary reference for financial instrument configuration review — defines the instrument type, transaction flow, and position management model used to classify instrument configuration gaps and settlement control findings
  evidence_level: primary
  last_verified: 2026-06-19

## Hedge accounting and exposure management

- Hedge Management and Hedge Accounting (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/hedge-management-and-accounting
  source_owner: SAP SE
  topic_supported: Exposure determination source configuration, hedge designation and documentation workflow (IFRS 9 / IAS 39), hedge relationship type (fair value hedge, cash flow hedge, net investment hedge), effectiveness testing configuration, hedge accounting valuation controls in SAP S/4HANA Cloud
  why_needed: Authoritative reference for hedge documentation completeness and effectiveness testing design — required to classify hedge designation gaps, derecognition risk, and IFRS 9 compliance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Bank Communication Manager and payment fraud controls

- SAP Bank Communication Management (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/bank-communication-management
  source_owner: SAP SE
  topic_supported: Bank communication channel configuration, payment approval workflow (payment request status management, dual-control release), SWIFT connectivity governance, host-to-bank file security, payment status monitoring in SAP S/4HANA Cloud
  why_needed: Defines the payment approval and bank communication control model — required to classify single-user payment release authority findings, dual-control gaps, and bank channel security weaknesses
  evidence_level: primary
  last_verified: 2026-06-19

## Liquidity management and forecasting

- Liquidity Management (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/liquidity-management
  source_owner: SAP SE
  topic_supported: Short-term and medium-term liquidity planning configuration, liquidity item hierarchy design, liquidity forecast data source integration (AR, AP, Treasury deals), variance analysis and actuals comparison, liquidity planning approval and governance controls
  why_needed: Defines the liquidity management planning and governance model — required to classify liquidity forecast completeness gaps, missing liquidity item coverage, and unmonitored cash flow variances
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and S/4HANA Treasury and Cash Management documentation describe the designed control model and configuration options for TRM, Cash Management, BAM, IHC, and Bank Communication Manager. They do not prove what approval workflows are active in the user's system, whether BAM four-eyes approval is enforced, what authorization objects are assigned to treasury roles, or whether dual-control for payment release is configured. Users must supply authorization object exports, bank account master data reports, BAM configuration summaries, payment run logs, hedge documentation exports, or written descriptions of their treasury landscape for concrete assessment.
