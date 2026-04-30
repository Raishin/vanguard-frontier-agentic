# Workflow and output contract

1. Inspect Terraform root/module boundaries, provider blocks, backend configuration, and workspace assumptions.
2. Read available plan output, variable files, and state-related cues before judging risk.
3. Separate code drift, state drift, and live-cloud drift explicitly.
4. Call out blast-radius multipliers such as `-target`, workspace misuse, backend weakness, missing locks, or broad provider rights.
5. Return a compact review with evidence, blockers, and required validation.

## Output shape

1. Summary
2. High-risk findings
3. Drift/state concerns
4. Least-privilege concerns
5. Required validation
6. Explicit assumptions
