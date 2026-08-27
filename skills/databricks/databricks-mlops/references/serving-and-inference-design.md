# Model Serving Endpoint And Inference-Table Design

Endpoint configuration, traffic control, inference logging, and at-least-once semantics.

- A Model Serving endpoint exposes one or more served entities. Each served entity is identified by name and routes inbound traffic via `traffic_config` with `traffic_percentage` and `served_entities` parameters; traffic is split across entities by percentage.
- Querying `POST /serving-endpoints/{name}/served-models/{served-model-name}/invocations` targets a specific served model directly and bypasses the traffic-split configuration.
- Provisioned concurrency caps the number of parallel requests an endpoint can serve; exceeding this cap throttles inbound requests. Require evidence of expected p99 concurrency and confirmation that provisioned-concurrency is set above it.
- Scale-to-zero reduces idle costs by shutting down instances when no traffic is detected for a period. Warm-start latency when traffic returns is typically 10–60 seconds depending on model size; a latency-sensitive SLO must be monitored before enabling in production.
- Route-optimized endpoints shorten network path by collocating serving compute with inference data; this is a networking optimization, not a model-selection change.
- Inference tables auto-log serving traffic to Unity Catalog Delta tables. The schema includes `databricks_request_id` (Databricks-assigned), `client_request_id` (caller-provided optional), `timestamp_ms` (request time), `status_code` (HTTP status), `execution_time_ms` (latency), `request` (JSON), and `response` (JSON).
- Inference-table delivery is AT-LEAST-ONCE, meaning a request may result in zero, one, or multiple log rows. Downstream analytics must deduplicate on request ID, not count rows as unique events.
- Inference logs appear in the Delta table within about one hour. Real-time serving metrics should not rely on inference-table content; use endpoint metrics API for immediate observability.

## Model Serving Configuration Impact Matrix

| Configuration | Effect | Risk If Not Set |
|---|---|---|
| Provisioned concurrency | Caps parallel requests | Traffic throttling under load |
| Scale-to-zero | Cuts idle costs | Warm-start latency spike on first request |
| Traffic split (traffic_config) | Routes % to each served entity | Champion/Challenger test relies on wrong model |
| Direct invocation path | Bypasses traffic split | Traffic config is bypassed if direct path is used |
| Inference tables enabled | Auto-logs request/response to Delta | No log if not enabled; at-least-once duplicates if enabled |
