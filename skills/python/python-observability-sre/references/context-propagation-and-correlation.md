# Context Propagation And Log Correlation

OpenTelemetry context propagation across services, threads, and async boundaries, and log-trace correlation.

- OpenTelemetry propagates context via a global textmap propagator that injects/extracts across service boundaries (extract from the inbound request, inject into the outbound call).
- Context must also be carried across thread-pool/executor and async coroutine boundaries or correlation is lost.
- Structured logs carrying the trace/span id correlate logs to traces.

## Sources

- https://opentelemetry.io/docs/concepts/context-propagation/
- https://opentelemetry-python.readthedocs.io/en/stable/
