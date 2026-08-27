# Workflow And Output

Diagnostic sequence and output contract for agent-architecture review.

## Workflow

1. Establish the agent framework (OpenAI SDK, LangGraph, LangChain, LlamaIndex, plain Python) and confirm ResponsesAgent wrapping.
2. Audit the retrieval index: which AI Search variant is used, which sync mode is configured, and whether they match the data-update frequency.
3. Confirm context engineering: chunking strategy (fixed-size windows, semantic splitting), grounding data source, context budget in tokens, and prompt assembly.
4. Inventory tools: which Unity Catalog functions are called (with privilege scope), which MCP servers are used (with category and governance), and any external functions.
5. Validate the model provider: confirm it is supported (OpenAI, Anthropic, Cohere, Bedrock, Vertex AI, Model Serving, custom proxy), and note any custom proxy requiring schema compatibility.
6. Review Unity AI Gateway policy: rate limits, traffic splits, content policies, and logging destination and cadence.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (sound / cautions / block) and the agent framework and ResponsesAgent wrapping confirmed.
- AI Search index variant/sync-mode, context engineering, tool inventory, model provider, and gateway policy findings.
- A severity-labelled finding list (critical / high / medium / low) with evidence-basis labels and safe next actions.
