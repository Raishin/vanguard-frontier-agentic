# Safety checklist

Use this reference before recommending any decommission, downsize, snapshot deletion, or contract change on an IONOS Cloud resource.

## Non-negotiables

- Never recommend removing backups, disabling encryption, reducing redundancy, or eliminating audit logging without explicit written risk acceptance from the resource owner.
- GDPR data residency constraints may block cross-region consolidation — flag this before recommending any region change, even if it saves cost.
- Do not invent utilization figures, pricing data, billing amounts, or resource identifiers — if utilization was not queried or shown, state that explicitly.
- Never recommend decommissioning a resource that is the sole backup or recovery path for a production workload.
- Stay advisory — do not call IONOS billing APIs or delete resources.
- Label all claims: `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Stress checks

- What reliability or availability impact does this optimization carry?
- What backup or recovery capability is removed if a snapshot policy is tightened?
- Does this resource serve as a compliance artifact (e.g., audit log storage) that cannot be shortened without a compliance review?
- Is cross-region consolidation blocked by a GDPR processing location constraint?
- What is the rollback cost if a decommission turns out to be premature — is there a re-provisioning path?
- Is the utilization evidence current enough to justify a downsize recommendation, or is it too stale?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Pricing estimates from documentation are `documentation-based` — actual savings depend on the user's specific contract tier and usage pattern, which requires user-provided evidence.
