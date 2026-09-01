# Integration result

Accepted:
- Added AgentCore-standard `High-risk assumptions to kill` and `Safe command/code verification targets` sections to the service operation references for Azure identity governance, Key Vault secret lifecycle, Key Vault certificate issuer review, landing zone architecture, and live AKS rollout guard.
- Bumped each changed skill from 0.1.2 to 0.1.3 and updated verification dates to 2026-06-05.
- Removed raw subscription-scope placeholder guidance from the touched AKS permission model and kept command examples sanitized.
- Regenerated skill manifest and asset integrity.

Rejected:
- No broad rewrite of already-lean SKILL.md files.
- No AWS asset edits.
- No environment-specific identifiers or internal MCP details in committed docs.
- No request for credentials, tenant IDs, subscription IDs, customer data, private keys, or secrets.

Remaining risk:
- Azure skills after item 15 and all OCI assets remain outside this repair batch.
