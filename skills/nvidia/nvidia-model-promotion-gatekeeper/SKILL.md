---
name: nvidia-model-promotion-gatekeeper
description: Use this skill when an operator is about to promote an NVIDIA NIM container from staging to production and needs a runtime-evidence go/no-go decision. The skill executes a fixed allowlist of cosign/crane/oras/grype commands against the candidate image, then emits a signed attestation JSON whose verdict is one of promote, block, or manual-review. Trigger when the user asks "is this NIM safe to promote", "verify this container before deploy", or hands the agent a `nvcr.io/...` image reference and a current-prod digest. Live tier counterpart to `nvidia-ngc-nim-supply-chain-governor` (which is static-review only).
allowed-tools: Read Grep Glob Bash(cosign verify --certificate-identity=* --certificate-oidc-issuer=* nvcr.io/*) Bash(cosign verify-attestation --type=spdxjson nvcr.io/*) Bash(cosign verify-attestation --type=cyclonedx nvcr.io/*) Bash(cosign download attestation nvcr.io/*) Bash(crane digest nvcr.io/*) Bash(crane manifest nvcr.io/*) Bash(crane config nvcr.io/*) Bash(oras discover --format json nvcr.io/*) Bash(oras manifest fetch nvcr.io/*) Bash(grype nvcr.io/* --output json --fail-on never) Bash(grype sha256:* --output json --fail-on never)
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-11"
  category: security
  lifecycle: experimental
  execution_tier: read-only-runtime
  required_egress:
    - nvcr.io
    - rekor.sigstore.dev
    - fulcio.sigstore.dev
    - tuf-repo-cdn.sigstore.dev
  requires_credentials:
    - NGC_API_KEY
  output_attestation:
    schema: schemas/attestation.schema.json
    signed_with: cosign-blob
  eval_fixtures: tests/fixtures/nvidia-model-promotion-gatekeeper/
---

# NVIDIA Model Promotion Gatekeeper

## Purpose

Before a NIM container moves from staging to production, prove four things with **runtime evidence**, then emit a tamper-evident attestation:

1. **Signature is valid and from the expected NVIDIA identity** — keyless cosign verify against the expected `--certificate-identity` and `--certificate-oidc-issuer`.
2. **Tag has not drifted** — `crane digest` resolution matches the operator-supplied `image_ref@sha256:...` pin.
3. **SBOM and model card are present and pinned** — SPDX or CycloneDX SBOM attestation present; model card present as OCI referrer or label, recorded by sha256.
4. **No CVE regression vs current-prod** — `grype` delta against the current production digest; new critical or high CVEs block promotion.

The skill never mutates: no `docker pull`, no `kubectl apply`, no registry write. It reads, verifies, and emits a JSON attestation the operator signs with `cosign sign-blob` and hands to audit.

## Default mode is dry-run

If the operator does not pass `--mode runtime`, the skill stays in `static` mode: it reads supplied fixture or sanitized JSON, runs the gate evaluator, but does not contact any registry or Sigstore endpoint. `evidence_level` is then `documentation-only` and `verdict` defaults to `manual-review`.

`--mode runtime` opt-in is per-session, never per-skill-default. The operator must hand the skill an explicit image reference plus a current-prod digest.

## Operating rules

- Resolve `image_ref` → `digest` via `crane digest` first. If the operator-supplied pin (`image_ref@sha256:...`) does not match the resolved digest, emit `verdict: block` with reason `digest_drift`.
- The signer identity and OIDC issuer to expect must be supplied as inputs (never defaulted), so a future change in NVIDIA's signing posture cannot silently be auto-accepted.
- Treat any registry prefix other than `nvcr.io/` as `verdict: block` with reason `unknown_registry`. The allowlist regex enforces this; the skill must also assert it explicitly so the failure mode is auditable.
- Treat a Fulcio cert with `notAfter < now` as `verdict: block` with reason `expired_cert`. A valid-looking signature on an expired cert is a replay artifact.
- Treat missing SBOM attestation as `verdict: block` with reason `missing_sbom`. Treat missing model card (no OCI referrer of type `application/vnd.nvidia.model-card+json` AND no `org.opencontainers.image.documentation` label) as `verdict: block` with reason `missing_model_card`.
- Treat a Grype delta showing one or more new CRITICAL or HIGH CVEs vs `current_prod_digest` as `verdict: block` with reason `cve_regression`.
- If Rekor or Fulcio is unreachable (air-gapped or transient), do **not** auto-block. Emit `evidence_level: partial` and `verdict: manual-review` with reason `rekor_unreachable`. Air-gapped deployments must promote on operator override, not on automatic ignore.
- The attestation must include `provenance.executed_commands` (exact argv, no env), `provenance.egress_hosts_contacted`, `provenance.runtime_mode`, and a 16+ char `nonce` whenever `runtime_mode=runtime`. Anti-replay is the operator's responsibility downstream, but the gatekeeper provides the field.
- Never write credentials. Never echo `$NGC_API_KEY`. The attestation `provenance.executed_commands` array must scrub any flag values that resemble secrets (no equals-form value capture for `--password`, `--token`, `--auth`).

## Required inputs

| Field | Required | Example |
|---|---|---|
| `image_ref` | yes | `nvcr.io/nim/meta/llama-3.3-70b:1.5.0` |
| `image_ref_pin` | yes (runtime mode) | `sha256:abc123...` (operator-supplied expected digest) |
| `current_prod_digest` | yes | `sha256:def456...` |
| `expected_signer_identity` | yes | `https://github.com/nvidia/nim-builder/.github/workflows/release.yml@refs/tags/v1.5.0` |
| `expected_oidc_issuer` | yes | `https://token.actions.githubusercontent.com` |
| `mode` | optional | `static` (default) \| `runtime` |

## Response shape

The attestation JSON is the canonical response. The agent additionally surfaces, in plain text:

1. **Verdict** — `promote` \| `block` \| `manual-review`
2. **Evidence level** — `live` \| `partial` \| `documentation-only`
3. **Top reasons** — verbatim from `verdict_reasons[]`
4. **Next action** — `cosign sign-blob --bundle attestation.bundle attestation.json` for `promote`; remediation hints for `block`; explicit operator decision request for `manual-review`
5. **Attestation path** — where the JSON was written

## References

- `references/allowlist-commands.md` — exact argv catalog with rationale per command
- `references/attestation-format.md` — attestation schema walk-through with a worked example
