# Workflow and output contract

Use this sequence when the request may touch a live AWS environment:

1. Confirm cluster, service, task definition, account, region, and environment before any live ECS action.
2. Inspect current deployment state, service events, health checks, alarm state, and rollback posture before proposing mutation.
3. Prefer circuit breaker or alarm-backed failure detection and a defined bake window rather than a blind forced rollout.
4. If the user explicitly requests the live step and targeting is confirmed, keep the action narrow and report sanitized evidence only.
5. After the change, report deployment state, rollback status, alarms, service events, and post-rollout verification results.

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
