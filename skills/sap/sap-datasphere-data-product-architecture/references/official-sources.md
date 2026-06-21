# Official sources — SAP Datasphere Data Product Architecture Review

Use this reference when grounding Datasphere space design, data flow and replication flow assessment, semantic model review, data product governance, data access control evaluation, and SAC/HANA Cloud integration assessment.

**Evidence level**: documentation-based (SAP Datasphere Help Portal). No live-system evidence is collected by this skill.

## Spaces and administration

- Spaces in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/spaces
  source_owner: SAP SE
  topic_supported: Space creation, storage quota assignment, user and role assignment to spaces, space-level connection management, space isolation model
  why_needed: Primary reference for classifying space design governance findings including over-permissive user assignment, quota misconfiguration, and missing isolation between business domains
  evidence_level: primary
  last_verified: 2026-06-19

## Data flows and replication flows

- Data flows in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/data-flows
  source_owner: SAP SE
  topic_supported: Data flow design, transformation operators, scheduling, target table configuration, run status monitoring
  why_needed: Defines the data flow model used to classify transformation pipeline design findings including missing error handling, incorrect scheduling, and target configuration gaps
  evidence_level: primary
  last_verified: 2026-06-19

- Replication flows in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/replication-flows
  source_owner: SAP SE
  topic_supported: Replication flow design, load type (initial load, delta load, initial and delta), source and target object selection, replication monitor
  why_needed: Defines the replication flow model distinct from data flows — required to correctly classify delta-load design findings and prevent data flow / replication flow confusion
  evidence_level: primary
  last_verified: 2026-06-19

## Semantic modeling

- Semantic modeling in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/semantic-modeling
  source_owner: SAP SE
  topic_supported: Graphical views, SQL views, semantic usage types (dimension, fact, analytical dataset, relational dataset), analytic model design, measure definition, dimension association, SAC live connection compatibility
  why_needed: Defines the semantic usage and analytic model object types — required to assess semantic layer completeness for SAC live connection consumption and measure/dimension correctness
  evidence_level: primary
  last_verified: 2026-06-19

## Data products and sharing

- Data products in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/data-products
  source_owner: SAP SE
  topic_supported: Data product definition, output port types (table sharing, data sharing service, Open SQL schema), cross-space data product sharing, external consumer access
  why_needed: Defines the data product model — required to classify incomplete output port configurations, overly broad sharing scope, and missing data product governance
  evidence_level: primary
  last_verified: 2026-06-19

- Cross-space sharing in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/cross-space-sharing
  source_owner: SAP SE
  topic_supported: Cross-space table and view sharing, sharing scope, read access grants between spaces
  why_needed: Defines the governed cross-space sharing model — required to distinguish correct cross-space sharing via data products from unauthorized direct access patterns
  evidence_level: primary
  last_verified: 2026-06-19

## Data access controls

- Data access controls in SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/data-access-controls
  source_owner: SAP SE
  topic_supported: Data access control (DAC) definition, criteria entity design, DAC assignment to views and analytic models, combination method (AND/OR), user and team DAC value assignment
  why_needed: Defines the row-level security model — required to classify missing DAC assignments, incorrect combination methods, and incomplete criteria entity definitions as access control findings
  evidence_level: primary
  last_verified: 2026-06-19

## What is SAP Datasphere

- What is SAP Datasphere
  https://help.sap.com/docs/sap-datasphere/sap-datasphere/what-is-sap-datasphere
  source_owner: SAP SE
  topic_supported: SAP Datasphere product overview, space model, data layer, business layer, integration with SAC and HANA Cloud
  why_needed: Provides the product-level context for all architecture assessments; defines the Datasphere space model and integration capabilities
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Datasphere Help Portal documentation describes the designed behavior of spaces, data flows, replication flows, semantic models, data products, and access controls. It does not prove which spaces exist in the user's tenant, which data flows are deployed, or whether DAC rules are correctly configured. Users must supply space configuration exports, model exports, data product definitions, architecture documents, or written descriptions for concrete assessment.
