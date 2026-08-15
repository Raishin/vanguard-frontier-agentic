# Evidence And Rollback Requirements

Reconciliation, idempotency in both senses, audit requirements, and the named inverse operation.

- Technical idempotency (a retry does not corrupt state) and business idempotency (a retry does not duplicate the real-world effect) are separate properties; a script can hold one without the other, and an idempotency claim must state which one it covers.
- A non-error exit code is evidence the process completed, not evidence it did what was intended — reconciliation is the step that checks the resulting state against the intended state.
- A checkpoint recorded before each batch (or unit of work) lets a resumed run avoid both re-applying already-committed effects and losing track of what remains — its absence forces a restart-from-zero that reopens the business-idempotency question.
- A named inverse operation must be specific to what the script actually mutated (a scoped inverse-write) — restore-from-backup is a fallback of last resort, not a rollback plan, and should be labelled as such if it is the only option offered.
- An audit trail sufficient to answer what ran and what it did after the fact requires recording who triggered it, when, the scope actually processed, and the outcome — not merely that the job succeeded.
