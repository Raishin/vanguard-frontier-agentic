# Workflow and output contract — SAP AI Governance / Security / Architecture Protocol

Use this reference for trigger classification, role activation logic, threat model structure, finding severity, governance domain taxonomy, handoff sequencing, and output format.

## Trigger classification taxonomy

| Trigger class | Description | Primary role activated |
|---------------|-------------|----------------------|
| `ai-core-workload` | AI Core model deployment, resource group design, or inferencing endpoint governance | sap-ai-core-genai-hub-governance-reviewer-agent |
| `genai-hub` | Generative AI Hub proxy configuration, foundation model selection, token budget, cost attribution | sap-ai-core-genai-hub-governance-reviewer-agent |
| `joule-governance` | Joule skill scope, data access permissions, adoption change management | sap-joule-governance-adoption-agent |
| `rag-design` | RAG architecture review: grounding data pipeline, vector store, embedding governance, retrieval access control | sap-ai-core-genai-hub-governance-reviewer-agent + sap-cap-architecture-reviewer-agent |
| `prompt-injection` | User-controlled or external text influencing system prompt, tool calls, or agent reasoning | sap-security-iam-grc-sod-reviewer-agent + sap-ai-core-genai-hub-governance-reviewer-agent |
| `data-leakage` | SAP business data transmitted to foundation model, embedded without access controls, or exposed in AI output | sap-security-iam-grc-sod-reviewer-agent + sap-ai-core-genai-hub-governance-reviewer-agent |
| `model-access-control` | Missing authentication on inferencing endpoints, overly broad role assignments, absent content filtering | sap-security-iam-grc-sod-reviewer-agent |
| `output-auditability` | Regulatory or audit requirement for AI-generated output traceability | All four roles activated; sap-security-iam-grc-sod-reviewer-agent leads compliance mapping |

## Governance domain taxonomy

| Domain | Scope | Typical findings |
|--------|-------|-----------------|
| `ai-workload-governance` | AI Core resource groups, model lifecycle, deployment controls, token budgets | Missing lifecycle gate for model promotion, unscoped resource group access, absent token budget enforcement |
| `data-governance` | Grounding data classification, PII in vector stores, cross-border AI inference, data retention for AI inputs/outputs | PII in RAG grounding data without access controls, unclassified data ingested into vector store |
| `access-control` | Authentication on inferencing endpoints, authorization for Generative AI Hub consumers, Joule skill scope, SoD in AI role assignments | Missing OAuth on AI Core endpoint, overly broad Generative AI Hub scope, SoD conflict in AI admin role |
| `threat-posture` | Prompt injection paths, insecure output handling, model supply chain risk, excessive agency in AI agents | Direct prompt injection via user input, indirect injection via grounding documents, LLM08 excessive agency in Joule |
| `auditability` | AI-generated output traceability, model version logging, input/output audit trail, human-in-the-loop controls | No audit log for AI-generated financial outputs, model version not captured in output metadata |
| `compliance` | EU AI Act risk category, GDPR AI processing obligations, SOX impact of AI outputs, ISO/IEC 42001 gap | High-risk AI system not assessed under EU AI Act, GDPR lawful basis not established for AI processing of personal data |

## Threat model structure

For each session, produce a threat model summary covering:

1. **Prompt injection** (OWASP LLM01) — assess whether user-controlled or externally sourced content can influence the system prompt, tool selection, or agent reasoning path. Confidence: confirmed / probable / possible / not applicable.
2. **Data leakage** (OWASP LLM06) — assess whether sensitive SAP business data (PII, financial, HR) is transmitted to a foundation model without adequate controls. Confidence as above.
3. **Insecure output handling** (OWASP LLM02) — assess whether AI-generated outputs are validated and sanitized before being consumed by downstream systems or presented to users. Confidence as above.
4. **Excessive agency** (OWASP LLM08) — assess whether the AI system (Joule, AI agent, CAP-orchestrated flow) can execute actions beyond what the business use case requires, or without adequate human-in-the-loop controls. Confidence as above.
5. **Model-access control gap** — assess whether inferencing endpoints, Generative AI Hub scopes, or Joule skill permissions are properly authenticated and least-privilege scoped. Confidence as above.
6. **Auditability gap** — assess whether AI-generated outputs that affect business decisions, financial records, or regulated processes are traceable to model version, input, and output artifacts. Confidence as above.

