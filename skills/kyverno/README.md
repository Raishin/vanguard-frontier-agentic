# 🛡️ Kyverno Skills

<p align="center">
  <!-- 🖼️ Add a Kyverno logo to assets/logos/cnative/kyverno/ and update this path -->
  <span style="font-size:3.5em">🛡️</span>
</p>

This folder contains Kyverno-focused skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local Kyverno skill:

- `kyverno-policy-review`

## Portfolio posture

Kyverno skills for evidence-backed admission policy review across `ValidatingPolicy`, `MutatingPolicy`, `GeneratingPolicy`, `DeletingPolicy`, and `ImageValidatingPolicy` — the stable `policies.kyverno.io/v1` API surface.

These skills are intentionally conservative:

- prefer `kubectl get policies.kyverno.io -A -o yaml` for live policy state grounding before any review
- treat `failureAction: Audit` in production as a critical finding — policy violations become silent
- treat `PolicyException` resources as audit-required escalation paths — every exception is a documented bypass
- challenge any policy with `background: false` and no admission match — the policy never runs
- prefer policies that compile to native `ValidatingAdmissionPolicy` (CEL) when complexity allows — fewer moving parts than the Kyverno controller
- challenge `ImageValidatingPolicy` with `verifyImages` skipped on CVE-only images — supply-chain attestation must remain
- use official Kyverno documentation (kyverno.io) for policy syntax, CEL expressions, and ValidatingAdmissionPolicy generation

Run `npm run validate` after changing cataloged Kyverno skills.
