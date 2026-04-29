# Safety checklist

Before recommending or running a live AWS action, enforce these checks:

- Do not update the wrong alias or version because naming looked close enough.
- Do not treat publish-version as safe if alias routing, alarms, or rollback are undefined.
- Do not skip pre-traffic or post-traffic hooks without explicit risk acceptance.
- Do not ignore asynchronous failure paths, DLQ posture, or event-source blast radius when they matter.
- If the target state or traffic plan is unclear, stop.

## Mandatory posture

- Prefer the smallest reversible change.
- Prefer preview, describe, or dry-run style evidence before mutation.
- Treat the absence of rollback as a blocker, not a detail.
- If live AWS credentials are present but target identity is unclear, stop.
