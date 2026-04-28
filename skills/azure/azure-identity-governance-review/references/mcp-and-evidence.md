# MCP and Evidence Path

## Official Azure / Entra Linkage

Ground this skill in official Microsoft Learn content only.

Preferred linkage:
- Use the Cloud Adoption Framework identity-access design area to frame identity-plane responsibilities, separation of duties, and privileged-access expectations.
- Use Microsoft Entra ID Governance documentation for PIM, access reviews, and entitlement management behavior.
- Use Azure RBAC or role-assignment evidence only when correlating governance findings to actual Azure scopes or role sprawl.

If live Azure tooling is available in the client, use Azure role-related evidence carefully for:
- which principals hold privileged Azure resource roles,
- whether assignment scope is broader than claimed,
- whether standing assignments remain where eligibility should exist.

Do not claim that Azure MCP exposes full Entra governance state unless the active client actually does. If governance evidence is missing, say so.

## Platform-Agnostic Execution

This skill must work in MCP-only, browser-only, and documentation-only environments. Prefer neutral evidence language:
- `<tenant>`
- `<management-group | subscription | resource-group | resource>`
- `<principal>`
- `<privileged role>`
- `<access package>`

If commands or portal paths are useful, keep them platform-neutral and adapt only after the user’s actual environment is known.

## Documentation Fallback When Live Data Is Unavailable

Live tenant evidence beats documentation. If live evidence is unavailable, denied, incomplete, or unsafe to collect:

- switch to documentation-grounded review mode,
- ask for sanitized exports or screenshots of assignments, PIM settings, review schedules, access packages, or ownership mappings,
- label each conclusion as `live evidence`, `documentation-based`, `sanitized evidence`, or `inference`,
- refuse to present documentation as proof of current tenant posture.

Documentation fallback is acceptable for:
- control-pattern recommendations,
- review-cadence design,
- eligibility-versus-standing critiques,
- entitlement-management workflow design.

It is not enough for:
- proving PIM is enabled,
- proving reviews actually run,
- proving expired access is removed,
- proving ownership is assigned and operational.
