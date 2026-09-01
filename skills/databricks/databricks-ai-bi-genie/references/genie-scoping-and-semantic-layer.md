# Genie Agent Scoping And Semantic Layer

Genie agent limits, metric-view correctness, trusted assets, and the semantic layer as grounding for natural-language accuracy.

- A Genie agent is limited to 30 tables or views, 10,000 conversations per agent, 10,000 messages per conversation, 100 instructions per agent, and 20 questions per minute per workspace throughput. Exceeding the table limit requires a documented request and approval.
- Metric views define sources, measures, and dimensions and generate correct SQL at runtime; core metric views are GA, metric-view parameters are PUBLIC PREVIEW (June 2026), and window measures are PUBLIC PREVIEW (August 2026). Local metric views are PUBLIC PREVIEW.
- Trusted assets are parameterized SQL queries and SQL functions; when the parameterized query text matches exactly, the response is marked verified. Exact-text matching means whitespace and formatting matter.
- Column comments do not sync from external tables; materialized views are the documented workaround for defining a semantic layer over external data.
- Removing an agent author invalidates embedded credentials (if the agent uses a PAT or credential owned by that author).

## Sources

- https://docs.databricks.com/aws/en/ai-bi/admin
- https://docs.databricks.com/aws/en/genie-agents/set-up
- https://docs.databricks.com/aws/en/business-semantics/metric-views/
- https://docs.databricks.com/aws/en/uc-semantics/
