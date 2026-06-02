# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/eks/latest/userguide/creating-access-entries.html
- https://docs.aws.amazon.com/eks/latest/best-practices/cluster-upgrades.html
- https://docs.aws.amazon.com/eks/latest/userguide/eks-add-ons.html
- https://docs.aws.amazon.com/eks/latest/userguide/security-iam.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- EKS access entries associate IAM principal ARNs with Kubernetes access, Kubernetes groups, and EKS access policies; they are a control point for cluster access review.
- EKS cluster upgrade best-practice guidance is relevant to platform operations because control plane, node, add-on, and workload compatibility can break independently.

Sampled live evidence:
- Read-only regional availability sampling reported Amazon EKS as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `EKS+DescribeCluster` and `EKS+ListAddons` were reported `isAvailableIn` in those regions.

Review implications:
- Do not approve EKS posture without evidence for cluster version, add-ons, node groups/Fargate profiles, access entries/RBAC, IRSA or pod identity, network policy, logging, and upgrade/rollback plan.
- Regional service availability does not prove account quota, cluster health, add-on compatibility, or Kubernetes object state.
