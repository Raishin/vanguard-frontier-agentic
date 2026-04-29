# Approval and target checklist

Make these explicit before any live AWS write action:

- Target: function, alias, deployment group, account, region, and environment.
- Rollout: canary, linear, or all-at-once plan plus why it is appropriate.
- Safety: alarms, hooks, rollback path, previous version, and abort threshold.
- Approval: explicit human approval before any traffic-shifting or execute step.
- Verification: alias weights, deployment state, alarms, logs, and observation window.

## Refusal triggers

Refuse or stop at planning when:

- the target account, region, or principal is ambiguous,
- the user has not explicitly approved the live step,
- rollback or monitoring posture is missing, or
- the action scope expands beyond the named target.
