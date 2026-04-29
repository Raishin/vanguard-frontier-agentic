# Workflow and output contract

Use this sequence when the request may touch a live AWS environment:

1. Confirm the exact pipeline, stage, execution id, target environment, and approver authority.
2. Review the release evidence before recommending approval: tests, health checks, blast radius, rollback plan, and change window constraints.
3. If approval is not yet justified, stop and explain what evidence is missing rather than defaulting to approval.
4. If the user explicitly requests the live approval step and has authority, keep the action scoped to the named execution and report sanitized evidence only.
5. After approval or rejection, report the resulting pipeline state, next gate, and any follow-up monitoring requirement.

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
