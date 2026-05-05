---
name: kubernetes-workload-identity-review
description: Use this skill for Kubernetes workload identity review covering AWS IRSA (IAM Roles for Service Accounts), Azure Workload Identity, GCP Workload Identity Federation, and the underlying ServiceAccount token volume projection plus OIDC issuer trust. Trigger when the user asks how a pod should authenticate to cloud services, whether long-lived credentials in a Secret can be replaced, whether the OIDC trust policy is correctly scoped, or whether ServiceAccount token reuse is a risk.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# Kubernetes Workload Identity Review

## Purpose

Review how pods authenticate to cloud services. Long-lived static credentials in Secrets are the largest unmanaged credential surface in most Kubernetes deployments. Workload identity replaces them with short-lived federated tokens via the cluster's OIDC issuer. The review covers ServiceAccount token projection, OIDC issuer trust policy, the cloud-provider IAM mapping, and the runtime check that the pod is actually using the federated token rather than falling back to a static credential.

## Lean operating rules

- Prefer live cluster evidence (`kubectl get serviceaccount,pods -A -o yaml` plus the cluster's OIDC issuer URL and the cloud-provider IAM trust policy) when the active client exposes it; otherwise fall back to official cloud-provider and Kubernetes documentation.
- Separate confirmed facts from inference. If the OIDC issuer URL, IAM trust policy, or pod's projected token volume was not queried, say so.
- Treat **a Pod with both a workload-identity ServiceAccount AND a long-lived credential Secret mounted** as a critical finding — credential precedence often falls back to the static credential, defeating the migration.
- Treat an **OIDC trust policy with `StringEquals` on `aud` but `StringLike` (wildcard) on `sub`** as a critical finding — any ServiceAccount in the cluster can assume the role.
- Treat **`automountServiceAccountToken: true` on pods that don't use the Kubernetes API** as a high finding — token is mounted and exfiltratable, even when not used.
- Challenge ServiceAccount tokens with no `audiences` claim — projected tokens should target a specific cloud audience (`sts.amazonaws.com`, `api://AzureADTokenExchange`, `https://iam.googleapis.com/projects/.../workloadIdentityPools/...`).
- Challenge token expiry windows longer than 1 hour — projected tokens should be short-lived.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [Evidence path and tooling](references/mcp-and-evidence.md) — use when choosing live evidence, confirming OIDC issuer and IAM trust state, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying provider-specific stress checks, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed AWS / Azure / GCP / Kubernetes documentation list and grounded insights.

## Response minimum

Return, at minimum:

- the cloud provider (AWS, Azure, GCP, or generic OIDC) and evidence level,
- the ServiceAccount → IAM identity binding (annotation, label, or trust policy claim) and whether it is correctly scoped,
- the OIDC trust policy scope (`aud`, `sub`, `iss`) — must constrain to a specific ServiceAccount,
- whether long-lived credentials still exist anywhere in the workload (Secret mounts, env vars, sidecars),
- the safest next actions and rollback plan,
- the assumptions or blockers that prevent stronger conclusions.
