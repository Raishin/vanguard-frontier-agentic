# Safety checklist

Use this reference before cost-changing, irreversible, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste API tokens, project secrets, customer data, or sensitive Hetzner account identifiers into chat.
- Prefer live Hetzner MCP read-only evidence when available. If unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.com/cloud/ and Context7.
- Do not invent resource IDs, pricing figures, current resource states, or utilization metrics.
- Do not recommend deleting Volumes, snapshots, or Floating IPs that serve as the only recovery or failover path without explicit user risk acceptance.
- Require explicit human approval before any recommendation that involves irreversible deletion, downtime-inducing resize, or removal of redundancy.
- Storage Box Snapshot Plans require both `hour` and `minute` parameters — flag incomplete backup schedules rather than assuming they work.
- Unattached Primary IPs and Floating IPs incur cost even without an attached server — always flag these, but confirm recovery intent before recommending deletion.
- Keep recommendations reversible, staged, and scoped to the requested project or environment.

## Stress checks

- Does this recommendation remove the only recovery path (last snapshot, last Volume backup)?
- Would this rightsizing break the workload's CPU, memory, or network contract?
- Is the resource genuinely idle or might it serve a burst, backup, or failover role?
- Is the evidence level sufficient to confidently estimate savings, or is it inference?
- What rollback path exists if a deletion or resize proves to be wrong?
- What compliance or billing audit evidence is missing?

## Evidence labels

Use `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's current Hetzner resource state or billing breakdown.
