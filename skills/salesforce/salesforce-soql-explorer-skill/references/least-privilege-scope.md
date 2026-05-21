<!-- Parent: salesforce-soql-explorer-skill/SKILL.md -->
# T1 Least-Privilege Scope — SOQL Explorer

> **verify-before-merge:2026-05-21** — Salesforce permission names, profile
> settings, and Connected App configuration options change across releases.
> Verify each named permission against the current Setup UI or Metadata API
> before deploying. The permission API names used here are correct as of
> Spring '26 <!-- verify-before-merge:2026-05-21 -->.

This document defines the least-privilege boundary for the T1 read-only
operational tier. Every claim here must be verified against a live Salesforce
org before production use. Do not assume parity with legacy sfdx behavior.

---

## OAuth Connected App Configuration

The skill authenticates via a Connected App using the OAuth 2.0 JWT Bearer
or Web Server flow. The Connected App must be configured as follows:

### Permitted OAuth Scopes (check exactly these two)

```
[x] Access and manage your data (api)
[x] Perform requests on your behalf at any time (refresh_token, offline_access)
[ ] Full access (full)                            ← MUST be unchecked
[ ] Web (web)                                     ← MUST be unchecked
[ ] Salesforce Platform API features (sfap_api)   ← MUST be unchecked
[ ] CDP Query API (cdp_query_api)                 ← MUST be unchecked
[ ] Manage user data via APIs (api) + openid      ← not needed; omit
```

Any scope beyond `api` and `refresh_token` is prohibited for T1 skills.

### IP Allowlisting

Set **Permitted Users** to "Admin approved users are pre-authorized" and
configure **IP Relaxation** to "Enforce IP restrictions". Add only the
IP ranges from which the skill runner (CI system, local developer machine,
or agent orchestrator) operates.

Do not set IP Relaxation to "Relax IP restrictions".

### Callback URL

Use a non-production callback URL (e.g., `http://localhost:1717/OauthRedirect`
for JWT flows) or the CI system's callback. Never use a production endpoint
as the callback for a T1 service account.

### Refresh Token Rotation

Enable **Refresh Token Rotation** in the Connected App OAuth settings.
This ensures that each token refresh issues a new refresh token and
invalidates the previous one — a prerequisite for the revocation model
described below.

---

## Run As Service Account — Profile Design

The "Run As" account is a dedicated Salesforce user (not a named human user)
whose profile and permission sets define the T1 access boundary.

### System Permissions — REQUIRED

```
[x] View Setup and Configuration
    (API: ViewSetup)
    Required for: sf org display, sobject describe, tooling API queries
```

No other system permissions are required for T1 SOQL exploration. Do not
grant any system permission not listed here.

### System Permissions — EXPLICITLY DENIED

These permissions must be absent from the Run As account's profile AND from
any permission set assigned to the account. Verify via Setup > Users >
[Run As User] > View Summary.

```
[ ] Modify All Data        (API: ModifyAllData)
[ ] View All Data          (API: ViewAllData)          ← system bypass, not record access
[ ] View Encrypted Data    (API: ViewEncryptedData)
[ ] Modify Metadata Through Metadata API Functions
                           (API: ModifyMetadata)
[ ] Author Apex            (API: AuthorApex)
[ ] Customize Application  (API: CustomizeApplication)
[ ] Manage Connected Apps  (API: ManageConnectedApps)
[ ] API Enabled            ← Wait — this IS required for CLI access. See note below.
```

> **Note on API Enabled:** The Run As account must have `API Enabled` checked
> (required for any CLI or API access). All other API-adjacent permissions
> (ModifyAllData, ViewAllData, etc.) must still be denied. `API Enabled`
> alone grants only the ability to make authenticated API calls subject
> to standard sharing and FLS — it does not bypass sharing or FLS.

### Object Permissions — Per-Object Read Only

For each sObject in scope:

```
[x] Read
[ ] Create
[ ] Edit
[ ] Delete
[ ] View All    ← MUST be unchecked (bypasses sharing)
[ ] Modify All  ← MUST be unchecked
```

