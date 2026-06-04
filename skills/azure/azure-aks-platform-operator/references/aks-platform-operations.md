# AKS platform operations

## What people get wrong

- They call AKS production-ready because the cluster exists and nodes are healthy. That ignores ingress, egress, identity, policy, upgrades, subnet capacity, workload health, and recovery.
- They treat Kubernetes manifests as proof of live posture. Desired state is not current state.
- They run production upgrades without proving surge capacity, IP availability, PDB behavior, add-on compatibility, or workload drain behavior.
- They use static credentials in pods instead of workload identity.
- They enable network policy without a default-deny model, DNS/log exceptions, or engine-specific caveats.

## Officially grounded service shape

Microsoft's AKS baseline architecture frames AKS as a multi-team platform with hub-spoke networking, private endpoints for dependencies, separate system and user node pools, Entra integration, private API options, GitOps-friendly operations, monitoring, and explicit upgrade practices. Microsoft Learn also stresses that Kubernetes evolves quickly and that production updates need testing, maintenance windows, quota, subnet capacity, and rollback strategy.

## Non-negotiable design rules

1. Separate system and user node pools; isolate workload pools when risk, OS, tenancy, or scaling differs.
2. Plan subnet and pod IP space for scale-out and upgrade surge, not just today's nodes.
3. Use workload identity or managed identity patterns for Azure access; do not normalize static secrets.
4. Use network policy intentionally: default deny, specific allows, DNS/logging exceptions, and engine-aware implementation.
5. Keep image supply chain private and authorized where production reliability or compliance matters.
6. Test cluster and node upgrades in preproduction before production.
7. Require telemetry for cluster, nodes, pods, ingress, workloads, and dependencies.

## Minimal safe implementation flow

1. Classify cluster topology and criticality.
2. Confirm version, node pools, zones, SKU capacity, OS mix, add-ons, and upgrade channel.
3. Review network: API exposure, ingress, egress, CNI, policy engine, private endpoints, DNS, firewall, and IP capacity.
4. Review identity: Entra integration for humans; workload identity for pods; least-privilege Azure permissions.
5. Review workload resilience: replicas, PDBs, probes, requests/limits, autoscaling, affinity, and disruption behavior.
6. Review operations: GitOps/pipeline flow, monitoring, alerts, backup, incident runbooks, and ownership.
7. Produce a go/no-go with blockers and reversible remediations.

## Safe verification targets

- Kubernetes and node image versions, supported version window, and release notes.
- Node-pool mode, zones, min/max, surge, max pods, taints, labels, and quotas.
- Network plugin, policy engine, public/private API, authorized IPs, ingress controller, and egress firewall.
- Workload identities and federated credentials for Azure resource access.
- Azure Monitor, Container Insights or managed Prometheus, diagnostic settings, alerts, and SLO dashboards.
- Backup/recovery or redeploy strategy for cluster state and persistent application data.

## When to push back

Push back on direct production changes, untested automatic upgrades, shared cluster-admin access, public API exposure with no justification, no default-deny policy, subnet plans that ignore surge, or observability claims without metrics and alerts.
