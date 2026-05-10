# Safety checklist

Use this reference before capacity recommendations that are cost-changing, infrastructure-splitting, or involve production-impacting provisioning decisions.

## Non-negotiables

- Never ask users to paste API tokens, project secrets, customer data, or sensitive Hetzner account identifiers into chat.
- Prefer live Hetzner MCP read-only evidence when available. If unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.com/cloud/ and Context7.
- Do not invent resource counts, quota limits, growth rates, provisioning lead times, or live project state.
- Hetzner has no native auto-scaling — never present a capacity plan that assumes auto-scaling will absorb demand spikes.
- Storage Box Snapshot Plans require both `hour` and `minute` parameters — flag incomplete snapshot schedules rather than assuming they work.
- Public IPs are opt-in since API v1.34 — count Primary IPs and Floating IPs separately; do not assume servers have public IPs in resource headroom calculations.
- Never recommend a project split or resource deletion without confirming which workloads depend on the affected project boundary.
- Require explicit human approval before recommending changes that would cause downtime, trigger a resize, or alter region distribution for production traffic.

## Stress checks

- Which resource type reaches quota exhaustion first on the current trajectory?
- Is any single region carrying all production load with no distribution plan?
- Does the capacity plan account for Hetzner's ~2–5 minute manual provisioning lead time?
- Would a recommended project split disrupt existing Network, Firewall, or Load Balancer configurations that span the current project boundary?
- Is the evidence level sufficient to project growth confidently, or is it inference from incomplete data?
- What rollback path exists if a region expansion or project split introduces unexpected connectivity gaps?
- What compliance or operational constraints limit which regions can be used?

## Evidence labels

Use `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's current resource counts, quota headroom, or growth trajectory.
