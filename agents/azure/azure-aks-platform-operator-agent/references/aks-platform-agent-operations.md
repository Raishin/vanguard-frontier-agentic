# Azure AKS Platform Operator Agent Operations

> Version note: AKS release cadence, supported Kubernetes versions, node images, upgrade options, automatic channels, and security controls change frequently. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, kubeconfigs, tenant or subscription identifiers, cluster IDs, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Calling a cluster production-ready because it exists and has nodes.
- Upgrading without validating PDBs, drain behavior, API deprecations, node surge, IP capacity, node image state, and rollback/blue-green options.
- Treating node OS patches, node image upgrades, and Kubernetes version upgrades as the same risk class.
- Using static credentials or over-broad cluster-admin access when workload identity and scoped RBAC are the safer path.
- Ignoring add-ons, service meshes, KEDA, Dapr, ingress, DNS, and policy dependencies when planning an upgrade.

## Officially grounded service shape

- AKS day-2 guidance distinguishes node OS security patches, node image upgrades, and Kubernetes version upgrades, each with different target, cadence, and maintenance implications.
- In-place cluster and node upgrades can affect performance; Microsoft guidance calls for PDBs, surge configuration, maintenance windows, staging validation, and post-maintenance health checks.
- AKS upgrades cordon and drain nodes; misconfigured PDBs can block upgrades, while force upgrade should be reserved for urgent security scenarios.
- AKS security guidance emphasizes Microsoft Entra ID plus Kubernetes RBAC for API-server access and least privilege.
- Node images receive frequent security and component updates; stale node images can put scaling, readiness, security, and support scope at risk.
- Network policy, workload identity, metadata endpoint restrictions, and observability are workload-scope guardrails, not optional decoration.

That is the key insight:

> The agent is not allowed to say “upgrade the cluster” until it proves the workload can survive cordon, drain, surge, API changes, identity boundaries, and rollback.

## Non-negotiable design rules

### 1. Block production approval without supported-version, node-image, PDB, surge, subnet/IP, and observability evidence.

### 2. Do not recommend disruptive cluster or node-pool mutations without an explicit maintenance window, blast-radius statement, and rollback or blue-green path.

### 3. Prefer Microsoft Entra integration, Kubernetes RBAC, workload identity, and scoped access over kubeconfig sprawl, static secrets, or cluster-admin grants.

### 4. Treat force upgrades as emergency-only and call out that they bypass normal disruption protections.

### 5. Treat sampled cluster evidence as scoped to that cluster, region, and time window only.

## Minimal safe implementation flow

- Classify the request: security review, upgrade readiness, node image patching, identity/RBAC, networking, observability, or incident triage.
- Identify cluster, node pools, workloads, add-ons, ingress, policy controls, and requested operation.
- Ground the decision in Microsoft Learn AKS lifecycle, upgrade, reliability, and security guidance.
- Collect read-only configured-environment evidence for versions, nodes, PDBs, deprecated APIs, identities, network policy, limits, and monitoring when available.
- Return go/no-go, blockers, staged rollout plan, rollback path, and residual evidence gaps.

## High-risk assumptions to kill

- A green cluster status means workloads are upgrade-ready.
- PDBs automatically protect availability; they can be missing or misconfigured.
- Node image freshness is optional because Kubernetes version is supported.
- Cluster-admin is acceptable for automation because it is temporary.
- Documentation proves the user's quotas, subnet IP capacity, maintenance windows, or workload health.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Kubernetes version support, available upgrades, node pool versions, node image versions, and release notes.
- PDBs, replicas, health probes, topology spread, surge/maxUnavailable, drain timeout, and undrainable-node behavior.
- Subnet IP headroom, VM quota, autoscaler state, system/user node pool separation, and zone distribution.
- Microsoft Entra integration, Kubernetes RBAC bindings, workload identity, secret usage, and admin credential exposure.
- Container Insights, upgrade events, alerts, rollback/blue-green traffic controls, and post-maintenance validation results.

## When to push back

- The user wants cluster-admin, static credentials, or broad automation permissions for normal operations.
- No workload disruption budget, staging test, maintenance window, or rollback path exists for an upgrade.
- Target version, node image state, or quota/IP capacity is inferred rather than evidenced.
- The proposed force upgrade is not tied to an urgent security response and explicit risk acceptance.
