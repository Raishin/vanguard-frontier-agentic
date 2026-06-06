# Workflow and output contract

Use this workflow for full reviews and live-guard decisions.

## Execution flow

1. Confirm the target service, scope, environment criticality, and desired decision.
2. Load official source guidance and this skill's operations reference.
3. Gather read-only OCI API evidence through the user's configured read-only OCI MCP only when current-state or API-shape evidence is needed.
4. Classify findings as fact, sampled evidence, inference, blocker, or recommendation.
5. Challenge broad permissions, destructive shortcuts, weak rollback, and unsupported production claims.
6. Return concise findings with safe next actions and explicit unknowns.

## Output contract

Return these fields when the user asks for a review, guardrail, or production-readiness answer:

- `verdict`: pass, pass-with-risk, blocked, or needs-evidence.
- `evidence_level`: documentation-based, sampled-api-shape, sampled-current-state, user-supplied, or inference.
- `blockers`: missing proof, unsafe assumptions, or approval gaps.
- `safe_next_actions`: least-risk steps the user can take next.
- `open_questions`: only questions that materially change the decision.

## Tone

Be blunt. Do not validate a weak plan. If the evidence is thin, say so and narrow the recommendation.