Grant Read access only on the specific objects the skill will query. Do not
grant Read on all objects by default — enumerate the scope explicitly per
matter or engagement.

### Field-Level Security (FLS)

For each field the skill may query:

```
[x] Read
[ ] Edit
```

FLS must be explicitly configured. The T1 Run As account must not have
Read access to:
- Fields marked as encrypted (Shield PE / PMLE)
- Fields containing PII (email, phone, SSN, health data, financial account
  numbers) unless the matter explicitly requires it and the field is not
  encrypted
- Fields in shadow objects or reporting snapshots unless the matter requires

Use the `sf sobject describe` output (see `cli-commands.md`) to verify
which fields the Run As account can actually read before constructing queries.

---

## IP Allowlisting at the Connected App Level

Configure the Connected App's IP restrictions to match the expected source
ranges for the skill runner:

| Environment | Expected Source |
|---|---|
| Local developer | Developer machine IP or VPN exit node |
| CI/CD pipeline | CI runner IP range (static or NAT gateway) |
| Agent orchestrator | Orchestrator cluster egress IP range |

Review and update IP allowlists quarterly or when infrastructure changes.
Do not use `0.0.0.0/0`.

---

## Refresh Token Rotation and Revocation

### Rotation cadence

Rotate the Run As account's refresh token:
- At minimum every 90 days
- Immediately upon any suspected compromise
- When the Run As account's org is decommissioned

### Revocation procedure

1. Log in to the Salesforce org as an administrator.
2. Navigate to Setup > Connected Apps > [App Name] > Manage > OAuth Usage.
3. Find the Run As account's active token and revoke it.
4. Alternatively, reset the Run As user's security token via Setup > Users >
   [Run As User] > Reset Security Token.
5. Confirm the skill can no longer authenticate by running `sf org display --target-org <alias>`.

Rotating the refresh token immediately invalidates all active sessions for
the Run As account without affecting any other user or integration.

---

## Audit Trail Enablement

Enable the following in the target Salesforce org to support the T1 audit model:

1. **Setup Audit Trail** — automatically enabled in all orgs. Captures
   metadata and setup changes. The Run As account's actions appear here.
2. **Event Monitoring** (add-on, if licensed <!-- verify-before-merge:2026-05-21 -->)
   — captures API query events, login events, and data export events.
   Strongly recommended for production-adjacent T1 use.
3. **Platform Event logging** — if the org uses Platform Events, confirm
   the Run As account cannot publish events (no Create permission on
   Platform Event objects).

The skill's audit envelope (see `SKILL.md`) is a local record only. The
org-side audit trail is the authoritative log for compliance purposes.

---

## Org Allowlist Verification

Before executing any query, the skill calls `sf org list --connected --json`
and verifies that the `--target-org` alias appears in the list. If the alias
is not in the list:

1. The skill does not attempt to authenticate.
2. The skill emits a refusal with reason `alias_not_authorized`.
3. The skill suggests running `sf org login web --alias <alias>` or
   `sf org login jwt --alias <alias>` as the appropriate remediation.

The Connected App allowlist (Permitted Users + IP restrictions) is the
enforcement layer. The `sf org list` check is a soft pre-flight that
reduces unnecessary authentication failures.

---

## Verify-Before-Merge Tags in This Document

The following items in this file must be re-verified against the live
Salesforce documentation before merging to main:

- Permission API names (`ModifyAllData`, `ViewAllData`, `ViewEncryptedData`,
  `ModifyMetadata`, `AuthorApex`, `CustomizeApplication`, `ManageConnectedApps`,
  `ViewSetup`) — confirm these match current API names in Spring '26 or later.
- Connected App OAuth scope labels — Salesforce has renamed scopes in past
  releases; confirm `api` and `refresh_token` (offline_access) are the
  correct scope identifiers.
- Event Monitoring availability and licensing model.
- `Refresh Token Rotation` Connected App option — confirm it is available
  in the org edition being targeted.
- Health Cloud and Financial Services Cloud as regulated-vertical indicators
  — confirm both are still the primary regulated cloud products.
