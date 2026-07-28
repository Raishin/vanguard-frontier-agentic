# Observability Review Checklist

The per-concern checklist applied to every observability review.

- PII/secrets: no secret or personal data appears in logs, spans, or metric labels; scrub at the boundary before emission.
- Cardinality: metric labels and span attributes stay low-cardinality; high-cardinality identifiers go on traces/exemplars only.
- Propagation: trace context is injected/extracted across service calls and carried across thread-pool/executor and async boundaries.
- Errors: exceptions are classified (retryable vs terminal) and logged with structured context, not swallowed or logged uniformly.
- Logs: structured logging carries the trace/span correlation id for every request.
- SLOs: each SLI is measured at the boundary and with the aggregation the stated SLO actually needs.
