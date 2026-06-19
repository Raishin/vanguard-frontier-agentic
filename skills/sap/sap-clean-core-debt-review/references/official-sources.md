# Official sources — SAP Clean Core Debt Review

Use this reference when grounding clean core compliance assessment, ABAP Cloud patterns, RAP extensibility, and S/4HANA upgrade risk.

**Evidence level**: documentation-based (SAP Help Portal, SAP API Business Hub). No live-system evidence is collected by this skill.

## SAP Clean Core and extensibility

- SAP Clean Core for SAP S/4HANA Cloud — extensibility overview
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/6aa39f1ac05441e5a23f484f31e477e7/f1b8dfac9f584e2a9c8b9b47e2b46ad3.html
  source_owner: SAP SE
  topic_supported: Clean Core definition, extensibility pillars (in-app key-user, in-app developer, side-by-side), compliance posture
  why_needed: Primary taxonomy for classifying custom code violations and remediation paths
  evidence_level: primary
  last_verified: 2026-06-19

- ABAP RESTful Application Programming Model (RAP)
  https://help.sap.com/docs/abap-cloud/abap-rap/abap-restful-application-programming-model
  source_owner: SAP SE
  topic_supported: RAP business objects, behavior definitions, CDS views, OData exposure — the strategic S/4HANA extensibility layer
  why_needed: Authoritative source for ABAP Cloud / RAP remediation paths
  evidence_level: primary
  last_verified: 2026-06-19

- In-app extensibility: developer extensibility (BAdIs and released APIs)
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/f1d8b32a5b944ed68fb0a27b0a23d5b5.html
  source_owner: SAP SE
  topic_supported: BAdI-based extensibility, released API consumption in ABAP Cloud, C1/C2 release contracts
  why_needed: Defines in-app developer extensibility boundary and released API contract types
  evidence_level: primary
  last_verified: 2026-06-19

- In-app extensibility: key-user extensibility
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/6aa39f1ac05441e5a23f484f31e477e7/5a4b1d13fe914f70aa0b5bfc2e1f1a8c.html
  source_owner: SAP SE
  topic_supported: Key-user apps for fields, logic, forms, workflows without custom ABAP
  why_needed: Defines the no-code/low-code in-app extensibility path as a clean core compliant alternative
  evidence_level: primary
  last_verified: 2026-06-19

- What is ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/what-is-abap-cloud
  source_owner: SAP SE
  topic_supported: ABAP Cloud programming model overview, tier structure, object release status
  why_needed: Defines the ABAP Cloud programming model that governs what is and is not permitted in clean core compliant custom code
  evidence_level: primary
  last_verified: 2026-06-19

- Custom code adaptation for S/4HANA upgrade
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/d4ee3b9d0c804588bde3d16ab55a54db.html
  source_owner: SAP SE
  topic_supported: Upgrade impact on custom code, deprecated API patterns, ABAP Test Cockpit usage
  why_needed: Maps custom code patterns to upgrade risk levels — critical for prioritizing remediation
  evidence_level: primary
  last_verified: 2026-06-19

- SAP API Business Hub — released APIs
  https://api.sap.com/
  source_owner: SAP SE
  topic_supported: C1/C2 released API catalog for S/4HANA, BTP services; used to validate remediation candidates
  why_needed: Authoritative source for confirming which SAP APIs carry a C1 or C2 release contract for clean core compliant consumption
  evidence_level: primary
  last_verified: 2026-06-19

- Custom code migration — ABAP Test Cockpit
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/6aa39f1ac05441e5a23f484f31e477e7/b98dc3a9e89b42fd8de17e31a1bfaab6.html
  source_owner: SAP SE
  topic_supported: ABAP Test Cockpit (ATC) check categories for clean core compliance; used to interpret ATC output supplied by the user
  why_needed: Provides the ATC finding taxonomy this skill uses to classify user-provided ATC results
  evidence_level: primary
  last_verified: 2026-06-19

- Custom code compatibility checks for ABAP platform
  https://help.sap.com/docs/ABAP_PLATFORM_NEW/fc4c71aa50014fd1b43721701471913d/4ec33a5ebf2d4f0892e7fb66b17c4e78.html
  source_owner: SAP SE
  topic_supported: Compatibility check framework, object release status categories, NOT_RELEASED classification
  why_needed: Defines the NOT_RELEASED vs RELEASED object status taxonomy central to upgrade-blocking debt classification
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and API Business Hub documentation describe design intent and release contracts. They do not prove which objects exist in the user's ABAP system, what their current release status is in that system version, or whether the user's landscape has applied the relevant SAP notes. Users must supply ATC reports, custom code migration app output, or object lists for concrete assessment.
