# Integration result

Accepted:
- Added AgentCore-standard `High-risk assumptions to kill` and `Safe command/code verification targets` sections to the operation references for App Service slot swaps, ARM deployment stacks, Cost Management budgets/quota actions, Entra role assignments, and Key Vault rotation/purge operations.
- Bumped each changed skill from 0.1.3 to 0.1.4 and updated verification dates to 2026-06-05.
- Removed raw subscription, resource-group, app, stack, vault, and role-assignment placeholder patterns from the processed reference packs.
- Regenerated skill manifest and asset integrity.

Rejected:
- No broad rewrite of already-lean SKILL.md files.
- No AWS asset edits.
- No environment-specific identifiers or internal MCP details in committed docs.
- No request for credentials, tenant IDs, subscription IDs, customer data, private keys, or secrets.

Remaining risk:
- Azure skills after item 20 and all OCI assets remain outside this repair batch.
