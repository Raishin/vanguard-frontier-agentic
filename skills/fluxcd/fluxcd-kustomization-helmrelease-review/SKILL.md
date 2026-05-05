---
name: fluxcd-kustomization-helmrelease-review
description: Use this skill when reviewing FluxCD Kustomization, HelmRelease, GitRepository, HelmRepository, or OCIRepository resources. Trigger when the user asks whether a Flux configuration is safe for production, whether SOPS encryption is required, whether prune is safe on a given workload, whether commit signature verification is enabled, or whether a Flux multi-tenant setup uses least-privilege ServiceAccounts.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: delivery
---

# FluxCD Kustomization and HelmRelease Review

## Purpose

Review FluxCD `Kustomization`, `HelmRelease`, `GitRepository`, `HelmRepository`, and `OCIRepository` resources for source trust guarantees, SOPS secret encryption, prune-enabled blast radius on stateful workloads, per-Kustomization ServiceAccount scoping, HelmRelease upgrade remediation safety, and health check completeness. FluxCD's default posture gives the `kustomize-controller` cluster-admin-equivalent reach — the security surface lives in per-Kustomization ServiceAccounts, commit signature verification, SOPS encryption at rest, and prune annotation guards.

## Lean operating rules

- Prefer user-provided sanitized resource YAML as primary evidence; official FluxCD docs are the authoritative fallback.
- Treat unencrypted Kubernetes `Secret` manifests committed to any Git source as a CRITICAL finding — anyone with repo read access (CI, PR participants, auditors) has those secrets.
- Treat `GitRepository.spec.ref.semver: ">=0.0.0"` or an unbound semver range in a production source as a HIGH finding — any tag push from a compromised upstream triggers a deploy.
- Treat the absence of `spec.verify.secretRef` (commit GPG signature verification) on production `GitRepository` sources as a HIGH finding.
- Treat `Kustomization.spec.serviceAccountName` not set as a HIGH finding — the kustomize-controller SA applies with cluster-admin-equivalent scope for all tenants.
- Treat `spec.prune: true` on Kustomizations covering stateful workloads (StatefulSets, PVCs, CRDs) without `kustomize.toolkit.fluxcd.io/prune: disabled` annotations as a HIGH finding.
- Treat `HelmRelease.spec.chart.spec.version: "*"` or an unbound version range as a HIGH finding — any upstream chart publish triggers an auto-upgrade.
- Treat `HelmRelease.spec.upgrade.remediation.retries: -1` (infinite retry) as a MEDIUM finding — a broken release blocks other reconciliation loops indefinitely.
- Keep the answer scoped: report what was reviewed, the evidence level, and the exact field path for each finding.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

- Scoped target (resource kind/name/namespace) and evidence level
- Source trust verdict (commit verification, semver pinning, SOPS encryption)
- Kustomization ServiceAccount scope assessment
- Prune safety verdict for any stateful workloads
- HelmRelease version pinning and upgrade remediation assessment
- Health check completeness verdict
- Safe next actions and open questions
