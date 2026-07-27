# Review Workflow And Output Contract

The rollback-and-recovery execution workflow and the required output shape.

## Workflow

1. Confirm a previously approved, tested rollback procedure exists for the exact affected target; refuse to improvise one if it does not.
2. Confirm rollback authority and the exact affected-target fingerprint match the request.
3. Confirm the rollback's preconditions — a captured before-state or snapshot reference — exist before executing.
4. Execute the pre-approved rollback against the exact affected target.
5. Capture and reconcile the post-rollback state and route verification to an independent check.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review) and the rollback request and target particulars.
- Pre-approval/target-binding, precondition, and post-rollback reconciliation findings.
- Control results, the audit event emitted, and safe next actions/open questions including any rollback authorship, approval, or authority the user must obtain.
