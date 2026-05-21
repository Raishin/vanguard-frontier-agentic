# Session Policy Reference

Reference for Salesforce session security configuration including timeout values,
clickjack protection, high-assurance session requirements, and HTTPS enforcement.

---

## Session Settings Location

Path: Setup > Security > Session Settings

All settings in this section apply org-wide unless overridden at the connected
app or profile level.

---

## Timeout Configuration

### Session Timeout Values

Salesforce allows timeout values from 15 minutes to 24 hours.

| User Type | Recommended Timeout | Maximum Acceptable |
|-----------|--------------------|--------------------|
| System Administrator | 15 minutes | 30 minutes |
| Integration API user | N/A (use server-to-server OAuth, not interactive session) | N/A |
| Standard internal user | 2 hours | 8 hours |
| External Community user | 2 hours | 12 hours |
| Guest User | 15 minutes (read-only context) | 30 minutes |

**Finding:** Org-wide session timeout > 8 hours is a MEDIUM finding.
**Finding:** System Administrator session timeout > 30 minutes is a HIGH finding.

### Timeout Behavior Settings

| Setting | Recommended Value |
|---------|------------------|
| Timeout Action | Lock (not Logout, to preserve user work) |
| Force Logout on Timeout | Enabled for Admin profiles |
| Disable Session Timeout Warning Popup | Disabled (users should see warning) |

---

## Lock Sessions Settings

### Lock Sessions to IP Address

When enabled, a session token is bound to the IP address used at login. The
session is invalidated if a request arrives from a different IP with the same
token.

**Recommended:** Enabled for all user types.

**Exception:** Mobile users on carrier networks may have rotating IPs. For mobile
use cases, evaluate the trade-off between security and usability.

**Setting location:** Session Settings > Lock sessions to the IP address from
which they originated.

### Lock Sessions to Domain

When enabled, session cookies are bound to the specific Salesforce subdomain.
Cross-subdomain cookie reuse is blocked.

**Recommended:** Enabled.

---

## Clickjack Protection

Clickjack (UI Redress) attacks embed Salesforce pages in an iframe on an
attacker-controlled page and trick users into clicking buttons they cannot see.

### Protection Levels


| Setting | Description | Recommendation |
|---------|-------------|----------------|
| Allow framing by any page | No protection | Never use in production |
| Allow framing by the same origin only | Allows same-domain embedding | Minimum |
| Don't allow framing by any page | Strict X-Frame-Options: DENY | Recommended |

Path: Setup > Security > Session Settings > Clickjack Protection Level

**Additional settings to enable:**
- Enable clickjack protection for non-setup Salesforce pages (standard pages)
- Enable clickjack protection for setup Salesforce pages

**Visualforce clickjack protection:**
Each Visualforce page can override with the `showHeader` attribute interaction
with `<apex:page>`. Review VF pages that embed external content.

---

## HTTPS Enforcement

### Require Secure Connections (HTTPS)

Path: Setup > Security > Session Settings > Require secure connections (HTTPS)

**Recommended:** Enabled. This setting forces all Salesforce traffic over HTTPS.

### HTTP Strict Transport Security (HSTS)

HSTS instructs browsers to only connect to Salesforce over HTTPS for a defined
period, preventing SSL-stripping attacks.

Salesforce enables HSTS by default on all production orgs. Verify it is not
disabled in custom domain configurations.

**Custom domain HSTS verification:**
```bash
curl -I https://yourcustomdomain.my.salesforce.com 2>/dev/null | grep -i strict
# Expected: strict-transport-security: max-age=31536000; includeSubDomains
```

---

## High-Assurance Session Requirements

A High Assurance session requires the user to authenticate with a stronger
method (MFA hardware token, certificate) before accessing sensitive areas.

### How High Assurance Works

Salesforce defines two session security levels:
- **Standard:** Regular username/password or SSO.
- **High Assurance:** MFA required (hardware key, Salesforce Authenticator app,
  TOTP authenticator).

### Required High Assurance Operations (Recommended)

Path: Setup > Security > Session Settings > Session Security Levels

| Operation | Risk If Not High Assurance |
|-----------|--------------------------|
| Manage Users | Account takeover via compromised admin session |
| Manage Connected Apps | OAuth token theft |
| Manage Auth. Providers | Identity provider tampering |
| Manage Certificates | PKI compromise |
| Manage Remote Sites | Add malicious callout targets |
| View Setup Audit Trail | Audit log access |
| Manage Encryption | Shield encryption key access |

### Assigning High Assurance to Permission Sets

You can require High Assurance for any Permission Set:
```
Setup > Permission Sets > [Set] > Session Activation Required = High Assurance
```

Users who activate this permission set in a Standard session will be prompted
to step up to High Assurance.

---

## OAuth Token Security Settings

Path: Setup > Security > OAuth and OpenID Connect Settings

| Setting | Secure Value |
|---------|-------------|
| Allow OAuth Username-Password Flows | Disabled (deprecated, no MFA support) |
| Allow OAuth User-Agent Flows | Disabled (implicit flow deprecated in OAuth 2.1) |
| Token Expiration for web apps | 2 hours maximum |
| Refresh Token Policy | Expire on first use or set fixed expiry |

**HIGH finding:** OAuth Username-Password Flow enabled in production.
This flow transmits credentials in the request body and bypasses MFA.

---

## Audit Settings

### Login History Retention

Salesforce retains login history for 6 months. For compliance requirements
beyond 6 months, export and store in a SIEM.

```sql
SELECT UserId, LoginTime, LoginType, LoginUrl, SourceIp,
       Status, Application, Browser, Platform
FROM LoginHistory
WHERE LoginTime = LAST_N_DAYS:90
ORDER BY LoginTime DESC
LIMIT 1000
```

### Setup Audit Trail

Path: Setup > Security > View Setup Audit Trail

Salesforce retains the Setup Audit Trail for 180 days. For compliance, export
regularly.

```bash
sf data query \
  --query "SELECT CreatedDate, CreatedByUser, Action, Section, Display \
           FROM SetupAuditTrail \
           ORDER BY CreatedDate DESC \
           LIMIT 2000" \
  -o my-org \
  --result-format csv > audit-trail.csv
```

---

## Session Security Review Checklist

- [ ] Session timeout <= 8 hours for standard users, <= 30 min for admins.
- [ ] Lock sessions to IP: Enabled.
- [ ] Lock sessions to domain: Enabled.
- [ ] Clickjack protection: Enabled for all pages.
- [ ] HTTPS required: Enabled.
- [ ] High Assurance required for: Manage Users, Connected Apps, Auth. Providers.
- [ ] OAuth Username-Password Flow: Disabled.
- [ ] OAuth User-Agent Flow (implicit): Disabled.
- [ ] Login history exported to SIEM if retention > 6 months required.
- [ ] MFA enforced via profile or org-wide policy.
