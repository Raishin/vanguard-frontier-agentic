---
name: contabo-live-instance-lifecycle-guard
description: Live-guard skill for Contabo VPS and VDS lifecycle operations including instance creation with product selection and region, reinstallation with image and Cloud-Init userData, and cancellation. Requires mandatory contract period acknowledgment (1, 3, 6, or 12 months), billing impact confirmation, and a rollback plan before any mutation. Hard-stops any lifecycle action that lacks explicit period acknowledgment or rollback documentation.
allowed-tools: Read Grep Glob Bash
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# Contabo Live Instance Lifecycle Guard

## Purpose

Act as the approval gate for Contabo VPS/VDS lifecycle mutations: inventory current state, surface billing obligations, confirm the rollback path, then execute only after explicit user sign-off on all three.

## When to use

Use this skill for:

- VPS or VDS instance creation (product selection, region, image, contract period, Cloud-Init userData, SSH key secret IDs)
- Instance reinstallation (image change, userData update, SSH key rotation)
- Instance cancellation (period remaining, early-termination billing impact)
- Verifying instance inventory before any bulk lifecycle operation
- Generating approval-ready change records with period, billing impact, and rollback plan

## Hard-stop conditions

REFUSE to execute any mutation unless ALL of the following are confirmed in writing:

1. **Target**: Instance ID (for reinstall/cancel) or product ID + region (for create)
2. **Contract period**: Explicit selection of 1, 3, 6, or 12 months with billing impact acknowledged
3. **Rollback plan**: Documented recovery path if the operation fails or produces unexpected results
4. **Named approving identity**: the full name or authenticated account identifier of the person authorizing this operation (not a role, alias, or ticket number alone)

## Lean operating rules

- Contabo has no official Terraform provider or SDK — recommend `cntb` CLI or REST API (curl + jq) for automation.
- Prefer official Contabo docs (https://api.contabo.com/, https://docs.contabo.com/) and Context7 when live MCP access is unavailable.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- OAuth2 password grant tokens expire in ~5 minutes — include token refresh handling in all automation examples. Refresh logic must not log token values.
- Include `x-request-id` (UUIDv4) in all REST API mutation calls for support traceability.
- SSH keys must be referenced via Contabo secret IDs — never include raw private key material in API payloads, scripts, or recommendations.
- Cloud-Init userData submitted by the user must be reviewed for embedded secrets, curl-pipe-sh patterns, and destructive commands before inclusion in the API payload. Flag and refuse to pass userData that contains raw credentials, unauthenticated remote execution, or commands that bypass audit trails.
- Inventory current instances via read-only API calls before proposing any mutation.
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

# Read-only inventory
curl -s \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "x-request-id: $(uuidgen)" \
  'https://api.contabo.com/v1/compute/instances' | jq .
```

## Response minimum

Return, at minimum:

- the target instance or product tier and evidence level,
- the contract period with billing impact,
- the hard-stop checklist status (all three items confirmed or blocked),
- the rollback plan,
- the assumptions or open questions that require user clarification before proceeding.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full lifecycle operation or formatting the approval-ready change record.
- [Safety checklist](references/safety-checklist.md) — use before any VPS or VDS mutation; all hard-stop gates must be confirmed before proceeding.
- [Official sources](references/official-sources.md) — use when grounding Contabo instance lifecycle API behavior, product specifications, or cntb CLI usage.
