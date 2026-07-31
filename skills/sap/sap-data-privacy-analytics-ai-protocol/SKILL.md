---
name: sap-data-privacy-analytics-ai-protocol
description: Cross-functional coordination protocol governing data privacy, analytics, and AI governance handoffs in SAP landscapes. Covers PII in Datasphere data products, analytics exports from SAP Analytics Cloud, RAG pipelines, embedding generation, prompt logging, SAP Generative AI Hub usage, Joule adoption, and model outputs that could leak sensitive business data. Defines data classification, consent and purpose limitation gates, approval requirements, and audit packaging. Never mutates live systems and never bypasses any guarded-mutating gate.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-19"
  category: compliance
  lifecycle: experimental
---

# SAP Data Privacy Analytics AI Protocol

## Purpose

This skill defines the cross-functional coordination and handoff contract between Data Privacy, Analytics, and AI Governance functions when SAP analytics or AI workloads involve personal data, sensitive business data, or outputs that could leak confidential information. It establishes data classification gates, consent and purpose limitation requirements, approved data flow paths for AI pipelines, logging and retention obligations for AI interactions, and approval requirements before sensitive data enters any analytics export, embedding pipeline, RAG index, or generative AI prompt.

This is a governance coordination document. It does not mutate any Datasphere space, SAP Analytics Cloud model, AI Core deployment, Generative AI Hub configuration, or Joule skill. All mutations remain gated behind the relevant guarded-mutating operator agents. No guarded gate is bypassed.

## When to use

Invoke this protocol when any of the following trigger conditions apply:

- **PII in analytics pipelines**: A Datasphere data product, SAP Analytics Cloud story, or exported dataset includes columns that contain or could derive personal identifiable information (name, email, employee ID, national ID, HR attributes, financial account identifiers, or any combination that constitutes indirect PII).
- **Sensitive business data in analytics exports**: Analytics exports from SAP Analytics Cloud, Datasphere, or integrated data sources include revenue forecasts, M&A pipeline data, margin data, compensation benchmarks, or any data classified as confidential or restricted under the organization's data classification policy.
- **RAG pipeline data ingestion**: A retrieval-augmented generation pipeline is being designed or reviewed that would index Datasphere data products, SAP Analytics Cloud content, ERP master data, or HR data for use as grounding context for a large language model.
- **Embedding generation from SAP data**: SAP data (ERP documents, HR records, financial statements, customer records) is being converted into vector embeddings for storage in a vector database used by an AI model.
- **Prompt logging and retention**: An AI system built on SAP Generative AI Hub, Joule, or a custom AI Core deployment logs user prompts or model responses in a way that may capture sensitive data from system context, few-shot examples, or user input.
- **Generative AI Hub usage review**: A team is deploying or expanding usage of SAP Generative AI Hub and the prompt designs, model configurations, or orchestration flows have not been reviewed for data privacy compliance or confidential data leakage risk.
- **Joule adoption governance**: Joule is being activated for a new business function (HR, Finance, Procurement, Sales) and the grounding data, user context injection, and output logging approach have not been assessed for PII handling, data minimization, and consent requirements.
- **Model outputs that could leak data**: A generative AI model produces outputs (generated text, structured responses, summaries, recommendations) that may contain or reconstruct sensitive data from training or retrieval context.

## Participating agents

The following agents operate within this protocol. Each holds a defined role and must not act outside its lane without explicit cross-function approval per the decision rights table.

- `sap-datasphere-data-product-architect-agent` — Specialist for Datasphere space governance, data product design, data access control layer (DAC), data sensitivity classification within Datasphere, and approved data flow paths. Does not modify live Datasphere spaces or data products.
- `sap-analytics-cloud-planning-governance-agent` — Specialist for SAP Analytics Cloud model governance, story-level data access controls, export policy compliance, and business intelligence data classification. Does not modify live SAC models or export configurations.
- `sap-ai-core-genai-hub-governance-reviewer-agent` — Specialist for SAP AI Core deployment governance, Generative AI Hub model and orchestration review, prompt design compliance, logging configuration assessment, and AI model output risk review. Does not modify live AI Core deployments or Generative AI Hub configurations.
- `sap-joule-governance-adoption-agent` — Specialist for Joule skill activation governance, grounding data scope review, user context injection assessment, PII handling in Joule interactions, and adoption readiness for business functions. Does not modify live Joule configurations.

## Required evidence

Before any cross-function handoff is initiated, the following evidence must be assembled by the requesting function:

