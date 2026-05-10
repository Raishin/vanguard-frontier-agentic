# Safety checklist

Use this reference before cost-reduction recommendations that involve resource deletion, reserved instance commitment, or removal of reliability or security controls.

## Non-negotiables

- Never ask users to paste `SCW_ACCESS_KEY`, `SCW_SECRET_KEY`, project IDs, or organization IDs into chat.
- Never recommend reserved instance purchase without first confirming current utilization rate — reserved instances are non-refundable on Scaleway.
- Never recommend deletion of backups, snapshots, or security controls without explicit risk acceptance from the resource owner.
- Never recommend cuts to Cockpit (monitoring) or logging without confirming an alternative observability path is in place.
- Do not invent pricing, resource utilization rates, egress costs, or billing totals. Use official Scaleway pricing pages and label any estimate as `estimated` when utilization data is unavailable.
- Require explicit user approval before recommending termination of any running instance or deletion of any storage resource.

## Stress checks

- Which recommended deletions are irreversible (SBS volumes, Object Storage buckets, snapshots)?
- Which reserved instance purchases lock in spend that cannot be recovered if workload changes?
- Which cost cuts reduce backup retention, security scanning, or observability coverage?
- What utilization data is missing that would change the rightsizing recommendation?
- What is the blast radius if a production instance is terminated instead of a dev/staging one?

## Evidence labels

Use `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Pricing and saving estimates labeled `documentation-based` or `inference` must be presented as estimates, not confirmed figures. Documentation alone never proves the user's live Scaleway billing or resource utilization state.
