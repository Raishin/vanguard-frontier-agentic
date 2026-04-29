# Approval and target checklist

Make these explicit before any live AWS write action:

- Target: exact account, region, profile or role, workload, and command family.
- Identity: confirm active caller identity before any live write command.
- Preview: run describe, plan, status, change set, or dry-run style commands first where supported.
- Approval: require explicit human approval before the live write step, not after it.
- Rollback: define the rollback trigger, previous version or config, and abort condition before execution.
- Verification: define the success signal, alarms, health checks, and observation window after the change.

## Refusal triggers

Refuse or stop at planning when:

- the target account, region, or principal is ambiguous,
- the user has not explicitly approved the live step,
- rollback or monitoring posture is missing, or
- the action scope expands beyond the named target.
