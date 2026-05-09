# Workflow and output contract

Use this reference only when performing the full KMS/secret review, implementation guidance, incident triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- KMS key ring inventory: project, region, key ring name, key names, algorithm, protection level (SOFTWARE vs. HSM)
- Key version state: PRIMARY, ENABLED, DISABLED, DESTROY_SCHEDULED, DESTROYED; rotation schedule; last rotated
- CMEK service agent bindings: which GCP services have the CryptoKeyEncrypterDecrypter binding on which keys
- CMEK-dependent resource inventory: Cloud SQL instances, BigQuery datasets, GCS buckets, Compute Engine disks, GKE node pools, Artifact Registry repositories
- Secret Manager: secret names, rotation schedule, last accessed, expiry date, IAM access patterns
- Envelope encryption: DEK/KEK layer clarity, re-encryption status if rotation has occurred
- HSM import plans: wrapping key procedure, supported import methods

## Safe workflow

1. **Frame scope**
   - Project(s) and environment (prod/staging/dev):
   - Compliance framework driving CMEK (PCI, HIPAA, FedRAMP, contractual):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized `gcloud kms keys list --keyring=... --location=...`, `gcloud kms keys describe`, or Terraform state exports.
   - For Secret Manager: `gcloud secrets list`, `gcloud secrets describe` output or IaC.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which CMEK service agent bindings are missing or misconfigured?
   - Which key versions are past their rotation schedule?
   - Which Cloud SQL instances would go down if the CMEK key became inaccessible?
   - Which secrets have no rotation schedule and have not been rotated recently?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - For key operations, require explicit approval and continuity planning before proceeding.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# GCP Secret and KMS Lifecycle Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## KMS key inventory
| Key ring | Key name | Algorithm | Protection | Primary version | Rotation schedule | Last rotated |
|---|---|---|---|---|---|---|
## CMEK dependency map
| Service | Resource | Key ring | Key name | Service agent binding | Status |
|---|---|---|---|---|---|
## Key rotation compliance
| Key name | Schedule | Last rotated | Compliant | Action needed |
|---|---|---|---|---|
## Secret Manager audit
| Secret | Last accessed | Rotation schedule | Expiry | Risk |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
