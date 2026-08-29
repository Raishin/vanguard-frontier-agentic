# Triggers, Watermarks, And Sink Semantics

Decision guide for trigger selection, watermark thresholds, and exactly-once idempotency.

- `Trigger.AvailableNow` (recommended) consumes all available records as an incremental batch, honours `maxBytesPerTrigger` and `maxFilesPerTrigger`, and is the default choice for incremental batch workloads; use it for bounded data (files, batch data) and always-available streams.
- `Trigger.ProcessingTime` triggers on a wall-clock interval and is appropriate for real-time workloads that can tolerate micro-batch latency; it does not honour `maxBytesPerTrigger`/`maxFilesPerTrigger`.
- `Trigger.Once` is deprecated from Databricks Runtime 11.3 LTS and should be replaced with `Trigger.AvailableNow` for all new code.
- Continuous Processing trigger has been experimental since Spark 2.3 and Databricks does not support or recommend it.
- Watermarks are declared via `withWatermark('eventTimeColumn', 'delayDuration')` before a stateful operation; records arriving inside the threshold are always processed; records outside it might still be processed but that is not guaranteed.
- `spark.sql.streaming.multipleWatermarkPolicy` takes `min` (default, safer against accidental late-marking) or `max` (lower latency, can drop slower-stream data); longer watermarks cost more state memory and latency.
- foreachBatch provides ONLY at-least-once write guarantees; exactly-once must be built using `batchId` for deduplication; for Delta writes, binding `txnVersion` to `batchId` makes Delta skip a replayed duplicate write.
- foreachBatch is incompatible with continuous mode — use `foreach` instead for continuous workloads.

## Trigger Selection Matrix

| Trigger Type | Use Case | Latency | State Memory | Serverless Support | Notes |
|---|---|---|---|---|---|
| AvailableNow | Incremental batch, files, bounded data | Batch | Low | Yes (required `max*PerTrigger`) | Default for incremental; respects `maxBytesPerTrigger`/`maxFilesPerTrigger` |
| ProcessingTime | Real-time, micro-batch | Seconds | Low | No | Wall-clock interval; not recommended for serverless |
| Trigger.Once | Legacy incremental | Batch | Low | Yes | Deprecated DBR 11.3+; use AvailableNow instead |
| Continuous | Sub-second latency (experimental) | Sub-second | Medium | No | Experimental since Spark 2.3; not recommended by Databricks |

## Sources

- https://docs.databricks.com/aws/en/structured-streaming/triggers
- https://docs.databricks.com/aws/en/structured-streaming/watermarks
- https://docs.databricks.com/aws/en/structured-streaming/foreach
- https://docs.databricks.com/aws/en/compute/serverless/streaming
