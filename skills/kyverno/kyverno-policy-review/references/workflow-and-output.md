# Workflow and Output Contract

## Workflow

### Step 1 — Identify the policy and its scope

1. Confirm the policy kind: `ValidatingPolicy`, `MutatingPolicy`, `GeneratingPolicy`, `DeletingPolicy`, `ImageValidatingPolicy` (stable `policies.kyverno.io/v1`), or legacy `ClusterPolicy` / `Policy`.
2. Confirm the match scope: namespace-scoped (`Policy`) vs cluster-scoped (`ClusterPolicy` / new v1 kinds).
3. Confirm the API version. The stable `policies.kyverno.io/v1` API is the recommended target — see the [Kyverno policy types overview](https://kyverno.io/docs/policy-types/overview/).
4. Confirm match conditions in `spec.match` — kinds, names, namespaces, labels, annotations. Any `kinds: ['*']` with no further filter is high-blast-radius.

### Step 2 — Identify the failure mode

1. Locate `spec.rules[].validate.failureAction` (newer API) or `spec.validationFailureAction` (legacy).
2. Two values exist: `Enforce` (admission denied on violation) and `Audit` (admission allowed, violation recorded in PolicyReport).
3. **Critical finding**: any production-relevant policy with `failureAction: Audit` and no plan to migrate to `Enforce`. The policy is a logging shim, not a control.
4. Also confirm `spec.background` — when `false`, the policy only evaluates at admission time; existing resources are not scanned.
5. Reference: [Validate rules — failureAction semantics](https://kyverno.io/docs/policy-types/cluster-policy/validate/).

### Step 3 — Challenge dangerous policy patterns

Flag the following as high-severity findings:

- **`failureAction: Audit` in production** — silent allow path; PolicyReports accumulate without enforcement.
- **`background: false` + match scope that does not match admission requests** — policy never runs; effectively dead code.
- **`match` with `kinds: ['*']` and no namespace selector** — cluster-wide blast radius; one mis-written CEL expression breaks every admission.
- **`exclude` clause that exempts entire `kube-system` or operator namespaces** — operators bypass policy that should still apply (e.g., image signing).
- **`failurePolicy: Ignore` on the underlying ValidatingWebhookConfiguration** — Kyverno controller failures silently allow. See the [Kubernetes admission webhook reference](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/).
- **CEL expressions referencing `request.userInfo` without a deny default** — easy to bypass with a service account named in an exception.

### Step 4 — Audit every PolicyException

A PolicyException is a documented bypass. Treat every one as audit evidence requiring four facts:

1. **Owner**: who created it and is on call for the exempted resources?
2. **Reason**: why does this resource not meet the policy?
3. **Expiry**: is there a date or condition under which this exception is removed? Kyverno does not enforce expiry — this must be a documented commitment.
4. **Scope**: which resources, namespaces, and rules are exempted?

Reference: [Kyverno PolicyExceptions](https://kyverno.io/docs/exceptions/).

Stress-test exceptions:

- An exception with `match.any.resources.kinds: ['*']` exempts everything — almost always too broad.
- An exception that exempts the `default` ServiceAccount — effectively exempts every workload that hasn't bound an SA.
- An exception that exempts a `ClusterPolicy` with `failureAction: Enforce` quietly demotes the policy to `Audit` for the matched scope.

### Step 5 — Audit ImageValidatingPolicy specifically

For `ImageValidatingPolicy` (and legacy `verifyImages` rules), confirm:

1. **Public key or KMS key reference** is present and points to a real attestation root (Sigstore / Cosign / Notary / KMS-backed).
2. **`mutateDigest: true`** — replaces the mutable image tag with the immutable digest at admission. Without this, the verified image can be replaced after admission.
3. **`verifyDigest: true`** — re-checks the digest against the verified attestation chain.
4. **`required: true`** on the verification rule — without this, missing signatures pass.
5. **`match` covers all production registries**, not just public Docker Hub.
6. **No `imageReferences: ['*']` with `skip: true`** — total signature bypass.

Reference: [Kyverno verify-images / ImageValidatingPolicy](https://kyverno.io/docs/policy-types/cluster-policy/verify-images/).

### Step 6 — Evaluate Kyverno vs native ValidatingAdmissionPolicy (CEL)

Native `ValidatingAdmissionPolicy` (CEL) shipped stable in Kubernetes 1.30 ([reference](https://kubernetes.io/docs/reference/access-authn-authz/validating-admission-policy/)). Kyverno can compile its own policies into native VAP — see [Kyverno docs on VAP generation](https://kyverno.io/docs/policy-types/cluster-policy/validate/).

Choose **native VAP** when:

- The policy is pure validation (no mutation, no generation, no image verification, no cleanup).
- The CEL expression alone is sufficient — no JMESPath, no API lookup, no `context.apiCall`, no foreach.
- You want fewer moving parts in the admission path (no Kyverno controller).

Stay with **Kyverno** when:

- You need mutation, generation, cleanup, or image verification.
- You need cross-resource lookups (`context.apiCall`).
- You need PolicyReports for compliance evidence.
- You need PolicyExceptions managed declaratively.

Recommend a path explicitly. "Could be native VAP" without a recommendation is incomplete review.

### Step 7 — Stress-test operational hygiene

- Prefer policies authored with `policies.kyverno.io/v1` over legacy `kyverno.io/v1` — the new API is the long-term path.
- Prefer explicit `match.any.resources.kinds` lists over wildcards.
- Prefer policies with `background: true` so existing resources are scanned (catches drift).
- Prefer policies that emit clear `message` text — admission rejections show this string to the user, and a vague rejection message wastes engineer time.
- Reports Server should be installed when policy reports are needed at scale — etcd-backed PolicyReports do not scale beyond a few thousand violations. See [Kyverno installation](https://kyverno.io/docs/installation/).

## Output

Return:

- **target**: policy kind, name, match scope, and API version,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **failure mode**: `Enforce` vs `Audit`, with judgment on whether this matches production posture,
- **risk findings** (with severity: high / medium / low) — including PolicyException audit, image verification posture, wildcard match, and admission webhook failurePolicy,
- **architectural recommendation**: stay with Kyverno, migrate to native VAP, or hybrid — with reason,
- **safest next actions** with sample manifest changes,
- **rollback plan**: how to remove or revert the policy without breaking running workloads,
- **assumptions and missing facts**.

## Security notes

- Never recommend `failureAction: Audit` for a production-tier policy unless there is a written rollout plan to `Enforce` with a date.
- Never recommend exempting `cluster-admin`, the controller's own ServiceAccount, or wildcards in PolicyExceptions.
- Never recommend disabling image signature verification "temporarily" without a tracked re-enable date.
- Do not print Cosign private keys, Rekor signature blobs, or registry credentials. Reference key names only.
