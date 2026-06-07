# OCI Agents Reference Quality Batch 006 Eval Report

Date: 2026-06-06
Provider: OCI
Asset type: agents
Batch size: 5

## Targets

1. `agents/oci/oci-storage-backup-steward-agent`
2. `agents/oci/oci-support-incident-coordinator-agent`
3. `agents/oci/oci-waf-cost-optimization-review-agent`
4. `agents/oci/oci-waf-reliability-review-agent`
5. `agents/oci/oci-waf-security-review-agent`

## Evidence used

Documentation-based evidence from official Oracle documentation:

- Storage backup: Block Volume backup behavior, policy-based/manual backup retention, cross-region backup copies, Object Storage lifecycle policies, and Object Storage replication semantics.
- Support incidents: Support Request availability, paid-account eligibility caveat, supported request categories, and support evidence collection guidance.
- WAF cost optimization: Cost Management overview, Cost Analysis/reports/budgets, Cloud Advisor cost-management recommendations, and cost report storage/access behavior.
- WAF reliability: OCI regions, availability domains and fault domains, DR planning guidance, Full Stack DR orchestration/prechecks, and backup/replication evidence boundaries.
- WAF security: Security Zones enforcement, Cloud Guard monitoring/detector recipes, and CIS OCI benchmark landing-zone reference architecture.

No environment-specific OCI evidence was committed. Guidance now refers generically to OCI API evidence through the user’s configured read-only OCI MCP and avoids connector/server labels.

## Changes made

- Replaced stale Oracle-MCP/server-label guidance with generic read-only OCI MCP wording.
- Removed default OCI CLI profile assumptions; profiles must be explicitly provided or confirmed by the user.
- Replaced overbroad `live evidence` labels with `sampled OCI API evidence` / sanitized user evidence / documentation-based / inference labels.
- Updated `official_docs` to component-specific Oracle documentation URLs.
- Bumped changed asset versions:
  - `oci-storage-backup-steward-agent`: `0.2.0` to `0.2.1`
  - `oci-support-incident-coordinator-agent`: `0.2.0` to `0.2.1`
  - WAF OCI review agents: `0.1.0` to `0.1.1`
- Updated `last_verified` to `2026-06-06` in adjacent metadata and catalog entries.
- Regenerated plugin, Cursor plugin, Kiro Powers, and asset integrity generated files.

## Validation

- Targeted stale phrase audit: PASS
- Version/catalog consistency audit: PASS
- `npm run validate:agent-schema`: PASS
- `npm run validate:asset-integrity`: PASS
- `npm run validate`: PASS (`VALIDATE_EXIT:0`)

## Residual risk

- Documentation evidence proves documented OCI behavior, not the user’s tenancy, compartments, IAM policies, limits, deployed resources, support entitlement, current cost posture, or DR readiness.
- Read-only OCI MCP wording remains intentionally generic; actual tool availability must be sampled in the user’s configured environment at runtime.
