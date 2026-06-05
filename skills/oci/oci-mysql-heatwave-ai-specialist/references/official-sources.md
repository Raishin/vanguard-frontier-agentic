# Official sources

Use these sources to ground service behavior before making production recommendations.

## Official documentation checked on 2026-06-05

- https://docs.oracle.com/iaas/mysql-database/index.html
- https://docs.oracle.com/en-us/iaas/mysql-database/doc/overview-heatwave.html
- https://docs.oracle.com/en/database/mysql/heatwave-aws/database-vector-store.html

## Sampled read-only evidence checked on 2026-06-05

- OCI API evidence through the user's configured read-only OCI MCP was used for command/API surface shape only.
- Microsoft Learn documentation through the user's configured documentation MCP was used only where Azure interconnect behavior is in scope.
- Command-help evidence confirms available list/filter surfaces; it does not prove permissions, resource existence, regional availability, capacity, quota, data correctness, traffic safety, or production readiness.

## Grounding rules

- Prefer the most specific official documentation page for the service or feature being discussed.
- If documentation and sampled API evidence appear to conflict, report the conflict and avoid stronger claims until resolved.
- Do not cite internal tool names, local environment labels, connector identifiers, or environment-specific details in committed docs.
- Do not paste sensitive identifiers or customer data into examples.
