# Documentation and Evidence Path

## Preferred evidence order

1. Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior.
2. Sampled read-only Azure evidence, when safely available, for current configured-environment observations.
3. Sanitized user-provided evidence.
4. Clearly labeled inference.

## What each evidence type can prove

- Microsoft Learn documentation can prove documented service behavior, supported concepts, limitations, and recommended patterns.
- Sampled read-only evidence can prove the sampled configured state at the time observed.
- Sanitized user evidence can prove only what the snippet shows.
- None of these alone prove broad regional availability, future success, full account posture, or production readiness.

## Safe usage pattern

- State whether each claim is documentation-based, sampled-current-state, user-provided, or inference.
- Use read-only queries before recommending changes.
- Do not include sensitive internal identifiers, tenant identifiers, subscription identifiers, or secrets in committed docs or final findings.
- If no sampled evidence is available, say the review is documentation-based and list the exact evidence still needed.

## Live-operation rule

For live operations, documentation is never enough. Require target confirmation, current-state evidence, explicit approval, rollback constraints, and post-action verification.
