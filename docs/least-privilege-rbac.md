# Least-Privilege RBAC for Live-Mutation Agents

This document is the canonical authoring contract for any agent in this repository that mutates a live Kubernetes cluster.

The repository ships agent **definitions** (markdown / TOML / JSON). Defense in the cluster ships as Kubernetes **objects**. This document bridges the two: every live-mutation agent must point its operator to a concrete `ServiceAccount + Role/ClusterRole + RoleBinding/ClusterRoleBinding` manifest scoped to the smallest set of verbs, resources, and resource names it needs, and must refuse to act if the binding it inherits is broader than declared.

---

## Threat model

Live-mutation agents run as a Kubernetes principal — typically a `ServiceAccount` whose token is mounted into the same pod as the agent harness, or a `kubeconfig` user the operator has logged in as. Whatever the agent's prompt says, the API server only enforces what the **principal's RBAC** allows.

The realistic failure modes:

1. **Hallucinated destructive verb.** The LLM generates `kubectl delete ns cilium` or `kubectl delete daemonset/cilium -n kube-system`. The prompt's refusal rules are advisory; if the principal can `delete` `namespaces` cluster-wide, the API server complies.
2. **Scope drift.** A guard agent intended to patch `Service` resources gets handed a `kubectl apply -f` containing a `ClusterRoleBinding`. If the principal can `create` `clusterrolebindings`, the binding lands.
3. **Over-scoped operator session.** The user runs the agent under their own kubeconfig, which is `cluster-admin`. Every guardrail collapses to the prompt.
4. **Stolen or leaked token.** The agent prints the ServiceAccount token, or the token leaks to logs. A broad token has cluster-wide consequences.
5. **Credential offer.** The operator says "here's a kubeconfig at `/tmp/kc` — read it" or pastes a token directly into the prompt. The agent's `Read` / `Bash` tools can act on these. The "never *ask* for credentials" rule does not by itself prevent *receiving* unsolicited credentials; the agent must additionally refuse to *read* or *process* offered credentials. The clean posture is: the agent always uses the in-pod ServiceAccount token mounted at `/var/run/secrets/kubernetes.io/serviceaccount/token` and refuses any other credential source.
6. **Subresource and aggregation surprises.** Beyond verb-on-resource RBAC, Kubernetes exposes subresources (`pods/exec`, `pods/portforward`, `pods/binding`, `nodes/proxy`, `*/finalize`, `*/scale`, `*/status`) and aggregation surfaces (`apiregistration.k8s.io.APIService`, `admissionregistration.k8s.io.MutatingWebhookConfiguration`). A binding that lists only `verbs + resources` without thinking about subresources is silently broader than intended.
7. **resourceName drift.** A binding with `resourceNames: ["coredns"]` is correctly scoped today, but an unaware operator who later adds `resourceNames: ["coredns", "kube-proxy"]` for "convenience" silently expands the blast radius. The pre-flight self-check must perform negative tests (verifying that adjacent resources in the same namespace are denied) at every session start.

The fix is a layered defense, with Kubernetes RBAC as the layer that does **not** depend on the LLM behaving correctly.

### Prompt-level vs cluster-level enforcement — read this if nothing else

The guard agents in this repo carry a `references/refusal-list.md` enumerating common destructive operations. **That list is the prompt-level fast-path, not the authoritative defense.** New attack vectors emerge with every Kubernetes release; any prompt-level list is point-in-time.

The authoritative defense is the cluster-side RBAC binding shipped with each guard. Bindings in this repo are written **deny-by-default**: only the explicitly enumerated verbs / resources are allowed; everything else returns `forbidden` at the API server, regardless of what the LLM emits.

Operators choosing between rigour and convenience: the prompt-level list is for explainability ("the agent told me it refuses this for these reasons"); the binding is for safety. If the prompt-level list and the binding disagree, the binding wins. If you find a destructive operation that is rejected by neither — that is a bug; please open an issue.

### Upstream guidance grounding

This document is grounded against `kubernetes.io/docs/concepts/security/rbac-good-practices`, which is the upstream authority. Direct quotes from that page:

