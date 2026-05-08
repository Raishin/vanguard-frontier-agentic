# RBAC pre-flight self-check

This is the mandatory first action of every session. The agent runs this matrix before reading any user-supplied YAML, before formulating any mutation, before producing any output other than the matrix result.

The matrix is grounded against `kubernetes.io/docs/concepts/security/rbac-good-practices` and `kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/kubectl_auth_can-i`. The canonical authoring contract is `docs/least-privilege-rbac.md`.

If any **must-not-be-yes** check returns `yes`, or any **must-be-yes** check returns `no`, the agent refuses to act and tells the user the binding is over- or under-scoped.

---

## Required RBAC manifest

Apply `references/least-privilege-rbac.yaml` (shipped with this skill) before invoking the agent. The manifest creates `ServiceAccount/vanguard-network-arch-guard` in namespace `vanguard-system`, a `ClusterRole` with the verbs documented in `permitted-mutations.md`, and a `ClusterRoleBinding`.

Per upstream `kubernetes.io/docs/concepts/security/rbac-good-practices`:

> *Avoid wildcard permissions, especially to all resources, as this grants access to current and future object types.*

The manifest enumerates each resource and verb. Review the deliberately-omitted block before applying.

---

## Operator principal check (run first)

This checks the operator's own kubeconfig — the principal whose `--as` we are about to use:

```bash
# If yes: operator is in system:masters or cluster-admin. Refuse.
kubectl auth can-i '*' '*' --all-namespaces
```

If this returns `yes`, the operator must switch to a kubeconfig with `impersonate` rights only and re-invoke. Per upstream RBAC good practices:

> *Administrators should avoid using `cluster-admin` accounts and instead provide low-privileged accounts with impersonation rights to prevent accidental modification of cluster resources.*

> *Do not add users to the `system:masters` group, as this bypasses all RBAC checks.*

---

## Agent ServiceAccount must-not-be-yes matrix

Run with `--as=system:serviceaccount:vanguard-system:vanguard-network-arch-guard`:

```bash
SA="system:serviceaccount:vanguard-system:vanguard-network-arch-guard"

# Cluster-admin equivalence
kubectl auth can-i '*' '*' --all-namespaces --as=$SA

# Namespace destruction
kubectl auth can-i delete namespaces --as=$SA
kubectl auth can-i delete namespaces/kube-system --as=$SA
kubectl auth can-i delete namespaces/cilium --as=$SA
kubectl auth can-i delete namespaces/istio-system --as=$SA

# kube-system control plane destruction
kubectl auth can-i delete daemonsets -n kube-system --as=$SA
kubectl auth can-i delete deployments -n kube-system --as=$SA
kubectl auth can-i patch daemonsets/cilium -n kube-system --as=$SA
kubectl auth can-i patch daemonsets/kube-proxy -n kube-system --as=$SA

# Pod execution / mutation in kube-system
kubectl auth can-i delete pods -n kube-system --as=$SA
kubectl auth can-i create pods/exec -n kube-system --as=$SA

# CRD operations
kubectl auth can-i create customresourcedefinitions --as=$SA
kubectl auth can-i delete customresourcedefinitions --as=$SA

# Broad secret access
kubectl auth can-i get secrets --all-namespaces --as=$SA
kubectl auth can-i list secrets --all-namespaces --as=$SA

# Privilege escalation
kubectl auth can-i create clusterrolebindings --as=$SA
kubectl auth can-i create clusterroles --as=$SA
kubectl auth can-i escalate roles --as=$SA
kubectl auth can-i bind roles --as=$SA
kubectl auth can-i impersonate users --as=$SA

# kube-controller-manager flag surface (not RBAC-able directly, but documented for transparency)
# Pod CIDR / Service CIDR resize is enforced by the cluster's RBAC + admission stack;
# this guard's binding never touches kube-system kube-controller-manager.
```

Every line above must print `no`. Any `yes` means the binding is over-scoped — refuse to run and tell the operator which line failed.

---

## Agent ServiceAccount must-be-yes matrix

```bash
SA="system:serviceaccount:vanguard-system:vanguard-network-arch-guard"

# Read state across the architecture surface
kubectl auth can-i get services --all-namespaces --as=$SA
kubectl auth can-i list services --all-namespaces --as=$SA
kubectl auth can-i get endpointslices --all-namespaces --as=$SA
kubectl auth can-i get nodes --as=$SA
kubectl auth can-i get configmaps -n kube-system --as=$SA

# Service spec patches (the agent's actual mutation surface)
kubectl auth can-i patch services --all-namespaces --as=$SA

# CoreDNS Corefile (resourceName-locked in the manifest)
kubectl auth can-i patch configmaps/coredns -n kube-system --as=$SA
kubectl auth can-i get configmaps/coredns -n kube-system --as=$SA

# Gateway API resources (write OK — Gateway API resources are user-owned, not control plane)
kubectl auth can-i create gateways.gateway.networking.k8s.io --all-namespaces --as=$SA
kubectl auth can-i patch gateways.gateway.networking.k8s.io --all-namespaces --as=$SA
kubectl auth can-i create httproutes.gateway.networking.k8s.io --all-namespaces --as=$SA
kubectl auth can-i create grpcroutes.gateway.networking.k8s.io --all-namespaces --as=$SA
kubectl auth can-i create referencegrants.gateway.networking.k8s.io --all-namespaces --as=$SA
```

Every line above must print `yes`.

---

## Programmatic alternative — SubjectAccessReview API

Harnesses that cannot shell out to `kubectl` should call the `authorization.k8s.io/v1` `SubjectAccessReview` API directly:

```bash
curl -s -X POST $KUBE_API_SERVER/apis/authorization.k8s.io/v1/subjectaccessreviews \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "apiVersion": "authorization.k8s.io/v1",
    "kind": "SubjectAccessReview",
    "spec": {
      "user": "system:serviceaccount:vanguard-system:vanguard-network-arch-guard",
      "resourceAttributes": {
        "verb": "delete",
        "resource": "namespaces"
      }
    }
  }'
```

Parse `.status.allowed` from the response. Same semantics as `kubectl auth can-i`.

---

## What the agent does with the matrix output

If every must-not row is `no` and every must-be row is `yes`:

```
Pre-flight: PASS
Bound principal: system:serviceaccount:vanguard-system:vanguard-network-arch-guard
RBAC posture: scoped per docs/least-privilege-rbac.md
Proceeding to next step.
```

If any row fails:

```
Pre-flight: FAIL
Failing check: <verb> <resource> <namespace>
Expected: no | Actual: yes (this verb is over-scoped on the bound ServiceAccount)
Action: refusing to proceed. Re-apply the manifest from references/least-privilege-rbac.yaml or scope down the existing binding before re-invoking.
```

No exceptions. No retries. The pre-flight is the gate.
