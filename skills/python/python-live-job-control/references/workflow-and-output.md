# Review Workflow And Output Contract

The job-control review workflow and the required output shape.

## Workflow

1. Identify the job/business-automation operation requested and every external/business side effect it can cause.
2. Confirm technical idempotency (safe re-run) and business idempotency (no duplicate business effect) before operating it.
3. If a retry is requested, bound it (an idempotency-guarded, dead-lettered, owner-approved retry) and refuse a blind "retry all" request.
4. Execute the approved, bounded operation and capture the process-level result.
5. Reconcile the actual business outcome independently of the process result before confirming completion.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions that must be resolved before this action may proceed; empty if approved), the evidence level and quality dimensions, and the job/business-automation particulars.
- Technical/business idempotency, bounded-retry, and business-outcome reconciliation findings.
- Control results, the audit event emitted, and safe next actions/open questions including any owner approval or reconciliation the user must obtain.
