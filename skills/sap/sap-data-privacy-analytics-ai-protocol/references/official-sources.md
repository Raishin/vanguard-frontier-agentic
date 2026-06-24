# Official sources — SAP Data Privacy Analytics AI Protocol

Use this reference when grounding data classification assessments, Datasphere data product governance, SAC analytics export controls, SAP AI Core and Generative AI Hub configuration review, Joule adoption governance, and regulatory compliance framing for cross-functional data privacy and AI governance escalations.

**Evidence level**: documentation-based (SAP Help Portal, GDPR text, NIST AI RMF, ISO/IEC 42001). No live-system evidence is collected by this skill.

## SAP Datasphere

- Introduction to Data Governance in SAP Datasphere
  https://help.sap.com/docs/datasphere/datasphere/introduction-to-data-governance
  source_owner: SAP SE
  topic_supported: Datasphere governance model, data ownership, data product lifecycle, semantic layer
  why_needed: Primary reference for Datasphere data governance model — defines the data product, space, and ownership concepts used throughout this protocol
  evidence_level: primary
  last_verified: 2026-06-19

- Data Access Control in SAP Datasphere
  https://help.sap.com/docs/datasphere/datasphere/data-access-control
  source_owner: SAP SE
  topic_supported: Row-level and column-level access control, data access control objects, assignment to analytic models
  why_needed: Required to assess whether PII-containing Datasphere data products have adequate access controls before they are approved for AI or analytics pipelines
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Analytics Cloud

- SAP Analytics Cloud Security Guide
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/security-guide
  source_owner: SAP SE
  topic_supported: SAC tenant security, model-level access, story sharing and export controls, data connection trust settings
  why_needed: Primary reference for SAC data access controls and export governance — required to assess analytics export compliance and story-level access restrictions
  evidence_level: primary
  last_verified: 2026-06-19

## SAP AI Core and Generative AI Hub

- What is SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core/what-is-sap-ai-core
  source_owner: SAP SE
  topic_supported: AI Core deployment model, resource groups, AI scenarios, pipeline execution
  why_needed: Primary reference for AI Core deployment governance — required to assess deployment configuration, resource group isolation, and logging behavior
  evidence_level: primary
  last_verified: 2026-06-19

- Generative AI Hub in SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core/generative-ai-hub-in-sap-ai-core
  source_owner: SAP SE
  topic_supported: Generative AI Hub model catalog, orchestration, prompt registry, grounding configuration, token usage, logging
  why_needed: Primary reference for Generative AI Hub governance review — defines the model deployment, orchestration flow, grounding data, and logging capabilities assessed by this protocol
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Joule

- What is Joule
  https://help.sap.com/docs/joule/joule/what-is-joule
  source_owner: SAP SE
  topic_supported: Joule AI assistant capabilities, SAP system integration, skill activation model, user context handling
  why_needed: Primary reference for Joule governance — defines the skill activation model and system integration scope used to assess Joule adoption readiness and grounding data risk
  evidence_level: primary
  last_verified: 2026-06-19

## Data Protection Regulation

- GDPR Article 5 — Principles Relating to Processing of Personal Data
  https://gdpr-info.eu/art-5-gdpr/
  source_owner: European Parliament and Council
  topic_supported: Lawfulness, fairness, transparency; purpose limitation; data minimization; accuracy; storage limitation; integrity and confidentiality; accountability
  why_needed: Defines the data protection principles that govern PII processing in analytics and AI workloads under GDPR — particularly purpose limitation and data minimization, which are directly applied in the consent and purpose documentation requirements of this protocol
  evidence_level: primary (regulatory)
  last_verified: 2026-06-19

## AI Risk Frameworks

- NIST AI Risk Management Framework (AI RMF)
  https://airc.nist.gov/RMF_Overview
  source_owner: NIST
  topic_supported: AI risk identification, assessment, and management; GOVERN, MAP, MEASURE, MANAGE functions; AI trustworthiness properties including privacy, security, and accountability
  why_needed: Secondary framework for AI governance escalation framing — NIST AI RMF GOVERN and MAP functions directly support the approval gate and audit package requirements for Generative AI Hub and Joule deployments
  evidence_level: secondary
  last_verified: 2026-06-19

- ISO/IEC 42001 — Artificial Intelligence Management System
  https://www.iso.org/standard/81230.html
  source_owner: ISO/IEC
  topic_supported: AI management system requirements, risk assessment for AI systems, transparency and explainability obligations, impact assessment
  why_needed: Secondary framework providing ISO/IEC 42001 context for AI system governance obligations cited in this protocol's approval requirements and audit package structure
  evidence_level: secondary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the designed capabilities, data models, and governance features of Datasphere, SAP Analytics Cloud, SAP AI Core, Generative AI Hub, and Joule. It does not prove which data products contain PII in the user's landscape, what grounding data is configured in the user's Generative AI Hub orchestration, or whether prompt logging is retaining personal data. Users must supply data flow diagrams, deployment configuration descriptions, system prompt excerpts (redacted), and prompt log samples for concrete protocol execution.
