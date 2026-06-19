# Official sources — SAP Manufacturing Execution Risk Review

Use this reference when grounding production order governance assessment, capacity planning and scheduling control review, MRP exception management analysis, shop-floor integration quality evaluation, quality management integration control assessment, backflush and goods movement governance review, and manufacturing SoD classification.

**Evidence level**: documentation-based (SAP Help Portal, SAP Production Planning documentation, SAP S/4HANA Manufacturing documentation, SAP Digital Manufacturing documentation, SAP Quality Management documentation). No live-system evidence is collected by this skill.

## SAP PP production order governance

- SAP Production Planning and Control — Production Orders (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/7a4f81cd8a6e42fdad22c35dc9aa9d18/4a3e5f7ca98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Production order document type and order type parameter configuration, scheduling parameter design, order release controls (availability check at release, missing parts list), milestone confirmation design, automatic goods receipt at order confirmation, goods issue control, and production order authorization objects in SAP S/4HANA On-Premise
  why_needed: Primary reference for production order governance assessment — defines the order type parameter, release control, and confirmation model used to classify production order authorization and release control findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP PP capacity planning and scheduling

- SAP Production Planning — Capacity Planning (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/7a4f81cd8a6e42fdad22c35dc9aa9d18/4a3f6e8da98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Work center capacity configuration (available capacity versions, shift definitions, capacity utilization), scheduling mode selection (finite scheduling, infinite scheduling), capacity leveling strategy profile design, leveling horizon, and capacity overload escalation in SAP S/4HANA On-Premise
  why_needed: Authoritative reference for capacity planning and scheduling control assessment — defines the work center capacity model, scheduling mode, and leveling strategy used to classify overloaded work center and unrestricted scheduling findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP MRP exception management

- SAP MRP — Exception Messages and Planning (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/7a4f81cd8a6e42fdad22c35dc9aa9d18/4a4a1f9ea98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: MRP type configuration (MRP type PD, VB, time-phased planning), planning horizon and firming horizon design, exception message categorization (10: bring forward, 20: reschedule in, 30: reschedule out, 50: increase quantity, 60: cancel process), MRP exception monitoring, and exception resolution governance in SAP S/4HANA On-Premise
  why_needed: Defines the MRP exception message model and planning horizon design — required to classify unresolved exception backlogs, over-procurement risk, and material shortage findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Digital Manufacturing (DM/MES) shop-floor integration

- SAP Digital Manufacturing — Integration with SAP S/4HANA
  https://help.sap.com/docs/SAP_DIGITAL_MANUFACTURING/57c1de85c3834d0e8cd25f4b35e39703/4b6e1f0fa98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Production order release and download to SAP Digital Manufacturing (MES), operation confirmation feedback from MES to SAP PP (OData/IDoc), material consumption and component issue confirmation, goods movement integration (GR against production order), and integration error monitoring and reprocessing in SAP Digital Manufacturing
  why_needed: Primary reference for DM/MES-to-SAP-PP shop-floor integration quality assessment — defines the order download, confirmation feedback, and goods movement integration model used to classify shop-floor integration failures and production order status gap findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Quality Management integration with PP

- SAP Quality Management — Integration with Production (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/93f8d7736f4f4ce68dece1c2cf4cd218/4a2d4e5ca98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Inspection lot creation configuration (inspection types 01, 03, 04, 08), inspection plan assignment and operation coverage, quality notification creation and routing, usage decision authorization (UD code, acceptance and rejection decisions), stock transfer from quality inspection to unrestricted use, and quality hold governance in SAP S/4HANA On-Premise
  why_needed: Authoritative reference for QM integration control assessment — defines the inspection lot, usage decision, and quality hold model used to classify quality hold bypass findings and usage decision authorization gaps
  evidence_level: primary
  last_verified: 2026-06-19

## SAP PP backflush and goods movement configuration

- SAP Production Planning — Backflushing and Goods Movements (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/7a4f81cd8a6e42fdad22c35dc9aa9d18/4a4b2f0fa98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Automatic goods receipt (backflush) configuration in production order type parameters, component backflush (BOM item-level flush setting), milestone confirmation and component issue design, movement type 261 (goods issue for production order) and 101 (goods receipt against production order) authorization, and goods movement control in SAP S/4HANA On-Premise
  why_needed: Defines the backflush and goods movement control model for manufacturing — required to classify uncontrolled component consumption, backflush SoD gaps, and phantom goods movement findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Quality Management — usage decision and regulated environments

- SAP Quality Management — Usage Decision and Stock Posting (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/93f8d7736f4f4ce68dece1c2cf4cd218/4a3c3e4ba98611d1e10000000a42189b.html
  source_owner: SAP SE
  topic_supported: Usage decision configuration (UD code assignment, automatic and manual usage decisions, follow-on functions), stock posting after usage decision (transfer to unrestricted use, blocked stock, returns), GMP and regulated industry requirements for electronic records and electronic signatures (FDA 21 CFR Part 11 context) in SAP QM, and dual-control for usage decision in regulated manufacturing
  why_needed: Defines the usage decision and stock posting control model — required to classify quality hold bypass findings, dual-control gaps in regulated manufacturing environments, and unauthorized release of non-conforming materials
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and S/4HANA Manufacturing documentation describe the designed control model and configuration options for production order governance, MRP, capacity planning, shop-floor integration, QM integration, and backflush. They do not prove what order type parameters are active in the user's system, whether availability checks are enforced at order release, whether inspection lots are being created for the relevant operations, whether DM/MES integration errors are being monitored, or whether backflush authorization is separated from goods movement adjustment authority. Users must supply production order status reports, MRP exception aging data, capacity utilization summaries, QM inspection lot reports, shop-floor integration status exports, backflush configuration summaries, or written descriptions of their manufacturing execution landscape for concrete assessment.
