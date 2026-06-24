# Official sources — SAP EWM TM Logistics Execution Review

Use this reference when grounding warehouse process type assessment, wave and task management control review, slotting and bin assignment evaluation, freight order and carrier management analysis, dock and yard management control assessment, shipment execution exception review, and EWM/TM–S/4HANA integration quality analysis.

**Evidence level**: documentation-based (SAP Help Portal, SAP Extended Warehouse Management documentation, SAP Transportation Management documentation, SAP S/4HANA Logistics documentation). No live-system evidence is collected by this skill.

## EWM warehouse process types and process steps

- SAP Extended Warehouse Management — Warehouse Process Types (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-ewm/warehouse-process-types
  source_owner: SAP SE
  topic_supported: Warehouse process type configuration, process step design, document type assignment, activity area assignment, queue configuration for picking, packing, and putaway operations in SAP EWM
  why_needed: Primary reference for warehouse process type control review — defines the process type and process step model used to classify unauthorized goods movement path findings and confirmation bypass risks
  evidence_level: primary
  last_verified: 2026-06-19

## EWM wave and task management

- SAP Extended Warehouse Management — Wave Management (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-ewm/wave-management
  source_owner: SAP SE
  topic_supported: Wave template design, wave creation and release rules, grouping criteria, task type assignment and interleaving, task confirmation controls, labor management integration, and resource assignment in SAP EWM
  why_needed: Authoritative reference for wave release and task management control assessment — defines the wave template, task creation, and task confirmation model used to classify wave release SoD gaps and task confirmation control findings
  evidence_level: primary
  last_verified: 2026-06-19

## EWM slotting and bin management

- SAP Extended Warehouse Management — Slotting and Rearrangement (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-ewm/slotting-and-rearrangement
  source_owner: SAP SE
  topic_supported: Slotting profile configuration, storage type and section search strategy design, bin capacity management, mixed storage restrictions, quantity limits, and bin assignment optimization in SAP EWM
  why_needed: Defines the slotting and bin assignment control model — required to assess bin capacity governance, unauthorized bin usage risk, and goods location discrepancy findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP TM freight order and carrier management

- SAP Transportation Management — Freight Order Management
  https://help.sap.com/docs/SAP_TM/sap-transportation-management/freight-order-management
  source_owner: SAP SE
  topic_supported: Freight order document type design, carrier selection rule configuration (lane-based, ranking-based, cost-optimized), tender process governance, freight agreement and rate table management, carrier assignment authorization, subcontracting chain controls in SAP TM
  why_needed: Primary reference for freight order and carrier management control assessment — defines the freight order lifecycle, carrier selection model, and freight agreement design used to classify unauthorized carrier substitution and freight rate deviation findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP TM dock and yard management

- SAP Transportation Management — Dock and Yard Management
  https://help.sap.com/docs/SAP_TM/sap-transportation-management/dock-and-yard-management
  source_owner: SAP SE
  topic_supported: Dock appointment scheduling (inbound/outbound), dock door assignment governance, yard zone and location configuration, gate in/gate out check process, vehicle tracking integration, and goods arrival/departure traceability in SAP TM
  why_needed: Defines the dock and yard management control model — required to assess dock appointment governance gaps, untracked goods arrival and departure risk, and yard traceability findings
  evidence_level: primary
  last_verified: 2026-06-19

## EWM–S/4HANA integration

- SAP EWM Integration with SAP S/4HANA (On-Premise)
  https://help.sap.com/docs/SAP_S4HANA_ON-PREMISE/s4hana-ewm/integration-with-sap-s4hana
  source_owner: SAP SE
  topic_supported: EWM-to-S/4HANA goods movement integration (transfer order to MIGO posting), delivery document flow (inbound delivery from purchase order, outbound delivery from sales order), posting change order processing, physical inventory document integration, and integration error handling in SAP EWM embedded S/4HANA
  why_needed: Authoritative reference for EWM–S/4HANA integration quality assessment — defines the goods movement integration model and delivery document flow used to classify inventory accuracy gaps and undetected goods movement findings
  evidence_level: primary
  last_verified: 2026-06-19

## TM dangerous goods management

- SAP Transportation Management — Dangerous Goods Management
  https://help.sap.com/docs/SAP_TM/sap-transportation-management/dangerous-goods-management
  source_owner: SAP SE
  topic_supported: Dangerous goods classification configuration (UN number, hazard class, packing group), dangerous goods checks at freight order level, ADR/IMDG/IATA regulatory compliance controls, quantity limit enforcement, dangerous goods document generation, and driver/vehicle restriction enforcement in SAP TM
  why_needed: Defines the dangerous goods management control model — required to classify dangerous goods compliance gaps, missing classification checks, and document generation control deficiencies in freight execution
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and S/4HANA EWM/TM documentation describe the designed control model and configuration options for warehouse process types, wave management, slotting, freight order management, dock and yard management, and logistics integration. They do not prove what process types are active in the user's EWM system, whether wave release authorization is separated from task confirmation, whether carrier selection rules enforce freight agreement rates, or whether EWM-to-S/4HANA integration errors are being monitored. Users must supply configuration exports, wave analysis reports, inventory discrepancy data, freight exception logs, carrier assignment summaries, integration status reports, or written descriptions of their EWM and TM landscape for concrete assessment.
