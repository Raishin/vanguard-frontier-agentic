# Azure Live AKS Rollout Guard Agent Operations

> Version note: Azure service behavior, API surfaces, permissions, and operational safety guidance change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Advancing a rollout because the deployment exists, while pods are not ready or PDBs allow no disruption.
- Treating kubectl rollout undo as always safe; double rollbacks and unknown revision history can cause churn.
- Ignoring maxUnavailable, maxSurge, readiness probes, liveness probes, startup probes, and graceful termination.
- Confusing cluster or node-pool upgrade settings with application deployment rolling-update settings.
- Executing live kubectl actions without target confirmation, rollback plan, and explicit approval.

## Officially grounded service shape

- AKS reliability guidance recommends PDBs, resource limits, multi-replica applications, topology spread, probes, and rolling update controls.
- PDBs protect voluntary disruptions; missing or misconfigured PDBs can allow downtime or block safe change.
- maxSurge allows extra pods during rolling updates; maxUnavailable limits unavailable pods during the update.
- Readiness probes determine whether a pod can receive traffic; liveness probes restart unhealthy containers; startup probes protect slow starts.
- Kubernetes deployments support rollout status, pause, resume, history, and undo, but live actions need target and approval discipline.

That is the key insight:

> The agent is a live-action gate. It must prove target, health, disruption budget, strategy, and rollback before allowing any rollout-changing command.

## Non-negotiable design rules

### 1. Never execute or recommend a live rollout action without explicit target, impact, approval, and rollback evidence.

### 2. Block advancement when PDB allowed disruptions, readiness, replicas, or rollout status contradict availability.

### 3. Treat undo, pause, resume, patch, scale, and apply as live mutations that need approval.

### 4. Prefer read-only describe, get, history, status, events, and logs before any mutation.

### 5. Label configured-environment observations as sampled and bounded to the cluster, namespace, workload, and time window.

## Minimal safe implementation flow

- Confirm target cluster, namespace, deployment, desired action, approval state, and rollback owner.
- Ground checks in Microsoft Learn AKS reliability guidance and Kubernetes rollout/PDB docs.
- Collect read-only evidence for rollout status, replicas, pods, PDBs, strategy, probes, events, history, and recent errors.
- Decide: advance, pause, resume, undo, or block; if action is live, require explicit human approval.
- Verify post-action health with rollout status, pod readiness, events, and open risk summary.

## High-risk assumptions to kill

- kubectl context is correct because the shell prompt says so.
- One healthy pod is enough for a production rollout.
- A PDB exists, therefore disruption is safe.
- Rollout undo is harmless and always returns to the desired version.
- Documentation proves this cluster health, target, approval, or rollback posture.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Current context, cluster identity, namespace, deployment selector, image/revision, and intended action.
- Rollout status, deployment conditions, replicas desired/current/available/unavailable, and ReplicaSet history.
- PDBs, allowed disruptions, multi-replica coverage, readiness/liveness/startup probes, graceful termination, maxSurge, and maxUnavailable.
- Pod events, recent restarts, CrashLoopBackOff, scheduling failures, resource pressure, and ingress/service health indicators.
- Approval record, rollback command target, verification command list, and post-action health evidence.

## When to push back

- The target cluster, namespace, deployment, or approval state is ambiguous.
- PDBs or readiness evidence show unsafe disruption.
- The user wants to paste kubeconfig, tokens, or raw environment dumps.
- The requested action would mutate live state without rollback and verification evidence.
