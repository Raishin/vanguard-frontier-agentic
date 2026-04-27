# Documentation Fallback

Use this reference when live OCI infrastructure data is unavailable, incomplete, denied, or unsafe to query for `oci-exadata-platform-architect`.

## Grounding order

1. Live official Oracle MCP evidence.
2. User-provided sanitized evidence such as exports, diagrams, screenshots, AWR summaries, tickets, or redacted configuration.
3. Context7 Oracle Cloud Infrastructure documentation (`/websites/oracle_en-us_iaas_content`).
4. Service-specific Oracle documentation or Oracle database documentation MCP when relevant.
5. Clearly labeled inference.

## Mandatory evidence labels

Every finding must be labeled as one of:

- `live evidence`
- `user-provided sanitized evidence`
- `documentation-based`
- `inference`

Documentation can explain how OCI and Oracle Database services should behave. It does not prove the user's current infrastructure state, capacity, quota, regional availability, or provider-specific entitlement.

## Context7 prompts to use

- `OCI Autonomous Database Exadata deployment options Oracle Database at Azure AWS Google Cloud Cloud Customer compatibility`
- `OCI database multicloud networking private endpoint Data Guard backup Recovery Service limits`
- `OCI Exadata cloud VM cluster autonomous VM cluster API CLI operations maintenance IORM`

Keep prompts free of customer names, identifiers, IP addresses, secrets, or proprietary architecture details.
