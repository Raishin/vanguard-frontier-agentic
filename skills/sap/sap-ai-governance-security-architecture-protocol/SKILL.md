---
name: sap-ai-governance-security-architecture-protocol
description: Cross-functional coordination protocol governing handoff contracts between SAP AI Governance, Security, and Architecture. Activates on AI Core workloads, Generative AI Hub deployments, Joule governance events, RAG design reviews, prompt-injection risk, data-leakage risk, model-access control gaps, and auditability of AI-generated outputs. Advisory and audit only — no live mutation.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: ai
  lifecycle: experimental
---

# SAP AI Governance / Security / Architecture Protocol

## Purpose

This skill is a cross-functional coordination and handoff contract between four complementary advisory roles: SAP AI Core and Generative AI Hub governance review, Joule governance and adoption assessment, SAP security/IAM/GRC/SoD analysis, and SAP CAP architecture review. It defines when each role activates, what evidence each role requires before it can produce output, how findings are handed off between roles, who holds decision rights over AI workload deployments and access control changes, and what approval is required before any irreversible change is recommended to a downstream guarded-mutating operator.

This protocol never mutates. It never invokes or bypasses any guarded-mutating operator gate. It produces governance advisory packages, risk assessments, and escalation triggers only.

## When to use

Activate this protocol when one or more of the following conditions apply:

- An AI Core workload is being planned, deployed, or reviewed — including custom model deployments, fine-tuning jobs, inferencing endpoints, or resource group configurations on SAP AI Core.
- A Generative AI Hub integration is in scope — including proxy model consumption via the Generative AI Hub API, foundation model selection, token budget governance, or cost attribution for generative AI usage.
- Joule is being governed or adopted — including Joule skill configuration, Joule integration with SAP applications, scope of Joule access to SAP business data, and change management for AI-assisted business process execution.
- A RAG (Retrieval-Augmented Generation) design is under review — including document grounding strategy, vector store selection (SAP HANA Cloud vector engine or external), embedding model governance, data ingestion pipeline design, and retrieval access controls.
- Prompt injection risk has been identified or reported — any scenario where user-controlled or externally sourced text may influence an LLM system prompt, tool call, or agent reasoning path in an SAP AI workload.
- Data leakage risk exists — where sensitive SAP business data (customer records, financial data, HR data) may be transmitted to a foundation model, embedded in a vector store without access controls, or exposed via an AI-generated output channel.
- Model-access control gaps have been found — including missing authentication on AI Core inferencing endpoints, overly broad role assignments to Generative AI Hub consumers, or absent content filtering on model outputs.
- Auditability of AI-generated outputs is required — where regulatory, compliance, or internal audit requirements mandate that AI-generated business decisions, documents, or recommendations are traceable to model version, input, and output artifacts.

## Participating agents

The following agents are parties to this protocol. Each operates in its own advisory domain. No agent in this list holds authority to execute irreversible changes unilaterally.

- `sap-ai-core-genai-hub-governance-reviewer-agent` — reviews SAP AI Core resource group design, model deployment governance, Generative AI Hub proxy configuration, token budget controls, and cost attribution. Primary entry point for AI workload governance reviews.
- `sap-joule-governance-adoption-agent` — assesses Joule skill configuration, business process scope, data access permissions granted to Joule, user adoption governance, and change management for AI-assisted workflows.
- `sap-security-iam-grc-sod-reviewer-agent` — reviews identity and access management, GRC policy compliance, segregation of duties (SoD) conflicts in AI role assignments, and security architecture for AI workloads. Activated for any finding involving authentication, authorization, content filtering, or regulatory compliance.
- `sap-cap-architecture-reviewer-agent` — reviews SAP Cloud Application Programming model (CAP) service design where CAP services expose AI capabilities, provide grounding data for RAG pipelines, or act as orchestration layers between Joule/Generative AI Hub and SAP business data. Activated when the AI workload has a CAP service layer.

## Required evidence

Before this protocol can produce a handoff package, the activating party must supply:

1. **AI workload description** — a written description of the AI workload or scenario under review, including the SAP AI product(s) involved, the business use case, and the data sources accessed.
2. **Architecture diagram or topology description** — a description or diagram of how the AI components interact: model source, grounding data path, orchestration layer, output consumers, and human-in-the-loop controls.
3. **Data classification** — what categories of SAP business data are in scope (e.g., customer PII, financial records, HR data, public catalog data) and their classification level under the organization's data classification policy.
4. **Access control inventory** — current role assignments for AI Core resource groups, Generative AI Hub scopes, Joule skill permissions, and any CAP service authorization policies.
5. **Compliance scope** — which regulatory frameworks or internal compliance requirements apply to this workload (e.g., EU AI Act risk category, GDPR, SOX, industry-specific AI governance policies).

Optional but recommended:

- Existing threat model or security assessment for the AI workload.
- Prompt template or system prompt (redacted of sensitive instructions and internal context if necessary).
- Token budget configuration and cost attribution setup.
- Audit log configuration for AI-generated output traceability.

## Redaction policy

All evidence submitted to this protocol must be redacted before processing:

- Remove all OAuth tokens, API keys, service keys, and AI Core resource group credentials.
- Remove all personally identifiable information (PII) from example prompts, RAG grounding documents, and model output samples.
- Remove all proprietary system prompt content, internal business logic, and trade-secret instructions from prompt templates beyond what is necessary to assess the governance risk.
- Remove all customer names, personal identification numbers, and financial account data from example payloads.
- Label redacted fields with `[REDACTED]`.

This protocol does not store, persist, or transmit evidence artifacts or prompt content. Evidence is used only within the advisory session in which it is provided.

## Decision rights

| Decision | Owner | Protocol role |
|----------|-------|---------------|
| Approve AI Core model deployment to production | AI Platform Lead + CISO or delegate | Protocol produces risk assessment; human approves |
| Approve Generative AI Hub foundation model selection | AI Governance Board or delegate | Protocol produces model governance assessment; human approves |
| Approve Joule skill activation for a business process | Business Process Owner + Joule Governance Lead | Protocol produces scope and risk summary; human approves |
| Approve RAG grounding data pipeline to production | Data Owner + AI Platform Lead + Privacy Officer | Protocol produces data governance assessment; human approves |
| Approve role assignment to AI Core resource group | IAM Lead + AI Platform Lead | Protocol produces SoD and least-privilege finding; human approves |
| Remediate prompt injection risk in a deployed endpoint | AI Platform Lead + Security Lead | Protocol produces remediation recommendation; human approves and executes |
| Approve content filtering configuration | AI Platform Lead + CISO | Protocol produces configuration recommendation; human approves |
| Mandate auditability controls for AI-generated outputs | Compliance Officer + AI Governance Board | Protocol produces auditability gap assessment; human mandates controls |

## Escalation owners

- **AI Governance Board or delegate** — primary escalation point for foundation model selection, token budget governance, and AI policy compliance.
- **CISO or Security Lead** — escalation point for prompt injection risk, data leakage risk, model-access control gaps, and content filtering findings.
- **Data Protection Officer / Privacy Officer** — escalation point for PII in grounding data, GDPR-relevant AI processing, and cross-border data transfer in model inference.
- **Compliance Officer** — escalation point for regulatory compliance (EU AI Act categorization, SOX impact of AI-generated financial outputs, industry-specific AI governance).
- **AI Platform Lead** — escalation point for AI Core resource group design, Generative AI Hub configuration, and deployment governance.
- **Joule Governance Lead** — escalation point for Joule skill scope, business process impact of AI-assisted execution, and Joule adoption change management.
- **Enterprise Architect** — escalation point for CAP service design defects, RAG architecture risks, and AI workload topology findings.

## Irreversible-action gate

The following actions are classified as irreversible or high-consequence and require explicit human approval before any recommendation is executed:

- Deploying a custom model to a production AI Core resource group (cannot be instantly rolled back without re-deploying the previous version).
- Granting a Joule skill access to a new category of SAP business data (immediately expands AI's data access scope; rollback requires skill reconfiguration and redeployment).
- Enabling a new foundation model in the Generative AI Hub for production consumer access (immediately available to all authorized consumers).
- Removing content filtering from an AI Core inferencing endpoint (immediately eliminates output safety controls for all consumers of that endpoint).
- Deleting or overwriting a vector store index used for RAG grounding (immediate data loss; requires full reingestion to restore).
- Revoking an AI Core resource group credential used by a production AI application (immediately breaks all applications using that credential).

This protocol does not invoke any guarded-mutating operator gate. It produces a signed handoff package. A human must present that package to the appropriate operator gate and confirm approval before any mutation proceeds.

## Approval requirements

| Action type | Minimum approvers | Approval form |
|-------------|------------------|---------------|
| Production model deployment | AI Platform Lead + CISO or delegate | Written sign-off in change record with risk assessment attached |
| Foundation model selection change | AI Governance Board | Board approval record with governance assessment |
| Joule skill data scope expansion | Business Process Owner + Privacy Officer | Written approval with data classification review |
| RAG grounding pipeline production promotion | Data Owner + AI Platform Lead + Privacy Officer | Written sign-off with data flow diagram attached |
| Content filtering configuration change | AI Platform Lead + CISO | Written approval with threat model reference |
| AI Core credential rotation | AI Platform Lead | Written approval with rotation procedure and consumer impact assessment |

## Audit package

At session close, this protocol produces an audit package containing:

1. **Workload summary** — AI products in scope, business use case, data classification, and compliance frameworks.
2. **Evidence inventory** — list of evidence artifacts received (types only; no raw content), with redaction confirmation.
3. **Per-role findings** — advisory output from each activated agent role, labelled by evidence level, risk tier, and governance domain.
4. **Threat model summary** — key identified threats (prompt injection, data leakage, model access, output auditability) with likelihood and impact assessment.
5. **Compliance gap summary** — gaps against declared regulatory frameworks (EU AI Act, GDPR, SOX, internal AI policy).
6. **Decision rights mapping** — pending decisions and named approvers for each recommended action.
7. **Irreversible-action gate status** — whether any irreversible action has been recommended, and whether human approval has been confirmed.
8. **Escalation log** — which escalation owners were notified and what action was requested.
9. **Handoff status** — whether the package is ready to submit to a guarded-mutating operator gate.

The audit package must be retained for the duration of the AI workload's production lifecycle and for the post-decommission retention period required by applicable regulatory frameworks.

## Refusal conditions

This protocol refuses to proceed when:

- No AI workload description or architecture topology has been provided — advisory output without evidence is not reliable.
- A request is made to directly invoke any guarded-mutating operator gate from within this protocol — all operator gates are human-mediated only.
- Raw OAuth tokens, API keys, or AI Core resource group credentials are present in submitted evidence without redaction — request redaction before proceeding.
- A full system prompt containing proprietary business logic or internal instructions is submitted without redaction — accept only the minimum structural elements needed to assess governance risk.
- A request is made to recommend an irreversible action without identifying the human approver — the approver must be named before the package is finalized.
- A request is made to assess the safety or harmlessness of specific AI-generated content — this protocol assesses governance posture, not content safety of individual outputs.
- The trigger event is outside the scope defined in "When to use" — redirect to the appropriate protocol or skill.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP AI Core documentation, SAP Generative AI Hub documentation, SAP Joule documentation, SAP CAP documentation, NIST AI RMF, ISO/IEC 42001, or OWASP LLM Top 10.
- `user-provided evidence` — architecture diagrams, access control inventories, compliance scope declarations, audit log configurations, or workload descriptions supplied by the user.
- `inference` — derived reasoning not directly confirmed by official documentation or user evidence; always label as inference and note the assumption.

## Response minimum

Every protocol session must return, at minimum:

- **Trigger classification** — which "When to use" condition(s) activated the protocol.
- **Participating roles activated** — which of the four agents' advisory domains are relevant to this session.
- **Evidence received** — inventory of what was supplied, with confirmation of redaction compliance.
- **Threat model summary** — key threats identified (prompt injection / data leakage / model access / auditability) with confidence level.
- **Findings per domain** — advisory output for each activated role, with evidence labels and risk tier.
- **Compliance gap summary** — gaps against declared regulatory frameworks.
- **Decision rights table** — who must approve each pending action.
- **Irreversible-action gate status** — whether irreversible actions are in scope and whether human approval has been confirmed.
- **Audit package readiness** — whether the session is complete enough to produce a handoff package.
- **Next step** — the specific human action required to proceed (e.g., "Present this package to the AI Governance Board and CISO for production model deployment approval").
