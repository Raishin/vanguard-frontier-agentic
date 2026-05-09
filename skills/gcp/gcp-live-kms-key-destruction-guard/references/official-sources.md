# Official Sources — GCP Live KMS Key Destruction Guard

Authoritative GCP documentation for Cloud KMS key lifecycle, CMEK, and key rotation operations.

## Core References

- **Destroying and Restoring Key Versions** — https://cloud.google.com/kms/docs/destroy-restore
  How to schedule key version destruction, the minimum 24-hour pending period, and how to restore a version before destruction completes.

- **Customer-Managed Encryption Keys (CMEK)** — https://cloud.google.com/kms/docs/cmek
  Overview of CMEK integration across GCP services: Cloud SQL, GCS, BigQuery, Compute Engine, and Secret Manager.

- **Key Rotation** — https://cloud.google.com/kms/docs/key-rotation
  How automatic and manual key rotation works, and why rotation is a non-destructive alternative to key version destruction.

- **Cloud KMS Key Hierarchy** — https://cloud.google.com/kms/docs/key-hierarchy
  Understanding key rings, keys, key versions, and the relationship between them.

- **Key Ring and Key Management** — https://cloud.google.com/kms/docs/creating-keys
  Creating and managing key rings and keys; restrictions on key ring deletion when keys have active versions.

- **Cloud KMS Audit Logging** — https://cloud.google.com/kms/docs/logging
  What KMS operations are logged in Cloud Audit Logs and how to enable comprehensive logging for compliance.

- **CMEK Org Policies** — https://cloud.google.com/compute/docs/disks/customer-managed-encryption#org_policy
  Organization Policy constraints for enforcing CMEK usage across projects and services.

- **Key Access Justifications** — https://cloud.google.com/assured-workloads/key-access-justifications/docs/overview
  Advanced controls for understanding and approving every key access request in regulated environments.
