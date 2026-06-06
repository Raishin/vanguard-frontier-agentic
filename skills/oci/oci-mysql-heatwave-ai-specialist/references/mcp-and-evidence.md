# MCP and evidence path

Use this reference to choose the right evidence path without leaking environment details.

## Evidence order

1. Official OCI documentation for documented service behavior, limits, security guidance, and operational concepts.
2. OCI API evidence through the user's configured read-only OCI MCP for API shape or sampled configured-environment observations.
3. Microsoft Learn documentation through the user's configured documentation MCP when Azure interconnect, ExpressRoute, or Azure operational patterns are in scope.
4. Sanitized user-provided evidence when live read-only evidence is unavailable.
5. Explicit inference only when evidence is incomplete.

## What each evidence type proves

- Official documentation proves documented cloud service behavior at the time checked.
- Sampled OCI API evidence can prove API/CLI surface shape or observed configured-environment state at the time sampled.
- Microsoft Learn documentation proves documented Azure behavior, not the user's subscription state.
- User-provided evidence proves only what was provided and only if sanitized.
- Inference is not proof; label it and keep recommendations conditional.

## What evidence does not prove

- Documentation does not prove the user's tenancy, compartments, IAM policies, limits, resources, billing state, security posture, or production readiness.
- Sampled API evidence does not prove all regions, all compartments, all resources, future posture, or full operational readiness.
- Command help proves command/API surface shape; it does not prove permission, resource existence, or safe execution.

## Safe phrasing

- “Official OCI documentation says...”
- “OCI API evidence through the user's configured read-only OCI MCP shows...”
- “Microsoft Learn documentation through the user's configured documentation MCP says...”
- “The current state was not queried, so this remains an assumption.”
- “This recommendation is documentation-based and needs environment validation before production change.”
