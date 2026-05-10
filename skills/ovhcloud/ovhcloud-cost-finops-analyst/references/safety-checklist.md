# Safety checklist

Use this reference before recommending resource deletion, commitment changes, rightsizing, or any cost action that could affect reliability, observability, or compliance posture.

## Non-negotiables

- Never ask users to paste API tokens, application keys, billing credentials, or account passwords into chat.
- Prefer OVHcloud billing and Public Cloud documentation for service behavior. If no live tooling is available, use repository evidence or official documentation and label the evidence level.
- Do not invent project IDs, resource names, pricing rates, utilization baselines, or Savings Plan commitment details.
- Require explicit user approval before recommending instance deletion, volume deletion, snapshot cleanup, or commitment cancellation.
- Never recommend removing backups, monitoring agents, log retention pipelines, or redundant components without explicit risk acceptance from the user.
- Separate confirmed waste from estimated savings. Never present projected savings as guaranteed.
- Use Context7 or official OVHcloud documentation for current billing and pricing behavior when the answer depends on OVHcloud-specific service details.

## Stress checks

- What resource deletion could break backups, log retention, monitoring, or redundancy?
- What rightsizing could cause performance degradation, outage, or SLA breach?
- What commitment cancellation could incur penalty charges or coverage gaps?
- What tagging or chargeback change could misattribute spend and cause organizational confusion?
- What compliance or audit evidence is missing?
- What rollback or recovery path is unproven if the cost action has negative effects?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live OVHcloud project spend, resource state, or Savings Plan coverage.
