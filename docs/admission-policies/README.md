# Layer 4 Admission Policies

This directory ships the **Layer 4 — Admission Control** complement to the Layer 3 RBAC
manifests documented in [`docs/least-privilege-rbac.md`](../least-privilege-rbac.md).

---

## Why L4 adds essential coverage that L3 alone cannot provide

Layer 3 (Kubernetes RBAC) enforces **verb × resource-type** bindings at the API server.
It answers the question: *"Is this principal allowed to perform this verb on this resource kind?"*

What RBAC **cannot** express:

| Scenario | RBAC verdict | Reality |
|---|---|---|
| A principal is allowed `PATCH` on `Namespace`, but uses it to strip all finalizers | Allowed | Bypasses namespace deletion protection |
| A principal is allowed `UPDATE` on `ValidatingWebhookConfiguration`, but rewrites a webhook to silence all admission checks | Allowed | Undermines every other admission control |
| A principal is allowed `CREATE` on `pods/exec` cluster-wide, but then execs into `kube-system/coredns` | Allowed unless namespace-scoped binding | Direct path to cluster credential exfiltration |

Layer 4 admission policies inspect **field content** inside the request body, not just the
verb/resource pair. They answer: *"Even though this principal is allowed this verb, does the
specific mutation violate an invariant we must protect?"*

---

## Two complementary implementations

### Kyverno ClusterPolicies (`kyverno/`)

[Kyverno](https://kyverno.io) is a widely-deployed Kubernetes-native policy engine. Its
`ClusterPolicy` resources support rich JMESPath and CEL expressions, pattern matching, and
precondition guards. Kyverno runs as a validating/mutating webhook and is compatible with
Kubernetes 1.25+.

**Apply all Kyverno policies:**

```bash
kubectl apply -f docs/admission-policies/kyverno/
```

**Verify Kyverno is installed:**

```bash
kubectl get pods -n kyverno
kubectl get clusterpolicies
```

### Kubernetes-native ValidatingAdmissionPolicy (`vap/`)

[ValidatingAdmissionPolicy](https://kubernetes.io/docs/reference/access-authn-authz/validating-admission-policy/)
(VAP) is a Kubernetes-native admission mechanism using CEL expressions, GA in Kubernetes 1.30.
It requires no additional software. VAP policies consist of two resources:

- `ValidatingAdmissionPolicy` — the policy spec and CEL expressions
- `ValidatingAdmissionPolicyBinding` — binds the policy to a scope (namespace selector, resource, etc.)

**Apply all VAP policies:**

```bash
kubectl apply -f docs/admission-policies/vap/
```

**Verify VAP support:**

```bash
kubectl api-resources | grep validatingadmissionpolicy
# Should show: validatingadmissionpolicies and validatingadmissionpolicybindings
```

> **Minimum Kubernetes version:** 1.30 for VAP GA. For clusters running 1.28–1.29, use
> Kyverno policies instead. Kyverno provides identical coverage on older clusters.

---

## Policy inventory

### Kyverno

| File | Policy name | Severity | What it blocks |
|---|---|---|---|
| `kyverno/deny-namespace-delete.yaml` | `vanguard-deny-namespace-delete` | high | DELETE on Namespace by non-platform principals |
| `kyverno/deny-crd-delete.yaml` | `vanguard-deny-crd-delete` | high | DELETE on CRD by non-platform principals |
| `kyverno/deny-finalizer-strip.yaml` | `vanguard-deny-finalizer-strip` | critical | PATCH/UPDATE that removes all finalizers |
| `kyverno/deny-kube-system-exec.yaml` | `vanguard-deny-kube-system-exec` | high | exec/portforward/proxy into kube-system pods |
| `kyverno/deny-webhook-writes.yaml` | `vanguard-deny-webhook-writes` | critical | Writes to MutatingWebhookConfiguration / ValidatingWebhookConfiguration |
| `kyverno/deny-apiservice-writes.yaml` | `vanguard-deny-apiservice-writes` | critical | Writes to APIService (API aggregation) |

### VAP (Kubernetes-native, 1.30+)

| File | Policy name | What it blocks |
|---|---|---|
| `vap/deny-namespace-delete.yaml` | `vanguard-deny-namespace-delete` | DELETE on Namespace by non-platform principals |
| `vap/deny-crd-delete.yaml` | `vanguard-deny-crd-delete` | DELETE on CRD by non-platform principals |
| `vap/deny-webhook-writes.yaml` | `vanguard-deny-webhook-writes` | Writes to webhook configurations |

---

## Platform principal exclusion model

All policies use a consistent two-condition exclusion. A principal is treated as a
**platform principal** (and therefore excluded from the deny) if it belongs to **either**:

1. The `vanguard-platform` Kubernetes group, **or**
2. Any `ServiceAccount` in the `vanguard-system` namespace (username prefix
   `system:serviceaccount:vanguard-system:`)

`cluster-admin` is not explicitly excluded in policy expressions because admission
policies are not evaluated for requests authenticated as `system:masters`. The
cluster-admin ClusterRoleBinding binds subjects to `cluster-admin` which inherits
`system:masters` bypass behavior in some distributions. A comment in each policy
clarifies this.

---

## Deployment order

Apply L3 RBAC first, then L4 admission policies:

```bash
# L3: RBAC (already shipped per-agent)
kubectl apply -f agents/kubernetes/kubernetes-live-network-policy-guard-agent/references/least-privilege-rbac.yaml
# ... (repeat for each live-guard agent)

# L4: Install Kyverno (if not already present)
kubectl create -f https://github.com/kyverno/kyverno/releases/latest/download/install.yaml

# L4: Apply policies
kubectl apply -f docs/admission-policies/kyverno/

# L4 (alt): VAP on K8s 1.30+
kubectl apply -f docs/admission-policies/vap/
```

---

## Testing policies

After applying, verify each policy blocks what it should:

```bash
# Test namespace delete is denied for a non-platform user
kubectl delete namespace default \
  --as=test-user \
  --as-group=test-group
# Expected: Error from server: admission webhook denied the request

# Test CRD delete is denied
kubectl delete crd somecrd.example.com \
  --as=test-user
# Expected: denied

# Test kube-system exec is denied
kubectl exec -n kube-system deploy/coredns -- /bin/sh \
  --as=test-user
# Expected: denied
```

---

## References

- L3 RBAC contract: [`docs/least-privilege-rbac.md`](../least-privilege-rbac.md)
- 5-layer defense model: [`docs/least-privilege-rbac.md#the-5-layer-defense`](../least-privilege-rbac.md#the-5-layer-defense)
- Kyverno documentation: <https://kyverno.io/docs/>
- Kubernetes VAP: <https://kubernetes.io/docs/reference/access-authn-authz/validating-admission-policy/>
- RBAC good practices: <https://kubernetes.io/docs/concepts/security/rbac-good-practices/>
