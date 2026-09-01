# Context Engineering And Tool Integration

Context assembly, grounding, token budgets, and MCP server governance.

- Context engineering is the selection, chunking, and assembly of grounding data for the agent's LLM calls. Sound design pairs the retrieval context size (token count) with the model's context window and the prompt's other uses (system prompt, tool definitions, conversation history).
- Chunking strategy affects retrieval quality: fixed-size windows are simple but may split semantic units; semantic chunking (via embeddings or NLP) preserves meaning but requires additional compute.
- Context budget (the token count allocated to retrieval results) must be set explicitly; the agent does not auto-limit retrieval based on model context, so a budget that is too large creates latency and cost.
- MCP (Model Context Protocol) servers are categorized by governance: managed MCP (Databricks-hosted for Genie, AI Search, Unity Catalog functions, SaaS connectors) are governed through Unity Catalog; external MCP (third-party over managed OAuth) delegate auth to the provider; custom MCP (Databricks Apps) require hosting and lifecycle.
- MCP servers on Databricks are governed through Unity Catalog for access control and through Unity AI Gateway for monitoring and policy. A tool defined as an MCP server is governed by these scopes.
- Unity Catalog functions can be exposed as agent tools directly. The agent's privilege to call the function is the same as the caller's privilege — the caller must have EXECUTE on the function and read privilege on the underlying data.
- External model providers supported on Databricks are exactly: OpenAI (including Azure OpenAI), Anthropic, Cohere, Amazon Bedrock, Google Cloud Vertex AI, Databricks Model Serving, and custom OpenAI-compatible proxies. Other providers (Gemini, Claude not through Bedrock) are not supported.
- Unity AI Gateway provides rate limiting, traffic splitting (useful for A/B testing model variants), fallbacks (to secondary providers if primary fails), budget management (per-token or per-minute caps), and request/response content policies (e.g., PII masks, input/output filters).

## MCP Server Category And Governance Scope

| Category | Hosting | Governance | Authentication | Use Case |
|---|---|---|---|---|
| Managed | Databricks-hosted | Unity Catalog access control | Databricks identity | Genie, AI Search, UC functions, SaaS connectors |
| External | Third-party server | Provider OAuth | Provider credentials | GitHub, Jira, other SaaS with OAuth |
| Custom | Databricks App | Unity Catalog access control | Databricks identity | Internal tools, proprietary functions |
