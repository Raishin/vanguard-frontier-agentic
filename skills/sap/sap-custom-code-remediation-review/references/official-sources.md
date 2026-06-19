# Official sources — SAP Custom Code Remediation Review

Use this reference when grounding ATC finding classification, deprecated API replacement guidance, custom code migration scope, ABAP Cloud constraints, and S/4HANA readiness check interpretation.

**Evidence level**: documentation-based (SAP Help Portal, SAP API Business Hub). No live-system evidence is collected by this skill.

## SAP Custom Code Migration tooling

- SAP custom code migration for S/4HANA — overview and SYCM
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/9f94d0640d6b42d48d4a6c5a7a4af63c/3d058e2f96c2419c938be56fa0ab5eb1.html
  source_owner: SAP SE
  topic_supported: Custom Code Migration App (transaction SYCM), scoping, analysis report structure, finding categories, object prioritization for S/4HANA conversion
  why_needed: Authoritative reference for interpreting SYCM output that users supply for review; defines the finding categories and object-level risk classification used in remediation planning
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP Test Cockpit (ATC) and release contracts

- Custom code compatibility checks for ABAP platform — release contracts
  https://help.sap.com/docs/ABAP_PLATFORM_NEW/fc4c71aa50014fd1b43721701471913d/4ec33a5ebf2d4f0892e7fb66b17c4e78.html
  source_owner: SAP SE
  topic_supported: ATC check framework, object release status (C1/C2/NOT_RELEASED), compatibility check categories, ATC variant configuration for S/4HANA readiness
  why_needed: Primary reference for classifying ATC finding severity by release contract status and mapping NOT_RELEASED findings to their remediation category
  evidence_level: primary
  last_verified: 2026-06-19

- ATC custom code check for S/4HANA — check variant and findings interpretation
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/6aa39f1ac05441e5a23f484f31e477e7/b98dc3a9e89b42fd8de17e31a1bfaab6.html
  source_owner: SAP SE
  topic_supported: ATC S/4HANA readiness check variant configuration, finding categories for upgrade-blocking vs. clean-core-compliance checks, exemption handling
  why_needed: Grounds classification of ATC finding priority levels (BLOCKER / ERROR / WARNING / INFO) within the S/4HANA readiness check scope
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP Cloud programming model constraints

- What is ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/what-is-abap-cloud
  source_owner: SAP SE
  topic_supported: ABAP Cloud tier structure (Tier 1: ABAP language subset, Tier 2: released API consumption, Tier 3: ABAP Platform and S/4HANA); forbidden language constructs in ABAP Cloud objects; object release status implications
  why_needed: Defines which ABAP language constructs and API access patterns are forbidden in ABAP Cloud tier objects — used to evaluate whether proposed ABAP Cloud remediation paths are valid
  evidence_level: primary
  last_verified: 2026-06-19

- Custom code adaptation for S/4HANA upgrade — deprecated patterns and replacement guidance
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/d4ee3b9d0c804588bde3d16ab55a54db.html
  source_owner: SAP SE
  topic_supported: Deprecated API patterns (classic BAPIs, internal FMs, DDIC structures), replacement guidance toward released APIs, custom code adaptation categories, ATC usage for detecting deprecated patterns
  why_needed: Authoritative list of deprecated custom code patterns and their S/4HANA replacement direction — primary reference for mapping ATC findings to concrete replacement actions
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP syntax and forbidden constructs in ABAP Cloud

- ABAP language for ABAP Cloud — forbidden statements and constructs
  https://help.sap.com/docs/ABAP_PLATFORM_NEW/c238d694b825421f940829321ffa326a/1defe0a4a17d4b72820013de44ea58fa.html
  source_owner: SAP SE
  topic_supported: ABAP language subset for ABAP Cloud: forbidden statements (CALL SCREEN, CALL DIALOG, SELECT * on SAP tables, CALL FUNCTION with non-released FMs), permitted alternatives, syntax check scope in ABAP Cloud objects
  why_needed: Required for evaluating whether user-provided ABAP source code or proposed ABAP Cloud replacements comply with ABAP Cloud language restrictions
  evidence_level: primary
  last_verified: 2026-06-19

## SAP API Business Hub — release contract validation

- SAP API Business Hub — released APIs catalog
  https://api.sap.com/
  source_owner: SAP SE
  topic_supported: C1/C2 released API catalog for S/4HANA, BTP services; used to validate that proposed replacement APIs and function modules carry a valid release contract
  why_needed: Authoritative source for confirming C1/C2 contract before recommending any replacement API as a clean-core-compliant remediation target; no replacement should be recommended without directing the user to verify its contract here
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and ABAP Cloud documentation describe language constraints, deprecated patterns, and replacement guidance at a general level. They do not prove which specific SAP APIs are deprecated in the user's source system release, which ATC findings apply to the user's specific custom object inventory, or whether a given replacement API is available in the user's target S/4HANA release. Users must supply ATC exports, SYCM output, or ABAP source artifacts for concrete remediation assessment. Release contracts for proposed replacements must always be verified on SAP API Business Hub.
