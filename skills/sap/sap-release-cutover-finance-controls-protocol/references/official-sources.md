# Official sources — SAP Release Cutover Finance Controls Protocol

Use this reference when grounding transport management governance, cutover readiness assessment, financial closing cockpit sequencing, revenue accounting configuration review, material ledger valuation impact assessment, and SOX IT general controls compliance framing.

**Evidence level**: documentation-based (SAP Help Portal, SAP Activate methodology, PCAOB AS 2201, NIST SP 800-53). No live-system evidence is collected by this skill.

## SAP Transport Management

- Transport Management in SAP ABAP Platform Cloud
  https://help.sap.com/docs/abap-platform-cloud/abap-platform-cloud/transport-management
  source_owner: SAP SE
  topic_supported: Transport Organizer, transport request lifecycle, import queue management, transport routes, change and transport system configuration
  why_needed: Primary reference for transport management model — defines the transport request, import queue, and transport route concepts used to assess transport collision risk and import sequencing
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Activate — Cutover Management

- Cutover Management in SAP Activate
  https://help.sap.com/docs/sap-activate/sap-activate-methodology/cutover-management
  source_owner: SAP SE
  topic_supported: SAP Activate cutover methodology, cutover checklist structure, go/no-go decision criteria, cutover execution and rollback guidance, hypercare readiness
  why_needed: Primary reference for cutover readiness governance — defines the checklist structure, go/no-go criteria, and rollback requirements applied in this protocol
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Migration — Cutover Activities

- Cutover Activities in SAP S/4HANA On-Premise Migration Guide
  https://help.sap.com/docs/sap-s-4hana/sap-s-4hana-on-premise-migration-guide/cutover-activities
  source_owner: SAP SE
  topic_supported: S/4HANA migration cutover task sequence, technical and functional cutover steps, data migration sign-off requirements, production system switch activities
  why_needed: Required for S/4HANA migration projects — defines cutover task ordering and sign-off requirements that inform the cutover readiness checklist assessed by this protocol
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Finance — Financial Closing Cockpit

- Financial Closing Cockpit in SAP S/4HANA Finance
  https://help.sap.com/docs/sap-s-4hana-finance/sap-s-4hana-finance/financial-closing-cockpit
  source_owner: SAP SE
  topic_supported: Period-end close task sequencing, closing cockpit configuration, dependent task management, status reporting, user assignment for close activities
  why_needed: Primary reference for financial period-end close sequencing — required to assess whether a transport import or cutover event conflicts with close activities in progress
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Finance — Revenue Accounting and Reporting

- Revenue Accounting and Reporting in SAP S/4HANA Finance
  https://help.sap.com/docs/sap-s-4hana-finance/sap-s-4hana-finance/revenue-accounting-and-reporting
  source_owner: SAP SE
  topic_supported: IFRS 15 / ASC 606 revenue accounting, performance obligation management, contract modification handling, revenue recognition configuration, RAR integration with SD and FI
  why_needed: Required to assess revenue recognition impact of transports modifying RAR configuration, SD billing plan setup, or contract account mapping — defines the configuration objects that affect recognized revenue in the current period
  evidence_level: primary
  last_verified: 2026-06-19

## SAP S/4HANA Finance — Material Ledger

- Material Ledger in SAP S/4HANA Finance
  https://help.sap.com/docs/sap-s-4hana-finance/sap-s-4hana-finance/material-ledger
  source_owner: SAP SE
  topic_supported: Material ledger configuration, actual costing, standard cost estimate, moving average price, parallel valuation, period-end closing for material ledger
  why_needed: Required to assess inventory valuation impact of transports modifying material ledger configuration, costing variants, or valuation area assignment — defines the configuration objects that affect balance sheet inventory values
  evidence_level: primary
  last_verified: 2026-06-19

## SOX IT General Controls

- PCAOB Auditing Standard AS 2201 — An Audit of Internal Control Over Financial Reporting
  https://www.pcaob.org/standards/auditing/as2201
  source_owner: PCAOB
  topic_supported: IT general controls evaluation, change management controls, access controls, IT operations controls, deficiency classification (control deficiency, significant deficiency, material weakness)
  why_needed: Primary regulatory reference for SOX IT general controls obligations — AS 2201 defines the change management control requirements that govern transport approval, freeze exception, and production change documentation requirements in this protocol
  evidence_level: primary (regulatory)
  last_verified: 2026-06-19

## IT Change Management

- NIST Special Publication 800-53 Rev 5 — CM (Configuration Management) and SA (System and Services Acquisition) Control Families
  https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final
  source_owner: NIST
  topic_supported: CM-3 (Configuration Change Control), CM-4 (Impact Analyses), CM-9 (Configuration Management Plan), SA-10 (Developer Configuration Management) — change approval, impact analysis, and audit trail requirements
  why_needed: Secondary framework reference for change management control requirements — NIST CM-3 and CM-4 directly support the transport approval, financial impact assessment, and rollback documentation requirements in this protocol
  evidence_level: secondary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the designed capabilities of transport management, Financial Closing Cockpit, Revenue Accounting and Reporting, and Material Ledger. It does not prove which transports are in the user's production import queue, what the current period-end close status is in the user's system, or whether specific configuration objects in the user's system are affected by a pending transport. Users must supply transport manifests, QA import results, financial impact assessments, and readiness checklist outputs for concrete protocol execution.
