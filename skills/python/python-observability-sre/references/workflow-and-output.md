# Review Workflow And Output Contract

The observability review workflow and the required output shape.

## Workflow

1. Identify the telemetry stack (logging, metrics, tracing library and exporters) and every point that emits request or exception data.
2. Check for secrets or PII in logs, spans, and metric labels, and confirm redaction at the boundary.
3. Check metric/span-attribute cardinality and confirm trace context propagates across service calls, thread-pool/executor, and async boundaries.
4. Check the error taxonomy (classification, structured context) and that logs are structured and correlated with trace/span ids.
5. Check instrumentation actually measures the stated SLOs at the right boundary, and record every claim needing a real backend to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the telemetry stack assumed.
- Secrets/PII, cardinality, context-propagation, error-taxonomy, and SLO-instrumentation findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any cardinality/cost or trace-completeness claim the user must confirm against a real backend.
