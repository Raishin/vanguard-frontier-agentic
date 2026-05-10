# Safety checklist

Before recommending or executing any live Kapsule mutation, enforce every item on this checklist. A single unresolved item is a hard stop.

## Hard stops — refuse if any of these are missing

- **No confirmed cluster ID + region/zone**: Do not proceed. Do not guess or infer the target cluster. Ask explicitly and wait for confirmation.
- **No live cluster health confirmation**: Do not proceed on `documentation-based` or `inference` health claims. Require live evidence (`scw k8s cluster get` output or `kubectl get nodes` output) or explicit user-provided sanitized evidence confirming all nodes Ready.
- **No PDB audit**: Do not proceed without a `kubectl get pdb --all-namespaces` result reviewed for all workload namespaces. Unprotected workloads must be explicitly accepted by the user — not silently assumed low-risk.
- **No rollback plan**: Do not proceed without a documented rollback path. For version upgrades: prior version noted, node pool recreation path identified. For node pool deletion: workload migration confirmed. For scaling: headroom at new pool size verified.
- **No named approving identity**: Do not proceed without an explicit human sign-off token, ticket reference, or written approval from an identified approver. A plan or advisory question is not approval.
- **No health check post-mutation**: Do not declare success without verifying node Ready status, workload pod status, and PDB violation absence after the mutation completes.

## Mandatory posture

- Prefer the smallest reversible mutation. Never chain destructive operations without re-confirming cluster health between steps.
- Treat `documentation-based` and `inference` evidence as insufficient for any destructive operation. Live evidence or user-confirmed sanitized evidence is the minimum bar.
- Never request `SCW_ACCESS_KEY` or `SCW_SECRET_KEY` directly. Credentials must be pre-configured in the environment.
- If live Scaleway MCP tooling is unavailable, say so explicitly. Do not substitute documentation-based claims for live evidence when gating a destructive operation.
- Treat Kapsule control-plane version upgrades as a one-way door — no downgrade is possible after completion. Escalate blast-radius awareness before approving.
- Treat CNI type as immutable post-creation — any request to change CNI requires cluster recreation. Require full blast-radius assessment and explicit approval before routing to any cluster recreation path.
- Node pool deletion is immediate: pods are evicted without grace once deletion is confirmed. Cordon-drain must complete and be verified before deletion proceeds.

## Stress checks

- Is the cluster ID and region/zone confirmed by the user — or assumed from context?
- Is cluster health confirmed with live evidence — or inferred from a prior session or documentation?
- Are all namespaces covered by the PDB audit — or only a subset?
- Is the rollback plan specific and tested — or generic and assumed?
- Is the approving identity named and traceable — or just implicit consent from the request?
- Could the proposed mutation affect a production workload that was not explicitly in scope?

## Evidence labels

Use `live evidence` (from live Scaleway API or kubectl output), `user-provided sanitized evidence` (user-shared CLI or console output), `documentation-based` (from official docs), or `inference` (deduced from context). Only `live evidence` and `user-provided sanitized evidence` are acceptable for gating a destructive live mutation.
