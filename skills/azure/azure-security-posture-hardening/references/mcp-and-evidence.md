# MCP and evidence path

Use this reference when deciding how to ground `azure-security-posture-hardening` guidance.

## Evidence order

1. Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior.
2. Sampled read-only Azure evidence when the user has configured it and current-state confirmation is necessary.
3. Sanitized user-provided evidence when no read-only evidence path is available.
4. Clearly labeled inference when evidence is incomplete.

## Boundaries

- Documentation evidence does not prove the user's tenant, subscription, RBAC, quotas, deployed resources, billing state, security posture, reliability state, or production readiness.
- Sampled read-only evidence proves only the sampled configured environment and time window.
- User-provided evidence can be incomplete or stale; preserve uncertainty.
- Never ask for credentials, tokens, secrets, tenant IDs, subscription IDs, resource IDs, customer data, private keys, or raw incident payloads.

## Required phrasing

Use generic phrasing such as "Microsoft Learn documentation through the user's configured documentation MCP". Do not expose internal tool names, profile names, environment names, or local identifiers in committed docs.
