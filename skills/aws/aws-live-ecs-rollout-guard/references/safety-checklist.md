# Safety checklist

Before recommending or running a live AWS action, enforce these checks:

- Do not force a new deployment just because the service looks stale.
- Do not ignore unhealthy tasks, missing alarm coverage, or rollback-disabled settings.
- Do not treat a task definition registration as equivalent to a safe live rollout.
- Do not widen blast radius across multiple services when one named service is the target.
- If deployment safety signals are weak or contradictory, stop.

## Mandatory posture

- Prefer the smallest reversible change.
- Prefer preview, describe, or dry-run style evidence before mutation.
- Treat the absence of rollback as a blocker, not a detail.
- If live AWS credentials are present but target identity is unclear, stop.
