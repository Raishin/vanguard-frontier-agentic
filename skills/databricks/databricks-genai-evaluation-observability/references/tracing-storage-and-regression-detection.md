# MLflow Tracing Storage And Regression-Detection Constraints

Trace storage choice, governance, and constraints for sound regression detection.

- MLflow Tracing storage defaults differ by version: experiment-based storage (legacy MLflow 2 path) retains traces in MLflow's backing store and is queryable via the MLflow API; Unity Catalog storage (MLflow 3+, `system.traces.*` OpenTelemetry Delta tables) stores traces indefinitely in Delta format, is SQL-queryable, and is governed by Unity Catalog access control.
- The `system.traces.payload` and `system.traces.metadata` Delta tables are OpenTelemetry-compliant and hold full trace data with no storage cap or retention limit (unlike experiment-based traces which are tied to experiment lifecycles).
- Production observability should use Unity Catalog trace storage for durability, governance, and SQL queryability. Experiment-based storage is acceptable for dev/staging but carries retention risk in production.
- Regression detection requires constancy across runs: the evaluation dataset, the judge and scorer selection, judge configuration (LLM model for judges), and expectation definitions must be identical. Any deviation (e.g., dataset drift, judge model upgrade, new feature flag) introduces confounding.
- Trace tags set via `mlflow.set_trace_tag(key, value)` provide rich context for regression analysis (e.g., model version, release date, user segment) and enable filtering in regression runs. Require minimal tagging (release/version) for production traces.
- A regression claim that compares a live release with a prior release but does not control for data changes (new training examples, new domains), feature flags, or environment changes is confounded and is not a valid regression.
- The Correctness judge configuration (LLM model) must be held constant across regression runs. Upgrading a judge's underlying model changes what is measured; the comparison is no longer valid until the prior release is re-evaluated with the new judge model.
- Judge validation against human labels on a holdout set (computing inter-rater agreement scores) is the prerequisite for claiming that a judge-score movement is evidence of a real change; without this validation, a score movement is ambiguous and may reflect judge error rather than product change.
