# Official sources

Use this reference only when grounding Contabo security behavior, API authentication flows, or secret management patterns.

## Contabo documentation

Use these as starting points, not as proof of the user's live instance configuration:

- https://api.contabo.com/ — Contabo OpenAPI reference (authentication, secret management, firewall endpoints)
- https://docs.contabo.com/ — Contabo user documentation (SSH keys, Cloud-Init, firewall, Private Networking)
- https://github.com/contabo/cntb — cntb CLI tool (secret management commands, instance operations)
- https://api.contabo.com/#tag/Instances — Instance operations API (Cloud-Init userData, SSH key secret IDs)

## Grounding rule

Official Contabo documentation describes platform security capabilities and API authentication patterns. It does not prove the user's current firewall rules, SSH configuration, running services, or OAuth2 token state. Prefer user-provided sanitized evidence or live read-only API responses for current-state claims. Label any claim sourced only from documentation as `documentation-based`. Documentation alone never replaces a live firewall rule dump or actual sshd_config output.
