# Workflow and Output Contract

## Execution flow

1. Scope the exact target, environment boundary, owner, requested operation, approval state, and rollback owner.
2. Load `official-sources.md`, then the component operations guide for service behavior and risk gates.
3. Gather sampled read-only evidence only when available and safe.
4. Compare observed posture against documented behavior, least-privilege expectations, and live-operation safety rules.
5. Refuse or defer mutation if target, approval, rollback, or evidence is incomplete.
6. Return a verdict with evidence level, blockers, safe next actions, and open questions.

## Required output

- `verdict`: pass, warn, fail, or blocked.
- `evidence_level`: documentation-based, sampled-current-state, user-provided, inference, or mixed.
- `scope`: what was reviewed and what was not reviewed.
- `approval_status`: explicit approval, missing approval, or not applicable for read-only review.
- `blockers`: issues that prevent a safe or production-ready conclusion.
- `findings`: severity-labeled risks with source labels.
- `rollback_posture`: exact rollback path or explicit non-reversibility caveat.
- `safe_next_actions`: reversible actions first; mutation only with explicit approval.
- `open_questions`: missing facts that would change the verdict.

## Stress checks

- What assumption would make this recommendation unsafe?
- Which role, policy, budget, quota, deployment, swap, or purge action has the largest blast radius?
- What evidence would disprove the claimed readiness?
- Is the answer accidentally treating documentation as environment-specific proof?

## Response discipline

Use Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior. Use sampled read-only Azure evidence only for current configured-environment observations and label it as sampled evidence.
