# Documentation Fallback

Use this reference when live OCI infrastructure data is unavailable, incomplete, denied, or unsafe to query for `oci-registry-artifact-governor`.

## Grounding order

1. Live official Oracle MCP evidence.
2. User-provided sanitized evidence such as exports, diagrams, screenshots, tickets, or redacted configuration.
3. Context7 Oracle Cloud Infrastructure documentation (`/websites/oracle_en-us_iaas_content`).
4. Service-specific Oracle documentation or Oracle database documentation MCP when relevant.
5. Clearly labeled inference.

## Mandatory evidence labels

Every finding must be labeled as one of:

- `live evidence`
- `user-provided sanitized evidence`
- `documentation-based`
- `inference`

Documentation can explain how OCI services should behave. It does not prove the user's current infrastructure state.

## Context7 prompts to use

- `OCI <service> best practices IAM monitoring limits troubleshooting`
- `OCI <service> CLI API concepts compartments policies logging metrics`
- `OCI <service> security reliability operations cost governance`

Keep prompts free of customer names, OCIDs, IP addresses, secrets, or proprietary architecture details.
