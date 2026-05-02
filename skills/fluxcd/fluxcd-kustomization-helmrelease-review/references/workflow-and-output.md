# Workflow and output contract

Use this reference only when performing a full FluxCD Kustomization or HelmRelease review, producing implementation guidance, triaging a GitOps drift incident, or completing a production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- `GitRepository` source trust: commit signature verification, semver pinning, SOPS decryption config
- `Kustomization` ServiceAccount scoping, prune safety, and health check completeness
- `HelmRelease` chart version pinning, upgrade remediation strategy, and timeout settings
- `HelmRepository` and `OCIRepository` source authentication and trust
- SOPS encryption status: whether Secret manifests in Git are encrypted
- Multi-tenant ServiceAccount isolation: whether tenant Kustomizations use scoped SAs

## Safe workflow

1. **Frame scope**
   - Cluster name and environment (dev / staging / production):
   - Flux version (`flux version`):
   - Number of Kustomizations and HelmReleases under review:
   - Multi-tenant mode in use (yes / no):
   - Required outcome:
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer user-provided sanitized resource YAML as primary evidence.
   - Supplement with `flux get all -A` and `flux get sources all -A` output if available.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Assess GitRepository source trust**
   Review `spec.ref` type and value, signature verification, and interval:
   ```yaml
   apiVersion: source.toolkit.fluxcd.io/v1
   kind: GitRepository
   metadata:
     name: fleet-infra
     namespace: flux-system
   spec:
     interval: 1m
     url: https://github.com/org/fleet-infra
     ref:
       # SAFE: pinned branch
       branch: main
       # HIGH risk: unbound semver — any tag triggers deploy
       # semver: ">=0.0.0"
     verify:
       # REQUIRED for production: commit GPG signature verification
       mode: HEAD
       secretRef:
         name: pgp-public-keys
   ```
   Absence of `spec.verify.secretRef` on a production source is a HIGH finding.
   `spec.ref.semver: ">=0.0.0"` is a HIGH finding.

4. **Verify SOPS encryption**
   Check whether `Secret` kind manifests exist in the Git repository unencrypted:
   ```bash
   # Find unencrypted Secret manifests in the repo
   grep -rl 'kind: Secret' . | xargs grep -L 'sops:'

   # CORRECT: SOPS-encrypted secret — sops: field present
   apiVersion: v1
   kind: Secret
   metadata:
     name: db-credentials
   sops:
     kms:
       - arn: arn:aws:kms:us-east-1:111122223333:key/...
   data:
     password: ENC[AES256_GCM,data:...,type:str]

   # CRITICAL: plaintext secret committed to Git
   apiVersion: v1
   kind: Secret
   data:
     password: cGFzc3dvcmQ=   # base64 only — trivially decodable
   ```
   Any plaintext `Secret` manifest in a Git source is a CRITICAL finding.

5. **Assess Kustomization ServiceAccount and prune settings**
   ```yaml
   apiVersion: kustomize.toolkit.fluxcd.io/v1
   kind: Kustomization
   metadata:
     name: tenant-a-workloads
     namespace: flux-system
   spec:
     interval: 5m
     path: ./clusters/prod/tenant-a
     prune: true
     # REQUIRED: scoped SA — otherwise kustomize-controller SA (cluster-admin) is used
     serviceAccountName: tenant-a-reconciler
     sourceRef:
       kind: GitRepository
       name: fleet-infra
     healthChecks:
       - apiVersion: apps/v1
         kind: Deployment
         name: api-server
         namespace: tenant-a
   ```
   Missing `serviceAccountName` is a HIGH finding. `prune: true` on a Kustomization covering
   StatefulSets or PVCs without prune-disabled annotations is a HIGH finding.
   Missing `healthChecks` means Flux reports Applied even when Deployments are crash-looping.

6. **Protect stateful resources from prune**
   ```yaml
   # Add this annotation to any resource that must never be pruned
   metadata:
     annotations:
       kustomize.toolkit.fluxcd.io/prune: disabled
   ```
   Review whether CRDs, PVCs, and namespaces containing production databases carry this annotation
   when `spec.prune: true` is set on the parent Kustomization.

7. **Assess HelmRelease version pinning and remediation**
   ```yaml
   apiVersion: helm.toolkit.fluxcd.io/v2
   kind: HelmRelease
   metadata:
     name: nginx-ingress
     namespace: ingress-nginx
   spec:
     interval: 10m
     chart:
       spec:
         chart: ingress-nginx
         # SAFE: pinned version
         version: "4.9.1"
         # HIGH risk: floating version — any new chart triggers auto-upgrade
         # version: "*"
         sourceRef:
           kind: HelmRepository
           name: ingress-nginx
     upgrade:
       remediation:
         # SAFE: bounded retries
         retries: 3
         remediateLastFailure: true
         # MEDIUM risk: infinite retries block reconciliation loops
         # retries: -1
     timeout: 5m
   ```

8. **Check multi-tenant isolation**
   In a multi-tenant Flux setup, each tenant namespace should have a dedicated ServiceAccount
   with scoped RBAC:
   ```yaml
   apiVersion: v1
   kind: ServiceAccount
   metadata:
     name: tenant-a-reconciler
     namespace: tenant-a
   ---
   apiVersion: rbac.authorization.k8s.io/v1
   kind: RoleBinding
   metadata:
     name: tenant-a-reconciler
     namespace: tenant-a
   subjects:
     - kind: ServiceAccount
       name: tenant-a-reconciler
       namespace: tenant-a
   roleRef:
     kind: ClusterRole
     name: edit
     apiGroup: rbac.authorization.k8s.io
   ```
   If all Kustomizations use the default `flux-system/kustomize-controller` SA, any tenant's Git
   source compromise gives cluster-admin-equivalent write to the entire cluster.

9. **Validate Flux health**
   ```bash
   # Check overall Flux reconciliation status
   flux get all -A

   # Check specific Kustomization
   flux get kustomization <name> -n flux-system

   # Check HelmRelease status
   flux get helmrelease <name> -n <namespace>

   # Check GitRepository source
   flux get source git <name> -n flux-system

   # Force reconciliation for testing
   flux reconcile kustomization <name> --with-source

   # Verify commit signature verification config
   kubectl get gitrepository <name> -n flux-system -o jsonpath='{.spec.verify}'
   ```

## Output contract

Return this structure:

```markdown
# FluxCD Kustomization and HelmRelease Review: <scope>

## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:

## Scope and assumptions
- Cluster / namespace:
- Flux version:
- Resources reviewed:
- Confirmed:
- Unknown:
- Out of scope:

## Findings

| Severity | Resource | Field | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|---|

## Source trust summary

| Source name | Kind | Ref type | Commit verification | SOPS enabled |
|---|---|---|---|---|

## Kustomization summary

| Name | Namespace | ServiceAccount | Prune | Health checks |
|---|---|---|---|---|

## HelmRelease summary

| Name | Chart version | Upgrade retries | Timeout |
|---|---|---|---|

## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>

## Validation
- Commands or checks:
- Expected result:

## Residual risk
- <risk or explicit none>
```
