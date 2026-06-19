# Official sources — SAP Release and Change Collision Review

Use this reference when grounding transport sequencing assessment, overtake and overwrite risk analysis, parallel project collision review, retrofit governance evaluation, downgrade protection assessment, and ChaRM or Cloud ALM change governance completeness review.

**Evidence level**: documentation-based (SAP Help Portal, SAP STMS documentation, SAP Solution Manager ChaRM documentation, SAP Cloud ALM application help). No live-system evidence is collected by this skill.

## SAP Transport Management System (STMS)

- SAP Transport Management System — Transport Routes and Import Queues
  https://help.sap.com/docs/ABAP_PLATFORM_NEW/4a368c163b08418890a406d413933ba/e28fbdf455c3421b8f4f32fdb5bc3e77.html
  source_owner: SAP SE
  topic_supported: STMS transport route configuration, import queue management, transport sequencing, object-level conflict detection, import mode configuration (sequential vs. sorted), system landscape definition
  why_needed: Primary reference for assessing transport sequencing correctness and import queue collision risk — defines the STMS configuration model used to evaluate whether transport routes, import order controls, and conflict detection capabilities are correctly configured
  evidence_level: primary
  last_verified: 2026-06-19

- SAP STMS — Downgrade Protection
  https://help.sap.com/docs/ABAP_PLATFORM_NEW/4a368c163b08418890a406d413933ba/fb20c1ea97e64d71a42d7f3def4c41c4.html
  source_owner: SAP SE
  topic_supported: Transport downgrade protection configuration at system and client level, detection of transports moving against the landscape import direction, emergency transport bypass procedure, downgrade protection error handling
  why_needed: Authoritative reference for downgrade protection assessment — defines what downgrade protection covers, how it is configured, and what bypass procedures are documented by SAP, used to classify downgrade protection gap findings and evaluate whether emergency transports correctly handle bypass authorization
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Solution Manager — Change and Release Management (ChaRM)

- ChaRM — Change Request Management Overview
  https://help.sap.com/docs/SAP_SOLUTION_MANAGER/56a0e6b74f7044099498e1fb5a4bed99/4e9b0c5d0ed7401de10000000a421937.html
  source_owner: SAP SE
  topic_supported: ChaRM change request types (normal change, urgent correction, emergency change), change record lifecycle, approval workflow configuration, transport-to-change-record linkage, change calendar and release cycle governance
  why_needed: Primary reference for ChaRM change governance assessment — defines the change record types, approval workflow, transport linkage model, and release cycle governance used to classify change record coverage, approval workflow, and urgent correction procedure findings
  evidence_level: primary
  last_verified: 2026-06-19

- ChaRM — Retrofit and Dual Landscape Management
  https://help.sap.com/docs/SAP_SOLUTION_MANAGER/56a0e6b74f7044099498e1fb5a4bed99/4a3e5c3b2d4f441be10000000a421937.html
  source_owner: SAP SE
  topic_supported: Retrofit procedure in ChaRM for dual landscape (parallel maintenance and main development lines), retrofit transport creation, retrofit sequencing, retrofit completeness tracking, dual landscape object integrity governance
  why_needed: Authoritative reference for retrofit governance assessment — defines the ChaRM retrofit procedure, transport chain requirements, and integrity controls used to evaluate whether the user's dual landscape retrofit strategy is correctly structured and whether retrofit backlogs represent an integrity risk
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Change Management

- Change Management in SAP Cloud ALM
  https://help.sap.com/docs/cloud-alm/applicationhelp/change-management
  source_owner: SAP SE
  topic_supported: Change record management in SAP Cloud ALM, change approval workflow, change types and lifecycle states, audit trail for change decisions, integration with transport management
  why_needed: Required to assess Cloud ALM change governance completeness for organizations transitioning from ChaRM — defines the change record structure, approval workflow, audit trail, and transport integration capabilities in Cloud ALM change management
  evidence_level: primary
  last_verified: 2026-06-19

- Deployment Management in SAP Cloud ALM
  https://help.sap.com/docs/cloud-alm/applicationhelp/deployment-management
  source_owner: SAP SE
  topic_supported: Transport deployment pipeline in SAP Cloud ALM, deployment scheduling, deployment approval gates, deployment-to-change-record traceability, deployment audit log
  why_needed: Required to assess transport-to-change-record traceability in Cloud ALM — defines how deployment management links transport requests to change records and approval decisions, used to evaluate whether the user's Cloud ALM deployment pipeline provides the governance controls needed to replace ChaRM transport linkage
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal, STMS documentation, ChaRM documentation, and SAP Cloud ALM change management documentation describe the designed transport management model, collision detection capabilities, downgrade protection configuration, retrofit procedure, and change governance workflow. They do not prove which transports are currently in the user's import queues, what objects are modified by the user's parallel transport tracks, whether the user's retrofit backlog is current, or whether the user's change records have 100% transport coverage. Users must supply transport sequencing plans, import queue descriptions, collision analysis outputs, ChaRM or Cloud ALM change record summaries, landscape architecture diagrams, and retrofit transport lists for concrete collision risk assessment.
