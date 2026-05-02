# 🔏 Sigstore Agents

<p align="center">
  <span style="font-size:3.5em">🔏</span>
</p>

Sigstore agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit Cosign signing posture, SBOM attestation, Rekor transparency log, and policy enforcement | read-only | not allowed |

## 📋 Supply chain review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `sigstore-cosign-supply-chain-review-agent` | Review Cosign image signing policy, SBOM/attestation presence, Rekor inclusion, keyless signing trust root, and admission policy enforcement via Kyverno or Policy Controller | read-only | — |

## 🛡️ Operating note

- `cosign verify` without `--certificate-identity` and `--certificate-oidc-issuer` accepts signatures from any Sigstore identity — this is not a security guarantee
- SBOM attestation presence does not imply SBOM correctness; review for completeness (all direct + transitive dependencies) and format (SPDX vs CycloneDX)
- Rekor append-only log provides audit trail but not enforcement — enforcement requires admission webhook (Kyverno ClusterPolicy or Sigstore Policy Controller)
- Keyless signing trust root is Fulcio CA — revocation is via Rekor transparency log, not CRL/OCSP

*Admission policy enforcement for Sigstore → `kubernetes-live-admission-policy-guard-agent` (kubernetes live-guard)*

## 📦 Install

```bash
# Install Sigstore supply chain review agent
npx vfa-export-agents --platform claude-code --agents sigstore-cosign-supply-chain-review-agent --repo .

# Install all Kubernetes supply chain security agents
npx vfa-export-agents --platform claude-code --role kubernetes-supply-chain-security-engineer --repo .
```
