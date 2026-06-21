# Workflow and output contract — SAP AI Core Generative AI Hub Governance

Use this reference for all classification, severity assignment, and output formatting.

## Governance domain taxonomy

| Domain | Scope |
|--------|-------|
| `Model Access` | Resource group isolation, AI API service key scope, model deployment access, separation of duties between aicore_admin/genai_manager/genai_viewer roles |
| `Data Privacy` | Classification of grounding data sources, personal data in prompts and embeddings, embedding storage location, data residency compliance, right-to-erasure for vectorized data |
| `Prompt Injection Risk` | Unguarded template variables in orchestration configs, system-prompt integrity, input/output content safety filter coverage, grounding output sanitization |
| `Grounding Data` | Data classification before ingestion, cross-resource-group vector store sharing, sensitive data in shared vector stores, metadata exposure in retrieval results |
| `Prompt Logging` | Prompt logging enablement state, retention period, access controls on log storage, compliance with data residency for logged prompts containing personal data |
| `Auditability` | Model version pinning, grounding source attribution in responses, audit log completeness for AI-assisted business decisions, explainability documentation |

## Severity classification

| Severity | Meaning | Examples |
|----------|---------|---------|
| `critical` | Regulatory breach, data exfiltration, or security bypass risk | Personal data ingested into a shared cross-tenant vector store with no isolation; orchestration endpoint accessible without authentication; prompt logs containing personal data stored outside the contracted data region |
| `high` | Governance control gap with material privacy or security impact | No content safety filters on input or output for a customer-facing orchestration pipeline; confidential business data in grounding documents with no access control on the vector store; `aicore_admin` role granted to end-users without separation of duties |
| `medium` | Audit or compliance gap | Model version not pinned (`latest` alias used) for auditable AI-assisted decisions; prompt logging enabled with no documented retention limit; no grounding source attribution in orchestration output for compliance-relevant responses |
| `low` | Best practice deviation | Missing metadata classification on grounding documents (even for non-personal data); no documented model change approval process |

## Common finding patterns

### Model Access
- `aicore_admin` role assigned to the same user who operates grounding pipelines (high — violates separation of duties)
- AI API service key with `global` scope instead of resource-group-scoped key (high — over-permissioned key)
- Multiple application teams sharing a single resource group with no isolation (high)

### Prompt Injection Risk
- Orchestration configuration with no `filtering_module_config` on the input path (high — prompt injection unfiltered)
- Orchestration configuration with no `filtering_module_config` on the output path (high — harmful model output unfiltered)
- Prompt template containing `{{ ?userInput }}` injected directly into system message without sanitization (high)
- Grounding output (`{{ ?groundingOutput }}`) injected into prompt without content check (medium — indirect prompt injection via malicious grounding document)
- Content safety thresholds set to maximum permissiveness (0 = allow) for all categories (high)

### Grounding Data
- Documents ingested into vector store without prior data classification review (high if organization handles personal or confidential data)
- Vector store shared across resource groups or tenants without access control review (critical if different tenants)
- Employee or customer PII present in grounding documents (critical — right-to-erasure obligations for vectorized data)
- `metadata_params` exposing internal URLs or source identifiers to end-user output without review (medium)

### Prompt Logging
- Prompt logging enabled for a pipeline where end-user inputs may contain personal data, with no retention policy documented (medium to high depending on regulation)
- Prompt logs stored in a data region different from where the personal data was originally collected (critical if GDPR cross-border transfer rules apply)
- No role restriction on who can read prompt logs (high)

### Auditability
- Model version set to `latest` or floating alias for a pipeline that produces audit-relevant business output (medium)
- No grounding source citation in orchestration response for compliance-relevant use case (medium)
- No audit log entry for AI-assisted decisions that affect individual users (high for regulated industries)

## Workflow

1. **Receive artifacts** — orchestration configuration files, role assignment descriptions, resource group layouts, grounding pipeline design, data classification documentation, or user descriptions.
2. **Classify each finding** by governance domain above.
3. **Assign severity** (critical / high / medium / low).
4. **Identify evidence level** (documentation-based / user-provided evidence / context7-supplementary / inference).
5. **Recommend specific governance control** — content safety filter addition, data classification step, log access restriction, model version pin, erasure procedure.
6. **Prioritize** — critical and high severity first; data privacy before auditability; prompt injection risk before logging.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Artifacts reviewed and governance domains in scope
2. Finding(s) per domain with severity and evidence label
3. Specific governance control recommendation per finding
4. Data privacy summary — whether personal data was identified in grounding pipeline scope and what protections are in place
5. Content safety coverage summary — input and output filter status per orchestration pipeline reviewed
6. Prioritized remediation sequence (critical → high → medium → low)
7. Escalation trigger if live AI Core access, prompt log inspection, or vector store query is required to complete the governance assessment — do not proceed with live access
