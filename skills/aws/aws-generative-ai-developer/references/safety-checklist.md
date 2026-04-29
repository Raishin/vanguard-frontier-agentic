# Safety checklist

Use before recommending AWS generative AI architecture, Bedrock integrations, data retention, or production rollout.

## Non-negotiables

- Do not ask for or print secrets, credentials, private keys, account numbers, customer identifiers, prompt logs, or private source data.
- Prefer least-privilege IAM, scoped data access, sanitized prompts, and explicit approval before mutation or production rollout.
- Prefer serverless managed services for this role unless a concrete blocker is provided.
- Do not normalize prompt-injection exposure, unsafe tool invocation, or storing sensitive prompts/outputs without retention controls.
- Confirm logging, tracing, rate limits, quotas, and cost visibility before production recommendations.

## Component risks

- **Bedrock / prompts:** prompt injection, unsafe tool use, over-broad model access, unbounded token cost.
- **Lambda / orchestration:** timeouts, retries, idempotency bugs, DLQ gaps, concurrency spikes.
- **Retrieval / storage:** data leakage, stale embeddings, public buckets, over-broad table permissions.
- **APIs / auth:** weak auth, noisy anonymous traffic, over-broad CORS, missing throttling.
- **State / memory:** retaining user data too long, mixing tenants, no delete path.

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
