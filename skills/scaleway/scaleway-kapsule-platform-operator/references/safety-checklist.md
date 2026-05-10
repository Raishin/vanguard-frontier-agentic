# Safety checklist

Use this reference before privileged, production-impacting, or irreversibility-adjacent Kapsule recommendations.

## Non-negotiables

- Never ask users to paste cluster IDs, node pool IDs, `SCW_ACCESS_KEY`, or `SCW_SECRET_KEY` into chat. Work from sanitized Terraform state, sanitized CLI output, or user-provided descriptions only.
- Do not invent cluster IDs, node pool IDs, Kubernetes versions, zone names, or Scaleway service quotas.
- Always flag CNI immutability before discussing CNI selection — CNI cannot be changed without cluster recreation.
- Always flag control-plane version irreversibility before discussing upgrades — Kapsule does not support downgrade to a prior Kubernetes minor version.
- Do not recommend `enforced` placement group for production without explicitly warning that it may block pod scheduling under node failure.
- Require explicit user approval before recommending any action that drains nodes, deletes node pools, or modifies autoscaling bounds on a production cluster.
- Use Context7 or official Scaleway Kubernetes documentation for current version support windows and CNI behavior when the answer depends on Scaleway service details.

## Stress checks

- Is the CNI immutable and does the recommended change require cluster recreation? If so, what is the blast radius?
- Is the Kubernetes version upgrade a one-way door — and is there a rollback path for workloads if the upgrade causes breakage?
- Could `enforced` placement group leave pods unschedulable after a zone failure or node replacement?
- Which namespaces lack PDB coverage, and what is the disruption risk during a node pool drain or upgrade?
- What evidence is missing that would change the readiness verdict?

## Evidence labels

Use `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Kapsule cluster state, node pool configuration, or current Kubernetes version. Findings labeled `inference` must be clearly marked as unconfirmed.
