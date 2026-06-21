# Official sources — SAP Guarded Transport Import

Use this reference when grounding SAP transport management commands, CTS+ API usage, STMS procedures, and Cloud TMS operations.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during steps 8 and 14 is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP Cloud Transport Management Service

- SAP Cloud Transport Management — what is SAP Cloud Transport Management
  https://help.sap.com/docs/cloud-transport-management/sap-cloud-transport-management/what-is-sap-cloud-transport-management
  source_owner: SAP SE
  topic_supported: Cloud TMS architecture, transport nodes, transport routes, import queue management
  why_needed: Authoritative source for Cloud TMS import sequence and transport node configuration used in steps 2, 7, 8, and 14
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud TMS — transport nodes and routes
  https://help.sap.com/docs/cloud-transport-management/sap-cloud-transport-management/transport-nodes-and-routes
  source_owner: SAP SE
  topic_supported: Transport node types (source, target, virtual), transport route configuration, import queue management per node
  why_needed: Defines the node and route model — required to assess blast radius (step 10) and target system scope (step 2 and 7)
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud TMS — importing transport requests
  https://help.sap.com/docs/cloud-transport-management/sap-cloud-transport-management/importing-transport-requests
  source_owner: SAP SE
  topic_supported: Import process flow, import log interpretation, error handling and rollback in Cloud TMS
  why_needed: Grounds step 15 (verify) and step 11 (rollback plan) with Cloud TMS-specific recovery procedures
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud TMS — Transport Management System integration
  https://help.sap.com/docs/cloud-transport-management/sap-cloud-transport-management/transport-management-system-integration
  source_owner: SAP SE
  topic_supported: Integration of Cloud TMS with on-premise STMS, CTS+ configuration, RFC destination setup for hybrid landscapes
  why_needed: Authoritative reference for hybrid landscape scenarios where Cloud TMS fronts an ABAP STMS backend — relevant for steps 2 and 7 target classification
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud TMS — segregation of duties in transport management
  https://help.sap.com/docs/cloud-transport-management/sap-cloud-transport-management/segregation-of-duties-in-transport-management
  source_owner: SAP SE
  topic_supported: SoD controls in Cloud TMS, role separation between transport requester and transport approver/importer, Cloud TMS role definitions
  why_needed: Authoritative reference for step 12 (SoD check) — defines the Cloud TMS role model and how requester/approver/importer separation must be verified
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP Platform Transport Management (STMS / CTS+)

- ABAP Platform — Transport Organizer and STMS
  https://help.sap.com/docs/abap-platform/abap-platform/transport-organizer-and-stms
  source_owner: SAP SE
  topic_supported: STMS configuration, transport route management, import queue operations, authorization objects for transport management
  why_needed: Authoritative source for STMS import procedures, authorization object requirements (S_TRANSPRT), and import log reading used in steps 8, 12, 14, and 15
  evidence_level: primary
  last_verified: 2026-06-19

## Change Management Integration

- SAP Solution Manager — change request management
  https://help.sap.com/docs/sap-solution-manager/sap-solution-manager/change-request-management
  source_owner: SAP SE
  topic_supported: Change request management (ChaRM) integration with transport management; ticket linkage for step 6; SoD enforcement patterns
  why_needed: Grounds step 6 (ticket linkage) and step 12 (SoD verification) with SAP-native change management integration context
  evidence_level: primary
  last_verified: 2026-06-19

- BTP change management
  https://help.sap.com/docs/btp/sap-business-technology-platform/change-management-in-sap-btp
  source_owner: SAP SE
  topic_supported: Change management controls for BTP-hosted services and configurations; approval workflows
  why_needed: Relevant for steps 5, 12, and 13 when transport targets include BTP subaccount configurations alongside ABAP transports
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes transport management design intent, authorization object requirements, and API specifications. It does not prove which transport routes exist in the user's landscape, which SAP systems are accessible, what the current import queue contains, or whether the user's STMS is configured for the described procedures. Users must supply live evidence from steps 8 and 15 for concrete transport assessment.
