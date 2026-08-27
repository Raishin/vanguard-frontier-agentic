# Routing Taxonomy And Worked Examples

The seven classification axes, the four routing outcomes, and worked examples showing how ambiguous requests resolve.

- The seven axes are read in order and the first one that disqualifies a route wins: implied runtime authority above T0 sends the task to the live-guard gate before any domain scoring happens, because a correct domain answer to a mutation request is still the wrong outcome.
- "Why did our Databricks bill spike?" routes first to `databricks-finops-cost-agent` because it owns the evidence source (`system.billing.usage`, `system.billing.list_prices`); `databricks-platform-reliability-agent` follows when the evidence points at compute or job behaviour, and `databricks-sql-performance-agent` follows when it points at warehouse query cost. Routing straight to a suspected cause bakes in an unverified hypothesis.
- "Give the analysts SELECT on all production catalogs" is a mutation request wearing a question's clothes. It routes to `databricks-unity-catalog-governance-agent` for the privilege-model design and to `databricks-identity-network-security-agent` for the principal design, in parallel; the broad catalog-wide grant is rejected on the design side, and execution — if any narrower grant survives review — is reachable only through the live-guard gate with named approval and rollback.
- "Our agent quality dropped after yesterday's release" routes first to `databricks-genai-evaluation-observability-agent` because it owns the trace and judge evidence; `databricks-genai-agent-engineering-agent` follows once the failing component is identified, and `databricks-developer-platform-agent` joins only if the release mechanism itself (bundle target, promotion path) is implicated. The value specialist joins only if a KPI impact with a baseline already exists.
- A parallel route is capped at four specialists. A task that appears to need five is under-specified, not genuinely five-domain: return `unclassified` and ask for the artifact that narrows it.
- `unclassified` is a successful outcome, not a failure. It is strictly better than a confident wrong route, because a wrong route costs the specialist's full evidence cycle before the mistake surfaces.
- Confidence is reported explicitly. A low-confidence route ships with the discriminating question attached so the human can correct it in one hop instead of discovering the misroute in the specialist's output.

## Sources

- https://docs.databricks.com/aws/en/lakehouse-architecture/
- https://docs.databricks.com/aws/en/admin/system-tables/
