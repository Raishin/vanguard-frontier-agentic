# Official sources — SAP ABAP Cloud RAP Review

Use this reference when grounding RAP behavior definition design, CDS view modeling, authorization control, draft handling, ABAP unit testing, and released API compliance.

**Evidence level**: documentation-based (SAP Help Portal — help.sap.com/docs/abap-cloud, SAP API Business Hub). No live-system evidence is collected by this skill.

## RAP core documentation

- ABAP RESTful Application Programming Model (RAP) — overview
  https://help.sap.com/docs/abap-cloud/abap-rap/abap-restful-application-programming-model
  source_owner: SAP SE
  topic_supported: RAP programming model overview, business object layers, OData service generation, RAP on S/4HANA and BTP ABAP Environment
  why_needed: Primary taxonomy for RAP artifact classification and component role assignment
  evidence_level: primary
  last_verified: 2026-06-19

- What is ABAP Cloud
  https://help.sap.com/docs/abap-cloud/abap-cloud/what-is-abap-cloud
  source_owner: SAP SE
  topic_supported: ABAP Cloud programming model tiers, tier-2 compliance rules, forbidden language constructs, object release status
  why_needed: Defines the ABAP Cloud tier compliance rules that govern what is and is not permitted in clean-core RAP objects
  evidence_level: primary
  last_verified: 2026-06-19

## Behavior Definition (BDEF)

- RAP Behavior Definition
  https://help.sap.com/docs/abap-cloud/abap-rap/behavior-definition
  source_owner: SAP SE
  topic_supported: BDEF syntax, managed vs. unmanaged implementation type, draft enablement (with draft), authorization master/dependent, alias, feature control, action and function declarations, association exposure
  why_needed: Authoritative reference for BDEF correctness review; defines required keywords and their semantics
  evidence_level: primary
  last_verified: 2026-06-19

## CDS data model

- RAP CDS Data Model
  https://help.sap.com/docs/abap-cloud/abap-rap/cds-data-model
  source_owner: SAP SE
  topic_supported: Interface CDS views (data definitions), projection CDS views, consumption views, CDS metadata extensions, access control (DCL), draft table configuration, UUID key design
  why_needed: Defines the three-layer CDS view hierarchy (interface / projection / consumption) and access control (DCL) patterns; used to classify view type misuse and annotation gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Authorization control

- RAP Authorization Control
  https://help.sap.com/docs/abap-cloud/abap-rap/authorization-control
  source_owner: SAP SE
  topic_supported: Authorization master and dependent assignment in BDEF, CHECK_AUTHORIZATION method implementation, AUTHORITY-CHECK for RAP operations, instance-based authorization
  why_needed: Defines the RAP authorization model; used to classify missing or pass-through authorization checks
  evidence_level: primary
  last_verified: 2026-06-19

## Draft handling

- RAP Draft Handling
  https://help.sap.com/docs/abap-cloud/abap-rap/draft-handling
  source_owner: SAP SE
  topic_supported: Draft-enabled entities, draft table configuration, draftActivate action, BeforeSave validation, lock expiry, draft discard, ETag handling
  why_needed: Defines RAP draft implementation requirements; used to identify incomplete draft configuration and missing validation handlers
  evidence_level: primary
  last_verified: 2026-06-19

## ABAP unit testing for RAP

- ABAP Unit Tests for RAP Business Objects
  https://help.sap.com/docs/abap-cloud/abap-rap/abap-unit-tests-for-rap-business-objects
  source_owner: SAP SE
  topic_supported: if_abap_behv_test_environment, if_cds_test_environment, test class structure, MODIFY ENTITIES test calls, ROLLBACK ENTITIES teardown, test double creation
  why_needed: Defines correct RAP unit test patterns; used to classify missing test doubles and absent teardown methods
  evidence_level: primary
  last_verified: 2026-06-19

## Released API compliance

- Released ABAP Object Types
  https://help.sap.com/docs/abap-cloud/abap-cloud/released-abap-object-types
  source_owner: SAP SE
  topic_supported: C1/C2 release contract definitions, object release status (RELEASED, NOT_RELEASED, DEPRECATED), what is permitted in ABAP Cloud tier-1 and tier-2
  why_needed: Defines the release contract taxonomy used for classifying NOT_RELEASED API consumption findings
  evidence_level: primary
  last_verified: 2026-06-19

- SAP API Business Hub — released APIs
  https://api.sap.com/
  source_owner: SAP SE
  topic_supported: C1/C2 released API catalog for S/4HANA and BTP ABAP Environment; used to verify replacement candidates for NOT_RELEASED objects
  why_needed: Authoritative source for confirming which ABAP objects carry a release contract; must be used to validate any proposed replacement API
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes design intent, required patterns, and release contracts. It does not prove which objects exist in the user's ABAP system, what their release status is in a specific system version, or whether SAP notes have been applied. Users must supply ABAP source code, CDS DDL definitions, BDEF source, or written descriptions of their RAP object model for concrete assessment.