1. **Data source inventory**: List of all SAP data sources involved (Datasphere spaces and data products, SAP Analytics Cloud models and stories, ERP datasets, HR datasets), with system identifiers, data classification level, and data owner.
2. **Data flow diagram**: Documented path from source data through any transformation, export, embedding generation, RAG indexing, or AI prompt construction step to the final output or consumer.
3. **PII and sensitivity assessment**: Identification of all personal data elements (direct and indirect PII) and sensitive business data elements present in each data source in scope. Assessment performed against the organization's data classification policy.
4. **Consent and purpose documentation**: Legal basis for processing personal data in analytics or AI workloads (consent records, legitimate interest assessment, or contractual basis), and confirmation that the stated AI or analytics purpose is within the scope of the original collection purpose.
5. **Logging and retention configuration**: Description of what is logged by the AI system (prompts, responses, retrieved context, user identifiers), where logs are stored, retention period, and access controls on log storage.
6. **Model and orchestration configuration**: For AI Core or Generative AI Hub deployments, the model ID(s) in use, orchestration flow, grounding data scope, system prompt content (with any sensitive business context highlighted), and whether model output is cached or stored.
7. **Export and sharing controls**: For analytics exports, the destination (external system, third party, public dataset), the access control applied to the export, and whether re-identification risk has been assessed.

## Redaction policy

Sensitive data elements must be treated as follows before evidence is shared across functions or included in audit documentation:

- **PII in data flow diagrams**: Replace specific field values with data type labels (e.g., `[EMAIL]`, `[EMPLOYEE_ID]`, `[SALARY]`). Include field names and classification levels, not actual data values.
- **System prompt content**: If the system prompt for a Generative AI Hub or AI Core deployment includes confidential business data (pricing rules, M&A targets, compensation bands), the prompt is shared only with the AI governance function and Legal. Operational teams see a classification summary, not the full prompt text.
- **Prompt logs**: Prompt log samples shared for governance review must be anonymized — remove user-identifying information, replace actual data values with type labels, and limit samples to the minimum needed for the review.
- **Embedding vectors**: Raw vector data is not shared outside the AI engineering function. The governance review covers the source data classification and the access control on the vector store, not the vectors themselves.
- **SAC story data**: Live story data containing confidential business metrics is not included in audit packages. The audit package references the story ID, data model, and classification level.

## Decision rights

| Decision | Primary authority | Secondary approval required | Notes |
|---|---|---|---|
| Classify a data element as PII or sensitive | Data Privacy Officer (or delegate) | Data Owner confirmation | Classification is advisory input to all downstream decisions |
| Approve a data product for AI/analytics use with PII | Data Privacy Officer | Legal (if cross-border data transfer involved) | Required before data enters any RAG index or embedding pipeline |
| Approve a Generative AI Hub deployment for a business function | AI Governance lead | Data Privacy Officer review | Required before production deployment |
| Approve Joule activation for a new business function | AI Governance lead + Business Function owner | Data Privacy Officer (if HR or payroll data in scope) | Required before Joule grounding data scope is expanded |
| Approve an analytics export containing sensitive business data | Data owner | Business unit CISO review | Required for external or third-party destinations |
| Approve prompt logging that captures user context with PII | Data Privacy Officer | AI Governance lead | Logging configuration must meet retention and minimization requirements |
| Suspend or restrict a running AI deployment | AI Governance lead | CISO (if security risk) | Routes through sap-ai-core-genai-hub-governance-reviewer-agent advisory output; execution is separate |
| Approve a cross-border data transfer for AI training or inference | Legal + Data Privacy Officer jointly | — | Requires transfer impact assessment for GDPR-regulated data |

## Escalation owners

| Scenario | First escalation owner | Second escalation owner |
|---|---|---|
| PII discovered in production AI prompt logs without approved legal basis | Data Privacy Officer | Chief Privacy Officer / General Counsel |
| Sensitive business data (M&A, pricing) found in RAG index without data owner approval | Chief Information Security Officer | General Counsel |
| Generative AI model output reconstructs confidential data | AI Governance lead | CISO + Chief Privacy Officer |
| Cross-border data transfer for AI inference without transfer mechanism | Legal | Chief Privacy Officer |
| Joule grounding data includes HR compensation or performance data without HR and DPO approval | Data Privacy Officer | Chief People Officer |
| Prompt logs retained beyond approved retention period | Data Privacy Officer | Legal |
| Third-party analytics export containing undisclosed PII | Data Privacy Officer + Legal jointly | — |

## Irreversible-action gate

The following actions are classified as irreversible or high-consequence and must not proceed without the approvals listed:

- **Indexing PII-containing data into a RAG vector store in production**: Requires Data Privacy Officer approval confirming legal basis, data minimization review, and access control validation on the vector store. Once indexed, retrieval paths are difficult to audit fully; approval must precede indexing.
- **Activating Joule for HR or payroll business functions**: Requires Data Privacy Officer review of the grounding data scope and user context injection design, plus HR lead confirmation that Joule responses will not expose compensation or performance data to unauthorized users.
- **Publishing a Datasphere data product containing PII as a federated source for AI pipelines**: Requires data owner sign-off and Data Privacy Officer approval. Publishing cannot be reversed without understanding all downstream consumers already connected.
- **Deploying a Generative AI Hub orchestration flow with a system prompt containing confidential business data**: Requires AI Governance lead approval and Data Privacy Officer review of the system prompt. Prompt content may be cached by intermediate infrastructure; deployment is effectively irreversible without a full redeployment.
- **Exporting an analytics dataset containing sensitive business data to an external or third-party destination**: Requires data owner and CISO approval. Data shared externally cannot be recalled; re-identification risk assessment must be completed before export.