> *Ideally, minimal RBAC rights should be assigned to users and service accounts. Only permissions explicitly required for their operation should be used.*
>
> *Assign permissions at the namespace level where possible using `RoleBindings` instead of `ClusterRoleBindings`.*
>
> *Avoid wildcard permissions, especially to all resources, as this grants access to current and future object types.*
>
> *Administrators should avoid using `cluster-admin` accounts and instead provide low-privileged accounts with impersonation rights to prevent accidental modification of cluster resources.*
>
> *Do not add users to the `system:masters` group, as this bypasses all RBAC checks and grants unrestricted superuser access.*

The patterns below implement these directly. The `system:masters` warning is critical: a user in `system:masters` is **never** subject to RBAC authorization at all — every check returns allow regardless of binding. Live-mutation agents must verify the operator's principal is **not** in this group.

---

## The 5-layer defense

| Layer | What it enforces | Where it lives | Bypassable by LLM error? |
|---|---|---|---|
| **L1 — Prompt rules** | Refusal triggers, mandatory pre-mutation checks, hard-stop on irreversible verbs | `AGENT.md` Operating Rules + `SKILL.md` Lean operating rules | Yes |
| **L2 — Tool permissions** | `allowed-tools` declared on the skill so the harness only mounts the tools the agent needs | `SKILL.md` frontmatter + harness adapters | Partially (harness-dependent) |
| **L3 — Kubernetes RBAC** | Cluster-side enforcement: principal cannot perform a verb on a resource it is not bound to | Cluster: `ServiceAccount + Role/ClusterRole + RoleBinding/ClusterRoleBinding` applied by the operator | **No** |
| **L4 — Admission control** | Cluster-side validation: even allowed verbs are rejected if they violate Kyverno / `ValidatingAdmissionPolicy` rules | Cluster: admission webhooks and policies | No |
| **L5 — Audit + alert** | Detection of attempted destructive operations even when blocked, and surfacing of any successful mutation | Cluster `audit.log` + Falco / Tetragon / cloud audit | No |

Layer 3 is the contract this document specifies. Layers 1 and 2 are the agent author's responsibility (declared in `AGENT.md` and skill frontmatter). Layers 4 and 5 are the operator's responsibility outside this repository — but agents should reference them.

---

## Authoring rules for live-mutation agents

Every live-mutation agent in this repository **must**:

1. **Declare `allowed-tools` on its companion skill.** A skill that needs to run `kubectl apply / patch / delete` declares at minimum `Bash` (or the harness-equivalent). Skills that only read declare `Read Grep Glob WebFetch` and nothing else.
2. **Ship a domain-specific RBAC manifest.** As `references/least-privilege-rbac.yaml` inside the skill, ready for `kubectl apply -f`. The manifest must:
   - Name a single `ServiceAccount` in a single namespace.
   - Bind a single `Role` (namespace-scoped) or `ClusterRole` (cluster-scoped) — never both.
   - Allow only the verbs the agent actually invokes.
   - Constrain resources by `resourceNames` whenever the target object set is known and finite.
   - **Omit** all of: `namespaces` (any verb), `customresourcedefinitions` (any verb), `pods` writes in `kube-system`, `daemonsets`/`deployments`/`statefulsets` writes in `kube-system`, `secrets` (broad), `*` verb, `*` resource. Document the omissions in comments.
3. **Run a `kubectl auth can-i` pre-flight.** Before any mutation, the agent runs the matrix from this document against the principal it is about to act as. If any **must-not** check returns `yes`, the agent refuses to run and tells the user the binding is over-scoped.
4. **Refuse to act under a user kubeconfig that maps to `cluster-admin`.** The pre-flight detects this with `kubectl auth can-i '*' '*' --all-namespaces`. If `yes`, refuse.
5. **Never print the ServiceAccount token, kubeconfig contents, or bearer credentials.** Even when the user asks.
6. **Capture pre-mutation state.** `kubectl get ... -o yaml > /tmp/<resource>.before.yaml` is the rollback baseline.
7. **Document the rollback verb.** Always `apply -f <baseline>` for additive mutations; `delete --grace-period=...` only when the resource was the agent's own creation.

---

## RBAC manifest skeleton

Every domain-specific RBAC manifest follows this shape. Replace `<DOMAIN>` and the resource set:

