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
kubectl auth can-i impersonate groups --as=$SA
kubectl auth can-i impersonate serviceaccounts --as=$SA

# Node lifecycle (refusal-list.md "Node operations")
kubectl auth can-i delete nodes --as=$SA
kubectl auth can-i patch nodes --as=$SA
kubectl auth can-i update nodes --as=$SA
kubectl auth can-i create pods/eviction --as=$SA
kubectl auth can-i get nodes/proxy --as=$SA
kubectl auth can-i create nodes/proxy --as=$SA

# Lease objects (refusal-list.md "Lease objects in kube-node-lease")
kubectl auth can-i patch leases.coordination.k8s.io -n kube-node-lease --as=$SA
kubectl auth can-i delete leases.coordination.k8s.io -n kube-node-lease --as=$SA

# Admission webhook configs (refusal-list.md "Admission webhook configurations")
kubectl auth can-i create mutatingwebhookconfigurations.admissionregistration.k8s.io --as=$SA
kubectl auth can-i patch mutatingwebhookconfigurations.admissionregistration.k8s.io --as=$SA
kubectl auth can-i delete mutatingwebhookconfigurations.admissionregistration.k8s.io --as=$SA
kubectl auth can-i create validatingwebhookconfigurations.admissionregistration.k8s.io --as=$SA
kubectl auth can-i patch validatingwebhookconfigurations.admissionregistration.k8s.io --as=$SA
kubectl auth can-i delete validatingwebhookconfigurations.admissionregistration.k8s.io --as=$SA

# APIService aggregation (refusal-list.md "APIService aggregation")
kubectl auth can-i create apiservices.apiregistration.k8s.io --as=$SA
kubectl auth can-i patch apiservices.apiregistration.k8s.io --as=$SA
kubectl auth can-i delete apiservices.apiregistration.k8s.io --as=$SA

# Pod and node subresources (refusal-list.md "Pod and node subresources")
kubectl auth can-i create pods/exec -n kube-system --as=$SA
kubectl auth can-i create pods/portforward --all-namespaces --as=$SA
kubectl auth can-i create pods/proxy --all-namespaces --as=$SA
kubectl auth can-i create pods/binding --all-namespaces --as=$SA

# CSR approval and TokenRequest minting (refusal-list.md "CSR approval and TokenRequest minting")
kubectl auth can-i update certificatesigningrequests.certificates.k8s.io --subresource=approval --as=$SA
kubectl auth can-i create certificatesigningrequests.certificates.k8s.io --as=$SA
kubectl auth can-i create serviceaccounts/token --all-namespaces --as=$SA

# Manual Endpoints / EndpointSlices writes (refusal-list.md "Manual Endpoints / EndpointSlices writes")
kubectl auth can-i create endpoints --all-namespaces --as=$SA
kubectl auth can-i patch endpoints --all-namespaces --as=$SA
kubectl auth can-i create endpointslices.discovery.k8s.io --all-namespaces --as=$SA
kubectl auth can-i patch endpointslices.discovery.k8s.io --all-namespaces --as=$SA

# kube-system ConfigMap writes outside the resourceName-locked allowlist
# (refusal-list.md "kube-system ConfigMap writes outside the resourceName-locked allowlist")
kubectl auth can-i patch configmaps/cilium-config -n kube-system --as=$SA           # MUST be no
kubectl auth can-i patch configmaps/kube-proxy -n kube-system --as=$SA              # MUST be no
kubectl auth can-i patch configmaps/kubelet-config -n kube-system --as=$SA          # MUST be no
kubectl auth can-i patch configmaps/cluster-info -n kube-public --as=$SA            # MUST be no

# PriorityClass / IngressClass / StorageClass (refusal-list.md "PriorityClass and IngressClass")
kubectl auth can-i delete priorityclasses.scheduling.k8s.io --as=$SA
kubectl auth can-i patch priorityclasses.scheduling.k8s.io --as=$SA
kubectl auth can-i delete ingressclasses.networking.k8s.io --as=$SA
kubectl auth can-i patch ingressclasses.networking.k8s.io --as=$SA
kubectl auth can-i patch storageclasses.storage.k8s.io --as=$SA

# Finalizer-stripping path (every resource exposes metadata.finalizers via patch;
# admission policy is the cleanest enforcement, but verify the binding does not
# grant patch on `namespaces` or `customresourcedefinitions` finalizers subresource):
kubectl auth can-i update namespaces/finalize --as=$SA
kubectl auth can-i update customresourcedefinitions/finalize --as=$SA
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

## resourceName-scoped binding verification (positive AND negative)

`kubectl auth can-i` does **not** by default surface `resourceNames` constraints. A binding that grants `patch configmaps` only on `resourceNames: ["coredns"]` may return ambiguous results when checked with the resource type alone. Always test BOTH the allowed and the denied resource name.

```bash
SA="system:serviceaccount:vanguard-system:vanguard-network-arch-guard"

# Positive — the bound resourceName MUST return yes
kubectl auth can-i patch configmaps/coredns -n kube-system --as=$SA              # expect: yes

# Negative — adjacent ConfigMaps in the same namespace MUST return no
kubectl auth can-i patch configmaps/cilium-config -n kube-system --as=$SA        # expect: no
kubectl auth can-i patch configmaps/kube-proxy -n kube-system --as=$SA           # expect: no
kubectl auth can-i patch configmaps/extension-apiserver-authentication -n kube-system --as=$SA  # expect: no

# Negative — same resourceName in a different namespace MUST return no
kubectl auth can-i patch configmaps/coredns -n default --as=$SA                  # expect: no
kubectl auth can-i patch configmaps/coredns -n kube-public --as=$SA              # expect: no
```

If any negative-test row returns `yes`, the binding is over-scoped — typically because `resourceNames` was omitted from the `ClusterRole` rule. Reapply the manifest from `references/least-privilege-rbac.yaml` and re-test.

This pattern generalises: any verb the agent is granted on a resourceName-locked resource MUST be tested with at least one positive (allowed name) and two negatives (one different name in the same namespace, one same name in a different namespace).

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
