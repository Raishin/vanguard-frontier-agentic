# Golden fixtures — `nvidia-model-promotion-gatekeeper`

Each fixture is a pair of files with the same basename:

- `inputs/NN-name.json` — the recorded **scenario**: operator inputs plus the stubbed outputs the gatekeeper would have observed if it had executed each allowlisted command.
- `expected/NN-name.json` — the expected **verdict** (`promote` / `block` / `manual-review`), the set of `verdict_reasons[]`, and the `evidence_level`.

`tests/validate-nvidia-promotion-gatekeeper.py` reads each `inputs/NN-name.json`, runs the pure-Python gate evaluator against its stubs, produces an attestation JSON, validates that attestation against `schemas/attestation.schema.json`, and asserts verdict + reasons match `expected/NN-name.json`.

This is the project's first eval-fixture pattern. Other live agents should copy this layout.

## Taxonomy

| # | Pair basename | Scenario | Expected verdict |
|---|---|---|---|
| 01 | `01-clean` | Signed by expected NVIDIA identity, SBOM + model card present, no new CVEs | `promote` |
| 02 | `02-unsigned` | `cosign verify` fails (no signature found) | `block` |
| 03 | `03-digest-drift` | Tag resolves to a digest different from operator-supplied pin | `block` |
| 04 | `04-missing-sbom` | No SPDX or CycloneDX attestation referrer | `block` |
| 05 | `05-missing-model-card` | No model card OCI referrer | `block` |
| 06 | `06-cve-regression` | Candidate adds new CRITICAL CVEs vs current-prod | `block` |
| 07 | `07-expired-cert` | Fulcio cert `notAfter` < now (replay artifact) | `block` |
| 08 | `08-wrong-issuer` | Signer identity matches but OIDC issuer is not the expected one | `block` |
| 09 | `09-unknown-registry` | `image_ref` is `docker.io/...`, not `nvcr.io/...` | `block` |
| 10 | `10-replay-stale-attestation` | Attestation older than the operator-configured TTL | `block` |
| 11 | `11-label-only-model-card` | Documentation label exists, but no fetched model-card OCI referrer sha256 | `block` |

Air-gap behavior (Sigstore unreachable) is exercised separately by setting `stub_outputs.rekor_reachable=false` in any fixture; that path emits `manual-review` with reason `rekor_unreachable`.

## Adding a fixture

1. Drop a new `inputs/NN-shortname.json` with the scenario + `stub_outputs`.
2. Add `expected/NN-shortname.json` with the expected `verdict`, the **set** of `verdict_reasons[]` (order does not matter), and `evidence_level`.
3. Run `python3 tests/validate-nvidia-promotion-gatekeeper.py`. It will replay every fixture and report.
