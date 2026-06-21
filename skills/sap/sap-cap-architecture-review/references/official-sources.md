# Official sources — SAP CAP Architecture Review

Use this reference when grounding CAP CDS modeling guidance, service authorization, multitenancy design, draft handling, and CAP testing patterns.

**Evidence level**: documentation-based (SAP CAP documentation at cap.cloud.sap, SAP Help Portal). No live-system evidence is collected by this skill.

## CAP authorization

- SAP CAP Authorization Guide
  https://cap.cloud.sap/docs/guides/security/authorization
  source_owner: SAP SE
  topic_supported: @requires and @restrict annotations, role-based access control, instance-level restrictions, where clauses, user attribute enforcement, CAP runtime authorization enforcement behavior
  why_needed: Primary source for classifying authorization gaps in CAP service and entity definitions; defines when a service or entity is unguarded
  evidence_level: primary
  last_verified: 2026-06-19

- SAP CAP Security — Data Protection and Privacy
  https://cap.cloud.sap/docs/java/security
  source_owner: SAP SE
  topic_supported: CAP Java security integration, Spring Security wiring, token-based authentication, data privacy annotations
  why_needed: Java-stack authorization specifics that differ from Node.js CAP runtime behavior
  evidence_level: primary
  last_verified: 2026-06-19

## CDS modeling and service layer

- SAP CAP Providing Services Guide
  https://cap.cloud.sap/docs/guides/providing-services
  source_owner: SAP SE
  topic_supported: CDS service definitions, projection patterns, auto-exposed entities, action and function definitions, service layer separation of concerns
  why_needed: Authoritative source for service design patterns, projection anti-patterns, and correct entity exposure
  evidence_level: primary
  last_verified: 2026-06-19

- CDS Definition Language (CDL) Reference
  https://cap.cloud.sap/docs/cds/cdl
  source_owner: SAP SE
  topic_supported: CDS syntax for entities, associations, compositions, aspects, annotations, views, and projections
  why_needed: Reference for assessing CDS model correctness, association direction, and annotation validity
  evidence_level: primary
  last_verified: 2026-06-19

## Multitenancy

- SAP CAP Multitenancy Guide
  https://cap.cloud.sap/docs/guides/multitenancy/
  source_owner: SAP SE
  topic_supported: MTX service setup, @sap/cds-mtxs configuration, tenant onboarding/offboarding, extensibility service wiring, subscriber passcode, tenant upgrade
  why_needed: Defines multitenancy architecture requirements for CAP SaaS applications; used to classify MTX wiring gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Draft handling

- SAP CAP Draft Support Guide
  https://cap.cloud.sap/docs/guides/fiori/draft-support
  source_owner: SAP SE
  topic_supported: Draft-enabled entities, draftActivate hook, BeforeSave validation, side effects, draft lock expiry, cancel behavior, Fiori Elements draft lifecycle
  why_needed: Defines correct draft implementation patterns; used to identify incomplete draft activation logic and missing side-effect handlers
  evidence_level: primary
  last_verified: 2026-06-19

## Testing

- SAP CAP Testing Guide
  https://cap.cloud.sap/docs/guides/testing/
  source_owner: SAP SE
  topic_supported: cds.test setup, Jest integration, mock authentication, CDS test environment, in-memory SQLite testing, beforeAll/afterAll teardown patterns
  why_needed: Defines CAP-specific testing patterns; used to classify test isolation gaps and missing test teardown
  evidence_level: primary
  last_verified: 2026-06-19

## BTP deployment

- SAP BTP — SAP Cloud Application Programming Model
  https://help.sap.com/docs/btp/sap-business-technology-platform/sap-cloud-application-programming-model
  source_owner: SAP SE
  topic_supported: CAP on BTP — MTA deployment, service bindings, environment profile configuration, BTP-specific package.json cds block settings
  why_needed: BTP deployment configuration requirements including profile wiring and service binding completeness
  evidence_level: primary
  last_verified: 2026-06-19

- SAP CAP Node.js cds Facade API Reference
  https://cap.cloud.sap/docs/node.js/cds-facade
  source_owner: SAP SE
  topic_supported: cds.requires, cds.env, cds.model, service registration, event handler wiring — runtime configuration API
  why_needed: Used to assess package.json cds block correctness and runtime configuration anti-patterns (e.g., direct db handle access)
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP CAP documentation describes design intent and recommended patterns. It does not prove which CDS annotations are enforced in a specific CAP runtime version deployed in the user's BTP subaccount, or whether the user's MTX configuration has been correctly activated. Users must supply CDS source files, handler code, package.json, and test files for concrete assessment.
