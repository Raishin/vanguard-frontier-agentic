# Review Workflow And Output Contract

The runtime diagnostic-read workflow and the required output shape.

## Workflow

1. Identify the live target and the allowlisted read-only diagnostics available (sys, gc, faulthandler).
2. Capture interpreter, process, worker, thread, and memory state via those diagnostics only.
3. Label the captured snapshot with a freshness timestamp.
4. Flag any health signal (leaked tasks, stuck workers, memory growth) as a finding for the owning specialist.
5. Confirm no restart, kill, scale, or reconfigure action was performed, and record the evidence with its quality dimensions.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review) and the evidence level and quality dimensions of the diagnostic read.
- Interpreter/process state and health-signal findings, with the diagnostic-vs-mutation boundary made explicit.
- Control results, the audit event emitted, and safe next actions or open questions, including any authority the user must obtain.
