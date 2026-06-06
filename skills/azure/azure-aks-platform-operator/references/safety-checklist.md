# Safety checklist for Azure AKS Platform Operator

## Non-negotiable gates

- Never ask for kubeconfig, bearer tokens, client certificates, private endpoint hostnames, cluster-admin credentials, or secret manifests.
- Do not approve production readiness without evidence for upgrade path, rollback path, node-pool separation, identity, network policy, ingress/egress, diagnostics, and ownership.
- Treat kubectl-admin access as break-glass, not normal operations. Prefer GitOps or audited pipeline changes for steady-state configuration.
- Require explicit approval before any write, drain, scale, upgrade, policy, identity, network, or workload mutation.
- Do not assume AKS Automatic, baseline self-managed AKS, private clusters, Windows node pools, and service-mesh clusters have the same risk profile.

## High-risk assumptions to kill

- "Managed Kubernetes means Microsoft owns day-2 operations." The platform abstracts parts of the control plane, but cluster design and workload operations remain user responsibilities.
- "The cluster is HA because it has multiple nodes." Node zones, system/user separation, PDBs, replicas, ingress, dependencies, and control-plane SLA matter.
- "Network policy exists, so east-west traffic is safe." Confirm default deny, DNS/log exceptions, namespace coverage, engine, and Windows behavior.
- "Autoscaler will save us." Requests/limits, pod disruption, quota, subnet capacity, and initialization time still determine safe scaling.
- "We can auto-upgrade production." Push back unless preproduction compatibility, maintenance windows, PDBs, quota, and rollback are proven.

## Evidence labels

- `docs_only`: Microsoft Learn-based AKS guidance only.
- `sampled_read_only`: live or configured-environment evidence was sampled safely. State scope and time.
- `manifest_review`: repo or user-supplied manifests were reviewed but live state was not proven.
- `mutation_ready`: current-state evidence, approval, blast radius, and rollback are documented.

## Minimum safe evidence

- Cluster purpose, environment, Kubernetes version, node pools, and operating system mix.
- Network plugin, policy engine, ingress, egress, private API exposure, and subnet capacity.
- Identity model for users and workloads.
- Upgrade channel or manual upgrade process, maintenance window, surge, PDBs, and preproduction validation.
- Metrics, logs, alerts, backup/recovery, runbooks, and ownership.
