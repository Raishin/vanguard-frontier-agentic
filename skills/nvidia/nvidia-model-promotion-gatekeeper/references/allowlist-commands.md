# Allowlist commands — exact argv and rationale

Every Bash invocation the gatekeeper makes must match one of these patterns. The Claude Code `allowed-tools` field encodes the same allowlist. Anything outside this list is denied at the harness layer.

## Signature verification

### `cosign verify --certificate-identity=<id> --certificate-oidc-issuer=<issuer> <image>`
Keyless verification against an expected signer identity and OIDC issuer. Both flags are mandatory — never call `cosign verify` without them in runtime mode. Without `--certificate-identity` and `--certificate-oidc-issuer`, cosign accepts any valid Sigstore signature, which defeats the gate.

### `cosign verify-attestation --type=spdxjson <image>` and `--type=cyclonedx <image>`
Asserts the presence and integrity of an SBOM attestation in SPDX-JSON or CycloneDX form. The gatekeeper accepts either format. Missing SBOM → `block` with reason `missing_sbom`.

### `cosign download attestation <image>`
Fetches the raw attestation envelope so the gatekeeper can hash it into the output `claims.sbom.sha256` field. This is the artifact audit downstream-verifies independently.

## Digest pinning

### `crane digest <image>`
Resolves a tag like `nvcr.io/nim/meta/llama-3.3-70b:1.5.0` to its current `sha256:...`. The result is compared against the operator-supplied `image_ref_pin`. Mismatch → `block` with reason `digest_drift`. This is the primary defense against tag mutability between staging-time and promote-time.

### `crane manifest <image>` and `crane config <image>`
Reads the OCI manifest and image config for diagnostic context only. The `org.opencontainers.image.documentation` label is not accepted as the model-card proof because it is only a pointer, not a fetched, hashed artifact.

## Model card and referrers

### `oras discover --format json <image>`
Lists OCI referrers attached to the image manifest. The gatekeeper looks for a referrer with `artifactType: application/vnd.nvidia.model-card+json` (or the equivalent NVIDIA-published media type). Missing referrer, label-only reference, or missing referrer sha256 → `block` with reason `missing_model_card`.

### `oras manifest fetch <image>`
Fetches a specific referrer manifest by digest so the gatekeeper can compute and record the model-card sha256.

## CVE delta

### `grype <image> --output json --fail-on never` and `grype sha256:<digest> --output json --fail-on never`
Scans the candidate image and the current-prod digest. The gatekeeper computes the delta (`new_critical`, `new_high`, `fixed_critical`). `--fail-on never` is mandatory: the gatekeeper makes the verdict, not Grype. New critical or high CVEs → `block` with reason `cve_regression`.

## Forbidden

- `docker pull` — would mutate local image cache.
- `kubectl apply` — would mutate cluster state. The gatekeeper is read-only.
- `cosign sign` / `cosign sign-blob` — signing is the **operator's** action after they accept the verdict, not the agent's.
- `curl`, `wget`, `git push` — out of allowlist; egress is via cosign/crane/oras only so the egress hosts are knowable up front.
- Any command containing `|`, `;`, `&`, `$`, backticks, or redirections — argv allowlist enforcement at the harness layer rejects shell metacharacters.
- Any registry prefix other than `nvcr.io/` — explicit `block` verdict, recorded reason `unknown_registry`.
