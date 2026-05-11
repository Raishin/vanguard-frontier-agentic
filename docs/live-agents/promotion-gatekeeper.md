# NVIDIA Model Promotion Gatekeeper — operator runbook

This is the reference live-execution agent for this repo. Other live agents
(GPU cost governor, NeMo Guardrails red-team, GPU fleet health probe)
should copy its trust posture, file layout, and fixture pattern.

## When to use it

You are a platform / SecOps engineer about to promote an NVIDIA NIM
container from staging to production. You want a runtime-evidence
**promote / block / manual-review** decision plus a tamper-evident
attestation JSON to hand to audit.

If you only want a static review of broader supply-chain posture (NGC
keys, AI Enterprise entitlement, air-gap mirror layout), use
`nvidia-ngc-nim-supply-chain-governor` instead.

## Required inputs

| Input | Source | Notes |
|---|---|---|
| `image_ref` | candidate manifest | e.g. `nvcr.io/nim/meta/llama-3.3-70b:1.5.0` |
| `image_ref_pin` | operator policy | `sha256:...` you expect the tag to resolve to |
| `current_prod_digest` | live cluster query | not from a yaml file that may be stale |
| `expected_signer_identity` | operator policy | NVIDIA's published signer URI for this image set |
| `expected_oidc_issuer` | operator policy | usually `https://token.actions.githubusercontent.com` |
| `mode` | operator | `static` (default, no egress) \| `runtime` |

The agent will ask once for each missing input and then fail closed with
`verdict: manual-review` reason `inputs_incomplete`. Never accept a
default for `expected_signer_identity` or `expected_oidc_issuer` — a
silent default is exactly the regression a future NVIDIA signing-posture
change would create.

## Threat model — what this gate stops

| Threat | Gate | Verdict |
|---|---|---|
| Attacker pushes unsigned image to `nvcr.io` namespace | `cosign verify` | `block / unsigned` |
| Attacker re-tags an old, signed-but-vulnerable image | digest pin compare | `block / digest_drift` |
| Image signed with a stolen / unauthorized identity | `--certificate-identity` check | `block / wrong_identity` |
| Image signed via a different OIDC issuer (e.g. `accounts.google.com`) | `--certificate-oidc-issuer` check | `block / wrong_issuer` |
| Replay of a signature whose Fulcio cert has expired | `cert_not_after < now` | `block / expired_cert` |
| Image with no SBOM attestation (no provenance audit trail) | `cosign verify-attestation --type=spdxjson` | `block / missing_sbom` |
| Image with no model card (no eval / lineage record) | `oras discover` + label fallback | `block / missing_model_card` |
| Promotion that introduces new critical / high CVEs vs current-prod | `grype` delta | `block / cve_regression` |
| Operator-supplied `image_ref` outside `nvcr.io/` namespace | allowlist regex | `block / unknown_registry` |
| Stale attestation replayed past TTL | `attestation_age_hours > ttl` | `block / stale_attestation` |
| Sigstore unreachable (air-gap, transient outage) | `rekor_reachable=false` | `manual-review / rekor_unreachable` |

## What this gate does **not** stop

- Model behavior issues (jailbreaks, prompt injection, eval regressions). Route to a future `nvidia-nemo-guardrails-red-team` agent.
- Runtime config issues (RBAC, network policy, MIG profile). Route to `nvidia-gpu-operator-kubernetes-hardening`.
- License / entitlement issues (AI Enterprise seat exhaustion). Route to `nvidia-ngc-nim-supply-chain-governor`.
- Deployment-time issues (rollout strategy, canary). Out of scope — this gate runs **before** the deploy decision, not as part of it.

## Runbook

1. Set `NGC_API_KEY` in your shell. Do not paste it into the agent prompt.
2. Look up the current production digest from the live cluster (e.g. `kubectl get deploy -n inference llama-3.3 -o jsonpath='{.spec.template.spec.containers[0].image}'`). Paste the digest, not the tag.
3. Look up your team's `expected_signer_identity` and `expected_oidc_issuer` from your policy doc. Do not let the agent guess these.
4. Invoke the agent with `mode: runtime` and the inputs.
5. On `verdict: promote`, sign the attestation: `cosign sign-blob --bundle attestation.bundle attestation.json`. Attach the bundle to your change ticket.
6. On `verdict: block`, the `verdict_reasons[]` enumerate every gate that failed. Do not retry until each reason is addressed.
7. On `verdict: manual-review`, an operator decision is required. Document the reason and the override authority in your change ticket.

## Air-gap operation

When `rekor.sigstore.dev` is unreachable (sovereign / air-gapped
environments), the gatekeeper does **not** auto-pass — that would be a
quiet bypass. It emits `verdict: manual-review` with reason
`rekor_unreachable` and `evidence_level: partial`. Air-gapped
deployments should configure an internal Rekor mirror and rerun, or
override with documented authority.

## CI gate

`npm run validate:promotion-gatekeeper` runs the deterministic gate
evaluator against all 10 golden fixtures under
`tests/fixtures/nvidia-model-promotion-gatekeeper/` and asserts each
emitted attestation is schema-valid and verdict-correct. Each scenario
is a pair: `inputs/NN-name.json` carries the operator inputs and
stubbed command outputs; `expected/NN-name.json` carries the expected
verdict, `verdict_reasons[]`, and `evidence_level`. This is the
project's first eval-fixture pattern; future live agents follow the
same `(inputs/, expected/, validator.py)` triple.

## Why two harnesses, not seven

Live agents carry an allowlist threat model that must be hand-verified
per harness — each harness has different `Bash(...)` enforcement
behavior, different credential-handling primitives, different
default-permission posture. Mass fan-out trades depth for breadth and
for live agents that trade is unsafe. Add a harness only when its
allowlist enforcement and credential-handling story has been
independently audited for this agent.
