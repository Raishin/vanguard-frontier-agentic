---
name: sap-ai-core-generative-ai-hub-governance
description: Governance review for SAP AI Core, AI Launchpad, and Generative AI Hub deployments. Assesses model access control, data privacy for RAG pipelines and embeddings, prompt-injection risk in orchestration configurations, grounding data classification, prompt-log handling, and auditability of AI outputs. Does not run models or access production AI deployments.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: ai
  lifecycle: experimental
---

# SAP AI Core Generative AI Hub Governance

## Purpose

Assess the governance posture of SAP AI Core, AI Launchpad, and Generative AI Hub deployments. Review model access control: resource group isolation, role assignments in AI Launchpad, and AI API service key scope. Review data privacy for RAG pipelines: classification of grounding data sources, embedding storage location, data residency of vector stores, and personal data exposure in retrieval results. Assess prompt-injection risk in orchestration configurations: unguarded template variables, absent input/output content safety filters, and missing grounding data sanitization. Review prompt-log handling and retention: whether prompts containing personal or confidential data are logged, where logs are stored, and whether log access is role-restricted. Assess auditability of AI outputs: traceability of model responses to grounding sources, audit trail completeness, and model version pinning. Does not invoke models, connect to live AI Core deployments, or process production data.

## When to use

Use this skill when the user asks to:

- review AI Core resource group structure and role assignment for model access isolation,
- assess AI Launchpad user role configuration: `aicore_admin`, `aicore_viewer`, `genai_manager`, `genai_viewer` role assignments and separation of duties,
- evaluate Generative AI Hub orchestration configuration for prompt-injection risk: input/output content safety filter coverage, unguarded template variable injection points, and system-prompt integrity,
- audit grounding data classification: whether documents in the Document Grounding Service or vector store are classified by data sensitivity before ingestion, whether personal data or confidential business data is ingested into shared vector stores,
- review prompt-log handling: whether prompt logging is enabled, log retention period, log access controls, and compliance with data residency requirements for logged prompts containing personal data,
- assess embedding and vector store data privacy: which data is vectorized, where embeddings are stored, cross-tenant vector store isolation, and right-to-erasure implications for embedded personal data,
- evaluate auditability of Generative AI Hub outputs: model version pinning, grounding source attribution in responses, audit log completeness for AI-assisted decisions, and explainability requirements.

## When not to use

- When the request requires connecting to a live AI Core deployment, running model inference, or accessing production prompt logs — this skill reviews configuration artifacts and governance descriptions only.
- When the request is about BTP account-level governance (entitlements, subaccounts, role collections) unrelated to AI services — use `sap-btp-governance-review`.
- When the request is about integration wiring between AI Core and other SAP services via Integration Suite — use `sap-integration-suite-review`.
- When the request is about training custom ML models on BTP AI Core (pipeline configuration, Docker images, custom training workflows) rather than Generative AI Hub governance.

## Does not touch live systems

This skill operates on user-provided orchestration configuration files, role assignment descriptions, resource group layouts, grounding pipeline configuration, data classification documentation, or written descriptions of the AI governance posture. It does not invoke any Generative AI Hub model, access AI Core deployment APIs, read prompt logs, query vector stores, or connect to AI Launchpad. All live inspection is out of scope.

## Lean operating rules

- Classify governance findings before recommending. Every finding must be assigned to a governance domain (Model Access / Data Privacy / Prompt Injection Risk / Grounding Data / Prompt Logging / Auditability) before remediation is proposed.
- Content safety filters are required on both input and output for customer-facing orchestration pipelines. An orchestration configuration without `filtering_module_config` on both the input and output path is a `high` finding for prompt injection and harmful output risk.
- Grounding data classification must precede ingestion. Ingesting unclassified documents into a vector store shared across resource groups or tenants is a `high` finding for data privacy. Personal data in grounding documents must be identified before vectorization.
- Prompt logging of personal data requires legal basis and residency compliance. If prompts submitted by end users may contain personal data (names, IDs, health data, financial data), prompt logging must be evaluated against the applicable data protection regulation (GDPR, local law). Unreviewed prompt logging with no retention limit is a `medium` finding.
- Embedding personal data creates right-to-erasure obligations. Personal data vectorized into embeddings cannot be trivially erased. Any grounding pipeline ingesting personal data must have a documented erasure procedure or must exclude personal data from vectorization.
- Model version must be pinned for AI-assisted decisions. An orchestration configuration using `latest` or a floating model version alias for any workflow that produces auditable business outputs is a `medium` finding for reproducibility.
- Resource group isolation is the primary access control boundary. All model deployments, vector stores, and grounding pipelines must be assigned to the appropriate resource group. Cross-resource-group access without documented justification is a `high` finding.
- Evidence from official SAP AI Core, AI Launchpad, and Generative AI Hub documentation takes precedence over inference.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in official SAP AI Core, AI Launchpad, or Generative AI Hub documentation (help.sap.com)
- `user-provided evidence` — orchestration configuration files, role assignment descriptions, grounding pipeline design, data classification documentation, or written descriptions provided by the user
- `context7-supplementary` — grounded in SAP Generative AI Hub orchestration configuration patterns from Context7 (supplementary; applies for content safety filter configuration and grounding module structure)
- `inference` — derived reasoning not directly confirmed by official docs or user evidence

## Live-environment rules and data privacy rules

**This skill does not touch live systems.** There is no AI Core API call, Generative AI Hub model invocation, vector store query, prompt log access, or AI Launchpad session in this skill's execution path.

**Data privacy rules for this review skill:**

- Do not request, accept, or process actual prompt logs, user message history, or inference outputs from production deployments.
- Do not request actual grounding documents, embeddings, or vector store contents. Accept only descriptions of data classification and pipeline configuration.
- Do not request AI Core service keys, OAuth tokens, or resource group credentials.
- When the user describes a grounding pipeline that handles personal data, advise on governance controls without requesting the underlying data.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — governance domain taxonomy, severity classification, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, common AI governance mistakes, when to push back.
- [Official sources](references/official-sources.md) — SAP AI Core, AI Launchpad, Generative AI Hub, and Document Grounding Service documentation.
- [Context7 framework docs](references/context7-framework-docs.md) — SAP Generative AI Hub orchestration configuration patterns (supplementary; applies for content safety filter and grounding module configuration).

## Response minimum

Return, at minimum:

- **Problem classification**: governance domain(s) in scope and specific finding(s) per domain.
- **Evidence used**: documentation-based / user-provided evidence / context7-supplementary / inference.
- **Risk level**: critical (regulatory breach, data exfiltration, or security bypass risk) / high (governance control gap with material privacy or security impact) / medium (audit or compliance gap) / low (best practice deviation).
- **Recommended action**: specific governance control per finding (add content safety filter, classify grounding data before ingestion, restrict prompt log access, pin model version, define erasure procedure for embedded personal data, etc.).
- **Refusal / escalation triggers**: if live AI Core access, prompt log inspection, or vector store content review is required to complete the governance assessment, state that clearly and do not proceed. If the configuration may contain personal data, do not accept it directly — request a redacted or anonymized description instead.
- **Business impact**: regulatory exposure (GDPR, local data protection law), AI output reliability risk, audit trail gap, or reputational risk from harmful model output.
- **Next verification step**: validate recommended governance controls against current SAP AI Core configuration documentation and applicable data protection regulation before deploying changes.
