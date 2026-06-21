# Workflow and output contract — SAP Data Privacy Analytics AI Protocol

Use this reference for all trigger classification, data assessment sequencing, cross-function handoff coordination, decision rights application, and output formatting.

## Trigger classification table

| Trigger class | Primary signal | Activating agent(s) | Data Privacy involvement | Legal involvement |
|---|---|---|---|---|
| `pii-in-analytics-pipeline` | PII columns in Datasphere data product or SAC story | sap-datasphere-data-product-architect-agent + sap-analytics-cloud-planning-governance-agent | Required (legal basis and minimization review) | Conditional (cross-border transfer or breach threshold) |
| `sensitive-business-data-in-export` | Confidential metrics in analytics export to external destination | sap-analytics-cloud-planning-governance-agent | Required (purpose limitation review) | Required (if third-party or cross-border) |
| `rag-pipeline-ingestion` | PII or sensitive data being indexed for RAG | sap-datasphere-data-product-architect-agent + sap-ai-core-genai-hub-governance-reviewer-agent | Required (pre-ingestion approval gate) | Conditional (cross-border or special category data) |
| `embedding-generation` | SAP data converted to embeddings for AI use | sap-ai-core-genai-hub-governance-reviewer-agent | Required (source data classification review) | Conditional (if source includes HR or financial data requiring special treatment) |
| `prompt-logging-with-pii` | AI system logs capturing user context with PII | sap-ai-core-genai-hub-governance-reviewer-agent | Required (retention and minimization review) | Required if logs retained beyond policy period |
| `genai-hub-deployment-review` | New or expanding Generative AI Hub deployment | sap-ai-core-genai-hub-governance-reviewer-agent | Required (system prompt and grounding data review) | Conditional (if sensitive business data in system prompt) |
| `joule-activation-governance` | Joule activation for new business function | sap-joule-governance-adoption-agent | Required (grounding data scope and PII handling review) | Conditional (if HR or payroll data in scope) |
| `model-output-leakage` | Generative AI output reconstructs or reveals sensitive data | sap-ai-core-genai-hub-governance-reviewer-agent | Required (assess source data and retrieval path) | Required (potential data breach determination) |

## Protocol workflow

### Phase 1 — Triage (AI Governance or Data Privacy lead)

1. Classify the trigger condition using the table above.
2. Identify which participating agents are activated.
3. Request the required evidence inventory from the responsible function.
4. Apply the redaction policy before any cross-function evidence share.

### Phase 2 — Data classification and legal basis assessment

1. `sap-datasphere-data-product-architect-agent` classifies data elements in Datasphere data products and confirms data access control configuration.
2. `sap-analytics-cloud-planning-governance-agent` assesses SAC model and story data classification and export control settings.
3. Data Privacy Officer confirms legal basis for each PII processing activity in scope and assesses purpose limitation compliance.
4. If cross-border data transfer is involved, Legal is notified immediately for transfer impact assessment.

### Phase 3 — AI pipeline and deployment review

1. `sap-ai-core-genai-hub-governance-reviewer-agent` reviews AI Core deployment configuration, Generative AI Hub orchestration, system prompt content, grounding data scope, and logging configuration.
2. `sap-joule-governance-adoption-agent` reviews Joule grounding data scope, user context injection design, and output handling for the business function in scope.
3. Logging and retention configuration is compared against approved retention period and data minimization requirements.

### Phase 4 — Approval gate evaluation

1. Each required approval is identified from the decision rights table.
2. Approval documentation is confirmed present or flagged as outstanding.
3. Irreversible-action gate is evaluated for each proposed action — no action proceeds without confirmed written approval.

### Phase 5 — Audit package assembly

1. All evidence, data classification results, legal basis confirmations, approval records, redaction logs, and action outcomes are consolidated into the audit package.
2. Residual risks are assessed and documented with compensating controls.

## Data classification labels

Use these labels consistently across all evidence and output:

| Label | Meaning |
|---|---|
| `PII-direct` | Data element that directly identifies a natural person (name, email, employee ID, national ID) |
| `PII-indirect` | Data element that, combined with other available data, could identify a natural person |
| `special-category` | Personal data warranting heightened protection under GDPR Art 9 or equivalent (health, ethnic origin, union membership, biometric, etc.) |
| `sensitive-business` | Confidential business data (revenue forecasts, M&A, margin, compensation benchmarks, pricing strategy) |
| `restricted` | Data accessible only to named roles under the organization's data classification policy |
| `internal` | Data for internal use only; not approved for external sharing or AI grounding without review |
| `public` | Data approved for unrestricted use |

## Output contract

Return, in order:

1. **Trigger classification**: Which trigger class(es) apply; which participating agents are activated.
2. **Data inventory and classification**: Data sources assessed, classification label per element, and any PII-direct, PII-indirect, special-category, or sensitive-business elements identified.
3. **Legal basis status**: Legal basis confirmed, pending, or absent for each PII processing activity. If absent, this is a blocking condition — escalate immediately.
4. **Redaction confirmation**: Explicit statement that sensitive elements have been identified and the redaction policy has been applied.
5. **AI pipeline assessment** (if applicable): Grounding data scope, logging configuration compliance, system prompt classification, embedding source classification, and retrieval path risk.
6. **Approval gate status**: For each required approval, primary authority, documentation status (present / outstanding), and gate status (cleared / blocked).
7. **Irreversible-action gate**: Whether any irreversible actions are pending, approval status, and whether the gate is cleared or blocked.
8. **Escalation notice**: If PII is present in production AI logs without legal basis, sensitive business data is in a RAG index without data owner approval, or a cross-border transfer is occurring without a transfer mechanism — escalation notice must appear before any other recommendation.
9. **Audit package status**: Populated items and outstanding items.
10. **Next step**: Single next action with named responsible owner.