This protocol does not execute any of these actions. It surfaces the required approvals and routes the action to the appropriate advisory output; execution remains with the relevant operator agent or system administrator.

## Approval requirements

All cross-function data governance actions under this protocol require written approval before execution. The audit package must include the approval record for every gate cleared.

Minimum approval quorum per action class:

- **Data classification decisions**: Data Privacy Officer sign-off with data owner confirmation.
- **AI pipeline approvals involving PII**: Data Privacy Officer + AI Governance lead.
- **Analytics export to external/third-party**: Data owner + CISO.
- **Cross-border data transfer for AI**: Legal + Data Privacy Officer jointly.
- **Joule or Generative AI Hub production deployment with sensitive grounding data**: AI Governance lead + Data Privacy Officer + (if HR data) HR lead.

## Audit package

The audit package for every protocol invocation must contain:

1. **Scope summary**: Business function, data sources in scope, AI or analytics workload description, and trigger condition.
2. **Data inventory**: All data elements assessed, classification level per element, data owner, and legal basis for processing (for PII).
3. **Data flow diagram**: Documented and reviewed path from source to consumer, with PII and sensitive data elements labeled per the redaction policy.
4. **PII and sensitivity assessment**: Results of the PII classification exercise, indirect PII combinations identified, and sensitive business data categories flagged.
5. **Consent and purpose documentation**: Legal basis records and purpose limitation confirmation.
6. **Logging and retention configuration review**: What is logged, retention period, access controls, and Data Privacy Officer confirmation of compliance.
7. **Approval records**: Written approvals for each gate cleared, with approver identity, date, and scope of approval.
8. **Irreversible-action gate log**: For each irreversible action, the gate status (cleared / blocked / pending) and the approval documentation reference.
9. **Escalation log**: Each escalation event, who was notified, when, and the response received.
10. **Residual risk statement**: Any data privacy or confidentiality risks not fully mitigated, the rationale for acceptance, and the compensating controls in place.

## Refusal conditions

This protocol and all participating agents must refuse the following requests:

- Approving the indexing of PII or sensitive business data into a RAG pipeline without documented Data Privacy Officer approval and confirmed legal basis.
- Sharing unredacted system prompt content containing confidential business data with operational teams.
- Confirming that an AI deployment is "privacy compliant" without a completed data flow assessment and Data Privacy Officer sign-off.
- Bypassing any irreversible-action gate listed in this protocol.
- Accepting verbal approval for any cross-function data governance action.
- Advising that prompt logs containing PII may be retained indefinitely — all prompt log retention must be bounded and consistent with the approved purpose and applicable data protection law.
- Recommending that a cross-border data transfer for AI inference proceed without a completed transfer impact assessment where GDPR or equivalent regulation applies.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP Datasphere, SAP Analytics Cloud, SAP AI Core, SAP Generative AI Hub, or SAP Joule documentation; or in GDPR, NIST AI RMF, ISO/IEC 42001, or equivalent regulatory guidance
- `user-provided evidence` — data flow diagrams, data classification outputs, system prompt excerpts (redacted), prompt log samples, deployment configurations, or written descriptions provided by the requesting function
- `inference` — derived reasoning not directly confirmed by official documentation or user-provided evidence; must always be labeled as such

## Response minimum

Every protocol invocation must return, at minimum:

- **Trigger classification**: Which trigger condition(s) apply and which participating agents are activated.
- **Data inventory status**: Which data sources have been assessed, which PII and sensitive data elements have been identified, and which assessments are outstanding.
- **Redaction confirmation**: Confirmation that sensitive data elements have been identified and the redaction policy has been applied to shared evidence.
- **Legal basis status**: Whether a legal basis for PII processing in the analytics or AI workload has been confirmed, is pending, or is absent.
- **Approval gate status**: For each required approval, who holds authority, what documentation is required, and whether the gate is cleared or blocked.
- **Irreversible-action gate status**: Whether any irreversible actions are pending, what approvals are outstanding, and whether the gate is cleared to proceed.
- **Logging and retention compliance status**: Whether the AI system's logging configuration has been reviewed and confirmed compliant with approved retention periods and data minimization requirements.
- **Escalation notice**: If PII is present in production AI logs without legal basis, if sensitive business data is in a RAG index without approval, or if a cross-border transfer is occurring without a transfer mechanism — the escalation notice must appear before any other recommendation, naming the escalation owners.
- **Audit package status**: Which audit package elements are populated and which are outstanding.
- **Next step**: The single next action the requesting function must take, with the responsible owner named.
