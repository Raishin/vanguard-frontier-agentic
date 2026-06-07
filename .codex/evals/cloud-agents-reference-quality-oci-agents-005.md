# OCI Agents Reference Quality Batch 005 Eval Report

Date: 2026-06-06
Provider: OCI
Asset type: agents
Batch size: 5

## Targets

1. `agents/oci/oci-recovery-service-operator-agent`
2. `agents/oci/oci-registry-artifact-governor-agent`
3. `agents/oci/oci-resource-search-inventory-analyst-agent`
4. `agents/oci/oci-security-compliance-reviewer-agent`
5. `agents/oci/oci-solution-architect-agent`

## Evidence used

Documentation-based evidence from official Oracle documentation:

- Recovery Service: protected database creation, backup automation, protection policies, backup health, recovery window, and data-loss exposure semantics.
- Container Registry: OCI-compliant artifacts, repository/image management, vulnerability scanning through Vulnerability Scanning, and automatic rescans when new CVEs are added.
- Resource Search: free text and advanced resource query behavior, supported resource indexing, regional scoping, and limited common result attributes.
- Security/compliance: Security Zones enforcement model, Cloud Guard dependency/integration, security service overview, and Compliance Documents access/API caveats.
- Solution architecture: OCI landing zone implementation, Core Landing Zone scope, secure/scalable foundation, and Well-Architected landing zone positioning.

No environment-specific OCI evidence was committed. Guidance now refers generically to OCI API evidence through the user’s configured read-only OCI MCP and avoids connector/server labels.

## Changes made

- Replaced stale Oracle-MCP/server-name guidance with generic read-only OCI MCP wording.
- Removed default OCI CLI profile assumptions; profiles must be explicitly provided or confirmed by the user.
- Replaced overbroad `live evidence` labels with `sampled OCI API evidence` / sanitized user evidence / documentation-based / inference labels.
- Updated `official_docs` to component-specific Oracle documentation URLs.
- Bumped changed asset versions from `0.2.0` to `0.2.1` in `AGENT.md`, adjacent `metadata.json`, and `catalog/agents.json`.
- Updated `last_verified` to `2026-06-06` in adjacent metadata and catalog entries.
- Regenerated plugin, Cursor plugin, Kiro Powers, and asset integrity generated files.

## Validation

- Targeted stale phrase audit: PASS
- Version/catalog consistency audit: PASS
- `npm run validate:agent-schema`: PASS
- `npm run validate:asset-integrity`: PASS
- `npm run validate`: PASS (`VALIDATE_EXIT:0`)

## Residual risk

- Documentation evidence proves documented OCI behavior, not the user’s tenancy, compartments, IAM policies, quotas, deployed resources, or regional availability.
- Read-only OCI MCP wording remains intentionally generic; actual tool availability must be sampled in the user’s configured environment at runtime.
