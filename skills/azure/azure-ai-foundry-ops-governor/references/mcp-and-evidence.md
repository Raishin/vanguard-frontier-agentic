# MCP and evidence path for Microsoft Foundry operations governance

Use Microsoft Learn documentation through the user's configured documentation MCP as the first grounding path for Azure service behavior. This file defines evidence boundaries; it must not imply that documentation proves the user's tenant, subscription, RBAC, quotas, deployed resources, or production readiness.

## Evidence ladder

1. `docs_only`: Microsoft Learn documentation and official architecture guidance. Use for documented behavior, caveats, and safe review criteria.
2. `sampled_read_only`: configured-environment evidence from read-only tools, if available and explicitly scoped. Use only for the sampled resource/time window.
3. `user_supplied`: sanitized outputs, IaC, diagrams, or metrics provided by the user. Treat as unverified unless independently checked.
4. `mutation_ready`: documentation plus current-state evidence plus explicit approval, blast-radius statement, and rollback path.

## Rules

- Do not expose environment-specific implementation details in committed docs or user-facing guidance.
- Do not ask for credentials, tokens, tenant identifiers, subscription identifiers, connection strings, private keys, customer data, or raw secrets.
- If current-state evidence was not sampled, say `not sampled`; do not imply it.
- If evidence is representative or partial, say so. A sample does not prove broad regional availability or production readiness.
- Prefer read-only evidence before mutation planning. Stop for approval before write operations.

## Final-answer evidence language

Use phrases like:

- "Based on Microsoft Learn documentation..."
- "Configured-environment evidence was not sampled in this review."
- "The following is an inference from the provided configuration, not proven live state."
- "This recommendation is mutation-ready only after explicit approval and rollback review."
