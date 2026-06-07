# Safety checklist

Use before Cosmos DB performance investigation recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

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

- **Misdiagnosis risk:** treating all 429s as outages, ignoring hot partitions, guessing at indexing, and increasing RU/s without bottleneck proof.
- **Observability risk:** broad diagnostics without retention/cost controls, sensitive fields in logs, and overgeneralizing short samples.
- **Cost and architecture:** hiding bad partition design with throughput, autoscale/manual mismatch, and untested repartitioning or indexing changes.
- **Client behavior:** SDK retry misconfiguration, timeout mismatch, non-idempotent retries, and missing query metrics.

## Evidence labels

Use sampled evidence, repo evidence, user-provided sanitized evidence, documentation-based, or inference. Documentation alone never proves the user's live Azure environment.
