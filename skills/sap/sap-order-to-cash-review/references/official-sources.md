# Official sources — SAP Order-to-Cash Review

Use this reference when grounding order management configuration assessment, pricing procedure design review, credit management control evaluation, billing and revenue recognition analysis, order block management review, fulfillment exception assessment, DSO driver analysis, and revenue completeness control evaluation.

**Evidence level**: documentation-based (SAP Help Portal, SAP S/4HANA Sales and Distribution documentation, SAP S/4HANA Finance documentation). No live-system evidence is collected by this skill.

## SAP S/4HANA Sales — Order Management and Document Type Configuration

- SAP S/4HANA Sales — Sales Order Processing
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-sales/sales-order-processing
  source_owner: SAP SE
  topic_supported: Sales order document type configuration, partner function determination, organizational assignment (sales area, distribution channel, division, plant, shipping point), order incompletion procedure configuration (incompletion log), order entry validation controls
  why_needed: Primary reference for assessing order management configuration quality — defines the SAP SD sales order processing model, document type assignment, and incompletion procedure framework used to classify order management findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Sales — Pricing Procedure Configuration

- SAP S/4HANA Sales — Pricing and Conditions
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-sales/pricing-and-conditions
  source_owner: SAP SE
  topic_supported: Pricing procedure design and assignment (VKOA, condition type determination), mandatory condition type configuration, pricing control parameters (calculation type, condition class, condition category), manual price override authorization, statistical condition design, free goods condition type governance, pricing error detection
  why_needed: Authoritative reference for assessing pricing procedure design and override control — defines the SAP SD pricing model and condition type framework used to classify pricing control findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Sales — Credit Management

- SAP S/4HANA Sales — Credit Management
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-sales/credit-management
  source_owner: SAP SE
  topic_supported: Credit control area configuration, credit limit assignment and governance, automatic credit check types (static check, dynamic check, maximum document value check), credit block reason design (reason A and B), credit block release authorization, SoD in credit management (sales order entry vs. credit limit authorization vs. credit block release)
  why_needed: Primary reference for assessing credit management control design — defines the SAP S/4HANA credit management model, credit check configuration, and credit block authorization framework used to classify credit management findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Sales — Billing and Billing Plan Configuration

- SAP S/4HANA Sales — Billing
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-sales/billing
  source_owner: SAP SE
  topic_supported: Billing document type configuration (F1 order-based, F2 delivery-based, credit memo, debit memo), billing plan types (milestone billing plan, periodic billing plan), billing schedule design, SD–FI account determination for revenue and deferred revenue, billing due list management, unbilled delivery monitoring
  why_needed: Defines the SAP SD billing and billing plan configuration model — required to assess billing cycle adherence, revenue recognition timing, and unbilled delivery revenue completeness findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Sales — Availability Check and Fulfillment

- SAP S/4HANA Sales — Availability Check and Transfer of Requirements
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-sales/availability-check-and-transfer-of-requirements
  source_owner: SAP SE
  topic_supported: ATP availability check configuration (checking rule, checking scope, replenishment lead time), transfer of requirements (TOR) design, delivery split logic and split criteria, partial delivery tolerance configuration, goods issue (GI) posting timing controls, fulfillment exception management
  why_needed: Reference for assessing fulfillment exception management and availability check design — defines the ATP and TOR model used to classify delivery split, partial delivery tolerance, and fulfillment exception findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP FSCM — Dispute Management and Collections

- SAP FSCM Dispute Management — Invoice Dispute Tracking and Resolution
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/dispute-management
  source_owner: SAP SE
  topic_supported: SAP Dispute Management configuration (dispute case type, reason code, status profile, processor assignment), dunning procedure configuration (dunning levels, dunning amounts, dunning interest, dunning escalation), cash application process design, deductions management, Days Sales Outstanding (DSO) analytics and receivables monitoring
  why_needed: Defines the SAP FSCM Dispute Management and dunning model — required to classify DSO driver findings including dispute resolution cycle time, dunning process gaps, cash application delays, and deductions management deficiencies
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Finance — Revenue Accounting and Reporting

- SAP S/4HANA Finance — Revenue Accounting and Reporting
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-finance/revenue-accounting-and-reporting
  source_owner: SAP SE
  topic_supported: IFRS 15 / ASC 606 revenue contract configuration, performance obligation design, standalone selling price assignment, revenue recognition event triggers, deferred revenue management, contract modification handling in SAP Revenue Accounting and Reporting (RAR)
  why_needed: Reference for assessing revenue recognition completeness and IFRS 15 compliance controls — defines the performance obligation and recognition event model used to classify revenue timing and completeness findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP S/4HANA Sales and Finance documentation describes the designed configuration model and process options for Order-to-Cash. It does not prove what pricing conditions are active in the user's system, what credit block aging patterns exist, whether billing plan milestones are being met, or what the current DSO drivers are in the user's Accounts Receivable portfolio. Users must supply configuration descriptions, order block aging reports, billing schedule adherence data, credit management summaries, DSO analytics, dispute management reports, dunning procedure documentation, or written descriptions of their OTC landscape for concrete assessment.
