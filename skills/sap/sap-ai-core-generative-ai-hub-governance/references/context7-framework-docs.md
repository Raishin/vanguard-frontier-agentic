# Context7 framework docs — SAP AI Core Generative AI Hub Governance

**Role**: supplementary. Official SAP AI Core and Generative AI Hub documentation (help.sap.com) is the primary source for all governance guidance. Context7-sourced Generative AI Hub documentation supplements with concrete orchestration configuration structure for content safety filtering and grounding module setup — both directly relevant to prompt-injection risk and grounding data governance.

**Library used**: SAP AI Core Generative AI Hub
Context7 library ID: `/websites/help_sap_sap-ai-core_generative-ai`
Lookup target: orchestration configuration, content safety filtering (filtering_module_config), grounding module (grounding_module_config), prompt template variables, metadata exposure in retrieval
Skill: `sap-ai-core-generative-ai-hub-governance`
Classification: supplementary — applies for prompt-injection risk and grounding data governance domains

---

## Orchestration — content safety filter configuration (supplementary)

Source: help.sap.com SAP AI Core Generative AI Hub (Context7 `/websites/help_sap_sap-ai-core_generative-ai`)
Reference: https://help.sap.com/docs/sap-ai-core/generative-ai/contextualized-retrieval-using-metadata-and-vector-search

Content safety filtering is configured via `filtering_module_config` in the orchestration request body. Both input and output paths must be configured independently:

```json
"filtering_module_config": {
    "input": {
        "filters": [
            {
                "type": "azure_content_safety",
                "config": {
                    "Hate": 2,
                    "SelfHarm": 2,
                    "Sexual": 2,
                    "Violence": 2
                }
            }
        ]
    },
    "output": {
        "filters": [
            {
                "type": "azure_content_safety",
                "config": {
                    "Hate": 2,
                    "SelfHarm": 2,
                    "Sexual": 2,
                    "Violence": 2
                }
            }
        ]
    }
}
```

**Governance relevance:**
- `filtering_module_config` must appear in every orchestration configuration that accepts user input or returns output to end users. Its absence is a `high` finding.
- The `input` filter screens prompts before they reach the model — primary defense against harmful input and some prompt injection patterns.
- The `output` filter screens model responses before they reach the caller — defense against harmful model output.
- Threshold values (0–6): lower values = more permissive (more content passes); higher values = more restrictive. Setting thresholds to 0 effectively disables filtering for that category — a `high` finding for customer-facing pipelines.
- `azure_content_safety` is the filter type available in the standard Generative AI Hub orchestration service. Governance reviews must confirm the filter type matches what the target deployment supports.

---

## Orchestration — grounding module and metadata exposure (supplementary)

Source: help.sap.com SAP AI Core Generative AI Hub (Context7 `/websites/help_sap_sap-ai-core_generative-ai`)
Reference: https://help.sap.com/docs/sap-ai-core/generative-ai/contextualized-retrieval-using-metadata-and-vector-search

The grounding module (`grounding_module_config`) retrieves documents from a vector store and injects them into the prompt via a named output parameter:

```json
"grounding_module_config": {
    "type": "document_grounding_service",
    "config": {
        "filters": [
            {
                "id": "filter1",
                "data_repositories": ["*"],
                "search_config": {},
                "data_repository_type": "vector"
            }
        ],
        "input_params": ["groundingRequest"],
        "output_param": "groundingOutput",
        "metadata_params": ["source", "webUrl"]
    }
}
```

**Governance relevance:**
- `data_repositories: ["*"]` queries all vector stores accessible in the resource group. Governance review should confirm this is intentional — if only specific repositories should be queried, explicit repository IDs should be listed.
- `metadata_params: ["source", "webUrl"]` exposes document source metadata in the orchestration response. Internal URLs, access-controlled document paths, or identifiers that reveal internal system topology may constitute an information disclosure finding if returned to end users without review.
- `output_param: "groundingOutput"` injects retrieved document content into the prompt template via `{{ ?groundingOutput }}`. If grounding documents can contain untrusted or externally sourced content, this injection point is a vector for indirect prompt injection. Review whether `{{ ?groundingOutput }}` appears in the system message (high risk) or user message (medium risk).
- `input_params: ["groundingRequest"]` takes the grounding query from a prompt template variable. Review whether this variable is user-controlled (`{{ ?userInput }}`) — if so, users can influence which documents are retrieved, potentially extracting information from the vector store beyond their intended scope.

---

## Scope boundaries for Context7 usage

Context7 SAP Generative AI Hub documentation applies to:

- **Prompt Injection Risk**: `filtering_module_config` structure, content safety threshold semantics, grounding output injection patterns
- **Grounding Data**: `grounding_module_config` structure, `data_repositories` scope, `metadata_params` exposure

It does not replace official SAP documentation for:
- AI Core resource group isolation and access control (use help.sap.com/docs/sap-ai-core/sap-ai-core-service-guide/resource-groups)
- AI Launchpad role definitions and separation of duties (use help.sap.com/docs/ai-launchpad/sap-ai-launchpad/roles-and-authorizations)
- Data residency and data protection requirements (use help.sap.com/docs/sap-ai-core/generative-ai/data-protection-and-privacy)
- Prompt logging behavior and retention (use help.sap.com/docs/sap-ai-core/generative-ai/data-protection-and-privacy)

Always label Context7-sourced guidance as `context7-supplementary` in responses.
