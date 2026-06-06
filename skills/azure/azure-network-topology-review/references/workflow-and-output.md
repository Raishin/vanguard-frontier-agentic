# Workflow and Output Contract

## Execution flow

1. Scope the exact target, environment boundary, owner, requested decision, and evidence available.
2. Load `official-sources.md`, then the component operations guide for service behavior and risk gates.
3. Gather sampled read-only evidence only when available and safe.
4. Compare observed posture against documented behavior, least-privilege expectations, and operational safety rules.
5. Return a verdict with evidence level, blockers, safe next actions, and open questions.

## Required output

- `verdict`: pass, warn, fail, or blocked.
- `evidence_level`: documentation-based, sampled-current-state, user-provided, inference, or mixed.
- `scope`: what was reviewed and what was not reviewed.
- `blockers`: issues that prevent a safe or production-ready conclusion.
- `findings`: severity-labeled risks with source labels.
- `safe_next_actions`: reversible actions first; mutation only with explicit approval.
- `open_questions`: missing facts that would change the verdict.

## Stress checks

- What assumption would make this recommendation unsafe?
- Which identity, migration, network, telemetry, route, or dispatch decision has the largest blast radius?
- What evidence would disprove the claimed readiness?
- Is the answer accidentally treating documentation as configured-environment proof?

## Response discipline

Use Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior. Use sampled read-only Azure evidence only for current configured-environment observations and label it as sampled evidence.
