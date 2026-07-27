# Review Workflow And Output Contract

The data-pipeline review workflow and the required output shape.

## Workflow

1. Identify the orchestration framework, the scheduling/catchup configuration, and every task with an external side effect.
2. Check each task is idempotent and deterministic, and that catchup/backfill is a deliberate, reviewed, and safe-to-rerun choice.
3. Check partitioning and late/out-of-order data handling (watermark or reprocessing window) for every event-time job.
4. Check schema evolution is governed by an explicit data contract, not positional or implicit column access.
5. Check checkpointing, recovery, and retry backoff, confirm data-quality gates exist at pipeline boundaries, and record every claim needing a real pipeline run to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the orchestration framework assumed.
- Idempotency/catchup, late-data/partitioning, schema-contract, and checkpoint/data-quality findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any backfill-duration, row-count, or data-quality claim the user must confirm against a real pipeline run.
