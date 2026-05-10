---
name: contabo-live-storage-operations-guard
description: Live-guard skill for Contabo Object Storage (S3-compatible) bucket operations including inventory audit, access policy review, retention policy enforcement, and deletion workflows. Hard-stops any bucket deletion requested without verified backup evidence and a documented rollback plan. Use when the user needs to manage, audit, or delete Contabo Object Storage buckets or objects.
allowed-tools: Read Grep Glob Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# Contabo Live Storage Operations Guard

## Purpose

Act as the approval gate for Contabo Object Storage mutations: audit current bucket inventory, access policies, and retention posture, then execute destructive operations only after verified backup evidence and explicit user sign-off.

## When to use

Use this skill for:

- Contabo Object Storage bucket inventory and object listing
- Access policy review (bucket ACLs, public access exposure)
- Retention policy enforcement and lifecycle rule audit
- Bucket or object deletion with backup verification gate
- Migration or consolidation of Object Storage across regions
- Generating approval-ready change records for storage mutations

## Hard-stop conditions

REFUSE to execute any bucket deletion or destructive Object Storage mutation unless ALL of the following are confirmed in writing:

1. **Target**: Bucket name and full inventory of current objects or confirmed backup location
2. **Backup evidence**: Verified backup of all data to be deleted (location, timestamp, verification method)
3. **Rollback plan**: Documented recovery path if the operation produces unexpected results

## Lean operating rules

- Contabo has no official Terraform provider or SDK — recommend `cntb` CLI or REST API (curl + jq) for automation.
- For S3-compatible Object Storage operations, use S3-compatible tools (aws CLI with `--endpoint-url` pointing at the Contabo Object Storage endpoint).
- Prefer official Contabo docs (https://api.contabo.com/, https://docs.contabo.com/) and Context7 when live MCP access is unavailable.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- OAuth2 password grant tokens expire in ~5 minutes — include token refresh handling in all automation examples. Refresh logic must not log token values.
- Include `x-request-id` (UUIDv4) in all Contabo REST API calls for support traceability.
- S3 access key and secret key for Object Storage API must be stored as environment variables, never hardcoded.
- Inventory current buckets and objects via read-only calls before proposing any mutation.
- Label claims as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Automation pattern (read-only inventory first)

```bash
# Load credentials from environment — never hardcode
: "${CONTABO_CLIENT_ID:?set in env}"
: "${CONTABO_CLIENT_SECRET:?set in env}"
: "${CONTABO_API_USER:?set in env}"
: "${CONTABO_API_PASSWORD:?set in env}"

# Refresh token before each operation
TOKEN=$(curl -s \
  -d "client_id=${CONTABO_CLIENT_ID}" \
  -d "client_secret=${CONTABO_CLIENT_SECRET}" \
  --data-urlencode "username=${CONTABO_API_USER}" \
  --data-urlencode "password=${CONTABO_API_PASSWORD}" \
  -d 'grant_type=password' \
  'https://auth.contabo.com/auth/realms/contabo/protocol/openid-connect/token' \
  | jq -r '.access_token')

# List Object Storage instances (read-only)
curl -s \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "x-request-id: $(uuidgen)" \
  'https://api.contabo.com/v1/storage/object-storages' | jq .
```

## Response minimum

Return, at minimum:

- the target bucket(s) and object inventory evidence level,
- the access policy and retention posture assessment,
- the hard-stop checklist status (all three items confirmed or blocked),
- the rollback plan,
- the assumptions or open questions that require user clarification before proceeding.
