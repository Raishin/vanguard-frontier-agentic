# Workflow and output contract

Use this reference when executing Contabo Object Storage mutations: bucket creation, access policy changes, retention policy updates, object or bucket deletion, or cross-region migration. Every step is mandatory before issuing any destructive call.

## Pre-mutation sequence

1. **Confirm target and inventory**
   - Confirm the Object Storage instance ID and region.
   - List all buckets in the instance via `GET /v1/storage/object-storages` and S3-compatible list commands.
   - For deletion: confirm the full object inventory of the target bucket. Do not delete a bucket described only by name without first listing its contents.
   - Verify OAuth2 token freshness. Tokens expire in ~5 minutes. Refresh immediately before any mutation call.

2. **Review access policy and retention posture**
   - Check bucket ACLs and public access settings. Confirm whether any bucket is publicly readable or writable.
   - Review any lifecycle or retention rules. Confirm whether deletion is blocked by a retention policy.
   - Label the access policy state as `live evidence`, `user-provided sanitized evidence`, or `inference`.

3. **Enforce hard-stop gates for destructive operations**
   - All four gates (bucket ID + region + inventory, backup evidence, rollback plan, named approving identity) must be confirmed in writing before any deletion or irreversible mutation.
   - If any gate is missing, stop and request the specific missing item. Do not infer or assume.

4. **Execute with traceability**
   - Include a fresh UUIDv4 `x-request-id` in all Contabo REST API mutation calls.
   - Use environment-variable-stored S3 access keys for S3-compatible operations. Never hardcode S3 credentials.
   - Log the request ID. Do not log the OAuth2 token, S3 secret key, or access key.

5. **Post-mutation verification**
   - Confirm the bucket or object state after the operation.
   - For deletions: confirm the bucket or objects no longer exist.
   - For access policy changes: verify the new ACL or public access setting is in effect.
   - Record the operation timestamp for audit purposes.

## Output contract

Return this structure:

```markdown
# Contabo Object Storage: <operation> — <bucket name or instance ID>
## Hard-stop gate status (required for destructive operations)
- [ ] Target confirmed: bucket name + region + object inventory reviewed
- [ ] Backup evidence: <location, timestamp, and verification method or "N/A — non-destructive">
- [ ] Rollback plan documented: <recovery path>
- [ ] Named approving identity: <full name or authenticated account identifier>
- [ ] OAuth2 token freshness: confirmed fresh (refreshed at <time>)
## Pre-mutation inventory
- Object Storage instance ID:
- Bucket name:
- Object count and estimated size: <count and size or "not yet queried">
- Access policy state: <public | private | mixed | inference>
- Retention policy: <present | absent | inference>
## Proposed action
- Operation: <create | update-acl | update-retention | delete-objects | delete-bucket | migrate>
- API or S3 call: <sanitized call with x-request-id or endpoint placeholder>
## Post-mutation verification
- Bucket or object state after operation:
- Access policy confirmed:
## Open risks or refusal reason
- <risk or explicit none>
```
