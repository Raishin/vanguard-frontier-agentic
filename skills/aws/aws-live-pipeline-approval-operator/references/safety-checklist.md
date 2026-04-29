# Safety checklist

Before recommending or running a live AWS action, enforce these checks:

- Do not treat possession of credentials as approval authority.
- Do not approve a pipeline stage without evidence tied to the current execution.
- Do not grant broad approval permissions when a pipeline-specific policy is sufficient.
- Do not ignore seven-day timeout or notification routing on manual approvals.
- If release evidence is stale, incomplete, or from the wrong execution, stop.

## Mandatory posture

- Prefer the smallest reversible change.
- Prefer preview, describe, or dry-run style evidence before mutation.
- Treat the absence of rollback as a blocker, not a detail.
- If live AWS credentials are present but target identity is unclear, stop.
