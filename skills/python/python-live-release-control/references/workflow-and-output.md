# Review Workflow And Output Contract

The bounded-release-control review workflow and the required output shape.

## Workflow

1. Confirm the request is bounded to exactly one release, canary increment, rollback, or single-instance restart, and reject fleet-wide/unbounded scope.
2. Confirm an independent approval bound to the exact plan digest and target, target-scoped JIT credentials, and a captured before-state exist before execution.
3. Confirm a pre-approved rollback exists and is reachable; refuse to execute without it.
4. Execute the one bounded action and capture the after-state; never self-attest success.
5. Route verification to an independent check and record whether the approval/bound was reused or changed.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions that must be resolved before this action may proceed; empty if approved), the evidence level and quality dimensions, and the exact bounded action and target.
- Bound-and-scope, approval/JIT/before-state, and independent-verification findings.
- Control results, the audit event emitted, and safe next actions/open questions including any approval, JIT credential, or rollback the user must obtain.
