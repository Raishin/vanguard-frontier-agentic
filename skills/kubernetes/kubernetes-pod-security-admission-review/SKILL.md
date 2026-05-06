---
name: kubernetes-pod-security-admission-review
description: Use this skill for Kubernetes Pod Security Admission (PSA) review covering namespace labels for the three profiles (privileged, baseline, restricted), enforce/audit/warn modes, version pinning, and the migration path from deprecated PodSecurityPolicy. Trigger when the user asks whether a namespace label flip is safe, whether a workload meets a stricter profile, whether the audit/warn modes should be promoted to enforce, or whether an exemption is justified.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# Kubernetes Pod Security Admission Review

## Purpose

Review the Kubernetes Pod Security Admission posture: namespace labels for `pod-security.kubernetes.io/enforce`, `audit`, and `warn`, the chosen profile (`privileged`, `baseline`, `restricted`), version pinning, and exemptions. PSA replaced the deprecated PodSecurityPolicy in Kubernetes 1.25. It is the foundation for any admission-time security story — Kyverno, OPA Gatekeeper, and other policy engines layer on top of (or alongside) PSA, not as replacements.

## Lean operating rules

- Prefer live cluster evidence (`kubectl get namespaces --show-labels` plus `kubectl get pods -n <ns> -o yaml`) when the active client exposes it; otherwise fall back to official Kubernetes documentation and sanitized YAML.
- Separate confirmed facts from inference. If namespace labels, cluster admission configuration, or running pod security context state was not queried, say so.
- Treat **a production namespace with `enforce: privileged`** as a critical finding — the most permissive profile is enabled in a tier where nothing should be running with host access, privilege escalation, or capabilities.
- Treat **a production namespace with no PSA label at all** as a critical finding — the cluster default applies, which is `privileged` unless the cluster admin set a different default in `AdmissionConfiguration`.
- Challenge namespaces with `audit`/`warn` set but `enforce` missing — security violations are only logged, not blocked.
- Challenge `enforce-version: latest` — every Kubernetes upgrade can change profile semantics; pin to a specific minor.
- Challenge `kube-system` and operator namespaces excluded from PSA without documentation of which workloads require privileged access.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.

## References

Load these only when needed:

- [Evidence path and tooling](references/mcp-and-evidence.md) — use when choosing live evidence, confirming cluster admission configuration, or switching to documentation mode.
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, applying profile-by-profile stress checks, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when you need the detailed Kubernetes documentation list and grounded insights.

## Response minimum

Return, at minimum:

- the scoped target (specific namespace, set of namespaces, or cluster default) and evidence level,
- the active profile (`privileged` / `baseline` / `restricted`) and active mode (`enforce` / `audit` / `warn`),
- whether currently-running pods would still admit at the proposed profile,
- the exemption posture (cluster `AdmissionConfiguration` exemptions, namespace label override),
- the safest next actions and rollback plan,
- the assumptions or blockers that prevent stronger conclusions.
