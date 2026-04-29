# Workflow and output contract

Use this sequence when the request may touch a live AWS environment:

1. Verify target identity first: account, region, profile or role path, environment name, and exact service or resource.
2. Inspect current live state before proposing mutation. Use dry-run, preview, describe, or status commands first when available.
3. Require an approval checkpoint before mutation. If the user has not explicitly authorized the live step, stop at plan or preview.
4. Prefer rollout controls such as change calendars, approval actions, alarms, canaries, circuit breakers, and stack policies when the target service supports them.
5. After any approved live step, report the exact command class, sanitized evidence, rollback posture, and post-change verification results.

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
