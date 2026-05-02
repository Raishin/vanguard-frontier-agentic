# Official Sources

Load these only when needed:

- [Kyverno documentation home](https://kyverno.io/docs/) — use as the entry point for any policy authoring, install, or operator-side question.
- [Kyverno policy types overview](https://kyverno.io/docs/policy-types/overview/) — use for the stable `policies.kyverno.io/v1` API surface (`ValidatingPolicy`, `MutatingPolicy`, `GeneratingPolicy`, `DeletingPolicy`, `ImageValidatingPolicy`).
- [Kyverno validate rules](https://kyverno.io/docs/policy-types/cluster-policy/validate/) — use for `failureAction`, `failurePolicy`, CEL validation expressions, `denyConditions`, and the Kyverno-to-ValidatingAdmissionPolicy compilation path.
- [Kyverno mutate rules](https://kyverno.io/docs/policy-types/cluster-policy/mutate/) — use for `patchStrategicMerge`, `patchesJson6902`, `foreach` mutations, and conditional mutation guards.
- [Kyverno generate rules](https://kyverno.io/docs/policy-types/cluster-policy/generate/) — use for `synchronize: true` (rule keeps generated resources in sync) and the security implications of generated RoleBindings or NetworkPolicies.
- [Kyverno verify-images / ImageValidatingPolicy](https://kyverno.io/docs/policy-types/cluster-policy/verify-images/) — use for Cosign keyless and key-based verification, attestation chains, `mutateDigest`, `verifyDigest`, and Sigstore Rekor / Notary configuration.
- [Kyverno PolicyExceptions](https://kyverno.io/docs/exceptions/) — use for `PolicyException` syntax, the audit posture exceptions create, and `match` / `exclude` semantics.
- [Kyverno cleanup policies](https://kyverno.io/docs/policy-types/cluster-policy/cleanup/) — use for `DeletingPolicy` cron-driven resource deletion patterns.
- [Kyverno installation](https://kyverno.io/docs/installation/) — use for Helm install, Reports Server enablement, and admission webhook timing.
- [Kyverno CLI (`kyverno apply`, `kyverno test`, `kyverno migrate-policy`)](https://kyverno.io/docs/kyverno-cli/) — use for offline policy testing and Kyverno-to-VAP migration.
- [Kyverno PolicyReport / ClusterPolicyReport](https://kyverno.io/docs/policy-reports/) — use for the OpenReports-format violation records the Reports Server stores.
- [Kubernetes ValidatingAdmissionPolicy (CEL)](https://kubernetes.io/docs/reference/access-authn-authz/validating-admission-policy/) — use for the native VAP CEL syntax that Kyverno compiles to.
- [Kubernetes admission webhook reference](https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/) — use for `failurePolicy: Fail` vs `Ignore`, webhook timeout, and the admission chain.
- [Sigstore Cosign documentation](https://docs.sigstore.dev/cosign/overview/) — use for signing flow that ImageValidatingPolicy verifies.

## Grounded insights worth carrying into the skill

- The stable Kyverno API is `policies.kyverno.io/v1` with five kinds: `ValidatingPolicy`, `MutatingPolicy`, `GeneratingPolicy`, `DeletingPolicy`, `ImageValidatingPolicy`. The legacy `kyverno.io/v1` `ClusterPolicy` and `Policy` kinds are still supported but deprecated.
- Kyverno can compile a `ClusterPolicy` (validate-only, CEL-only) into a native `ValidatingAdmissionPolicy` so admission is enforced by the Kubernetes API server without the Kyverno controller in the request path. This is the leanest deployment when the policy fits VAP's capabilities.
- `failureAction: Audit` (newer API) and `validationFailureAction: audit` (legacy) silently allow violations. Many security incidents have been traced back to a policy that was set to `Audit` "temporarily" and never promoted to `Enforce`.
- `PolicyException` resources exempt resources from policy. Every exception is a bypass with no built-in expiry, owner, or revoke trigger — the documentation discipline must come from process.
- `ImageValidatingPolicy` without `mutateDigest: true` allows a verified tag to be re-pointed to a different image after admission. This is a known image-replacement attack path.
- Reports Server is a separate component that decouples PolicyReport storage from etcd. Without it, PolicyReports at Fortune 50 scale (millions of resources × dozens of policies) overwhelm etcd.
- Kyverno's default admission webhook timeout is 10 seconds. Policies that perform `context.apiCall` lookups can hit this timeout and fall back to `failurePolicy` — if `failurePolicy` is `Ignore` (default), violations silently pass.
- The cleanup controller (which powers `DeletingPolicy`) is a separate deployment and must be installed explicitly via Helm value `cleanupController.enabled=true`.
- `background: false` disables the periodic scan of existing resources. The policy only runs at admission, so resources created before the policy existed are never evaluated — useful for migrations, dangerous as a default.
- Aggregated CRDs (Kyverno does not ship these, but operators may) can match Kyverno policies in unexpected ways — confirm `match.any.resources.kinds` does not pick up CRDs from third-party operators.
