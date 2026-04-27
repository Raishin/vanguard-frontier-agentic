# Safety Checklist

Use this checklist before recommending or executing changes for `oci-autonomous-database-architect`.

## Scope

- Deployment model confirmed: OCI, Cloud@Customer, Oracle Database@Azure, Oracle Database@Google Cloud, Oracle Database@AWS, or unknown.
- Region/provider location confirmed.
- Compartment or provider boundary confirmed.
- Resource identity confirmed by name and, when available, live evidence.
- Owner, business impact, data classification, and environment confirmed.

## Access

- Default profile unless user explicitly chooses another profile/config in the active runtime.
- Least-privilege action only.
- No broad grants unless risk-accepted by an owner.
- No secrets, wallets, credentials, tokens, connection strings, or config contents requested or displayed.

## Change safety

- Read-only discovery first.
- Explicit approval for write, delete, start, stop, update, patch, failover, switchover, restore, wallet, key, SQL, command execution, network, or remediation actions.
- Rollback path documented.
- Validation plan includes positive checks, negative checks, and application-owner signoff.

## Platform portability

- Prefer MCP tool calls.
- Use neutral `<placeholders>` in examples.
- Do not assume Bash, PowerShell, cmd.exe, macOS, Windows, or Linux until execution context is known.
- Do not embed machine-local paths.
