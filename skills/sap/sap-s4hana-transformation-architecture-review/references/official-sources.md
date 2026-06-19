# Official sources — SAP S/4HANA Transformation Architecture Review

Use this reference when grounding transformation strategy assessment, SAP Activate methodology guidance, deployment model evaluation, SAP Readiness Check interpretation, and simplification item analysis.

**Evidence level**: documentation-based (SAP Help Portal, SAP Activate community, SAP Readiness Check portal). No live-system evidence is collected by this skill.

## SAP Activate methodology

- SAP Activate methodology — overview and phases
  https://help.sap.com/docs/SAP_ACTIVATE/80d20672e1e74bde9f0c7f84cda1e3a6/a7c2d4805c1e4c558d5b09e8c0e17e49.html
  source_owner: SAP SE
  topic_supported: SAP Activate phase structure (Discover, Prepare, Explore, Realize, Deploy, Run), deliverables per phase, accelerators, fit-to-standard workshop approach
  why_needed: Primary methodology framework used to assess whether the user's transformation program is phase-aligned and deliverable-complete
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA system conversion (brownfield)

- SAP S/4HANA system conversion guide
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/9f94d0640d6b42d48d4a6c5a7a4af63c/a5ecd75fc12f4ecdad27a0f00e2c5b6c.html
  source_owner: SAP SE
  topic_supported: Brownfield (system conversion) process: prerequisites, pre-conversion checks, conversion steps, simplification item handling, post-conversion tasks
  why_needed: Authoritative technical reference for assessing brownfield conversion approach, scope, and risk — including prerequisite SAP notes and technical conversion tooling
  evidence_level: primary
  last_verified: 2026-06-19

- S/4HANA simplification item catalog
  https://help.sap.com/docs/SAP_S4HANA_ON-PREM/9f94d0640d6b42d48d4a6c5a7a4af63c/b37bc9afc3964b08b3c46b5f08b8e0d1.html
  source_owner: SAP SE
  topic_supported: Simplification item list for S/4HANA conversion: impacted business functions, deprecated objects, mandatory configuration adjustments, manual re-implementation items
  why_needed: Grounds classification of simplification item hits from user-supplied Readiness Check reports; defines conversion-blocking vs. functional-change vs. optional items
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA Cloud Public Edition (greenfield / SaaS)

- SAP S/4HANA Cloud Public Edition — implementation guide
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/d9607e6b65dd4a79a3da8c38a48c0b83.html
  source_owner: SAP SE
  topic_supported: Greenfield implementation in S/4HANA Cloud Public Edition (multi-tenant SaaS): fit-to-standard as mandatory approach, extensibility constraints (key-user and side-by-side only), three-system landscape (DEV/TEST/PROD)
  why_needed: Defines the architectural constraints and implementation approach unique to Cloud Public Edition — critical for assessing whether the user's transformation plan is compatible with this deployment model
  evidence_level: primary
  last_verified: 2026-06-19

- S/4HANA Cloud — deployment model comparison
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/e5522a8a7b174979/7e94748d64f04bf898e5a5f2c3893c0f.html
  source_owner: SAP SE
  topic_supported: Comparison of S/4HANA Cloud Public Edition vs. Cloud Private Edition vs. on-premise deployment: update cycles, extensibility permissions, operations responsibilities, license considerations
  why_needed: Primary reference for assessing deployment model trade-offs when the user is evaluating or has already chosen a deployment target
  evidence_level: primary
  last_verified: 2026-06-19

## Selective data transition (SDT)

- SAP S/4HANA selective data transition approach
  https://help.sap.com/docs/SAP_S4HANA_CLOUD/4c55a23e22e84e09addd63f374cf33a7/c6ef9ca4b7bc47b2878fc0d8e15fd5ce.html
  source_owner: SAP SE
  topic_supported: Selective data transition (SDT) methodology: use cases, tooling (SAP LT Replication Server, Data Migration Cockpit), data selection criteria, cutover complexity vs. greenfield and brownfield
  why_needed: Authoritative reference for assessing when SDT is appropriate, what data migration tooling applies, and what additional complexity SDT introduces compared to a pure brownfield or greenfield approach
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Readiness Check

- SAP Readiness Check for SAP S/4HANA — overview and output interpretation
  https://help.sap.com/docs/SAP_READINESS_CHECK/dee5e6d76e9847e8bdb3adb33ee3c4b4/c3f3b4dc8c8d429db9bb7a69bf4b0f45.html
  source_owner: SAP SE
  topic_supported: SAP Readiness Check tool: simplification item impact assessment, custom code analysis scope, business function usage detection, add-on compatibility, technical prerequisite checks, output report structure
  why_needed: Defines how to interpret the Readiness Check report the user may supply — maps finding categories to transformation risk levels and required remediation actions before conversion begins
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and SAP Activate documentation describe methodology, recommended phases, deployment model constraints, and conversion tooling. They do not prove which simplification items affect the user's specific source ECC or S/4HANA release, which business functions are active in their system, or whether the user's landscape has applied the required SAP notes. Users must supply SAP Readiness Check output, project scoping documents, or landscape descriptions for concrete transformation assessment.
