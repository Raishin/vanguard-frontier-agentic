# Safety checklist

Use before Microsoft Foundry / Azure AI Foundry recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

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

- **Identity and access:** broad Owner/Contributor grants, full-access keys, stale role names, project/resource scope confusion, and unreviewed custom roles.
- **Model and agent release:** quota gaps, rate-limit surprises, missing rollback, untested deployment type, and unsafe public exposure.
- **Network and dependencies:** private endpoint approval gaps, public dependent services, private DNS assumptions, and managed identity permission drift.
- **Data and diagnostics:** prompt/content logging boundaries, sensitive traces, retention gaps, and unclassified telemetry exports.

## Evidence labels

Use sampled evidence, repo evidence, user-provided sanitized evidence, documentation-based, or inference. Documentation alone never proves the user's live Azure environment.
