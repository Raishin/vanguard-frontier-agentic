# OCI Agents Reference Quality Batch 008 Eval Report

Date: 2026-06-06
Provider: OCI
Asset type: agents
Batch size: 5

## Targets

1. `agents/oci/oci-live-network-security-rule-guard-agent`
2. `agents/oci/oci-live-oke-rollout-guard-agent`
3. `agents/oci/oci-live-resource-manager-stack-guard-agent`
4. `agents/oci/oci-live-vault-key-destruction-guard-agent`
5. `agents/oci/oci-maestro-agent`

## Evidence used

Documentation-based evidence from official Oracle documentation:

- Network security rules: security list updates replace the full rule set; NSG rules have rule identifiers and separate add/update/remove operations.
- OKE rollout guard: OKE overview, managed node pool updates, node pool modification/cordon/drain guidance, and managed node cycling behavior.
- Resource Manager: stack/job workflow, plan/apply/destroy job creation, stack/job management, and plan rollback flow.
- Vault key destruction: key deletion scheduling, pending deletion/inaccessibility and irreversible deletion warnings, vault deletion cascading key behavior.
- Maestro routing safety: IAM policy attachment blast radius, regions/availability domains, Security Zones, and Cloud Guard grounding for routing decisions.

No environment-specific OCI evidence was committed. Guidance now avoids internal connector labels and labels sampled API facts separately from documentation-based claims.

## Changes made

- Replaced remaining `live evidence` labels with `sampled OCI API evidence` wording.
- Updated component-specific official documentation URLs.
- Bumped changed asset versions from `0.1.0` to `0.1.1` in `AGENT.md`, adjacent `metadata.json`, and `catalog/agents.json`.
- Updated `last_verified` to `2026-06-06`.
- Regenerated plugin, Cursor plugin, Kiro Powers, and asset integrity generated files.

## Validation

- Targeted stale phrase audit: PASS
- Repo-wide OCI stale phrase audit: PASS (`0` matches for the audited stale patterns)
- Version/catalog consistency audit: PASS
- `npm run validate:agent-schema`: PASS
- `npm run validate:asset-integrity`: PASS
- `npm run validate`: PASS (`VALIDATE_EXIT:0`)

## Residual risk

- Documentation evidence proves documented OCI behavior only; it does not prove the user’s tenancy, compartments, IAM policies, deployed resources, mutation safety, or production readiness.
- Read-only OCI MCP wording remains intentionally generic; runtime evidence must still be sampled in the user’s configured environment.
