# MCP and evidence path

Use this reference to choose the right evidence path without leaking environment details.

## Evidence order

1. Microsoft Learn documentation through the user's configured documentation MCP for documented Azure behavior.
2. Sampled read-only Azure evidence when the user has configured it and current-state posture matters.
3. Sanitized user-provided evidence when read-only evidence is unavailable.
4. Explicit inference only when evidence is incomplete.

## What each evidence type proves

- Microsoft Learn proves documented Azure service behavior, Well-Architected guidance, and Microsoft security recommendations.
- Sampled read-only Azure evidence can prove observed configuration or API results in the configured environment at the time sampled.
- User-provided evidence proves only what the user provided, and only if identifiers and secrets are sanitized.
- Inference is not proof; label it and keep recommendations conditional.

## What evidence does not prove

- Documentation does not prove the user's tenant, subscriptions, RBAC, quotas, deployed resources, billing state, security posture, or production readiness.
- Sampled read-only evidence does not prove broad regional availability, all accounts, all subscriptions, all resources, or future posture.
- Secure score, compliance state, and recommendation counts do not prove risk acceptance, owner readiness, or incident-response capability.

## Safe phrasing

- “Microsoft Learn documentation through the user's configured documentation MCP says...”
- “Sampled current-state evidence shows...”
- “The current state was not queried, so this remains an assumption.”
- “This recommendation is documentation-based and needs environment validation before production change.”
