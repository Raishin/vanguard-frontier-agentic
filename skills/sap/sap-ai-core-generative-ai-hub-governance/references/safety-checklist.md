# Safety checklist — SAP AI Core Generative AI Hub Governance

Use before making any finding or recommendation, especially for data privacy and prompt-injection risk findings.

## Non-negotiables

- Do not access, connect to, or request access to any live AI Core deployment, Generative AI Hub orchestration endpoint, vector store, or AI Launchpad session.
- Do not accept or request AI Core service keys, OAuth tokens, resource group credentials, or BTP service binding secrets.
- Do not accept or request actual prompt logs, user message history, inference output from production deployments, or grounding document contents. Request only descriptions of data classification and pipeline configuration.
- Do not accept vector store embeddings or raw grounding data. If the user attempts to share this, redirect them to provide a description of the data types and classification status instead.
- Do not recommend disabling content safety filters (`filtering_module_config`) as a performance optimization. Missing input or output content safety filters is a high or critical finding.
- Do not recommend storing prompt logs in a region different from where personal data was collected without advising the user to review applicable data transfer regulations (GDPR Chapter V, local laws).
- Do not validate data residency compliance from memory alone. Direct the user to verify against the current SAP AI Core data residency documentation for their contracted region.
- Do not conflate AI Core resource group isolation with BTP subaccount tenant isolation. Resource groups provide workload isolation within a subaccount; cross-tenant isolation requires subaccount-level separation.

## What people get wrong

- **Assuming content safety is on by default**: SAP Generative AI Hub orchestration does not enable content safety filters by default. `filtering_module_config` must be explicitly added to the orchestration configuration for both input and output paths.
- **Treating grounding output as trusted**: Grounding documents retrieved from a vector store may contain injected content if the ingestion pipeline accepted untrusted external documents. Injected content in `{{ ?groundingOutput }}` can manipulate model behavior (indirect prompt injection). Grounding output is not automatically safe to inject into system messages.
- **Overlooking right-to-erasure for embeddings**: Once personal data is vectorized and stored in a vector store, deleting the source document does not delete the embedding. A documented erasure procedure for the embedding itself is required to comply with right-to-erasure requests.
- **Assuming resource group isolation = tenant isolation**: Multiple tenants sharing a single BTP subaccount with different resource groups are not fully isolated. Subaccount-level isolation is needed for full tenant separation of AI workloads.
- **Ignoring metadata exposure**: When `metadata_params` is configured to return `source` and `webUrl` from grounding retrieval, this information is visible in the orchestration response. Internal URLs, document IDs, or access-controlled source identifiers exposed in responses may constitute an information disclosure finding.
- **Floating model versions in auditable workflows**: Using `model_version: "latest"` in an orchestration configuration means model behavior can change without notice. For any workflow where AI outputs are used in auditable business decisions, the model version must be pinned and changes must go through a change management process.

## When to push back

- Push back when the user asks to review actual prompt logs or grounding document contents — request a description of the data types and classification status instead.
- Push back when the user proposes to disable content safety filters to improve response latency on customer-facing pipelines.
- Push back when the user proposes to use a single shared resource group for multiple application teams with different data access levels.
- Push back when the user attempts to ingest personal data into a vector store without a documented erasure procedure.
- Push back when the request requires live AI Core API access or production deployment inspection — this skill is advisory only.
- Push back when the user proposes logging prompts containing personal data without a documented retention policy and legal basis.

## Evidence labels

- `documentation-based` — grounded in official SAP AI Core, AI Launchpad, or Generative AI Hub documentation (help.sap.com)
- `user-provided evidence` — orchestration configuration files, role assignment descriptions, grounding pipeline design, data classification documentation, or written descriptions provided by the user
- `context7-supplementary` — SAP Generative AI Hub orchestration configuration patterns (content safety filter structure, grounding module config) sourced from Context7 (supplementary to official SAP docs)
- `inference` — derived reasoning not directly confirmed; must always be labeled as such
