# Safety checklist — SAP Data Privacy Analytics AI Protocol

Use before any cross-function handoff, before any approval gate is cleared, and before any evidence is shared outside its originating function.

## Non-negotiables

- Do not approve or recommend approval of any RAG pipeline ingestion, embedding generation, or AI deployment involving PII without confirmed legal basis and Data Privacy Officer sign-off.
- Do not share unredacted system prompt content containing confidential business data (M&A, pricing, compensation) with operational teams or outside the AI governance and Legal functions.
- Do not confirm an AI deployment is "privacy compliant" without a completed data flow assessment, legal basis confirmation, and Data Privacy Officer written approval.
- Do not bypass the irreversible-action gate. Any action listed under that section of SKILL.md requires all listed written approvals before execution.
- Do not accept verbal approval for any cross-function data governance action. Written approval is mandatory.
- Do not advise that prompt logs containing PII may be retained indefinitely. All prompt log retention must be bounded by an approved retention period consistent with the processing purpose and applicable data protection law.
- Do not recommend a cross-border data transfer for AI inference or training without a completed transfer impact assessment where GDPR or equivalent regulation applies.
- Do not share raw vector embedding data outside the AI engineering function. The governance review covers source data classification and vector store access controls, not the vectors themselves.

## What people get wrong

- **Treating data minimization as optional for AI grounding**: Data minimization is a legal requirement under GDPR and equivalent laws, not an optimization. Grounding a RAG pipeline with a full Datasphere data product that includes PII when only non-personal business attributes are needed is a legal compliance gap, not just a technical choice.
- **Assuming Generative AI Hub model providers do not retain prompt data**: Model providers may retain prompt and completion data for safety or improvement purposes per their terms. The governance review must confirm the data processing terms for each model in use, especially for models accessed via the Generative AI Hub that are hosted by third parties.
- **Conflating a SAC export approval with a data privacy assessment**: A business user approving an analytics export does not constitute a data privacy review. Export of PII or sensitive business data to an external destination requires a separate Data Privacy Officer review and, where applicable, a transfer impact assessment.
- **Forgetting that Joule injects user context from connected SAP systems**: Joule retrieves context from connected SAP systems (HR, Finance, Procurement) to generate responses. If Joule is activated for a business function that has access to HR compensation or performance data, the grounding data scope includes that data — even if the intent was limited to a different use case.
- **Using analytics exports as a proxy for AI training data without review**: A SAC story or Datasphere export that is acceptable for internal business analytics may not be acceptable as AI training data without additional de-identification, legal basis review, and data owner approval.
- **Assuming AI output leakage only occurs through direct retrieval**: A generative AI model can reconstruct or infer sensitive data from patterns in its training data or retrieval context even when the original data is not directly quoted. Model output leakage risk assessment must consider indirect reconstruction, not only verbatim retrieval.

## When to push back

- Push back (and escalate immediately) when PII is discovered in production AI prompt logs without a confirmed legal basis for processing — escalate to the Data Privacy Officer before any other action.
- Push back when asked to approve RAG indexing or embedding generation involving PII without Data Privacy Officer written approval.
- Push back when asked to confirm that a system prompt containing confidential business data is safe without reviewing the prompt content and AI deployment configuration.
- Push back when a cross-border data transfer for AI inference or training is proposed without a transfer impact assessment.
- Push back when asked to clear an irreversible-action gate without all required written approvals documented.
- Push back when asked to confirm legal basis from memory — require Data Privacy Officer confirmation grounded in a records-of-processing-activities entry or equivalent documentation.
- Push back when Joule is proposed for activation on an HR or payroll business function without HR lead and Data Privacy Officer review of the grounding data scope.

## Evidence labels

- `documentation-based` — grounded in SAP Datasphere, SAP Analytics Cloud, SAP AI Core, SAP Generative AI Hub, SAP Joule, GDPR, NIST AI RMF, or ISO/IEC 42001 documentation
- `user-provided evidence` — data flow diagrams, data classification outputs, system prompt excerpts (redacted), prompt log samples, deployment configuration descriptions, or legal basis documentation provided by the requesting function
- `inference` — derived reasoning not directly confirmed by official documentation or user-provided evidence; must always be labeled as such and must not be used as the sole basis for any approval gate clearance
