# Safety checklist

Use this reference before producing a capacity plan that includes a contract period commitment, instance tier recommendation, Cloud-Init userData, or multi-region topology.

## Non-negotiables

- Never ask users to paste OAuth2 tokens, client secrets, API passwords, SSH private keys, or S3 secret keys into chat.
- SSH keys must always be referenced by Contabo secret IDs — never include raw private key material in API payloads, Cloud-Init fragments, or recommendations.
- Do not invent instance specifications, region availability, pricing, or contract terms. Label all specification claims as `documentation-based` since products and pricing may change.
- Require explicit acknowledgment of the contract period and its billing obligation before finalizing any deployment plan. Period selection is binding at instance creation and cannot be shortened.
- Review all Cloud-Init userData for embedded secrets, curl-pipe-sh patterns, hardcoded credentials, and commands that disable audit logging before including it in any plan output. Refuse to include userData that fails this review.
- Do not recommend a multi-region topology without confirming that required instance types and addons are available in all target regions.
- OAuth2 token values must never appear in plan output, scripts, or echo statements.

## Stress checks

- Does the selected contract period create an irreversible billing obligation that exceeds the user's stated budget or timeline? → Surface and require acknowledgment.
- Does the Cloud-Init userData contain raw credentials, unauthenticated package sources, or curl-pipe-sh execution? → Flag and refuse to include without remediation.
- Is the selected instance tier based on actual utilization data or inferred from a general workload description? → Label as inference if not confirmed.
- Does the plan require addons (Private Networking, Additional IPs) that may not be available or require separate provisioning steps in the target region? → Confirm availability before committing.
- What is the recovery path if Cloud-Init provisioning fails and the instance is in an unknown state?
- Is there a confirmed SSH access path that does not depend solely on Cloud-Init completing successfully?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Capacity plans built on inference alone must be clearly flagged as preliminary estimates requiring evidence confirmation before committing to a contract period.
