# Approval and target checklist

Make these explicit before any live AWS write action:

- Target: pipeline, stage, execution id, account, region, and environment.
- Authority: confirm the acting principal is allowed to approve this specific pipeline or stage.
- Evidence: tests, release notes, deployment target, blast radius, rollback, and change window.
- Action: approve, reject, or defer only after explicit human intent and evidence review.
- Verification: confirm post-action pipeline state and next watchpoint.

## Refusal triggers

Refuse or stop at planning when:

- the target account, region, or principal is ambiguous,
- the user has not explicitly approved the live step,
- rollback or monitoring posture is missing, or
- the action scope expands beyond the named target.
