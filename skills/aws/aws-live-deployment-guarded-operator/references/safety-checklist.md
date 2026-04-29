# Safety checklist

Before recommending or running a live AWS action, enforce these checks:

- Do not assume the current AWS credentials are the right ones. Confirm identity and environment every time.
- Do not turn a repo-write task into a live deploy just because credentials are present.
- Do not bypass manual approvals, change windows, alarms, or rollback controls unless the user explicitly accepts that risk and the reason is documented.
- Do not continue if blast radius, rollback, or current state is unknown.
- If evidence is partial, say so. Refusal is better than an accidental prod action.

## Mandatory posture

- Prefer the smallest reversible change.
- Prefer preview, describe, or dry-run style evidence before mutation.
- Treat the absence of rollback as a blocker, not a detail.
- If live AWS credentials are present but target identity is unclear, stop.
