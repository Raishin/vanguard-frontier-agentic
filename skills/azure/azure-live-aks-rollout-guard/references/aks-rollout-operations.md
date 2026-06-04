# Azure AKS Rollout Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Calling a rollout safe because kubectl apply succeeded.
- Ignoring PDBs, replicas, readiness probes, and maxUnavailable until a drain is already blocked.
- Using maxUnavailable to avoid quota issues without admitting the disruption tradeoff.
- Starting a node-pool upgrade without IP, compute quota, and rollback-window evidence.
- Running rollback commands twice because no one tracked current rollout state.

## Officially grounded service shape

Microsoft Learn evidence says AKS rolling upgrades add surge nodes, cordon and drain old nodes, optionally wait for soak time, reimage nodes, repeat, and remove surge nodes. Production node pools are recommended to use max surge rather than disruptive in-place unavailability where possible. Max unavailable can disrupt workloads and increase failures from unsatisfied PodDisruptionBudgets. Blue-green node-pool upgrade includes a final soak window during which rollback is available before old nodes are removed.

- Deployment rollouts and AKS node-pool upgrades are separate but coupled failure domains.
- AKS node rolling upgrades use surge, cordon, drain, optional soak, reimage, and cleanup phases.
- PDBs can protect availability but can also block drains when configured too tightly for replicas and topology.
- Blue-green upgrade provides a validation and rollback window, but rollback disappears after the final commit/removal point.
- Read-only evidence can prove current health snapshots, not future rollout success.

## Non-negotiable design rules

- Stop if target subscription/resource group/cluster/namespace/workload/principal or approval is ambiguous.
- Require preflight evidence for pods, deployments, events, PDBs, HPA, nodes, quotas, and recent errors before mutation.
- Prefer pause/status/describe/dry-run/plan checks before apply, restart, drain, upgrade, or undo.
- Define rollback trigger, command, owner, and verification before execution.
- Sanitize outputs; never print tokens, kubeconfig contents, environment values, or customer data.

## Minimal safe implementation flow

- Confirm target and read-only context: cluster, namespace, workload, node pool, current principal, and requested operation.
- Collect rollout status, deployment spec, PDBs, replicas, readiness, events, node health, capacity, and monitoring signals.
- Compare rollout strategy, maxUnavailable/maxSurge, PDB, and drain/soak settings against documented AKS and Kubernetes behavior.
- Gate mutation on explicit approval, rollback plan, and health criteria.
- After action, verify deployment availability, events, logs/metrics summary, and rollback posture.

## Safe verification targets

- Deployment has enough replicas and PDB allowance for expected disruption.
- Node pools have surge/capacity/IP quota or an explicit accepted maxUnavailable tradeoff.
- Readiness/liveness/startup probes and termination grace match rollout and drain behavior.
- Rollback command and validation window are known before mutation.
- Post-rollout status shows desired replicas available, no new critical events, and monitored health criteria met.

## When to push back

- The user asks to mutate a live cluster without explicit approval.
- PDBs or replicas make disruption mathematically unavoidable.
- Quota/capacity evidence is missing for surge-based upgrades.
- Rollback cannot be explained before starting the rollout.
