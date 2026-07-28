# Data-Pipeline Review Checklist

The per-concern checklist applied to every data-pipeline review.

- Idempotency: every task is idempotent and deterministic; overwrite-by-partition or merge-by-key, not blind append.
- Catchup: catchup is a deliberate choice, and every backfill is confirmed safe to re-run before it is triggered.
- Late data: event-time jobs handle late/out-of-order arrivals via a watermark or reprocessing window.
- Schema: consumers use an explicit data contract; no positional or implicit column access.
- Checkpoint/retry: long jobs checkpoint and resume idempotently; retries use bounded backoff for transient errors only.
- Quality: data-quality gates and lineage evidence exist at pipeline boundaries.
