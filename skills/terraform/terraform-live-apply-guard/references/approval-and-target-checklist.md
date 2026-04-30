# Approval and target checklist

Make these explicit before any live Terraform write action:

- Target: exact Terraform root/module, backend, workspace, identity, environment, and variable input set.
- Plan evidence: whether the plan is speculative or saved, when it was produced, and against which state.
- Lock posture: whether the backend supports locking, whether the lock is healthy, and whether anyone is already running a mutation.
- Approval: explicit human approval before any apply-class command, especially when using a saved plan or auto-approval path.
- Recovery: rollback or recovery posture, backups/state snapshots where relevant, and post-apply verification signals.

## Refusal triggers

Refuse or stop at planning when:

- backend, workspace, or identity is ambiguous,
- the plan evidence is stale or missing,
- lock/state risk is unclear,
- the user has not explicitly approved the live step, or
- the request quietly escalates from planning into mutation.
