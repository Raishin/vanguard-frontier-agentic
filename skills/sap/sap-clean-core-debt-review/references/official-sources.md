# Official sources — SAP Clean Core Debt Review

Use this reference when grounding clean core compliance assessment, ABAP Cloud patterns, RAP extensibility, and S/4HANA upgrade risk.

**Evidence level**: documentation-based (SAP Help Portal, SAP API Business Hub). No live-system evidence is collected by this skill.

## SAP Clean Core and extensibility

- What is ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/what-is-abap-cloud
  source_owner: SAP SE
  topic_supported: ABAP Cloud programming model overview, tier structure, object release status
  why_needed: Defines the ABAP Cloud programming model that governs what is and is not permitted in clean core compliant custom code
  evidence_level: primary
  last_verified: 2026-06-19

- Clean Core extensibility — extensibility overview and pillars
  https://help.sap.com/docs/abap-cloud/abap-cloud/clean-core-extensibility
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

- Released APIs in ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/released-apis
  source_owner: SAP SE
  topic_supported: C1/C2 released API catalog for ABAP Cloud, release contract types, API browser usage
  why_needed: Defines released API contract types and consumption rules for in-app developer extensibility boundary
  evidence_level: primary
  last_verified: 2026-06-19

- Key-user extensibility in ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/key-user-extensibility
  source_owner: SAP SE
  topic_supported: Key-user apps for fields, logic, forms, workflows without custom ABAP
  why_needed: Defines the no-code/low-code in-app extensibility path as a clean core compliant alternative
  evidence_level: primary
  last_verified: 2026-06-19

- ABAP Test Cockpit in ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/abap-test-cockpit
  source_owner: SAP SE
  topic_supported: ABAP Test Cockpit (ATC) check categories for clean core compliance; used to interpret ATC output supplied by the user
  why_needed: Provides the ATC finding taxonomy this skill uses to classify user-provided ATC results
  evidence_level: primary
  last_verified: 2026-06-19

- SAP API Business Hub — SAP S/4HANA Cloud product overview and released APIs
  https://api.sap.com/products/SAPS4HANACloud/overview
  source_owner: SAP SE
  topic_supported: C1/C2 released API catalog for S/4HANA Cloud, BTP services; used to validate remediation candidates
  why_needed: Authoritative source for confirming which SAP APIs carry a C1 or C2 release contract for clean core compliant consumption
  evidence_level: primary
  last_verified: 2026-06-19

- Side-by-side extensibility on SAP BTP
  https://help.sap.com/docs/abap-cloud/abap-cloud/side-by-side-extensibility-on-btp
  source_owner: SAP SE
  topic_supported: BTP side-by-side extension patterns using CAP, APIs, events; when side-by-side is appropriate vs. in-app extensibility
  why_needed: Defines the clean-core-compliant side-by-side remediation path for custom code that cannot be addressed within in-app extensibility constraints
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and API Business Hub documentation describe design intent and release contracts. They do not prove which objects exist in the user's ABAP system, what their current release status is in that system version, or whether the user's landscape has applied the relevant SAP notes. Users must supply ATC reports, custom code migration app output, or object lists for concrete assessment.
