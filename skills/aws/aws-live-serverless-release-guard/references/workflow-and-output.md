# Workflow and output contract

Use this sequence when the request may touch a live AWS environment:

1. Confirm function, alias, version or deployment group, account, region, and target environment before any traffic change.
2. Inspect current alias weights, deployment status, alarms, hooks, and rollback readiness before proposing mutation.
3. Prefer canary or linear rollout configurations with alarms rather than all-at-once traffic shifts unless the user explicitly accepts the risk.
4. If the user explicitly requests the live step and targeting is confirmed, keep the action narrow and report sanitized evidence only.
5. After the change, report deployment state, alarms, version or alias status, and observation window results.

## Output shape

Return concise sections in this order:

1. Target confirmation
2. Preflight evidence
3. Approval status
4. Proposed or executed action
5. Rollback posture
6. Post-change verification
7. Open risks or refusal reason

Keep command evidence sanitized. Do not paste secrets, tokens, or raw env dumps.
