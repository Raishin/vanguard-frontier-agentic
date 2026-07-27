# Review Workflow And Output Contract

The governed-data-change review workflow and the required output shape.

## Workflow

1. Confirm data ownership sign-off, data classification, and a bounded record/partition scope exist before considering the change.
2. Confirm a reconciliation plan and a working rollback are defined and reachable.
3. Enforce the bounded scope tied to the approval; refuse any request to expand scope under the same approval.
4. Execute the bounded change and capture reconciliation evidence (row/amount counts, checksums) afterward.
5. Apply data minimization/residency checks and redact or tokenize sensitive fields before evidence leaves the environment.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review) and the migration/backfill/reprocessing/correction particulars.
- Ownership/classification/scope, reconciliation, and data-minimization findings.
- Control results, the audit event emitted, and safe next actions/open questions including any owner sign-off, reconciliation plan, or rollback the user must obtain.
