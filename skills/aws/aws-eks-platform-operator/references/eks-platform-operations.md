# EKS Platform Operations Guide

Use this reference for Amazon EKS platform operations across cluster access, Kubernetes RBAC, IRSA, EKS Pod Identity, node groups, Karpenter, VPC CNI, add-ons, upgrades, observability, autoscaling, ingress, and multi-tenant workload safety.

## What people get wrong

The lazy story is:

> EKS is Kubernetes, so standard cluster checks are enough.

Wrong. EKS failures sit at the AWS/Kubernetes seam: IAM-to-RBAC mapping, pod identity, CNI IP exhaustion, security group boundaries, add-on drift, node lifecycle, and control-plane version skew.

Common bad assumptions:

- Cluster admin in Kubernetes is the same as AWS account admin.
- IRSA or Pod Identity automatically enforces least privilege.
- Managed node groups remove node lifecycle risk.
- Karpenter fixes capacity without disruption risk.
- VPC CNI networking behaves like overlay networking.
- Add-on upgrades are low risk if the cluster version is supported.

## EKS failure modes

- Access entries, aws-auth, IAM roles, and Kubernetes RBAC create hidden privilege paths.
- Service accounts share broad roles or trust policies across namespaces.
- Pods cannot schedule due to IP exhaustion, security groups for pods, taints, topology spread, or PDB constraints.
- Karpenter/node group changes drain critical workloads without disruption budget evidence.
- CNI/CoreDNS/kube-proxy/add-on drift breaks networking or DNS.
- Cluster upgrade ignores API deprecations, webhook compatibility, managed add-ons, and workload tests.

## Minimum safe workflow

1. Identify cluster version, endpoint exposure, accounts/Regions, node strategy, add-ons, and tenant model.
2. Review access control: IAM principals, access entries/aws-auth, Kubernetes RBAC, IRSA/Pod Identity, and break-glass.
3. Check workload safety: namespaces, network policies, pod security, secrets, image provenance, and resource limits.
4. Check capacity and networking: node groups, Karpenter, VPC CNI IPs, subnets, security groups, load balancers, and ingress.
5. Check operations: upgrades, PDBs, autoscaling, observability, backups, runbooks, and incident history.
6. Recommend staged, reversible changes with drain/rollback plans.
7. Never mutate cluster access, node groups, add-ons, or workloads without explicit approval.

## Verification targets

- EKS cluster version, endpoint access, access entries/aws-auth, IAM roles, Kubernetes RBAC, and audit logs
- IRSA/Pod Identity service account mappings, trust policies, namespace scoping, and token audience boundaries
- managed node groups, self-managed nodes, Karpenter NodePools/NodeClasses, AMIs, labels, taints, and disruption settings
- VPC CNI config, subnet IP capacity, security groups for pods, network policies, ingress/load balancer controller, and DNS/CoreDNS health
- add-on versions, upgrade insights, deprecated APIs, PDBs, HPA/KEDA/Cluster Autoscaler, metrics/logs/traces, and backup/restore evidence
- workload readiness, rollout strategy, image scanning, secrets handling, and incident/change timeline

## When to push back

Push back if the user asks to:

- grant cluster-admin broadly to unblock access
- upgrade EKS without deprecated API and add-on compatibility checks
- change Karpenter/node disruption settings without PDB and workload impact review
- ignore VPC CNI IP capacity or subnet constraints
- treat IRSA/Pod Identity as least privilege without policy review
- mutate production cluster state from advisory evidence alone
