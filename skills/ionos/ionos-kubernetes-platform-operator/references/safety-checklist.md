# Safety checklist

Use this reference before advising any IONOS managed Kubernetes control-plane upgrade, node pool scale-down, or kubeconfig rotation.

## Non-negotiables

- Control-plane upgrades are irreversible on IONOS managed Kubernetes — never recommend an upgrade without a confirmed rollback plan and PDB audit for all production workloads.
- Node pool scale-down may evict pods without PDB protection — require PDB coverage confirmation before recommending any scale-down.
- Never request, echo, or transmit kubeconfig credentials, bearer tokens, service account tokens, or customer identifiers.
- Do not invent cluster UUIDs, node pool IDs, Kubernetes versions, or live cluster state.
- Verify the cluster datacenter region matches the declared GDPR processing location before advising any cluster operation.
- Stay advisory — do not call IONOS Kubernetes API endpoints or apply cluster mutations directly.
- Label all claims: `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Stress checks

- Are all production Deployments and StatefulSets covered by PodDisruptionBudgets?
- Does the upgrade version skip a minor version (unsupported on IONOS managed K8s)?
- Are deprecated Kubernetes APIs in active use that will break after the upgrade?
- Is the kubeconfig scoped to the minimum required permissions, or does it carry cluster-admin?
- Does the LAN attachment for the node pool match the intended network isolation topology?
- Is the autoscaling min setting high enough to prevent complete node pool drain under load?
- Does the cluster region match the declared GDPR processing location?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Documentation describes supported Kubernetes versions and behavior — it does not prove the user's cluster state, PDB definitions, or workload placement. Require current cluster evidence for any upgrade or scale-down verdict.
