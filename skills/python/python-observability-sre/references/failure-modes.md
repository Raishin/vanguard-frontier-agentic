# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- An access-token value logged at debug level ends up indexed in the log aggregator, readable by every engineer with log access.
- A raw request URL containing user ids, used as a metric label, explodes the metrics backend's cardinality and the dashboard stops loading.
- A request that crosses a thread-pool executor without propagating context loses its trace, so a slow downstream call can never be correlated back to the originating request.
- A blanket `except Exception: pass` around a payment call hides a failure from every alert until customers complain.
- A p99-latency SLO is defined but no span measures the boundary it claims to cover, so the SLO dashboard shows a number that doesn't correspond to real user experience.
