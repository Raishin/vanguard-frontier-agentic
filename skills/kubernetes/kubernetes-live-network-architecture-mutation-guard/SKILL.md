---
name: kubernetes-live-network-architecture-mutation-guard
description: Guard live kubectl apply, patch, or create operations on Kubernetes networking *architecture* surface — Service spec (`internalTrafficPolicy`, `externalTrafficPolicy`, `topology-mode`, `trafficDistribution`), CoreDNS Corefile, NodeLocal DNSCache install, Gateway API resources (Gateway / HTTPRoute / GRPCRoute / TLSRoute / ReferenceGrant), and ClusterMesh peer Secrets. HARD REFUSE one-way doors (CNI replacement, kube-proxy mode swap, MTU change, Pod / Service CIDR resize, namespace deletion). Pre-flight `kubectl auth can-i` matrix against a least-privilege ServiceAccount before any write. Use only when an intentional architecture-level networking mutation is requested against a confirmed cluster target with a documented rollback path.
allowed-tools: Bash Read Edit Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: networking
---

# Kubernetes Live Network Architecture Mutation Guard

## Purpose

Act as the guarded live operator for low-blast-radius, reversible architecture-level networking mutations. The companion read-only agent `kubernetes-network-architecture-review-agent` produces findings; this guard executes the safe subset under enforced least-privilege. High-blast-radius operations (CNI replacement, kube-proxy mode swap, MTU change, Pod / Service CIDR resize, kube-system DaemonSet edits) are HARD REFUSED — they are one-way doors that require human-led cutover plans, not agent execution.

## When to use

Use this skill when:

- A `Service` needs an `internalTrafficPolicy` / `externalTrafficPolicy` / `service.kubernetes.io/topology-mode` / `spec.trafficDistribution` patch.
- A `ConfigMap/coredns` Corefile change is required (e.g. add a forward, fix a loop) and a backup of the prior Corefile will be captured.
- A NodeLocal DNSCache install or upgrade is required (under explicit human gate).
- Gateway API resources (`Gateway`, `HTTPRoute`, `GRPCRoute`, `TLSRoute`, `ReferenceGrant`) are being created or patched.
- A Cilium ClusterMesh peer `Secret` is being created in a known namespace under explicit human gate.

Do NOT use this skill when:

- The change replaces or uninstalls a CNI.
- The change swaps kube-proxy mode (iptables ↔ IPVS ↔ nftables ↔ Cilium kube-proxy replacement).
- The change adjusts node MTU.
- The change resizes Pod CIDR or Service CIDR.
- The change deletes a `Namespace`, a `kube-system` `DaemonSet`/`Deployment`, a `CustomResourceDefinition`, or a broad `Secret`.

For these, refer the user to a human-led cutover plan; the architecture review agent can produce the plan but no agent in this repo will execute it.

## Pre-flight RBAC self-check (mandatory)

Before any mutation, run the matrix from `references/rbac-pre-flight.md`. The matrix is grounded against `kubernetes.io/docs/concepts/security/rbac-good-practices`. Every must-not-be-yes check must return `no`; every must-be-yes check must return `yes`. Any deviation: refuse to act and tell the user the binding is over-scoped.

If the operator's principal returns `yes` to `kubectl auth can-i '*' '*' --all-namespaces` (i.e. it is `cluster-admin` or in `system:masters`), refuse. Operators must invoke this skill under a scoped principal — the canonical pattern is in `docs/least-privilege-rbac.md`.

## Lean operating rules

