# Safety checklist

Use this reference before recommending vRack topology changes, VLAN additions or removals, private network modifications, security group updates, or any action that could affect network isolation or connectivity.

## Non-negotiables

- Never ask users to paste API tokens, application keys, SSH private keys, or account passwords into chat.
- Prefer OVHcloud networking docs and Terraform provider docs for service behavior. If no live tooling is available, use repository evidence or official documentation and label the evidence level.
- Do not invent vRack IDs, VLAN IDs, subnet CIDRs, security group rule IDs, or live topology state.
- Require explicit user approval before recommending vRack member detachment, VLAN removal, private subnet deletion, or security group rule changes that affect production traffic paths.
- Never recommend a topology change without a confirmed rollback plan and blast-radius assessment.
- If the current vRack member inventory has not been confirmed, stop and require it before proceeding with topology change recommendations.
- Use Context7 or official OVHcloud documentation for current vRack service behavior, VLAN limits, and MCK private network attachment semantics.

## Stress checks

- What topology change could isolate a production member and cause a service outage?
- What VLAN ID conflict could cause cross-tenant or cross-tier traffic leakage?
- What security group rule change leaves backend services reachable from the public internet or wrong tier?
- What private network modification could break Kubernetes cluster networking or inter-node communication?
- What DNS change could cause resolution failures, delegation breaks, or split-horizon inconsistencies?
- What rollback or recovery path is available if the topology change has unintended effects?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live vRack membership, VLAN assignments, security group rules, or routing state.
