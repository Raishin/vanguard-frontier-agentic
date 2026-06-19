# Official sources — SAP Datasphere Data Product Architecture Review

Use this reference when grounding Datasphere space design, data flow and replication flow assessment, semantic model review, data product governance, data access control evaluation, and SAC/HANA Cloud integration assessment.

**Evidence level**: documentation-based (SAP Datasphere Help Portal). No live-system evidence is collected by this skill.

## Spaces and administration

- Administering SAP Datasphere spaces
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/20828efb80714786b2cf91e39b9706dd.html
  source_owner: SAP SE
  topic_supported: Space creation, storage quota assignment, user and role assignment to spaces, space-level connection management, space isolation model
  why_needed: Primary reference for classifying space design governance findings including over-permissive user assignment, quota misconfiguration, and missing isolation between business domains
  evidence_level: primary
  last_verified: 2026-06-19

## Data flows and replication flows

- Creating a data flow
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/9bd12cf116ae40e3a8fc1fc4ccf4ac2e.html
  source_owner: SAP SE
  topic_supported: Data flow design, transformation operators, scheduling, target table configuration, run status monitoring
  why_needed: Defines the data flow model used to classify transformation pipeline design findings including missing error handling, incorrect scheduling, and target configuration gaps
  evidence_level: primary
  last_verified: 2026-06-19

- Creating a replication flow
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/8f0b7b0b1c2c4a5a8ab74c4b3ef3d4b4.html
  source_owner: SAP SE
  topic_supported: Replication flow design, load type (initial load, delta load, initial and delta), source and target object selection, replication monitor
  why_needed: Defines the replication flow model distinct from data flows — required to correctly classify delta-load design findings and prevent data flow / replication flow confusion
  evidence_level: primary
  last_verified: 2026-06-19

## Semantic modeling

- Creating an analytic model
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/e5fbe9e2cb93484dab8b1963145e565f.html
  source_owner: SAP SE
  topic_supported: Analytic model design, measure definition, dimension association, variable and filter configuration, SAC live connection compatibility
  why_needed: Defines the analytic model object type — required to assess semantic layer completeness for SAC live connection consumption and measure/dimension correctness
  evidence_level: primary
  last_verified: 2026-06-19

- Modeling data in the data layer
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/4142201ec1aa49faad89a688a6f67e3c.html
  source_owner: SAP SE
  topic_supported: Graphical views, SQL views, semantic usage types (dimension, fact, analytical dataset, relational dataset), star schema modeling
  why_needed: Defines the semantic usage model — required to flag views exposed to SAC without correct semantic usage annotation, which breaks live connection model compatibility
  evidence_level: primary
  last_verified: 2026-06-19

## Data products and sharing

- Sharing data via data products
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/d9ae898d3a164f6caa67eb8c83e3e1a0.html
  source_owner: SAP SE
  topic_supported: Data product definition, output port types (table sharing, data sharing service, Open SQL schema), cross-space data product sharing, external consumer access
  why_needed: Defines the data product model — required to classify incomplete output port configurations, overly broad sharing scope, and missing data product governance
  evidence_level: primary
  last_verified: 2026-06-19

- Sharing tables and views across spaces
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/5b99c9b7e0164e6a98b4e44fb43b91f3.html
  source_owner: SAP SE
  topic_supported: Cross-space table and view sharing, sharing scope, read access grants between spaces
  why_needed: Defines the governed cross-space sharing model — required to distinguish correct cross-space sharing via data products from unauthorized direct access patterns
  evidence_level: primary
  last_verified: 2026-06-19

## Data access controls

- Securing data with data access controls
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/a03d08e4705f4e09a4b58e10e372de64.html
  source_owner: SAP SE
  topic_supported: Data access control (DAC) definition, criteria entity design, DAC assignment to views and analytic models, combination method (AND/OR), user and team DAC value assignment
  why_needed: Defines the row-level security model — required to classify missing DAC assignments, incorrect combination methods, and incomplete criteria entity definitions as access control findings
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Analytics Cloud and HANA Cloud integration

- Connecting SAP Analytics Cloud to SAP Datasphere
  https://help.sap.com/docs/SAP_DATASPHERE/c8a54ee704e94e15926551293243fd1d/e8bc44aac2f6461eb5e6fd5acf6a1f3a.html
  source_owner: SAP SE
  topic_supported: SAC live connection to Datasphere, model type compatibility (analytic models, analytical datasets), import connection vs. live connection trade-offs, connection configuration
  why_needed: Defines the SAC-Datasphere integration model — required to classify live vs. import connection misuse and model type compatibility findings for SAC story design
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Datasphere Help Portal documentation describes the designed behavior of spaces, data flows, replication flows, semantic models, data products, and access controls. It does not prove which spaces exist in the user's tenant, which data flows are deployed, or whether DAC rules are correctly configured. Users must supply space configuration exports, model exports, data product definitions, architecture documents, or written descriptions for concrete assessment.
