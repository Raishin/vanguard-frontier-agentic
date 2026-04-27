# Safety Checklist

Use this checklist before recommending or executing changes for `oci-identity-access-governor`.

## Scope

- Region confirmed.
- Compartment or compartment tree confirmed.
- Resource identity confirmed by name and, when available, live evidence.
- Owner and business impact confirmed.
- Environment confirmed: dev/test/stage/prod.

## Access

- Default profile unless user explicitly chooses another profile/config in the active runtime.
- Least-privilege action only.
- No tenancy-wide grants unless risk-accepted.
- No secrets requested or displayed.

## Change safety

- Read-only discovery first.
- Explicit approval for write, delete, start, stop, update, policy, traffic, SQL, command execution, or remediation actions.
- Rollback path documented.
- Validation plan includes positive and negative checks.

## Platform portability

- Prefer MCP tool calls.
- Use neutral `<placeholders>` in examples.
- Do not assume Bash, PowerShell, cmd.exe, macOS, Windows, or Linux until execution context is known.
- Do not embed machine-local paths.
