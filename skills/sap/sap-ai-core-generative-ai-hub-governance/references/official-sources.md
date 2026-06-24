# Official sources — SAP AI Core Generative AI Hub Governance

Use this reference when grounding AI Core model access, AI Launchpad role governance, Generative AI Hub orchestration, grounding data classification, data privacy, and audit requirements.

**Evidence level**: documentation-based (SAP Help Portal — help.sap.com). No live-system evidence is collected by this skill.

## SAP AI Core — service overview and access control

- What is SAP AI Core
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/what-is-sap-ai-core
  source_owner: SAP SE
  topic_supported: AI Core service overview, supported use cases, service plan tiers (standard, extended), BTP subaccount integration
  why_needed: Establishes which AI Core service plan features are available; service plan determines available governance controls
  evidence_level: primary
  last_verified: 2026-06-19

- SAP AI Core Resource Groups
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/resource-groups
  source_owner: SAP SE
  topic_supported: Resource group creation, isolation scope, assignment of model deployments, vector stores, and grounding pipelines to resource groups; cross-resource-group access controls
  why_needed: Resource group is the primary access control and isolation boundary for AI Core workloads; used to classify cross-resource-group access findings
  evidence_level: primary
  last_verified: 2026-06-19

- SAP AI Core Security and Data Protection
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/security-and-data-protection
  source_owner: SAP SE
  topic_supported: Data residency, encryption at rest and in transit, tenant isolation, AI Core security posture, responsible AI use policy
  why_needed: Authoritative source for data residency requirements and security baseline; used to classify data protection gaps
  evidence_level: primary
  last_verified: 2026-06-19

## AI Launchpad — roles and authorizations

- What is SAP AI Launchpad
  https://help.sap.com/docs/ai-launchpad/sap-ai-launchpad/what-is-sap-ai-launchpad
  source_owner: SAP SE
  topic_supported: AI Launchpad overview, supported AI Core connection types, user interface for managing AI workloads
  why_needed: Establishes AI Launchpad scope and integration with AI Core; context for role-based access review
  evidence_level: primary
  last_verified: 2026-06-19

- SAP AI Launchpad Roles and Authorizations
  https://help.sap.com/docs/ai-launchpad/sap-ai-launchpad/roles-and-authorizations
  source_owner: SAP SE
  topic_supported: aicore_admin, aicore_viewer, genai_manager, genai_viewer role definitions; separation of duties; role collection assignment in BTP
  why_needed: Defines the role taxonomy for AI Launchpad access governance; used to classify overprivileged role assignments and missing separation of duties
  evidence_level: primary
  last_verified: 2026-06-19

## Generative AI Hub — orchestration and governance

- Generative AI Hub in SAP AI Core
  https://help.sap.com/docs/sap-ai-core/generative-ai/generative-ai-hub-in-sap-ai-core
  source_owner: SAP SE
  topic_supported: Generative AI Hub capabilities, supported models, model access via orchestration service, enterprise governance runtime
  why_needed: Primary overview for Generative AI Hub scope; confirms enterprise governance is the intended operating posture
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Generative AI Hub Orchestration
  https://help.sap.com/docs/sap-ai-core/generative-ai/orchestration
  source_owner: SAP SE
  topic_supported: Orchestration service configuration, llm_module_config, templating_module_config, filtering_module_config (content safety), grounding_module_config, prompt template variables, model version selection
  why_needed: Authoritative source for orchestration configuration structure; used to classify missing content safety filters and unguarded template variables
  evidence_level: primary
  last_verified: 2026-06-19

## Generative AI Hub — document grounding

- SAP Generative AI Hub Document Grounding
  https://help.sap.com/docs/sap-ai-core/generative-ai/document-grounding
  source_owner: SAP SE
  topic_supported: Document Grounding Service configuration, vector store creation and scope, data repository types (vector, SharePoint), grounding pipeline ingestion, metadata handling, retrieval configuration
  why_needed: Defines grounding pipeline architecture; used to classify data classification gaps before ingestion and cross-tenant vector store isolation findings
  evidence_level: primary
  last_verified: 2026-06-19

## Generative AI Hub — data protection and privacy

- SAP Generative AI Hub Data Protection and Privacy
  https://help.sap.com/docs/sap-ai-core/generative-ai/data-protection-and-privacy
  source_owner: SAP SE
  topic_supported: Prompt data handling, data residency for AI-processed data, personal data in prompts and grounding documents, prompt logging behavior, data retention, right-to-erasure considerations for vectorized data
  why_needed: Authoritative source for data privacy requirements for Generative AI Hub workloads; used to classify prompt logging and embedding privacy findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation describes design intent, supported governance controls, and data protection posture for standard AI Core service plans. It does not prove which controls are active in a specific customer's AI Core instance, what data has been ingested into their vector stores, or how their orchestration pipelines are currently deployed. Users must supply orchestration configuration files, role assignment descriptions, grounding pipeline design, or written descriptions for concrete governance assessment.
