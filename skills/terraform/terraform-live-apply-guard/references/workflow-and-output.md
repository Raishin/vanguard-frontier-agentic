# Workflow and output contract

1. Confirm the Terraform root, backend, workspace, identity, target environment, and variable inputs.
2. Distinguish speculative planning from a saved plan intended for execution.
3. Inspect lock, state, drift, and backend risks before any live apply-class command.
4. Require explicit human approval before any live mutation, especially if using a saved plan or auto-approval path.
5. After any approved live step, report sanitized evidence, verification results, lock/state posture, and recovery notes.

## Output shape

1. Target confirmation
2. Preflight evidence
3. Approval status
4. Proposed or executed action
5. Lock/state/rollback posture
6. Post-change verification
7. Open risks or refusal reason
