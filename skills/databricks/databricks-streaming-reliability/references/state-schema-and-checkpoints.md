# State Schema Immutability And Checkpoint Compatibility

Hard rules for state schema changes, checkpoint isolation, and restart safety.

- State schema must remain the SAME across restarts — additions, deletions, and type changes to stateful operations are breaking changes and surface as `StateStoreKeySchemaNotCompatible`; the only safe path to a schema change is a full state reset and data reprocessing.
- Two queries must never share one checkpoint location; sharing causes state corruption and incorrect results and is detected as a runtime error when the second query tries to read the checkpoint.
- Checkpoint metadata includes offsets, commits, state, and query metadata; a checkpoint survives a query restart as long as the query structure is unchanged (same source, same stateful operations, same target).
- Source evolution (stable user-defined source names allowing reorder/add/remove without losing checkpoint state) requires Databricks Runtime 18.2 and above; on older DBR versions, source structure changes require checkpoint reset.

## Sources

- https://docs.databricks.com/aws/en/structured-streaming/checkpoints
- https://docs.databricks.com/aws/en/structured-streaming/stateful-streaming
- https://docs.databricks.com/aws/en/structured-streaming/production
