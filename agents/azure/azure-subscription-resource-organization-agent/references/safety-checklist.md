# Safety checklist

Use before recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

## Non-negotiables

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for Azure service behavior.
- Use read-only configured-environment evidence only when available and label it as sampled evidence.
- Never ask for credentials, tokens, tenant identifiers, subscription identifiers, billing identifiers, connection strings, certificates, private keys, kubeconfigs, negotiated discount sheets, or customer data.
- Require explicit approval before recommending or executing mutations, deletes, privilege changes, secret-bearing reads, billing-impacting actions, or production-impacting operations.
- State what is unknown; documentation proves service behavior, not the user's deployed state.
- Keep action permissions least-privilege and scoped to the task.
- Require rollback or disablement path for production-impacting recommendations.
- Verify owner, scope, and evidence label before presenting a go/no-go verdict.

## Component risks

- Identity and access: standing privilege, broad roles, stale owners, weak approval, and missing review outcome evidence.
- Network and data exposure: public access, private DNS gaps, sensitive logs, secret-bearing reads, and unclassified exports.
- Production operations: missing rollback, unclear target, untested recovery, stale alerts, and unsafe automation.
- Cost and governance: missing cost owner, untagged spend, commitment underutilization, false savings claims, policy bypass, and undocumented drift.

## Evidence labels

Use sampled evidence, repo evidence, user-provided sanitized evidence, documentation-based, or inference. Documentation alone never proves the user's live Azure environment.
