# Safety checklist

Use before App Service production readiness recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

## Non-negotiables

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior.
- Use read-only configured-environment evidence only when available and label it as sampled evidence.
- Never ask for credentials, tokens, tenant IDs, subscription IDs, connection strings, certificates, private keys, or customer data.
- Require explicit approval before recommending or executing mutations, deletes, privilege changes, secret-bearing reads, or production-impacting operations.
- State what is unknown; documentation proves service behavior, not the user's deployed state.
- Keep action/tool permissions least-privilege and scoped to the task.
- Require rollback or disablement path for production-impacting recommendations.
- Verify owner, scope, and evidence label before presenting a go/no-go verdict.

## Component risks

- **Release safety:** direct production deployment, no slot, no smoke test, no warm-up, and no rollback-by-swap path.
- **Secrets and identity:** publish profiles, connection strings, certificates, app-setting secrets, missing managed identity, and weak Key Vault usage.
- **Network boundary:** public access assumptions, private endpoint versus VNet integration confusion, DNS gaps, and dependency reachability risk.
- **Recovery and monitoring:** untested restore, non-restored settings, missing alerts, weak health endpoint semantics, and single-instance production plans.

## Evidence labels

Use sampled evidence, repo evidence, user-provided sanitized evidence, documentation-based, or inference. Documentation alone never proves the user's live Azure environment.
