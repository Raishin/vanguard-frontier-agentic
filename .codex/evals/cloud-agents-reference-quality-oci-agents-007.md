# OCI Agents Reference Quality Batch 007 Eval Report

Date: 2026-06-06
Provider: OCI
Asset type: agents
Batch size: 5

## Targets

1. `agents/oci/oci-certificates-issuer-review-agent`
2. `agents/oci/oci-cloud-guard-responder-agent`
3. `agents/oci/oci-live-autonomous-db-lifecycle-guard-agent`
4. `agents/oci/oci-live-cost-budget-runaway-guard-agent`
5. `agents/oci/oci-live-iam-policy-compartment-guard-agent`

## Evidence used

Documentation-based evidence from official Oracle documentation:

- Certificates: Certificates overview, certificate/CA lifecycle operations, CA listing, and CA details including CA type and key/signature attributes.
- Cloud Guard: Cloud Guard overview, target scope and recipes, responder recipes and required policy caution, and problem remediation flow.
- Autonomous Database lifecycle: documented start/stop/restart behavior, backup/restore notes, and backup durability/RPO guidance.
- Budgets/cost: budget behavior and alert cadence, Cost Management overview, cost reports, and Cloud Advisor cost recommendations.
- IAM/compartments: policy attachment blast radius, compartment deletion behavior, and compartment API/CLI authorization caveats.

No environment-specific OCI evidence was committed. Guidance now avoids internal connector labels and labels sampled API facts separately from documentation-based claims.

## Changes made

- Replaced remaining `live evidence` labels with `sampled OCI API evidence` wording.
- Replaced stale Cloud Guard references to Oracle MCP tooling with generic read-only OCI MCP wording.
- Corrected OCI Certificates wording from `CA ARN type` to `CA type`.
- Updated component-specific official documentation URLs.
- Bumped changed asset versions and catalog versions.
- Updated `last_verified` to `2026-06-06`.
- Regenerated plugin, Cursor plugin, Kiro Powers, and asset integrity generated files.

## Validation

- Targeted stale phrase audit: PASS
- Version/catalog consistency audit: PASS
- `npm run validate:agent-schema`: PASS
- `npm run validate:asset-integrity`: PASS
- `npm run validate`: PASS (`VALIDATE_EXIT:0`)

## Residual risk

- Documentation evidence proves documented OCI behavior only; it does not prove the user’s tenancy, compartments, IAM policies, deployed resources, or safe mutation readiness.
- Read-only OCI MCP wording is intentionally generic; runtime evidence must still be sampled in the user’s configured environment.
