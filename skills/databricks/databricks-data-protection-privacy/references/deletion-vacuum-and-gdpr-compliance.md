# Deletion, VACUUM, And GDPR Compliance

DELETE/MERGE logical deletion, VACUUM physical removal, REORG PURGE, retention windows, and GDPR erasure obligation alignment.

- DELETE and MERGE mark data logically deleted but retain historical versions for time-travel and rollback. Only VACUUM removes historical file versions from cloud storage.
- VACUUM operates on a retention window (default 30 days); a data file older than this window can be removed. A VACUUM retention window longer than a GDPR erasure deadline (e.g., 30 day retention but 7-day erasure obligation) silently defeats the compliance requirement.
- When deletion vectors are enabled, REORG TABLE ... APPLY (PURGE) is required before VACUUM to physically remove rows; skipping REORG leaves logically-deleted rows until a separate compaction occurs.
- A compliance design coordinating GDPR erasure requires attention to both the logical deletion path (DELETE or MERGE) and the VACUUM window; setting the window longer than the deadline is a configuration bug that creates liability.
- Lineage tracking (system tables) retains a rolling 1-year window; deletion events recorded in the window are discoverable; deletion events outside the window are lost.
