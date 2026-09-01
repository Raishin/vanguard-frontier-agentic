# Workflow And Output

Diagnostic sequence and output contract for AI/BI Genie and dashboard design review.

## Workflow

1. Establish agent scope: name the 30 tables/views the agent is scoped to, and check instruction count (<=100). Refuse-and-ask if config is missing.
2. Review metric-view definitions: confirm measures, dimensions, and sources are correctly defined; flag parameters and window measures as PUBLIC PREVIEW.
3. Check trusted assets: confirm parameterized SQL queries are designed with exact-text matching in mind (whitespace matters).
4. Validate dashboard configuration: count pages (<=15), datasets (<=100), widgets per page (<=100), and peak row rendering (<=10k for charts, <=100k for tables).
5. Interpret benchmark results: report the LLM-judge confidence (88.1% +/- 5.5%), Cohen's kappa (0.64 +/- 0.13), and explain that <85% is within margin of error (not validation).
6. Review data permissions: 'Individual data' (row filters/masks applied per viewer) versus 'Share data' (row filters/masks completely bypassed). Flag 'Share data' as requiring executive sign-off.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (pass / pass-with-conditions / block) and agent scope (table count, throughput) assumed.
- Agent scoping, metric-view, dashboard limit, benchmark, and permission findings with evidence-basis labels.
- Severity-labelled security findings (critical / high / medium / low) and safe next actions.
- Explicit findings on 'Individual data' versus 'Share data' permission and executive sign-off status.
- Any agent config, metric definition, or security review gaps that would change the verdict.
