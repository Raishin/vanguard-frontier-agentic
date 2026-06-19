# Official sources — SAP Guarded Transport Import

Use this reference when grounding SAP transport management commands, CTS+ API usage, STMS procedures, and Cloud TMS operations.

**Evidence level**: documentation-based (SAP Help Portal). Live evidence gathered during steps 8 and 14 is labeled separately as `live evidence` per the audit format in `references/live-environment-access.md`.

## SAP Cloud Transport Management Service

- SAP Cloud Transport Management — overview and getting started
  https://help.sap.com/docs/TRANSPORT_MANAGEMENT_SERVICE/7f7160ec0d8546c6b3eab72fb5ad6fd8/5fef9d6b1cb047b2b18d6069c8ab051b.html
  source_owner: SAP SE
  topic_supported: Cloud TMS architecture, transport nodes, transport routes, import queue management
  why_needed: Authoritative source for Cloud TMS import sequence and transport node configuration used in steps 2, 7, 8, and 14
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud TMS REST API reference
  https://help.sap.com/docs/TRANSPORT_MANAGEMENT_SERVICE/7f7160ec0d8546c6b3eab72fb5ad6fd8/66fd7283260c4c90b1c9a40f629f7e9b.html
  source_owner: SAP SE
  topic_supported: Cloud TMS REST API — GET import queue (step 8), POST import trigger (step 14), response codes and error handling
  why_needed: Defines the exact API calls permitted for read-only queue inspection and approved import trigger
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud TMS — import process and controls
  https://help.sap.com/docs/TRANSPORT_MANAGEMENT_SERVICE/7f7160ec0d8546c6b3eab72fb5ad6fd8/2c4e1bff1a47480082e56c56f4c9b08a.html
  source_owner: SAP SE
  topic_supported: Import process flow, import log interpretation, error handling and rollback in Cloud TMS
  why_needed: Grounds step 15 (verify) and step 11 (rollback plan) with Cloud TMS-specific recovery procedures
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP Platform Transport Management (STMS / CTS+)

- STMS and CTS transport management for ABAP systems
  https://help.sap.com/docs/ABAP_PLATFORM_NEW/8f6c1c5c774b4b3085023a3cf28a1f62/60de2b8fd1a84e6fa01b0a10a7acafdc.html
  source_owner: SAP SE
  topic_supported: STMS configuration, transport route management, import queue operations, authorization objects for transport management
  why_needed: Authoritative source for STMS import procedures, authorization object requirements (S_TRANSPRT), and import log reading used in steps 8, 12, 14, and 15
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA Cloud Transport

- S/4HANA Cloud transport and release management
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/ecd34e8b9b6a4c2e8c38a40f4a4d0c52.html
  source_owner: SAP SE
  topic_supported: S/4HANA Cloud transport process, release management, three-system landscape transport controls
  why_needed: Defines transport controls specific to S/4HANA Cloud (three-system landscape: DEV, TEST, PROD) used in steps 2 and 7
  evidence_level: primary
  last_verified: 2026-06-19

## Change Management Integration

- SAP Solution Manager — change request management
  https://help.sap.com/docs/SAP_SOLUTION_MANAGER/89950cbc0e864c2893cb2f27ba0f7200/5ebbbfb4069d4d4eadea52f3fa7e0e07.html
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
