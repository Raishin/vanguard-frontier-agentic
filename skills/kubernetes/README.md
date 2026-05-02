# ☸️ Kubernetes Skills

<p align="center">
  <!-- 🖼️ Add a Kubernetes logo to assets/logos/cloud/kubernetes/ and update this path -->
  <span style="font-size:3.5em">☸️</span>
</p>

This folder contains Kubernetes-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **2** local Kubernetes skills:

- `kubernetes-rbac-review`
- `kubernetes-live-rbac-mutation-guard`

## Portfolio posture

Kubernetes skills for evidence-backed RBAC review and guarded live cluster mutation.

These skills are intentionally conservative:

- prefer `kubectl auth can-i` and `kubectl get ... -o yaml` for live state grounding before any review or mutation
- capture the full current RBAC object state before every write — RBAC is additive with no built-in undo
- treat `escalate`, `bind`, and `impersonate` verbs as hard stops requiring platform-team sign-off
- never approve wildcard verb/resource grants (`verbs: ["*"]` or `resources: ["*"]`) without CISO-level justification
- always assess cluster-scope vs namespace-scope necessity — prefer a Role over a ClusterRole when namespace scope is sufficient
- use official Kubernetes documentation for RBAC behavior and policy

Run `npm run validate` after changing cataloged Kubernetes skills.
