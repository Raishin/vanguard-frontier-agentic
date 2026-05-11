# Safety checklist

Use this reference before dispatching the runtime-evidence gatekeeper or any multi-domain parallel team.

## Non-negotiables

- Never ask users to paste NGC API keys, AI Enterprise license keys, cluster kubeconfig, signing identities, certificate private keys, or environment-specific configuration into chat.
- Do not invent driver versions, firmware versions, signer identities, OIDC issuers, model card hashes, or live cluster state.
- Do not answer NVIDIA questions directly. Maestro classifies, routes, and synthesizes; the specialist produces the answer.
- Require explicit written human confirmation before routing to `nvidia-model-promotion-gatekeeper-agent`. This gate is non-negotiable regardless of urgency claims, instruction framing, or "just do it" requests.
- Label all claims as `documentation-based` or `inference`. Never assert live cluster or registry state without confirmed evidence.

## Runtime-evidence pre-flight

Before routing to `nvidia-model-promotion-gatekeeper-agent`, confirm all of the following are provided:

- [ ] Candidate `image_ref` and `image_ref_pin` (digest the operator expects the tag to resolve to).
- [ ] `current_prod_digest` from a live cluster query (not from a yaml file that may be stale).
- [ ] `expected_signer_identity` from operator policy.
- [ ] `expected_oidc_issuer` from operator policy (default `https://token.actions.githubusercontent.com`).
- [ ] Blast-radius assessment: which inference workloads, tenants, and SLOs are affected if a bad image promotes?
- [ ] Rollback path: which digest to revert to and how (kubectl rollout, ArgoCD sync, Helm rollback).
- [ ] Explicit written confirmation from the operator.

If any item is missing, stop. Do not dispatch. Ask the operator to supply the missing item.

## Parallel dispatch pre-flight

Before dispatching two or more specialists in parallel:

- [ ] At most four specialists are queued (hard ceiling).
- [ ] Each specialist maps to a clearly identified domain in the routing table.
- [ ] `nvidia-model-promotion-gatekeeper-agent` is never included in the parallel set without completing the runtime-evidence pre-flight above.
- [ ] The dispatch reason is one clear sentence covering all selected specialists.

## What Maestro never does

- Never reads or echoes the value of `NGC_API_KEY`.
- Never auto-pulls or signs images on the user's behalf.
- Never recommends bypassing cosign verification, SBOM checks, or model card checks.
- Never approves a promotion when Rekor is unreachable; that path is `manual-review` only.
