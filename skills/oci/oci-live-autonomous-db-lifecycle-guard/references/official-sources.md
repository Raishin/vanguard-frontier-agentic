# Official sources

Use these sources to ground service behavior before making production recommendations.

## Oracle documentation checked on 2026-06-05

- https://docs.oracle.com/en-us/iaas/autonomous-database-serverless/doc/scale-autonomous-database.html
- https://docs.oracle.com/en-us/iaas/autonomous-database-serverless/doc/start-stop-autonomous-database.html
- https://docs.oracle.com/en-us/iaas/autonomous-database-serverless/doc/autonomous-clone.html
- https://docs.oracle.com/en-us/iaas/autonomous-database-serverless/doc/backup-recovery-autonomous.html

## Sampled read-only OCI API evidence checked on 2026-06-05

- OCI API evidence through the user's configured read-only OCI MCP was used for command/API surface shape only.
- Command-help evidence confirms available list/filter surfaces; it does not prove permissions, resource existence, regional availability, capacity, quota, data correctness, or production readiness.

## Grounding rules

- Prefer the most specific Oracle documentation page for the service or feature being discussed.
- If documentation and sampled API evidence appear to conflict, report the conflict and avoid stronger claims until resolved.
- Do not cite internal tool names, local environment labels, connector identifiers, or environment-specific details in committed docs.
- Do not paste sensitive identifiers or customer data into examples.
