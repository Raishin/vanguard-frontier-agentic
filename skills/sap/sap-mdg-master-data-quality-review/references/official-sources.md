# Official sources — SAP MDG Master Data Quality Review

Use this reference when grounding MDG data model assessment, validation and derivation rule review, governance workflow analysis, consolidation and mass processing evaluation, data quality KPI framework review, and key mapping configuration assessment.

**Evidence level**: documentation-based (SAP Help Portal, SAP Master Data Governance documentation). No live-system evidence is collected by this skill.

## MDG data model and entity types

- SAP Master Data Governance — Data Modeling
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/data-modeling
  source_owner: SAP SE
  topic_supported: MDG data model framework, entity type definition, node structure (single-object, multi-object, reuse object), key field configuration, data model extensibility using EEWB/AXT
  why_needed: Primary reference for evaluating MDG data model design completeness, extensibility approach soundness, and alignment with the standard MDG node and entity type model
  evidence_level: primary
  last_verified: 2026-06-19

## Validation and derivation rules (BRFplus)

- MDG Validation and Derivation Rules
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/validation-and-derivation-rules
  source_owner: SAP SE
  topic_supported: BRFplus-based validation rule configuration for MDG, derivation rule design, mandatory field check rules, cross-field referential integrity rules, derivation sequence and dependency order, BAdI-based custom validation hooks
  why_needed: Authoritative reference for assessing validation rule coverage and derivation design — required to classify validation gap findings and silent derivation overwrite risks
  evidence_level: primary
  last_verified: 2026-06-19

## Governance workflow

- MDG Governance Workflow
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/governance-workflow
  source_owner: SAP SE
  topic_supported: Change request type definition, MDG workflow template design (SAP Business Workflow or Flexible Workflow), step ownership and agent determination, parallel vs. sequential approval step configuration, deadline and escalation configuration, workflow routing via custom BAdI
  why_needed: Defines the MDG workflow governance model — required to assess workflow step ownership gaps, missing escalation paths, and change request routing correctness
  evidence_level: primary
  last_verified: 2026-06-19

## Duplicate check and matching

- MDG Duplicate Check
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/duplicate-check
  source_owner: SAP SE
  topic_supported: Duplicate check configuration for MDG, match profile setup, matching algorithm selection (fuzzy vs. exact), threshold configuration, duplicate candidate display and resolution workflow
  why_needed: Primary reference for duplicate detection review — required to classify matching threshold misconfiguration and false positive or false negative duplicate check findings
  evidence_level: primary
  last_verified: 2026-06-19

## Consolidation and mass processing

- MDG Consolidation
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/consolidation
  source_owner: SAP SE
  topic_supported: MDG consolidation data model, source system record import, matching and best record calculation, consolidation workflow, mass processing configuration, error handling for consolidation failures
  why_needed: Defines the MDG consolidation and mass processing model — required to assess best record calculation logic, matching rule quality, and mass change authorization control gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Data quality KPI framework

- MDG Data Quality — Key Performance Indicators
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/data-quality-management
  source_owner: SAP SE
  topic_supported: MDG data quality KPI definition, rule-to-KPI mapping, data quality dimension coverage (completeness, conformance, uniqueness, timeliness), KPI threshold configuration, MDG data quality cockpit and dashboard
  why_needed: Primary reference for evaluating KPI framework completeness and metric coverage — required to identify KPI blind spots and threshold misconfiguration findings
  evidence_level: primary
  last_verified: 2026-06-19

## Key mapping and replication

- MDG Key Mapping
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/key-mapping
  source_owner: SAP SE
  topic_supported: Key mapping object type configuration, source system–to–MDG key mapping completeness, MDG replication framework (SOA/Web Services replication model), ALE/IDoc replication alternative, target system key mapping and distribution model
  why_needed: Defines the MDG key mapping and replication model — required to assess key mapping gaps, replication model design, and replication error monitoring coverage
  evidence_level: primary
  last_verified: 2026-06-19

## MDG for Finance (G/L accounts, cost centers)

- MDG for Finance
  https://help.sap.com/docs/SAP_MASTER_DATA_GOVERNANCE/master-data-governance/mdg-for-finance
  source_owner: SAP SE
  topic_supported: MDG Finance domain scope (G/L account, cost center, profit center, cost element, internal order), MDG for Finance data model, integration with SAP S/4HANA Finance master data
  why_needed: Domain-specific reference for MDG Finance governance review — required when the in-scope MDG domains include financial master data managed in the MDG Finance edition
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal and MDG documentation describe the designed governance model, data model framework, validation rule configuration approach, workflow design options, and data quality KPI framework. They do not prove what BRFplus rules are active in the user's MDG system, what change request types are configured, what KPIs are measured, or whether key mapping is complete. Users must supply BRFplus rule summaries, workflow configuration descriptions, data model documents, KPI dashboard exports, or written descriptions of their MDG landscape for concrete assessment.
