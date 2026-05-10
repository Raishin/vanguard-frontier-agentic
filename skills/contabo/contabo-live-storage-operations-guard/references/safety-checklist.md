# Safety checklist

Before executing any Contabo Object Storage mutation (bucket deletion, object deletion, access policy change, retention policy change, cross-region migration), enforce every item on this checklist. Never proceed without all mandatory gates confirmed for destructive operations.

## Hard-stop gates — all required for deletion and irreversible mutations

Do not execute any bucket deletion, bulk object deletion, or irreversible mutation unless ALL of the following are confirmed in writing by the user:

1. **Target confirmed with inventory**: Bucket name AND region AND full object inventory reviewed. Do not delete a bucket based on name alone — confirm object count and verify there is nothing unexpected.
2. **Backup evidence verified**: Confirmed backup of all data to be deleted, including backup location, timestamp, and verification method (e.g., restored successfully to staging, checksum matched). "I think we have a backup" is not verification.
3. **Rollback plan documented**: A concrete data recovery path is on record. For object deletion: where the backup lives and how to restore. For bucket deletion: whether recreation is possible and what configuration is required.
4. **Named approving identity on record**: The full name or authenticated account identifier of the authorizing person. A role title, team name, ticket number, or alias alone is not sufficient.
5. **Retention audit complete**: Confirm no retention or compliance policy blocks deletion. Deleting objects under an active retention lock may be a compliance violation.

## Non-negotiables

- Do not execute bucket deletion if the object inventory has not been reviewed in this session. A prior audit is not sufficient — inventory must be confirmed fresh.
- Do not treat bucket deletion as reversible. Deleted objects in Contabo Object Storage cannot be recovered without a verified external backup.
- Do not recommend making a bucket publicly accessible without explicit user acknowledgment of the data exposure risk and the specific use case requiring public access.
- S3 access keys and secret keys for Object Storage must be stored in environment variables. Never hardcode, echo, or include them in any output, log file, or script.
- Do not log, echo, or include OAuth2 token values in any output, log file, or script.
- Include a fresh UUIDv4 `x-request-id` header in every Contabo REST API mutation call.
- For S3-compatible operations, use `aws s3` or `aws s3api` with `--endpoint-url` pointing to the Contabo Object Storage endpoint — never against the default AWS endpoint.

## Mandatory posture

- Prefer read-only inventory first. Always call `GET /v1/storage/object-storages` and list the target bucket contents before any deletion or mutation.
- Prefer the least-destructive operation. If an access policy change achieves the goal without deletion, recommend that first.
- Treat missing backup evidence as a hard blocker, not a detail to resolve after deletion.
- If any hard-stop gate is missing, stop completely and list exactly which gates remain open. Do not proceed partially.
- If public access is about to be enabled, surface the full data exposure risk explicitly before the user confirms.

## Stress checks

- Is there data in this bucket that is not captured in the confirmed backup? → Stop until resolved.
- Is any object in this bucket subject to a compliance retention policy? → Confirm legality of deletion before proceeding.
- Is the access policy change exposing sensitive data publicly? → Require explicit use-case justification.
- What is the recovery time if the backup restore is needed? Is it within the user's acceptable window?
- Is the S3 endpoint URL correct for the Contabo region, not a default AWS endpoint?
- Is the OAuth2 token fresh enough for the full sequence of API calls required?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Never proceed with a destructive bucket operation based on inference about the object inventory or backup state.
