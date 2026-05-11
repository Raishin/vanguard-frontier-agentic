# Golden fixtures — `nvidia-model-promotion-gatekeeper`

Each `NN-name.json` is a recorded scenario: the operator inputs, plus the stubbed outputs the gatekeeper would have observed if it had executed each allowlisted command. The fixture is consumed by `tests/validate-nvidia-promotion-gatekeeper.py`, which runs a pure-Python gate evaluator against the stubs, produces an attestation JSON, validates it against `schemas/attestation.schema.json`, and asserts the verdict and `verdict_reasons[]` match `expected/NN-name.json`.

This is the project's first eval-fixture pattern. Other live agents should copy this layout.

## Taxonomy

| # | Fixture | Scenario | Expected verdict |
|---|---|---|---|
| 01 | `01-clean.json` | Signed by expected NVIDIA identity, SBOM + model card present, no new CVEs | `promote` |
| 02 | `02-unsigned.json` | `cosign verify` fails (no signature found) | `block` |
| 03 | `03-digest-drift.json` | Tag resolves to a digest different from operator-supplied pin | `block` |
| 04 | `04-missing-sbom.json` | No SPDX or CycloneDX attestation referrer | `block` |
| 05 | `05-missing-model-card.json` | No model card OCI referrer and no documentation label | `block` |
| 06 | `06-cve-regression.json` | Candidate adds new CRITICAL CVEs vs current-prod | `block` |
| 07 | `07-expired-cert.json` | Fulcio cert `notAfter` < now (replay artifact) | `block` |
| 08 | `08-wrong-issuer.json` | Signer identity matches but OIDC issuer is not the expected one | `block` |
| 09 | `09-unknown-registry.json` | `image_ref` is `docker.io/...`, not `nvcr.io/...` | `block` |
| 10 | `10-replay-stale-attestation.json` | Attestation older than the operator-configured TTL | `block` |

Air-gap behavior (Sigstore unreachable) is exercised separately by setting `stub_outputs.rekor_reachable=false` in any fixture; that path emits `manual-review` with reason `rekor_unreachable`.

## Adding a fixture

1. Drop a new `NN-shortname.json` in this directory.
2. Add `expected/NN-shortname.json` with the expected `verdict` and the **set** of `verdict_reasons[]` (order does not matter).
3. Run `python3 tests/validate-nvidia-promotion-gatekeeper.py`. It will replay every fixture and report.
