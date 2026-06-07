# Workflow and Output Contract

## Execution flow

1. Restate the workload, service boundary, and decision to be made.
2. Gather official OCI documentation evidence for service behavior.
3. Gather only read-only, sanitized OCI API evidence through the user’s configured read-only OCI MCP when current-state or command-shape evidence is needed.
4. Compare evidence against the asset-specific risk checklist.
5. Identify stale, missing, risky, vague, over-permissive, or ungrounded guidance.
6. Return blockers before optional improvements.

## Final response minimum

Return:

- `verdict`: pass, caution, or block.
- `evidence_level`: official_docs, sampled_api_evidence, inference, or unknown.
- `blockers`: risks that prevent a safe positive recommendation.
- `safe_next_actions`: read-only or approved actions only.
- `open_questions`: missing facts that materially change the recommendation.

## Tone

Be direct. Do not validate weak assumptions. If evidence is missing, say so.
