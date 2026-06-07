# Official sources

Use this reference when grounding current OCI behavior for `oci-cloud-guard-responder`.

## Oracle documentation sources

- https://docs.oracle.com/en-us/iaas/cloud-guard/home.htm
- https://docs.oracle.com/en-us/iaas/Content/cloud-guard/using/cg-concepts.htm

## Current documentation refresh (2026-06-05)

- Official OCI documentation is the primary source for documented service behavior.
- OCI API evidence through the user's configured read-only OCI MCP is useful for API shape and sampled configured-environment observations.
- Documentation evidence is not live customer-state evidence. It does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, billing state, security posture, or production readiness.
- Use sampled read-only OCI API evidence only when current-state confirmation is required. Label it as sampled evidence, not broad proof.

## Grounding rule

Docs explain service behavior. Current-state claims require sampled read-only evidence or sanitized user-provided evidence. If current state was not queried or shown, say so.
