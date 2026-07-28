# Data-Change-Control Review Checklist

The per-concern checklist applied to every migration/backfill/reprocessing/correction request.

- Ownership: a named data owner (distinct from the requester) signs off before any migration/backfill/reprocessing/correction.
- Classification: the data class of every affected field is recorded before the change.
- Scope: the change is bounded to a named record/partition scope tied to the approval; scope is never expanded under an existing approval.
- Reconciliation: row/amount counts and checksums are captured and compared after the change.
- Correctness: technical completion (the job ran) is never treated as proof of data correctness.
- Minimization: regulated/personal data is never copied to a third-party tool without an approved data-flow review, and is redacted/tokenized in evidence.