## Finding severity classification

| Severity | Criteria |
|----------|---------|
| `critical` | Active data leakage of PII or regulated data to an unapproved model; unauthenticated inferencing endpoint exposed to untrusted callers; confirmed prompt injection with demonstrated impact; regulatory breach (GDPR Article 83 exposure, SOX material impact from AI-generated financial output) |
| `high` | Prompt injection vector identified but not yet confirmed exploited; Joule skill scope exceeding business process requirements with no compensating control; RAG grounding data containing unclassified sensitive data; missing content filtering on a customer-facing AI endpoint |
| `medium` | Missing audit log for AI-generated outputs in a compliance-relevant process; SoD conflict in AI role assignment without mitigating control; overly broad Generative AI Hub consumer scope without usage monitoring; EU AI Act risk classification not performed for a deployed AI system |
| `low` | Missing token budget enforcement with no current cost overrun; model version not captured in output metadata but no compliance requirement mandating it; Joule adoption change management plan not documented |

## Workflow

1. **Classify trigger** — identify which trigger class(es) apply from the workload description and evidence.
2. **Activate relevant roles** — determine which of the four participating agent roles are relevant.
3. **Inventory evidence** — list all artifacts provided; confirm redaction compliance; request missing mandatory items.
4. **Produce threat model summary** — assess all six threat dimensions with confidence levels.
5. **Classify governance domain** — map each finding to the governance domain taxonomy.
6. **Produce per-role findings** — severity, evidence label, and recommended advisory action for each finding.
7. **Map compliance gaps** — assess findings against declared regulatory frameworks.
8. **Map decision rights** — for each recommended action, identify the named human approver.
9. **Gate irreversible actions** — for any finding involving a listed irreversible action, confirm human approval is required; do not pre-approve.
10. **Produce audit package** — assemble the audit package as defined in SKILL.md.

## Output contract

Return:

1. **Trigger classification** — which trigger class(es) activated, with evidence label.
2. **Roles activated** — which participating agent domains are in scope.
3. **Evidence inventory** — types of artifacts received; redaction confirmation; missing mandatory evidence.
4. **Threat model summary** — all six threat dimensions with confidence levels.
5. **Per-domain findings** — governance domain, severity, evidence label, and recommended advisory action for each finding.
6. **Compliance gap summary** — gaps against declared regulatory frameworks with applicable standard clauses.
7. **Decision rights table** — pending decisions mapped to named approver roles.
8. **Irreversible-action gate status** — list of any irreversible actions in scope; human approval status (confirmed / not yet / not applicable).
9. **Escalation log** — escalation owners to notify and the action requested from each.
10. **Audit package readiness** — complete / incomplete (with blocking items).
11. **Next human step** — specific action required to proceed.

## Handoff to guarded-mutating operator gate

This protocol does not invoke any guarded-mutating operator gate directly. When the audit package is complete and human approval has been confirmed for all irreversible actions, the protocol produces a handoff summary containing:

- Approved action type and scope (model deployment, Joule skill activation, content filter configuration, credential rotation).
- AI workload identifier (AI Core scenario ID, Generative AI Hub deployment name, Joule skill name).
- Governance domain and threat model basis for the action.
- Named approvers who confirmed the action.
- Timestamp of approval confirmation.
- Reference to the change record where approval is documented.

A human presents this handoff summary to the appropriate operator gate. The operator gate independently verifies approval before executing any mutation.
