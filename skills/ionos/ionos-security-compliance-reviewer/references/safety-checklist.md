# Safety checklist

Use this reference before recommending any configuration change, remediation, or posture hardening that touches a production IONOS Cloud environment.

## Non-negotiables

- Never request, echo, or transmit bearer tokens, API keys, database credentials, or customer account identifiers.
- Never recommend disabling encryption at rest or in transit for any production workload.
- GDPR data residency is a hard blocker — if the declared processing region and the active datacenter region do not match, flag this as CRITICAL before any other finding.
- Do not invent datacenter IDs, resource names, token scopes, or live configuration state.
- Require explicit evidence of API token scope before approving any IAM posture as compliant.
- Stay advisory — do not call IONOS Cloud API endpoints or apply configuration changes.
- Label all claims: `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Stress checks

- What can expose data outside the declared GDPR region? (Cross-border endpoint mismatch, replication settings)
- What can allow unauthorized access to production resources? (Overly broad API token, missing LAN firewall rule)
- What can break audit trail continuity? (Logging disabled, short retention period)
- What can expose credentials or API tokens? (Plaintext config, unrestricted token scope)
- What compliance evidence is missing or assumed rather than verified?
- Is the regional endpoint correct for each IONOS service in scope? (`de-txl`, `de-fra`, `fr-par`, `es-vit`, `gb-lhr`, `gb-bhx`, `us-las`, `us-mci`, `us-ewr`)

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live IONOS Cloud compliance posture — it describes expected service behavior only.