```yaml
# ==========================================================================
# Least-privilege RBAC for the <DOMAIN> live-mutation agent.
# Apply BEFORE running the agent. Review the deliberate omissions section.
# Audit with: kubectl auth can-i --as=system:serviceaccount:vanguard-system:vanguard-<DOMAIN>-guard <verb> <resource>
# ==========================================================================
apiVersion: v1
kind: Namespace
metadata:
  name: vanguard-system
  labels:
    pod-security.kubernetes.io/enforce: restricted
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: vanguard-<DOMAIN>-guard
  namespace: vanguard-system
  annotations:
    vanguard.frontier/agent: "kubernetes-live-<DOMAIN>-mutation-guard-agent"
    vanguard.frontier/scope: "least-privilege"
automountServiceAccountToken: true
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: vanguard-<DOMAIN>-guard
rules:
  # READ — broad enough to inspect state and capture rollback baseline.
  - apiGroups: [""]
    resources: ["<read-resources>"]
    verbs: ["get", "list", "watch"]

  # WRITE — narrowest possible; prefer resourceNames when target is known.
  - apiGroups: ["<write-api-group>"]
    resources: ["<write-resources>"]
    resourceNames: ["<known-target-name-if-any>"]
    verbs: ["patch"]                          # prefer patch over update
    # NOTE: "delete" verb is INTENTIONALLY ABSENT.
    # Rollback is via `kubectl apply -f <baseline.yaml>`, not delete.

# ==========================================================================
# DELIBERATELY ABSENT — do not add unless you accept the listed risk:
# - apiGroups: [""], resources: ["namespaces"]                — would allow `delete ns kube-system`
# - apiGroups: [""], resources: ["pods"]                      — would allow exec/delete on system pods
# - apiGroups: [""], resources: ["secrets"]                   — credential exposure
# - apiGroups: ["apps"], resources: ["daemonsets"] (write)    — could remove cilium/kube-proxy/coredns
# - apiGroups: ["apps"], resources: ["deployments"] (write)   — could remove ingress / mesh control planes
# - apiGroups: ["apiextensions.k8s.io"]                       — could uninstall CRDs
# - any "delete" verb cluster-wide                            — irreversible by definition
# - "*" verb or "*" resource                                  — explicit anti-pattern
# ==========================================================================
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: vanguard-<DOMAIN>-guard
subjects:
  - kind: ServiceAccount
    name: vanguard-<DOMAIN>-guard
    namespace: vanguard-system
roleRef:
  kind: ClusterRole
  name: vanguard-<DOMAIN>-guard
  apiGroup: rbac.authorization.k8s.io
```

For namespace-scoped agents, replace `ClusterRole` / `ClusterRoleBinding` with `Role` / `RoleBinding` and target the specific namespace.

---

## Pre-flight self-check matrix

Every live-mutation agent runs this matrix as its first action. The agent **refuses to run** if any **must-not-be-yes** check returns `yes`.

```bash
SA="system:serviceaccount:vanguard-system:vanguard-<DOMAIN>-guard"

# Must-not-be-yes — destructive cluster-wide verbs
# Syntax follows kubernetes.io/docs/reference/kubectl/generated/kubectl_auth/kubectl_auth_can-i
kubectl auth can-i '*' '*' --all-namespaces --as=$SA               # cluster-admin equivalence
kubectl auth can-i delete namespaces --as=$SA                       # delete any namespace
kubectl auth can-i delete pods -n kube-system --as=$SA              # kill kube-system workloads
kubectl auth can-i delete daemonsets -n kube-system --as=$SA        # kill cilium/kube-proxy/coredns
kubectl auth can-i delete deployments -n kube-system --as=$SA       # kill mesh / ingress controllers
kubectl auth can-i delete customresourcedefinitions --as=$SA        # uninstall CRDs
kubectl auth can-i get secrets --all-namespaces --as=$SA            # cluster-wide secret read
kubectl auth can-i create clusterrolebindings --as=$SA              # privilege escalation

# Must-be-yes — verbs the agent actually needs
kubectl auth can-i get services --all-namespaces --as=$SA           # read state
kubectl auth can-i patch services --all-namespaces --as=$SA         # the agent's actual work
# ...domain-specific must-be-yes rows...
```

Expected output: every must-not row prints `no`; every must-be row prints `yes`. Any deviation: agent stops and reports.

