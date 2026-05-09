# IAM Permissions — GCP Live KMS Key Destruction Guard

## Minimum Read Roles (Discovery and Audit)

| Role | Purpose |
|------|---------|
| `roles/cloudkms.viewer` | View key rings, keys, key versions, and their state without mutation rights |
| `roles/cloudkms.cryptoOperator` | Inspect key version metadata and usage; cannot destroy or create keys |

## Required Mutation Roles (Live Operations)

| Role | Purpose | Scope Guidance |
|------|---------|---------------|
| `roles/cloudkms.admin` | Schedule key version destruction, restore pending-destruction versions, delete key rings | Bind at the specific project level only; never at org or folder level |

## Narrowing Guidance

- Bind `roles/cloudkms.admin` to a dedicated break-glass service account — never to developer or CI/CD service accounts.
- For read-only CMEK dependency audits and key version inspection, `roles/cloudkms.viewer` is sufficient.
- Consider using `roles/cloudkms.cryptoKeyVersionsDeleter` as a narrower alternative to `roles/cloudkms.admin` if destruction is the only required mutation.
- Implement 4-eyes approval (two-person integrity) for any key version destruction using a custom IAM approval workflow or Google Cloud Assured Workloads controls.
- Enable `constraints/cloudkms.allowedProtectionLevels` org policy to restrict key creation to hardware-backed (HSM) keys in sensitive environments.

## Anti-Patterns — Never Grant

- `roles/owner` — grants full resource control; never appropriate for a KMS guard agent.
- `roles/editor` — includes KMS admin capabilities without the explicit intent.
- `roles/cloudkms.admin` at org or folder level — blast radius is every KMS key ring in the hierarchy.
- `roles/cloudkms.cryptoKeyEncrypterDecrypter` on the key — grants access to decrypt data, not just manage key lifecycle; separate from destruction authority.
- Granting destruction rights to the same identity that owns the encrypted data — violates separation of duties.

## Audit Trail

All KMS key lifecycle operations (destroy, restore, create) are logged in **Cloud Audit Logs** under `cloudkms.googleapis.com`. Ensure `ADMIN_WRITE` audit log types are enabled for the KMS service. Export to long-term storage — key destruction events are forensically critical and must be retained beyond the default 400-day window.
