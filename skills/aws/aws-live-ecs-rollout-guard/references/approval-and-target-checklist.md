# Approval and target checklist

Make these explicit before any live AWS write action:

- Target: cluster, service, task definition, account, region, and environment.
- Safety: circuit breaker, alarms, health checks, bake time, and rollback behavior.
- Evidence: current deployment status, service events, latest healthy revision, and blast radius.
- Approval: explicit human approval before any live rollout or force-new-deployment step.
- Verification: service deployment state, alarms, events, and observation window after the change.

## Refusal triggers

Refuse or stop at planning when:

- the target account, region, or principal is ambiguous,
- the user has not explicitly approved the live step,
- rollback or monitoring posture is missing, or
- the action scope expands beyond the named target.
