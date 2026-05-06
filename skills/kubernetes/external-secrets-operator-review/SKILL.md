---
name: external-secrets-operator-review
description: Use this skill when reviewing External Secrets Operator (ESO) configuration, including SecretStore, ClusterSecretStore, ExternalSecret, and PushSecret resources. Trigger when a user provides ESO YAML manifests, asks about secret rotation interval compliance, questions whether ClusterSecretStore scope is too broad, or wants to audit the auth method used to reach an external secret store (AWS Secrets Manager, Azure Key Vault, GCP Secret Manager, HashiCorp Vault, 1Password).
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# External Secrets Operator Review

## Purpose
This skill reviews External Secrets Operator configuration for access scope creep, authentication anti-patterns, secret refresh interval compliance, dataFrom blast radius, template misconfiguration, and PushSecret privilege escalation. ESO is a trust bridge between your cluster and your external secret store — a misconfigured ClusterSecretStore or a broad `dataFrom.find` regex can expose every credential in your vault to every namespace, silently, with no audit trail.

## Lean operating rules
- Treat any `ClusterSecretStore` that lacks a `namespaceSelector` or `namespaces` restriction as HIGH — it grants every namespace in the cluster the ability to reference external secrets through that store.
- Treat `dataFrom.find` with a regex that matches more than a single defined secret path prefix (e.g., `name.regexp: .*` or `path: /`) as HIGH — it pulls all matching secrets from the external store into one K8s Secret, creating an enormous blast radius if the Secret is mounted or leaked.
- Treat static credentials in `SecretStore.spec.provider.*.auth.secretRef` (a K8s Secret holding external store credentials) as HIGH — this is a credential-to-access-credentials anti-pattern; prefer IRSA, Azure Workload Identity, GCP Workload Identity, or Vault Kubernetes auth.
- Treat `refreshInterval` greater than 24 hours on any credential that has an external rotation policy shorter than the interval as MEDIUM — the cluster will use a stale, already-rotated secret until the next sync, breaking the workload.
- Treat `target.creationPolicy: Owner` without a documented backup or recreation procedure as MEDIUM — accidental deletion of the ExternalSecret deletes the managed K8s Secret, crashing workloads that mount it.
- Treat `PushSecret` resources with auth scoped to write-all on a store path as HIGH — PushSecret's write path requires elevated permissions; verify the auth scope is minimum-necessary.
- Flag `target.template` misconfigurations that could silently omit required secret keys — a partial K8s Secret causes workload startup failures or silent use of zero-value credentials.
- Do not recommend disabling `refreshInterval` entirely (`refreshInterval: 0`) — that disables automatic rotation pickup.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- SecretStore vs ClusterSecretStore scope assessment (namespace selector coverage)
- Authentication method findings (IRSA/workload-identity vs static credentials)
- dataFrom scope audit (find regex blast radius, extract path coverage)
- refreshInterval compliance findings
- target.creationPolicy and template correctness findings
- PushSecret privilege assessment (if present)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
