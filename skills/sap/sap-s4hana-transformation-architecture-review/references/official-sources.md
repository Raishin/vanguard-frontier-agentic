# Official sources — SAP S/4HANA Transformation Architecture Review

Use this reference when grounding transformation strategy assessment, SAP Activate methodology guidance, deployment model evaluation, SAP Readiness Check interpretation, and simplification item analysis.

**Evidence level**: documentation-based (SAP Help Portal, SAP Activate community, SAP Readiness Check portal). No live-system evidence is collected by this skill.

## SAP Activate methodology

- SAP Activate methodology — overview and phases
  https://help.sap.com/docs/sap-activate/sap-activate-methodology/sap-activate-methodology
  source_owner: SAP SE
  topic_supported: SAP Activate phase structure (Discover, Prepare, Explore, Realize, Deploy, Run), deliverables per phase, accelerators, fit-to-standard workshop approach
  why_needed: Primary methodology framework used to assess whether the user's transformation program is phase-aligned and deliverable-complete
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA system conversion (brownfield)

- SAP S/4HANA system conversion guide
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/system-conversion
  source_owner: SAP SE
  topic_supported: Brownfield (system conversion) process: prerequisites, pre-conversion checks, conversion steps, simplification item handling, post-conversion tasks
  why_needed: Authoritative technical reference for assessing brownfield conversion approach, scope, and risk — including prerequisite SAP notes and technical conversion tooling
  evidence_level: primary
  last_verified: 2026-06-19

- S/4HANA simplification items
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/simplification-items
  source_owner: SAP SE
  topic_supported: Simplification item list for S/4HANA conversion: impacted business functions, deprecated objects, mandatory configuration adjustments, manual re-implementation items
  why_needed: Grounds classification of simplification item hits from user-supplied Readiness Check reports; defines conversion-blocking vs. functional-change vs. optional items
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA Cloud Public Edition (greenfield / SaaS)

- SAP S/4HANA Cloud — fit-to-standard approach
  https://help.sap.com/docs/sap-s4hana-cloud/sap-s4hana-cloud/fit-to-standard
  source_owner: SAP SE
  topic_supported: Greenfield implementation in S/4HANA Cloud Public Edition (multi-tenant SaaS): fit-to-standard as mandatory approach, extensibility constraints (key-user and side-by-side only), three-system landscape (DEV/TEST/PROD)
  why_needed: Defines the architectural constraints and implementation approach unique to Cloud Public Edition — critical for assessing whether the user's transformation plan is compatible with this deployment model
  evidence_level: primary
  last_verified: 2026-06-19

- S/4HANA Cloud — deployment options
  https://help.sap.com/docs/sap-s4hana-cloud/sap-s4hana-cloud/deployment-options
  source_owner: SAP SE
  topic_supported: Comparison of S/4HANA Cloud Public Edition vs. Cloud Private Edition vs. on-premise deployment: update cycles, extensibility permissions, operations responsibilities, license considerations
  why_needed: Primary reference for assessing deployment model trade-offs when the user is evaluating or has already chosen a deployment target
  evidence_level: primary
  last_verified: 2026-06-19

## Selective data transition (SDT)

- SAP S/4HANA selective data transition approach
  https://help.sap.com/docs/sap-s4hana-on-premise/sap-s4hana-on-premise/selective-data-transition
  source_owner: SAP SE
  topic_supported: Selective data transition (SDT) methodology: use cases, tooling (SAP LT Replication Server, Data Migration Cockpit), data selection criteria, cutover complexity vs. greenfield and brownfield
  why_needed: Authoritative reference for assessing when SDT is appropriate, what data migration tooling applies, and what additional complexity SDT introduces compared to a pure brownfield or greenfield approach
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Readiness Check

- SAP Readiness Check for SAP S/4HANA — what is SAP Readiness Check
  https://help.sap.com/docs/sap-readiness-check/sap-readiness-check/what-is-sap-readiness-check
  source_owner: SAP SE
  topic_supported: SAP Readiness Check tool: simplification item impact assessment, custom code analysis scope, business function usage detection, add-on compatibility, technical prerequisite checks, output report structure
  why_needed: Defines how to interpret the Readiness Check report the user may supply — maps finding categories to transformation risk levels and required remediation actions before conversion begins
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and SAP Activate documentation describe methodology, recommended phases, deployment model constraints, and conversion tooling. They do not prove which simplification items affect the user's specific source ECC or S/4HANA release, which business functions are active in their system, or whether the user's landscape has applied the required SAP notes. Users must supply SAP Readiness Check output, project scoping documents, or landscape descriptions for concrete transformation assessment.
