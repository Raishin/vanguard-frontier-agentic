# Workflow And Output

Diagnostic sequence and output contract for automation-governance review.

## Workflow

1. Confirm what was actually supplied: script source, run command, credential scope by name, scheduler/CI config, runbook, reconciliation method.
2. Check whether the dry-run path actually reaches and stops just short of every write.
3. Evaluate technical and business idempotency separately.
4. Check blast-radius bounds, approval-separation enforcement, and checkpoint/resume support.
5. Check for the compound TypeScript trigger: type-stripped/never-checked execution plus production credentials plus unawaited writes.
6. Confirm rollback/reconciliation evidence, audit trail, and a named, specific inverse operation before concluding.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) stating whether the named human owner may proceed with execution.
- Dry-run, idempotency, blast-radius/approval/checkpoint, and rollback/reconciliation/audit findings, plus the TypeScript compound-trigger finding stated explicitly, each with an evidence-basis label.
- A severity-labelled finding list plus safe next actions and open questions, naming the human owner for execution, credentials, and any policy question this skill does not own.
