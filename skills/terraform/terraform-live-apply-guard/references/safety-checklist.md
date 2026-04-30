# Safety checklist

- Do not run `terraform apply` against an ambiguous backend, workspace, or credential context.
- Do not treat CLI workspaces as a substitute for separate credentials and strong environment isolation; official docs explicitly warn against that pattern for complex deployments.
- Do not normalize `-auto-approve`, `-lock=false`, or `force-unlock`.
- Saved plans are safer for consistency, but they are also more dangerous if stale or approved blindly because apply will not prompt again.
- If the plan evidence is stale, missing, or from the wrong workspace, stop.
