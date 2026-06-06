# Safety checklist

Use before OCI Cloud Guard Responder production recommendations, privileged access, cloud mutations, remediation actions, or readiness claims.

## Non-negotiables

- Do not ask for or print credentials, tokens, private keys, API keys, config files, tenancy identifiers, compartment identifiers, resource identifiers, customer data, wallets, or secrets.
- Default to read-only discovery and advisory output.
- Require explicit approval before any create, update, delete, start, stop, reboot, failover, restore, revoke, remediate, or command-execution action.
- Keep permissions least-privilege and scoped to the confirmed resource boundary.
- Separate documentation evidence from OCI API evidence through the user's configured read-only OCI MCP and sanitized user evidence.
- Treat API availability and command help as API-shape evidence, not permission to mutate or proof of configured resources.

## Evidence labels

Use `documentation-based`, `sampled OCI API evidence`, `sampled current-state evidence`, `repo evidence`, `user-provided sanitized evidence`, or `inference`. Documentation alone never proves the user's live OCI posture.
