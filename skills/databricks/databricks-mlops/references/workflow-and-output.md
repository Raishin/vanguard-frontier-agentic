# Workflow And Output

Diagnostic sequence and output contract for model-lifecycle review.

## Workflow

1. Establish the MLflow version, target registry (Workspace or Unity Catalog), and the three-level model namespace to be used.
2. Review the promotion strategy: which aliases (Champion, Challenger) are assigned and how traffic or serving endpoints route to them.
3. For feature-store design, confirm primary keys are present (composite allowed), TIMESERIES is set if point-in-time lookups are needed, and the schema is compatible with FeatureEngineeringClient.
4. Audit the serving-endpoint configuration: traffic-split percentage, provisioned-concurrency cap, scale-to-zero settings, and whether any direct-invocation paths bypass the traffic split.
5. For inference-table designs, confirm the output schema includes `databricks_request_id` or `client_request_id`, and flag any downstream analytics that assumes unique rows.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (sound / cautions / block) and the MLflow version and registry URI confirmed.
- Alias/promotion, feature-store correctness, serving-endpoint sizing, and inference-logging findings.
- A severity-labelled finding list (critical / high / medium / low) with evidence-basis labels and safe next actions.
