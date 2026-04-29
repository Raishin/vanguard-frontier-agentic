# Workflow and output contract

Use this sequence when the request may touch a live AWS environment:

1. Confirm target identity, stack or workspace, and environment before any live infrastructure action.
2. Run validation and preview first: template validation, synth, plan, diff, change set, or drift detection as appropriate.
3. Check whether stack policies, rollback triggers, alarms, or resource protection should be in place before execution.
4. If execution is explicitly approved, keep the command bounded and report sanitized evidence plus rollback posture.
5. After execution, verify final status, changed resources, alarms, and any residual drift or failed rollback risk.

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
