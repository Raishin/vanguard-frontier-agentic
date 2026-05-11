# Attestation format — walk-through

The gatekeeper emits a JSON document conforming to `schemas/attestation.schema.json`. The operator signs the file with `cosign sign-blob --bundle attestation.bundle attestation.json` and hands the bundle to audit.

## Worked example (verdict: promote)

```json
{
  "attestation_version": "1.0.0",
  "agent": {
    "id": "nvidia-model-promotion-gatekeeper-agent",
    "version": "0.1.0",
    "execution_tier": "read-only-runtime"
  },
  "subject": {
    "image_ref": "nvcr.io/nim/meta/llama-3.3-70b:1.5.0",
    "resolved_digest": "sha256:5f5fbd7a6e9b0c3a2e1d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3f2e1d0c9b8a7f",
    "registry": "nvcr.io",
    "current_prod_digest": "sha256:9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0f9a8b"
  },
  "claims": {
    "signature": {
      "verified": true,
      "signer_identity": "https://github.com/nvidia/nim-builder/.github/workflows/release.yml@refs/tags/v1.5.0",
      "issuer": "https://token.actions.githubusercontent.com",
      "cert_not_after": "2027-01-15T00:00:00Z",
      "rekor_log_index": 134567890
    },
    "sbom": {
      "present": true,
      "format": "spdx",
      "sha256": "sha256:1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b"
    },
    "model_card": {
      "present": true,
      "sha256": "sha256:0f1e2d3c4b5a6978a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1",
      "source": "oci-referrer"
    },
    "cve_delta": {
      "vs_digest": "sha256:9a8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0f9a8b",
      "new_critical": 0,
      "new_high": 0,
      "fixed_critical": 2,
      "regressed": false
    }
  },
  "evidence_level": "live",
  "verdict": "promote",
  "verdict_reasons": ["all_gates_passed"],
  "provenance": {
    "executed_commands": [
      "crane digest nvcr.io/nim/meta/llama-3.3-70b:1.5.0",
      "cosign verify --certificate-identity=https://github.com/nvidia/nim-builder/.github/workflows/release.yml@refs/tags/v1.5.0 --certificate-oidc-issuer=https://token.actions.githubusercontent.com nvcr.io/nim/meta/llama-3.3-70b@sha256:5f5fbd7a...",
      "cosign verify-attestation --type=spdxjson nvcr.io/nim/meta/llama-3.3-70b@sha256:5f5fbd7a...",
      "oras discover --format json nvcr.io/nim/meta/llama-3.3-70b@sha256:5f5fbd7a...",
      "grype nvcr.io/nim/meta/llama-3.3-70b@sha256:5f5fbd7a... --output json --fail-on never",
      "grype sha256:9a8b7c6d... --output json --fail-on never"
    ],
    "egress_hosts_contacted": ["nvcr.io", "rekor.sigstore.dev", "fulcio.sigstore.dev"],
    "runtime_mode": "runtime",
    "harness": "claude-code",
    "operator": "alice@example.com"
  },
  "timestamp": "2026-05-11T10:42:17Z",
  "nonce": "5b3f7a91c2d8e4f60a1b2c3d4e5f6a7b",
  "expires_at": "2026-05-12T10:42:17Z"
}
```

## Field rules

- `verdict` is one of `promote | block | manual-review`. Never any other value.
- `verdict_reasons[]` enumerates every reason — if `verdict=block` due to both `digest_drift` and `cve_regression`, both are listed.
- `evidence_level: live` requires runtime mode plus successful Sigstore reach. Sigstore unreachable → `partial`. Static-mode replay → `documentation-only`.
- `nonce` is required when `runtime_mode=runtime`. 16+ characters, operator-controlled, included so the attestation is non-replayable downstream.
- `expires_at` is operator policy — default 24h. Audit may reject attestations older than this on intake.
- `provenance.executed_commands[]` is the exact argv string, but with any flag value matching `(?i)(--password|--token|--auth)=\S+` rewritten to `--<flag>=<REDACTED>`.

## Audit verification flow

1. `cosign verify-blob --bundle attestation.bundle attestation.json` — proves the operator signed it.
2. Schema-validate against `schemas/attestation.schema.json`.
3. Re-resolve `subject.image_ref` and confirm `subject.resolved_digest` still matches (catches post-attestation tag mutation).
4. Confirm `timestamp` is within `expires_at` window and `nonce` has not been seen before.
