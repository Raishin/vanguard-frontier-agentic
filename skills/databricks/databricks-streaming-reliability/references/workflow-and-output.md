# Workflow And Output

Streaming-reliability review sequence and output contract.

## Workflow

1. Establish the query source (Structured Streaming API or Auto Loader source), state schema, and checkpoint location — refuse if missing.
2. Check for state-schema changes between restarts: flag additions, deletions, or type changes to state-keying columns as breaking.
3. Verify checkpoint isolation: confirm the checkpoint location is unique to this query and not shared with other queries.
4. Review watermark declaration: confirm watermarks are declared before stateful operations (groupBy, join, etc.); assess threshold sizing and `multipleWatermarkPolicy` choice.
5. Validate trigger choice: flag deprecated `Trigger.Once` (use AvailableNow instead); confirm ProcessingTime or Continuous is chosen intentionally; for serverless, require AvailableNow/Once and `maxFilesPerTrigger`/`maxBytesPerTrigger`.
6. Assess sink semantics: determine whether exactly-once is required; if so, verify foreachBatch includes idempotency logic and `batchId` deduplication; for Delta writes, confirm `txnVersion` is bound to `batchId`.
7. Evaluate state-store configuration: for large stateful workloads, confirm RocksDB is enabled; verify changelog checkpointing is enabled (or explicitly disabled with justification on DBR 17.3+).
8. For backfill scenarios: confirm checkpoint isolation from incremental runs; verify state-schema is immutable across both phases; confirm partial-backfill plan is safe.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (safe / safe-with-operational-changes / unsafe-refactor-required) and the scope of this review.
- State-schema, checkpoint, watermark, trigger, sink-semantics, state-store, and restart/backfill findings.
- A severity-labelled finding list (critical / high / medium / low), each with evidence basis, and safe next actions for the user.
