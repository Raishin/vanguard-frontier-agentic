---
name: sigstore-cosign-supply-chain-review
description: Use this skill when reviewing Sigstore Cosign supply chain security for Kubernetes workloads. Trigger when the user asks whether images are properly signed, whether Kyverno imageVerify policy is correctly scoped, whether SLSA provenance attestations exist, whether SBOM attestations are present, whether keyless signing is in use, or whether Rekor transparency log posture is appropriate for private images.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: security
---

# Sigstore Cosign Supply Chain Review

## Purpose

Review Cosign image signing verification, Kyverno imageVerify admission policy, SBOM and SLSA provenance attestations, Rekor transparency log posture, and keyless vs key-based signing configuration against supply chain integrity, SLSA level claims, and Kubernetes admission-time enforcement. Sigstore's security model depends entirely on the identity constraints baked into admission policy — an imageVerify rule with no issuer or subject constraint is functionally equivalent to no verification at all.

## Lean operating rules

- Prefer live evidence (`cosign verify`, `kubectl get clusterpolicies`, `cosign verify-attestation`) when the active client exposes it; otherwise fall back to official Sigstore documentation and sanitized YAML from the user.
- Separate confirmed facts from inference. If Kyverno policy state, Rekor log inclusion, or provenance attestation presence was not directly queried, say so.
- Treat a Kyverno imageVerify policy missing both `issuer` and `subject` constraints as a critical finding — any Sigstore-signed image from any identity passes.
- Treat `exclude` rules in imageVerify that match broad glob patterns (`*` or `registry.io/*`) as a high finding — third-party images bypass verification.
- Treat SLSA L2+ claimed but no SLSA provenance attestation verifiable via `slsa-verifier` as a high finding.
- Treat long-lived Cosign keypairs stored as CI secrets as a high finding — keyless OIDC Workload Identity is the preferred pattern.
- Treat `COSIGN_NO_TLOG=1` on non-private-Rekor setups as a medium finding — public transparency is disabled without a private transparency alternative.
- Keep the answer scoped, evidence-labeled, and explicit about what was not queried.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

Return, at minimum:
- the scoped target (image, imageVerify policy, CI pipeline signing step, or SLSA level claim) and evidence level,
- the signing identity (keyless OIDC via Fulcio, long-lived key, or unverified),
- the admission enforcement posture (Kyverno imageVerify, policy-controller, or none),
- the attestation inventory (SBOM present/absent, SLSA provenance present/absent),
- the Rekor transparency posture (public log, private log, or disabled),
- the safest next actions and any assumptions or blockers.