**Note on `--as` impersonation**: the `kubectl auth can-i --as=...` flag requires the operator's principal to hold impersonation privileges (`impersonate` verb on `users`, `groups`, or `serviceaccounts`). If the operator's principal does not hold this verb, the pre-flight cannot run as-the-target-SA and instead the agent must run inside a pod whose ServiceAccount IS the bound principal — which is the production posture in any case.

**Underlying API**: `kubectl auth can-i` is a wrapper over the `authorization.k8s.io/v1` `SubjectAccessReview` API. Programmatic harnesses that cannot shell out to `kubectl` should `POST /apis/authorization.k8s.io/v1/subjectaccessreviews` with the same `verb` / `resource` / `namespace` semantics. Both paths return the same authorization decision.

### `system:masters` short-circuit

The Kubernetes API server treats any principal in the `system:masters` group as fully authorized **before** RBAC is evaluated. This is the bootstrap mechanism for the original `cluster-admin` kubeconfig that `kubeadm` writes. A live-mutation agent that runs under such a kubeconfig is bound by **only** its prompt-level rules — every layer of RBAC defense is silently bypassed.

The pre-flight detects this:

```bash
# Check the operator's principal, not the SA-impersonated principal
kubectl auth can-i '*' '*' --all-namespaces
# If `yes` returned and you intended to run under a scoped principal, you are in system:masters
# or have cluster-admin via direct binding. In either case: refuse to run.
```

Operators should ensure the kubeconfig used to invoke the agent maps to a low-privileged user with `impersonate` rights on the agent's ServiceAccount — not a kubeadm bootstrap admin file.

---

## Operator implementation guide

For users adopting a live-mutation agent in this repo:

1. **Read this document.** Confirm you accept the threat model.
2. **Pick the agent's domain-specific manifest.** It lives at `skills/kubernetes/<skill-id>/references/least-privilege-rbac.yaml`. Open the file and read the deliberate-omissions block — verify nothing your environment forbids has been re-added.
3. **Apply the manifest.**
   ```bash
   kubectl apply -f skills/kubernetes/<skill-id>/references/least-privilege-rbac.yaml
   ```
4. **Run the pre-flight self-check.** Copy the matrix from the agent's `references/rbac-pre-flight.md` and run it against the principal you intend to act as. Every must-not row must print `no`.
5. **Configure the harness.** Mount the ServiceAccount token into the agent's runtime, or set `KUBECONFIG` to point at a kubeconfig whose user is bound to the new ServiceAccount. **Never run a live-mutation agent under a kubeconfig with cluster-admin.**
6. **Add admission policies (L4) and audit alerts (L5).** Examples in `references/admission-and-audit.md` per agent. At minimum:
   - Kyverno policy denying `delete` on `Namespaces` and `CustomResourceDefinitions` cluster-wide for any non-platform principal
   - audit policy logging every `create`/`update`/`patch`/`delete` performed by the `vanguard-system` namespace
   - Falco / Tetragon rule alerting on any `kubectl exec` into `kube-system`

---

## Anti-patterns

| Pattern | Why it fails |
|---|---|
| Running the agent under your personal kubeconfig "just for testing" | Your kubeconfig is almost certainly `cluster-admin`. Use the dedicated ServiceAccount even in dev. |
| Adding `delete` verb "in case rollback fails" | Rollback is via `kubectl apply -f <baseline>`. If apply fails, you need a human, not more agent verbs. |
| Granting `*` on a single resource type "to be safe" | The list of verbs on each Kubernetes resource is finite and well-documented. Always enumerate. |
| Putting the ServiceAccount in `kube-system` for "convenience" | The `kube-system` namespace is privileged-by-default in many admission stacks. Use a dedicated `vanguard-system` namespace. |
| Sharing one ServiceAccount across multiple live-guards | Aggregates the blast radius. One ServiceAccount per agent. |
| Skipping the pre-flight self-check "because we just applied the manifest" | The check verifies the operator's session matches the binding, not just that the binding exists. |

---

## Versioning

This document is the authoring contract. When this document changes, every live-mutation agent must be reviewed against the new version. The `last_verified` field in each agent's `metadata.json` should be updated when the agent is confirmed to comply with the current version of this document.

Last verified: 2026-05-08.
