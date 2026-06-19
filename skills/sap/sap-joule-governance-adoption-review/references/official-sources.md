# Official sources — SAP Joule Governance and Adoption Review

Use this reference when grounding Joule scope grounding assessment, data access boundary review, audit log configuration evaluation, role-aware answer assessment, hallucination and over-trust risk evaluation, and change management readiness review.

**Evidence level**: documentation-based (SAP Help Portal, SAP Joule service guide, SAP Trust Center). No live-system evidence is collected by this skill.

## SAP Joule — Product Overview

- What is Joule
  https://help.sap.com/docs/joule/serviceguide/what-is-joule
  source_owner: SAP SE
  topic_supported: Joule copilot product overview, supported SAP solution integrations (S/4HANA Cloud, SuccessFactors, Ariba, BTP), skill and action model, natural language interaction model
  why_needed: Primary reference for Joule scope and capability model — defines which SAP solutions Joule connects to, what skill types exist (query, action, navigation), and the foundational product boundaries used to assess scope grounding findings
  evidence_level: primary
  last_verified: 2026-06-19

- Joule Capabilities
  https://help.sap.com/docs/joule/serviceguide/joule-capabilities
  source_owner: SAP SE
  topic_supported: Joule skill catalog per connected SAP solution, capability activation model, action types (read-only versus write-back), cross-application capability scope
  why_needed: Required to assess skill activation scope — defines which capabilities are available per SAP solution and which capabilities include write-back or state-mutation actions that require explicit governance controls
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Joule — Security and Data Access

- Joule Security
  https://help.sap.com/docs/joule/serviceguide/security
  source_owner: SAP SE
  topic_supported: Joule authorization model, data access boundary enforcement, integration with underlying SAP system authorization (S/4HANA objects, SuccessFactors RBP), session security, audit logging configuration
  why_needed: Primary reference for Joule data access boundary governance — defines how Joule enforces the underlying SAP authorization model and what audit logging capabilities are available
  evidence_level: primary
  last_verified: 2026-06-19

- Joule Data Protection and Privacy
  https://help.sap.com/docs/joule/serviceguide/data-protection-and-privacy
  source_owner: SAP SE
  topic_supported: Data residency for Joule interaction data, prompt and response data handling, cross-border transfer controls for Joule interaction logs, personal data processed by Joule
  why_needed: Required to assess data privacy compliance for Joule interaction logging — defines where Joule interaction data is stored, what personal data may be included in prompts or responses, and data residency constraints
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Joule — Administration and Roles

- Joule Administration
  https://help.sap.com/docs/joule/serviceguide/administration
  source_owner: SAP SE
  topic_supported: Joule tenant configuration, skill and capability activation, connected system registration, rollout scope configuration, administrator responsibilities
  why_needed: Required to assess Joule capability activation governance — defines the administrative model for controlling which skills are activated, which connected systems are in scope, and which user populations have access to Joule
  evidence_level: primary
  last_verified: 2026-06-19

- Joule Roles and Authorizations
  https://help.sap.com/docs/joule/serviceguide/roles-and-authorizations
  source_owner: SAP SE
  topic_supported: BTP role collections required for Joule access, role-aware context configuration per SAP solution, administrator versus end-user role separation
  why_needed: Defines the BTP role model for Joule access and role-aware answer context — required to assess whether role-aware configuration is correctly set up and whether role misconfiguration can cause over-disclosure of business data
  evidence_level: primary
  last_verified: 2026-06-19

## SAP AI Core — Security (Joule foundation)

- SAP AI Core Security and Data Protection
  https://help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/security-and-data-protection
  source_owner: SAP SE
  topic_supported: AI Core resource group isolation, data residency, security controls for AI model inference underlying Joule, data protection for AI-generated content
  why_needed: Joule is built on SAP AI Core infrastructure. This reference is required to assess the governance of Joule's underlying AI model access, resource group isolation, and data residency compliance at the AI infrastructure layer
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Trust Center — AI Ethics and Responsible Use

- SAP AI Trust Center
  https://www.sap.com/about/trust-center/ai.html
  source_owner: SAP SE
  topic_supported: SAP AI ethics principles, responsible AI framework, human oversight requirements, AI use policy guidance for SAP AI-powered products including Joule
  why_needed: Reference for assessing Joule adoption governance against SAP's responsible AI framework — defines SAP's stated human oversight requirements and AI ethics principles, which inform acceptable-use policy and over-trust risk assessment
  evidence_level: supplementary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal documentation and the SAP Trust Center describe the designed Joule capability model, authorization integration, data protection framework, and responsible AI principles. They do not prove which Joule skills are currently activated in the user's tenant, whether role-aware configuration is correctly implemented, or whether an acceptable-use policy has been adopted. Users must supply Joule configuration documentation, skill activation lists, data access boundary descriptions, audit log configuration summaries, acceptable-use policy documents, or written governance posture descriptions for concrete assessment.
