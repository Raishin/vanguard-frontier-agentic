# Review Workflow And Output Contract

The automation-governance review workflow and the required output shape.

## Workflow

1. Identify the automation's type, trigger, owner (if any), inputs/outputs, and data classification.
2. Check for a named owner and for segregation of duties on any sensitive request/approve/execute path.
3. Check for a reconciliation control, idempotency, a rollback path, and retained run evidence proportional to exposure.
4. Check whether notebook or spreadsheet-adjacent automation has hidden state and whether it has been captured as an owned, version-controlled job.
5. Quantify the exposure (value-at-risk, toil, control gaps, key-person dependency) and produce a continue/harden/replatform/retire recommendation with a reversible next step, routing any accounting/legal/regulatory question out.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the automation type assumed.
- Ownership/segregation-of-duties, reconciliation/rollback, and hidden-state/exposure findings.
- A severity-labelled finding list, each with an evidence-basis label, plus a continue/harden/replatform/retire recommendation and any accounting/legal/regulatory question routed to the appropriate board.
