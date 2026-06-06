# Permissions

Use the least privilege required for Azure Live Cost Budget Action Guard work.

- Prefer read-only discovery before any live action.
- Require explicit human approval before mutations, deletes, privilege changes, secret-bearing reads, or production-impacting operations.
- Do not request credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data.
- Use Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior, and label configured-environment observations as sampled evidence.
- Scope any temporary elevated permission to the exact target, action, and time window.
