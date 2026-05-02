# Workflow and Output Contract

## Workflow

### Step 1 — Identify the scope and collect raw evidence

1. Confirm the review target: a specific container image, a Kyverno ClusterPolicy/Policy, a CI pipeline signing step, or a SLSA level claim.
2. For image signing evidence, run:
   ```bash
   cosign verify \
     --certificate-identity-regexp "https://github.com/<org>/<repo>/.github/workflows/" \
     --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
     <registry>/<image>:<tag>
   ```
   A successful exit means a valid keyless signature exists for that identity + issuer pair. An exit code 1 means no matching signature.
3. For Kyverno admission policy evidence, run:
   ```bash
   kubectl get clusterpolicy,policy -A -o yaml | grep -A 30 "verifyImages"
   ```
   Collect every `verifyImages` block. Note whether `attestors.entries.keyless.subject` and `attestors.entries.keyless.issuer` are set.
4. If Cosign policy-controller is in use instead of Kyverno, collect:
   ```bash
   kubectl get clusterimagepolicy -o yaml
   ```
   Inspect `spec.authorities[].keyless.identities[].issuer` and `.subject` fields.

### Step 2 — Audit the imageVerify / ClusterImagePolicy identity constraints

The most critical control is whether the admission policy constrains **who** signed the image, not just **that** it was signed.

Check each policy rule for:

1. **`issuer`** — the OIDC token issuer (e.g., `https://token.actions.githubusercontent.com` for GitHub Actions). Without this, any OIDC provider's identity can satisfy the check.
2. **`subject`** — the specific identity within the issuer (e.g., `https://github.com/org/repo/.github/workflows/release.yml@refs/heads/main`). Without this, any identity at that issuer passes.
3. **`glob` vs exact match** — subject globs like `https://github.com/org/*` allow any workflow in the org to satisfy the check.

Example of a correctly scoped Kyverno imageVerify rule:
```yaml
verifyImages:
  - imageReferences:
      - "registry.internal.company.com/*"
    attestors:
      - entries:
          - keyless:
              subject: "https://github.com/org/repo/.github/workflows/release.yml@refs/heads/main"
              issuer: "https://token.actions.githubusercontent.com"
              rekor:
                url: https://rekor.sigstore.dev
```

Flag as **CRITICAL** if both `subject` and `issuer` are absent — the policy accepts any Sigstore-signed image regardless of signer.

Flag as **HIGH** if `issuer` is set but `subject` is absent — any identity at that issuer passes (e.g., any GitHub Actions workflow anywhere on GitHub).

### Step 3 — Audit `exclude` rules and policy coverage

1. List all `exclude` blocks in every imageVerify policy:
   ```bash
   kubectl get clusterpolicy -o yaml | grep -A 10 "exclude"
   ```
2. Flag as **HIGH** any exclude that matches:
   - A broad registry glob (`docker.io/*`, `*`)
   - A namespace containing workloads with access to sensitive data
3. Confirm whether ALL namespace-resident Deployments, StatefulSets, DaemonSets, and Jobs are subject to the policy. Kyverno policies with no `matchResources.namespaceSelector` apply cluster-wide — verify this is intentional.

Example of a dangerous broad exclusion:
```yaml
exclude:
  resources:
    images:
      - "docker.io/*"   # All Docker Hub images skip verification
```

### Step 4 — Audit SLSA provenance attestations

1. Check whether a SLSA provenance attestation exists:
   ```bash
   cosign verify-attestation \
     --type slsaprovenance \
     --certificate-identity-regexp "https://github.com/<org>/<repo>/" \
     --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
     <registry>/<image>:<tag>
   ```
2. For images claiming SLSA L2+, verify with slsa-verifier:
   ```bash
   slsa-verifier verify-image \
     --source-uri github.com/<org>/<repo> \
     --source-branch main \
     <registry>/<image>:<tag>
   ```
3. Check whether the build was ephemeral (GitHub Actions or Tekton Chains) — SLSA L3 requires an ephemeral, isolated build environment. Builds on persistent, developer-accessible runners cannot claim L3.

Flag as **HIGH** if SLSA L2 is claimed but `slsa-verifier verify-image` fails or returns no matching attestation.

### Step 5 — Audit SBOM attestations

1. Verify SBOM attestation presence:
   ```bash
   cosign verify-attestation \
     --type spdxjson \
     --certificate-identity-regexp "https://github.com/<org>/<repo>/" \
     --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
     <registry>/<image>:<tag>
   ```
