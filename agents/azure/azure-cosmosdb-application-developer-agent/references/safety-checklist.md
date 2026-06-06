# Safety checklist

Use before Cosmos DB application development recommendations that affect access, cost, network exposure, data, compliance, production availability, or automation.

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

- **Data model risk:** low-cardinality keys, hot logical partitions, oversized documents, cross-partition transactions, and unmeasured access patterns.
- **Cost and performance:** query-by-id instead of point reads, unmeasured RU charge, default indexing on write-heavy workloads, and strong consistency cost surprises.
- **Reliability and SDK behavior:** blind retry loops, missing diagnostics, non-idempotent writes, timeout mistakes, and unsafe bulk ingestion.
- **Data protection:** raw customer documents, keys, connection strings, unredacted diagnostics, and sensitive query examples.

## Evidence labels

Use sampled evidence, repo evidence, user-provided sanitized evidence, documentation-based, or inference. Documentation alone never proves the user's live Azure environment.
