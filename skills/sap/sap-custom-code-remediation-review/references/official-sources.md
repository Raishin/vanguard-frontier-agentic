# Official sources — SAP Custom Code Remediation Review

Use this reference when grounding ATC finding classification, deprecated API replacement guidance, custom code migration scope, ABAP Cloud constraints, and S/4HANA readiness check interpretation.

**Evidence level**: documentation-based (SAP Help Portal, SAP API Business Hub). No live-system evidence is collected by this skill.

## SAP Custom Code Migration tooling

- Custom code migration for S/4HANA — overview and SYCM
  https://help.sap.com/docs/abap-cloud/abap-cloud/custom-code-migration
  source_owner: SAP SE
  topic_supported: Custom Code Migration App (transaction SYCM), scoping, analysis report structure, finding categories, object prioritization for S/4HANA conversion
  why_needed: Authoritative reference for interpreting SYCM output that users supply for review; defines the finding categories and object-level risk classification used in remediation planning
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP Test Cockpit (ATC) and release contracts

- ABAP Test Cockpit in ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/abap-test-cockpit
  source_owner: SAP SE
  topic_supported: ATC check framework, object release status (C1/C2/NOT_RELEASED), compatibility check categories, ATC variant configuration for S/4HANA readiness
  why_needed: Primary reference for classifying ATC finding severity by release contract status and mapping NOT_RELEASED findings to their remediation category
  evidence_level: primary
  last_verified: 2026-06-19

- ATC S/4HANA readiness checks — check variant and findings interpretation
  https://help.sap.com/docs/abap-cloud/abap-cloud/atc-s4hana-readiness-checks
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

- Released APIs in ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/released-apis
  source_owner: SAP SE
  topic_supported: Deprecated API patterns (classic BAPIs, internal FMs, DDIC structures), replacement guidance toward released APIs, C1/C2 release contract reference
  why_needed: Authoritative list of deprecated custom code patterns and their S/4HANA replacement direction — primary reference for mapping ATC findings to concrete replacement actions
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP syntax and forbidden constructs in ABAP Cloud

- ABAP Cloud restrictions — forbidden statements and constructs
  https://help.sap.com/docs/abap-cloud/abap-cloud/abap-cloud-restrictions
  source_owner: SAP SE
  topic_supported: ABAP language subset for ABAP Cloud: forbidden statements (CALL SCREEN, CALL DIALOG, SELECT * on SAP tables, CALL FUNCTION with non-released FMs), permitted alternatives, syntax check scope in ABAP Cloud objects
  why_needed: Required for evaluating whether user-provided ABAP source code or proposed ABAP Cloud replacements comply with ABAP Cloud language restrictions
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA simplification items

- Simplification items for S/4HANA
  https://help.sap.com/docs/abap-cloud/abap-cloud/simplification-items
  source_owner: SAP SE
  topic_supported: Simplification item catalog, impact on custom code, deprecated functions and APIs, mandatory adjustments during system conversion
  why_needed: Defines the simplification item taxonomy used alongside ATC findings to prioritize remediation work — conversion-blocking items must be resolved before a system conversion
  evidence_level: primary
  last_verified: 2026-06-19

## SAP API Business Hub — release contract validation

- SAP API Business Hub — SAP S/4HANA Cloud product overview and released APIs
  https://api.sap.com/products/SAPS4HANACloud/overview
  source_owner: SAP SE
  topic_supported: C1/C2 released API catalog for S/4HANA, BTP services; used to validate that proposed replacement APIs and function modules carry a valid release contract
  why_needed: Authoritative source for confirming C1/C2 contract before recommending any replacement API as a clean-core-compliant remediation target; no replacement should be recommended without directing the user to verify its contract here
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and ABAP Cloud documentation describe language constraints, deprecated patterns, and replacement guidance at a general level. They do not prove which specific SAP APIs are deprecated in the user's source system release, which ATC findings apply to the user's specific custom object inventory, or whether a given replacement API is available in the user's target S/4HANA release. Users must supply ATC exports, SYCM output, or ABAP source artifacts for concrete remediation assessment. Release contracts for proposed replacements must always be verified on SAP API Business Hub.