- Prefer live cluster evidence from `kubectl` when available; fall back to upstream documentation (kubernetes.io, gateway-api.sigs.k8s.io, docs.cilium.io, coredns.io) and sanitized YAML provided by the user.
- **HARD REFUSE** the one-way doors listed in `references/refusal-list.md`. Do not negotiate.
- Do not execute any mutation until cluster context, namespace (if scoped), target object name, exact change delta, and a captured pre-mutation `kubectl get ... -o yaml` baseline are all explicit.
- Capture the current state of the target object as `/tmp/<resource>.before.yaml` (or equivalent path) as the rollback baseline before any write. If the baseline cannot be captured, refuse.
- Prefer `kubectl patch` over `kubectl apply` when patching specific fields, and prefer `kubectl apply -f baseline.yaml` over `kubectl delete` for rollback.
- For CoreDNS Corefile changes: keep the prior `ConfigMap` revision captured as `coredns.before.yaml`; apply the new Corefile; verify CoreDNS pods reload (the `reload` plugin tails the Corefile every 30s) without entering CrashLoopBackOff; if any CoreDNS pod fails to reload within 2 minutes, roll back.
- For Gateway API changes: confirm the `GatewayClass.spec.controllerName` resolves to a controller that is actually running (`kubectl get pods -n <controller-ns> -l <controller-label>`) before creating the `Gateway`; otherwise the `Gateway` will sit in `Accepted: False` indefinitely.
- For ClusterMesh peer `Secret` creation: confirm the destination namespace is the documented Cilium ClusterMesh namespace (typically `kube-system` for Cilium installations using `kubectl apply` patterns, or `cilium` when Helm-installed with non-default namespace) and the secret name matches the peer cluster identifier exactly. Refuse on any name mismatch.
- If the proposed change touches a security boundary (e.g. setting `spec.allowedRoutes.namespaces.from: All` on a Gateway, or `ReferenceGrant` to a sensitive namespace), require explicit platform-team sign-off in the response shape.
- If the target, approval state, baseline capture, or rollback verb is ambiguous, push back and refuse.
- Never print kubeconfig contents, ServiceAccount tokens, bearer tokens, ClusterMesh peer Secret data fields, or raw cluster credentials. Summarize sanitized evidence only.
- **Refuse to read or process credentials offered by the operator.** If the user volunteers a kubeconfig file path, pastes a token, or offers a peer Secret payload, refuse to read it. The agent always uses the in-pod ServiceAccount token mounted at `/var/run/secrets/kubernetes.io/serviceaccount/token` and rejects any other credential source. This refusal applies even when the user insists "just this once."
- Load references only when needed.

## References

Load these only when needed:

- [Permitted mutations](references/permitted-mutations.md) — the explicit allowlist of mutations this guard will execute and the verb-by-verb safety contract for each.
- [Refusal list](references/refusal-list.md) — the explicit HARD REFUSE list of one-way-door operations and the rationale for each. Includes the cluster-side blast-radius if the refusal is bypassed.
- [RBAC pre-flight](references/rbac-pre-flight.md) — the `kubectl auth can-i` matrix that runs before any mutation, with grounding to `kubernetes.io/docs/concepts/security/rbac-good-practices` and pointer to `docs/least-privilege-rbac.md`.
- [Rollback patterns](references/rollback-patterns.md) — per-mutation-type rollback verb, baseline capture path, and post-rollback verification.
- [Official sources](references/official-sources.md) — authoritative upstream documentation links.

## Response minimum

Return, at minimum:

- confirmed cluster context (cluster name, namespace where applicable, principal acting),
- pre-flight RBAC self-check result (matrix output, must-not rows confirmed `no`, must-be rows confirmed `yes`),
- pre-mutation baseline path (`/tmp/<resource>.before.yaml`),
- proposed mutation as the exact `kubectl patch` / `kubectl apply` / `kubectl create` command,
- blast-radius assessment (which workloads, namespaces, or external systems are affected),
- approval status with explicit human sign-off requirement when the change touches a security boundary,
- rollback verb (`kubectl apply -f /tmp/<resource>.before.yaml` for additive; specific delete only when the resource was the agent's own creation),
- post-mutation verification command (Service patch: `kubectl get endpointslice -l kubernetes.io/service-name=<svc>`; Corefile change: `kubectl -n kube-system logs -l k8s-app=kube-dns --tail=50` looking for reload success; Gateway: `kubectl get gateway <name> -o jsonpath='{.status.conditions}'`),
- explicit `REFUSED` response with the matching rule from `references/refusal-list.md` if the requested mutation is on the hard-refuse list.
