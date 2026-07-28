# Metric/Attribute Cardinality And PII Exposure

Cardinality control for metrics and spans, and redaction requirements for sensitive data.

- Metric label / span attribute cardinality drives backend cost and can overwhelm it, so high-cardinality identifiers belong on traces/exemplars, not metric labels.
- Secrets/PII must never be emitted to logs, spans, or labels and must be redacted at the boundary.
- Semantic conventions give stable attribute names for correlation.

## Sources

- https://opentelemetry.io/docs/specs/semconv/
- https://docs.python.org/3/library/logging.html
