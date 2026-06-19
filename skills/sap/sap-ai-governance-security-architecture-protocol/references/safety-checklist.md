# Safety checklist — SAP AI Governance / Security / Architecture Protocol

Use before finalizing any finding, handoff package, or escalation trigger. This checklist is mandatory for all advisory sessions involving AI workload deployments, data scope changes, content filter modifications, or regulatory compliance assessments.

## Non-negotiables

- Do not accept or process OAuth tokens, API keys, AI Core service keys, resource group credentials, or certificate private keys. Refuse immediately and request redaction before proceeding.
- Do not accept system prompt content containing proprietary business logic, internal instructions, or trade-secret content beyond the structural minimum needed to assess a specific governance risk (e.g., whether user input is interpolated into a system prompt without sanitization).
- Do not accept PII — customer names, personal identification numbers, health data, payment card data — in example prompts, RAG grounding document samples, or model output excerpts. Request redaction before processing.
- Do not invoke or recommend invoking any guarded-mutating operator gate from within this protocol. All operator gates are human-mediated only. This protocol produces the package; a human submits it.
- Do not recommend removing content filtering from an inferencing endpoint without first identifying the compensating control that will replace it. Content filter removal without a compensating control immediately exposes all consumers of that endpoint.
- Do not classify a finding as `critical` without tracing the specific threat to user-provided evidence or official SAP/OWASP/NIST documentation. Severity inflation without evidence is not permitted.
- Do not produce regulatory compliance determinations (EU AI Act high-risk classification, GDPR lawful basis assessment, SOX materiality judgment) as authoritative legal advice. Label such assessments as `inference` and recommend engagement with legal counsel or a qualified compliance officer.
- Do not evaluate the content safety of specific AI-generated outputs (e.g., whether a particular model response is harmful or biased). This protocol assesses governance posture, access controls, and auditability; content safety evaluation of individual outputs is out of scope.

## What people get wrong

- **Conflating resource group isolation with data isolation**: SAP AI Core resource groups provide workload separation within a tenant, but they do not automatically enforce data-level access controls for grounding data in vector stores or object storage. A RAG pipeline that stores all customer data in a single HANA Cloud vector store without row-level or schema-level access controls is not data-isolated, even if the AI Core resource group is scoped correctly.
- **Treating the Generative AI Hub as a safe transmission channel for all SAP data**: The Generative AI Hub routes inference requests to foundation models, which may be hosted by third-party providers (OpenAI, Anthropic, Google, etc.) under SAP data processing agreements. The specific data processing agreement terms, data residency, and model training opt-out status must be confirmed for each foundation model before transmitting regulated or sensitive data. Do not assume all Generative AI Hub models are covered by the same data governance terms.
- **Underestimating indirect prompt injection in RAG pipelines**: Prompt injection via user input (direct injection) is well-understood. Indirect prompt injection — where malicious instructions are embedded in a document that is retrieved as grounding context — is less commonly assessed but equally dangerous in RAG architectures. Any SAP scenario that ingests externally sourced documents (supplier invoices, customer emails, third-party reports) into a RAG grounding store is potentially exposed to indirect injection.
- **Assuming Joule is read-only**: Joule can execute actions in SAP applications depending on the skills and permissions configured. A Joule skill that is granted write access to SAP business data (creating purchase orders, approving workflows, modifying master data) has execution scope, not just retrieval scope. The governance review must assess the action scope of each Joule skill, not just its data access scope.
- **Missing the SoD risk in AI admin role assignments**: AI platform administrator roles (AI Core admin, Generative AI Hub admin) are often granted to a small group of platform engineers. If the same individuals can both deploy models and approve model governance decisions, this is a SoD violation in the AI governance process. Separate the model deployment operator role from the model governance approval role.
- **Conflating model version control with model governance**: Capturing the model version in deployment metadata is necessary but not sufficient for auditability. Full auditability of AI-generated outputs in regulated processes requires capturing the model version, the input (prompt + grounding context), and the output at the time of generation — not just the deployed model version.

## When to push back

- Push back when the user submits a complete system prompt without redaction of proprietary business logic or internal instructions. Request the minimum structural excerpt needed to assess the specific governance risk.
- Push back when the user asks for a legal determination on EU AI Act risk category, GDPR lawful basis, or SOX materiality. Provide a governance framework assessment; recommend legal counsel for authoritative determination.
- Push back when the user asks to directly execute an AI Core model deployment, Joule skill reconfiguration, or content filter change from within this protocol. This protocol is advisory only; redirect to the operator gate with the completed handoff package.
- Push back when no architecture description or access control inventory has been provided. AI governance findings without evidence of the workload topology are unreliable; request the minimum topology description before proceeding.
- Push back when the user asks to assess content safety of a specific AI-generated output (e.g., "is this response harmful?"). This is out of scope; refer to SAP's responsible AI policies and appropriate content safety tooling.
- Push back when a request requires live access to SAP AI Launchpad, AI Core APIs, or Joule administration interfaces. This protocol accepts only user-provided artifacts; live system access is out of scope.

## Evidence labels

- `documentation-based` — grounded in SAP AI Core documentation, SAP Generative AI Hub documentation, SAP Joule documentation, SAP HANA Cloud vector engine documentation, NIST AI RMF, OWASP LLM Top 10, or ISO/IEC 42001
- `user-provided evidence` — architecture descriptions, access control inventories, compliance scope declarations, audit log configurations, or workload descriptions supplied by the user
- `inference` — derived reasoning not directly confirmed by official documentation or user evidence; always label explicitly, note the assumption, and recommend validation before acting on the finding
