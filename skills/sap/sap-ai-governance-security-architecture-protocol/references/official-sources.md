# Official sources — SAP AI Governance / Security / Architecture Protocol

Use this reference when grounding findings on SAP AI Core governance, Generative AI Hub configuration, Joule adoption, SAP HANA Cloud vector engine for RAG, AI security architecture, and applicable AI governance frameworks.

**Evidence level**: documentation-based (SAP Help Portal, NIST, OWASP LLM Top 10, ISO/IEC 42001). No live-system evidence is collected by this protocol.

## SAP AI Core — governance and security

- What is SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/what-is-sap-ai-core
  source_owner: SAP SE
  topic_supported: SAP AI Core product overview, runtime architecture, inferencing endpoint lifecycle, resource group model, BTP integration
  why_needed: Baseline taxonomy for classifying AI Core workload governance findings — required to distinguish resource group, model deployment, and inferencing endpoint risks
  evidence_level: primary
  last_verified: 2026-06-19

- Resource groups in SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/resource-groups
  source_owner: SAP SE
  topic_supported: AI Core resource group isolation model, tenant separation, resource group scoping for model deployments and scenarios
  why_needed: Primary reference for assessing isolation posture between AI workloads — required when evaluating whether resource group design provides adequate tenant and data separation
  evidence_level: primary
  last_verified: 2026-06-19

- Security in SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/security
  source_owner: SAP SE
  topic_supported: AI Core authentication (OAuth 2.0 client credentials), authorization model, secure credential handling, network security, data encryption at rest and in transit
  why_needed: Primary security reference for AI Core — defines the authentication and authorization model used to assess model-access control gaps and missing security controls
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Generative AI Hub

- Generative AI Hub in SAP AI Launchpad
  https://help.sap.com/docs/sap-ai-launchpad/sap-ai-launchpad/generative-ai-hub-in-sap-ai-launchpad
  source_owner: SAP SE
  topic_supported: Generative AI Hub overview, foundation model catalog, prompt editor, model proxy API, token consumption, chat and completion endpoints
  why_needed: Primary reference for Generative AI Hub governance — defines the model access model, proxy API scope, and token consumption mechanics used to assess cost attribution and access control findings
  evidence_level: primary
  last_verified: 2026-06-19

- Consume foundation models via SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/consume-foundation-models
  source_owner: SAP SE
  topic_supported: Foundation model deployment via SAP AI Core, model configuration parameters, content filtering options, inference API integration patterns
  why_needed: Defines how foundation models are accessed and configured — required to assess content filtering gaps, model version governance, and inference API authentication posture
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Joule

- About Joule
  https://help.sap.com/docs/joule/joule/about-joule
  source_owner: SAP SE
  topic_supported: Joule product overview, Joule skill architecture, integration with SAP applications, natural language processing of SAP business data, user interaction model
  why_needed: Primary reference for Joule governance findings — defines the scope of Joule's data access, skill activation model, and integration surface with SAP business applications
  evidence_level: primary
  last_verified: 2026-06-19

## SAP HANA Cloud vector engine — RAG grounding

- SAP HANA Cloud vector engine
  https://help.sap.com/docs/sap-hana-cloud/sap-hana-cloud-database/sap-hana-cloud-sap-hana-database-vector-engine
  source_owner: SAP SE
  topic_supported: HANA Cloud vector store capabilities, embedding storage, vector similarity search, access control for vector store data, integration with SAP AI Core for RAG
  why_needed: Primary reference for RAG grounding architecture assessments when SAP HANA Cloud is used as the vector store — required to evaluate data access controls and retrieval authorization in RAG pipelines built on HANA Cloud
  evidence_level: primary
  last_verified: 2026-06-19

## AI governance frameworks — secondary references

- NIST AI Risk Management Framework (AI RMF 1.0)
  https://nvlpubs.nist.gov/nistpubs/ai/nist.ai.100-1.pdf
  source_owner: NIST (US National Institute of Standards and Technology)
  topic_supported: AI risk identification, measurement, management, and governance across the AI lifecycle; four core functions: GOVERN, MAP, MEASURE, MANAGE
  why_needed: Secondary governance framework reference — used to classify AI governance findings against an internationally recognized framework and to assess compliance gap severity for organizations that have adopted NIST AI RMF
  evidence_level: secondary
  last_verified: 2026-06-19

- OWASP Top 10 for Large Language Model Applications
  https://owasp.org/www-project-top-10-for-large-language-model-applications/
  source_owner: OWASP Foundation
  topic_supported: LLM01 prompt injection, LLM02 insecure output handling, LLM03 training data poisoning, LLM06 sensitive information disclosure, LLM08 excessive agency, LLM09 overreliance, and related LLM-specific risks
  why_needed: Secondary security reference — provides classification taxonomy for prompt injection, data leakage, and model-access control findings in SAP AI workloads; maps findings to OWASP LLM Top 10 categories for security team communication
  evidence_level: secondary
  last_verified: 2026-06-19

- ISO/IEC 42001:2023 — AI Management System
  https://www.iso.org/standard/81230.html
  source_owner: ISO (International Organization for Standardization)
  topic_supported: AI management system requirements, AI policy, risk management integration, AI impact assessment, auditability and traceability of AI systems
  why_needed: Secondary governance framework reference — used when organizations have declared ISO/IEC 42001 compliance as a requirement; maps auditability, impact assessment, and governance gap findings to ISO standard clauses
  evidence_level: secondary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes the designed capabilities, configuration options, and security architecture of SAP AI Core, Generative AI Hub, Joule, and SAP HANA Cloud. It does not prove the governance posture, access control configuration, or threat exposure of a specific AI workload deployment. Users must supply architecture descriptions, access control inventories, compliance scope declarations, and audit log configurations for concrete advisory output. NIST AI RMF, OWASP LLM Top 10, and ISO/IEC 42001 are secondary references that provide classification frameworks; they do not supersede SAP product documentation for SAP-specific configuration guidance.
