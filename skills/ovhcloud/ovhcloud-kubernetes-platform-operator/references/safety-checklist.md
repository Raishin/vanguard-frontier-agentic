# Safety checklist

Use this reference before recommending cluster upgrades, node pool deletions, scale-to-zero operations, RBAC changes, or any action that could disrupt running workloads.

## Non-negotiables

- Never ask users to paste kubeconfig files, service account tokens, API tokens, or cluster certificates into chat.
- Prefer OVHcloud Managed Kubernetes docs and Terraform provider docs for service behavior. If no live tooling is available, use repository evidence or official documentation and label the evidence level.
- Do not invent cluster names, node pool IDs, Kubernetes versions, autoscaling limits, or live workload state.
- Require explicit user approval before recommending cluster deletion, node pool deletion, forced drain, or scale-to-zero on production workloads.
- Do not recommend a node pool upgrade or replacement without confirmed PodDisruptionBudgets and drain readiness for all affected workloads.
- Keep recommendations reversible; prefer blue-green node pool rotation over in-place forced replacement.
- Use Context7 or official OVHcloud documentation for current Kubernetes version support windows and MCK-specific behavior.

## Stress checks

- What workloads lack a PDB and would be disrupted by a node drain or pool replacement?
- What Kubernetes version upgrade introduces API deprecations or breaking add-on changes?
- What RBAC binding creates cluster-wide write access or privilege escalation paths?
- What network policy gap leaves backend services exposed across namespaces?
- What autoscaling configuration could trigger uncontrolled cost growth?
- What rollback path is available if the upgrade fails mid-process?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live cluster state, workload PDB coverage, or RBAC configuration.
