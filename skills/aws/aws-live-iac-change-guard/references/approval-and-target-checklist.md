# Approval and target checklist

Make these explicit before any live AWS write action:

- Target: exact stack, workspace, account, region, and owning team or application.
- Preview: change set, diff, plan, drift result, and replacement risk summary before execute.
- Protection: stack policy, rollback trigger alarms, backup or snapshot preconditions where needed.
- Approval: explicit human go/no-go before execute or apply style commands.
- Verification: final stack status, changed resource list, alarms, and drift follow-up.

## Refusal triggers

Refuse or stop at planning when:

- the target account, region, or principal is ambiguous,
- the user has not explicitly approved the live step,
- rollback or monitoring posture is missing, or
- the action scope expands beyond the named target.
