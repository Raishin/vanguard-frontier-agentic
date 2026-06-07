# Oracle and OCI MCP Grounded Advisor Operations Reference

## What people get wrong

- An Oracle-published MCP package is safe in production by default.
- Tool availability means the user has permission or should use the tool.
- A read/write-capable connector is acceptable for discovery.
- Database or cloud identifiers can be pasted into prompts for convenience.
- Community examples replace official Oracle source verification.

## Officially grounded service shape

- Oracle-published MCP sources document available Oracle tooling and integration paths; they do not prove the user’s local runtime wiring, permissions, or safety posture.
- MCP can connect assistants to external tools, data sources, and services, so authentication, authorization, tool boundaries, and auditability are first-class review items.
- OCI API evidence through the user’s configured read-only OCI MCP can prove sampled command shape or configured-environment observations, not broad tenancy posture.
- Documentation evidence and configured-environment evidence must be separated in every recommendation.

## Non-negotiable design rules

- Verify official Oracle source, package identity, capability set, auth model, read/write boundary, data sensitivity, and audit path before recommending use.
- Prefer read-only discovery for advisory work and require explicit approval for any mutation-capable path.
- Do not name local connector labels, profile names, internal tool names, or environment-specific identifiers in committed guidance.
- Do not ask for credentials, tokens, wallets, private keys, tenancy details, database connection strings, customer data, or config contents.
- Mark every claim as official-documentation evidence, sampled configured-environment evidence, inference, or unknown.

## Minimal safe implementation flow

- Identify the Oracle product, tool, or protocol question.
- Verify current official Oracle source and documented capability.
- Use OCI API evidence through the user’s configured read-only OCI MCP only for sanitized command shape or sampled current-state evidence.
- Evaluate auth, permission, data exposure, and mutation boundaries before advising.
- Return a grounded recommendation with explicit uncertainties.

## High-risk assumptions to kill

- Documentation proves service behavior; it does not prove the user's deployed posture.
- Sampled API evidence proves only the sampled command shape or observation.
- Read-only discovery is not approval for mutation.
- Missing evidence is a blocker, not a detail to smooth over.

## Safe command/code verification targets

- Prefer schema, manifest, link, and asset-integrity validation for repository edits.
- Prefer read-only list/get/help operations for cloud evidence.
- Redact or omit identifiers and sensitive values from notes and reports.

## Safe verification targets

- Official OCI documentation URL is attached to each service-behavior claim.
- Sampled API evidence is labeled with scope and limitation.
- Approval gates are explicit for every proposed mutation.
- Evidence gaps are listed as open questions.

## When to push back

- The request treats MCP as a trust boundary instead of a tool invocation protocol.
- The user wants production enablement without auth, permission, audit, and data-sensitivity review.
- The prompt asks for secrets, config contents, identifiers, or customer data.
