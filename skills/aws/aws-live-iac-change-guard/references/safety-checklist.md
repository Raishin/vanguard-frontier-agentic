# Safety checklist

Before recommending or running a live AWS action, enforce these checks:

- Do not execute direct live IaC changes against an ambiguous stack, account, or region.
- Do not treat change-set creation as approval to execute it.
- Do not weaken stack policies or rollback triggers casually just to force a change through.
- Do not ignore drift, replacement risk, or stateful resource blast radius.
- If preview evidence is missing or contradictory, stop and say so.

## Mandatory posture

- Prefer the smallest reversible change.
- Prefer preview, describe, or dry-run style evidence before mutation.
- Treat the absence of rollback as a blocker, not a detail.
- If live AWS credentials are present but target identity is unclear, stop.
