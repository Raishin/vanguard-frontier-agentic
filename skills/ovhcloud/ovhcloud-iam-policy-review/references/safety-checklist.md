# Safety checklist

Use this reference before privileged, destructive, access-granting, compliance-impacting, or production-impacting recommendations.

## Non-negotiables

- Never ask users to paste API tokens, application keys, OAuth2 client secrets, session tokens, or account passwords into chat.
- Prefer OVHcloud IAM docs and Terraform provider docs as the source of truth for policy behavior. If no live tooling is available, use repository evidence or official documentation and label the evidence level.
- Do not invent account IDs, NIC handles, URN paths, identity group names, or live policy state.
- Require explicit user approval before recommending policy changes, group membership changes, or OAuth2 scope reductions that affect production workloads.
- Use Context7 or official OVHcloud documentation for current IAM service behavior when the answer depends on OVHcloud-specific semantics.
- Keep remediation least-privilege, reversible, and scoped to the requested account boundary.

## Stress checks

- What URN scope can expose unintended resources or actions?
- What action list can escalate privilege or perform irreversible operations?
- What missing condition block allows access from untrusted origins?
- What identity group aggregation creates unexpected cross-service blast radius?
- What compliance or audit evidence is missing?
- What rollback or validation path is unproven?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live OVHcloud IAM state.
