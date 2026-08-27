# Workflow And Output

Diagnostic sequence and output contract for evaluation and observability review.

## Workflow

1. Establish the tracing instrumentation strategy: which APIs or auto-instrumentation decorators are used, and which spans are captured.
2. Confirm trace storage choice: experiment-based or Unity Catalog Delta tables. If Delta tables, confirm SQL-query access and governance requirements.
3. Review the evaluation dataset: schema, expected-response or expected-facts definitions, and consistency of expectations across eval samples.
4. Audit judge selection: name each judge, confirm it is from the 17 built-in set, and confirm configuration (LLM model for judges like Correctness) is documented.
5. Confirm judge validation status: do human labels exist on a holdout set for this judge? If so, report inter-rater agreement. If not, flag the judge as un-validated.
6. For regression detection: confirm the evaluation dataset, judge selection, and judge configuration are held constant across the two releases being compared. Name any confounding factors (new data, feature flags, environment changes) introduced between releases.
7. For human feedback: confirm feedback is collected on production traces, validated for inter-rater agreement, and bias-checked before use in expectations.
8. Audit cost/latency observability: confirm data sources (traces, system tables) and aggregation cadence.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (sound / cautions / block) and the tracing instrumentation strategy and trace-storage choice confirmed.
- Judge selection list, judge validation status, and regression-detection confounding-factor audit.
- A severity-labelled finding list (critical / high / medium / low) with evidence-basis labels and safe next actions.
