# Official sources

Use this reference only when you need source grounding for GCP KMS and Secret Manager service behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:

- https://cloud.google.com/kms/docs/key-management-service — Cloud KMS overview, key hierarchy, key rings, and key versions
- https://cloud.google.com/kms/docs/cmek — CMEK overview, service agent binding requirements per GCP service
- https://cloud.google.com/secret-manager/docs/overview — Secret Manager overview, rotation, access, and audit logging
- https://cloud.google.com/kms/docs/key-rotation — Key rotation behavior, auto-rotation schedules, and re-encryption guidance
- https://cloud.google.com/kms/docs/importing-a-key — HSM key import wrapping procedure and supported key types
- https://cloud.google.com/kms/docs/destroy-restore — Key version destruction and restoration windows
- https://cloud.google.com/secret-manager/docs/rotation-schedule — Secret Manager automatic rotation with Pub/Sub notifications
- https://cloud.google.com/sql/docs/mysql/cmek — Cloud SQL CMEK configuration, service agent binding, and continuity implications

## Grounding rule

Official documentation explains GCP service behavior. It does not prove the user's current key rotation status, CMEK binding state, secret expiry configuration, or Cloud SQL dependency. Prefer sanitized user-provided evidence (`gcloud kms keys describe`, `gcloud kms keys list`, Terraform state, or Secret Manager metadata exports) for current-state claims. Never infer production key state from documentation alone.