2. For CycloneDX SBOM format:
   ```bash
   cosign verify-attestation \
     --type cyclonedx \
     <image>
   ```
3. Check whether the SBOM was generated at build time (accurate) or at image scan time (less reliable — may miss build-time artifacts).
4. For workloads handling PII or financial data, flag as **MEDIUM** if no SBOM attestation is present — without an SBOM, dependency vulnerability provenance cannot be confirmed.

### Step 6 — Audit Cosign key management (keyless vs key-based)

1. Check CI pipeline signing steps for evidence of keyless OIDC flow:
   ```yaml
   # Correct keyless pattern in GitHub Actions
   - name: Sign image
     env:
       COSIGN_EXPERIMENTAL: "1"   # Enables keyless (OIDC Workload Identity)
     run: |
       cosign sign --yes ${{ env.IMAGE_REF }}
   ```
2. Flag as **HIGH** if the CI pipeline uses `cosign sign --key cosign.key` or references a `COSIGN_PRIVATE_KEY` secret — long-lived key material in CI secrets is a secret sprawl risk.
3. Verify that keyless signing uses the correct OIDC token source:
   - GitHub Actions: `id-token: write` permission must be set in the workflow.
   - Tekton Chains: `CHAINS-GCP-SERVICE-ACCOUNT` or equivalent OIDC binding must be configured.

Example correct GitHub Actions OIDC signing permission:
```yaml
permissions:
  id-token: write
  contents: read
  packages: write
```

Flag as **HIGH** if `id-token: write` is absent from the workflow — keyless signing will silently fail or fall back to anonymous signing.

### Step 7 — Audit Rekor transparency log posture

1. Check whether public Rekor logging is active (default) or disabled:
   ```bash
   # Default: public Rekor is used
   cosign sign --yes <image>

   # Disabled: no transparency log entry created
   COSIGN_NO_TLOG=1 cosign sign --yes <image>
   ```
2. Flag as **MEDIUM** if `COSIGN_NO_TLOG=1` is set without a private Rekor instance configured — disabling transparency logging removes third-party verifiability and auditability.
3. For images containing internal service references, infrastructure hostnames, or internal artifact paths, flag public Rekor logging as a **MEDIUM** information disclosure risk. These images should use a private Rekor instance.
4. To verify a signature was logged to Rekor:
   ```bash
   cosign verify \
     --certificate-identity-regexp "<signer>" \
     --certificate-oidc-issuer "<issuer>" \
     <image> | jq '.[0].optional.Bundle.Payload.logIndex'
   ```
   A non-null `logIndex` confirms the signature is in the public Rekor transparency log.

### Step 8 — Verify admission enforcement is active

1. Confirm Kyverno is installed and the webhook is active:
   ```bash
   kubectl get mutatingwebhookconfiguration,validatingwebhookconfiguration | grep kyverno
   kubectl get pods -n kyverno
   ```
2. Confirm imageVerify policy is in `Enforce` mode (not `Audit`):
   ```bash
   kubectl get clusterpolicy <policy-name> -o jsonpath='{.spec.validationFailureAction}'
   ```
   `Enforce` blocks non-conforming images at admission. `Audit` only logs — images still deploy.
3. Flag as **HIGH** if imageVerify policy is in `Audit` mode in production — unsigned images are not blocked.

## Output

Return:

- **target**: image reference, ClusterPolicy name, or CI pipeline step, with the evidence source,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **signing identity**: keyless OIDC (Fulcio) vs long-lived key, with the specific issuer and subject,
- **admission enforcement**: Kyverno imageVerify / policy-controller / none, with policy mode (Enforce/Audit),
- **identity constraint audit**: issuer and subject present/absent, glob scope, exclude rule coverage,
- **attestation inventory**: SLSA provenance present/absent, SBOM present/absent, format,
- **Rekor posture**: public log / private log / disabled, with information disclosure risk if applicable,
- **risk findings** (with severity: critical / high / medium / low),
- **safest next actions** with sample policy or workflow YAML,
- **assumptions and missing facts**.

## Security notes

- Never recommend disabling imageVerify enforcement in production to unblock a deployment — the correct path is to fix the signing pipeline.
- Never recommend broad `exclude` rules as a permanent fix for third-party image coverage gaps.
- Never request or print private Cosign keys, OIDC tokens, registry credentials, or cosign.key file contents.
- Always confirm admission policy is in `Enforce` mode before concluding that unsigned images are blocked.
- A Kyverno imageVerify policy in `Audit` mode with no `Enforce` policy provides zero actual enforcement — treat this as a critical gap.
