# Network Policy Reference

Reference for Salesforce network access controls including IP allowlists,
login IP ranges, CSP Trusted Sites, and connected app network policies.

---

## IP Restriction Layers in Salesforce

Salesforce provides multiple independently configurable IP restriction layers.
They are not equivalent and must all be reviewed.

| Layer | Where Configured | Scope | Enforcement Point |
|-------|-----------------|-------|------------------|
| Org-wide trusted IP ranges | Setup > Network Access | All users | Login block if not in range |
| Profile login IP ranges | Setup > Profiles > Login IP Ranges | Users on that profile | Login block |
| Connected App IP restrictions | Setup > Connected Apps > [App] > IP Ranges | OAuth API sessions for that app | API call block |
| Named Credential IP | Not configurable at Named Credential level | N/A | Controlled by callout destination |

---

## Org-Wide Trusted IP Ranges

Path: Setup > Security > Network Access

Trusted IP ranges affect the SMS/email verification challenge. If a user logs
in from a non-trusted IP, Salesforce sends a verification challenge. Trusted
ranges bypass this challenge.

```
Recommended: Do NOT add overly broad ranges such as 0.0.0.0-255.255.255.255
This disables the verification challenge for all users globally.

Acceptable: Office IP ranges, VPN egress IPs, CI/CD pipeline IPs.
Review: Any entry covering a /8 or /16 subnet needs justification.
```

### Query trusted IP ranges via Metadata API
```bash
sf org retrieve metadata \
  --metadata NetworkAccess \
  -o my-org \
  --target-dir /tmp/network-policy/
cat /tmp/network-policy/force-app/main/default/networkAccess/NetworkAccess.networkAccess-meta.xml
```

### Audit script (anonymous Apex)
```apex
// Network access ranges cannot be queried via Apex — use Metadata API
// as shown above or review via Setup UI.
// However, you can inspect Profile-level login IP ranges:
for (Profile p : [SELECT Id, Name FROM Profile WHERE UserLicense.Name != 'Guest']) {
    System.debug('Profile: ' + p.Name + ' | Id: ' + p.Id);
}
// Then use Metadata API to extract LoginIpRanges per profile
```

---

## Profile Login IP Ranges

Path: Setup > Profiles > [Profile Name] > Login IP Ranges

Login IP ranges restrict which IPs a user on that profile can log in from.
Profiles with no login IP ranges configured allow login from any IP (subject
to trusted IP challenge for untrusted IPs).

### High-Risk Profile Findings

| Finding | Risk Level |
|---------|-----------|
| System Administrator profile with no login IP range | HIGH |
| Integration user profile with no login IP range | HIGH |
| Integration user profile with IP range 0.0.0.0/0 | CRITICAL |
| Guest User profile with login IP range (misconfigured — Guest login not IP restricted at profile level) | MEDIUM |

### Recommended Login IP Ranges by Profile Type

| Profile Type | Recommended Range |
|-------------|------------------|
| System Administrator | Specific corporate IPs + VPN egress only |
| Integration User | Middleware server IPs only (no user interactive login) |
| Standard internal user | Corporate VPN range acceptable |
| External Community user | No restriction (users are globally distributed) |
| Guest User | No effective restriction via login IP ranges |

---

## CSP Trusted Sites

Path: Setup > Security > CSP Trusted Sites

Salesforce enforces a Content Security Policy on all Lightning pages. External
resources (images, scripts, fonts, API endpoints, WebSockets) must be listed
in CSP Trusted Sites or the browser will block them.

### CSP Directive Mapping in Salesforce

| CSP Directive | Controls |
|---------------|----------|
| `connect-src` | XHR/fetch API calls, WebSocket connections |
| `img-src` | Images loaded from external URLs |
| `style-src` | External stylesheets |
| `font-src` | External fonts |
| `frame-src` | Embedded iframes |
| `script-src` | External scripts (Salesforce blocks most external scripts by default) |

### CSP Trusted Site Audit Checklist

- [ ] No wildcard domains (`*.example.com` acceptable; `*` or `*.com` is a finding).
- [ ] HTTPS enforced for all entries (no `http://` in CSP Trusted Sites).
- [ ] Entries for localhost or internal staging domains removed before production.
- [ ] `frame-src` includes only explicitly required embedding origins.
- [ ] Review annually; remove origins for decommissioned integrations.

### Retrieve CSP Trusted Sites via SOQL
```sql
SELECT Id, EndpointUrl, IsActive, Context, Description
FROM CspTrustedSite
WHERE IsActive = true
ORDER BY EndpointUrl
```

---

## Session Security Settings

Path: Setup > Security > Session Settings

### Critical Settings to Review

| Setting | Secure Configuration | Risk if Misconfigured |
|---------|---------------------|----------------------|
| Session Timeout | 15 min (admin), 2-8 hours (standard) | Longer = session hijacking risk |
| Lock sessions to IP | Enabled | Prevents session token reuse from different IP |
| Lock sessions to domain | Enabled | Prevents cookie leakage across subdomains |
| Force logout on session timeout | Enabled | Ensures clean session termination |
| Clickjack Protection | Enabled for all pages | Prevents UI redress attacks |
| Content Sniff Protection | Enabled | Prevents MIME-type sniffing |
| HSTS | Enabled | Prevents SSL-stripping attacks |
| XSS Protection | Enabled | Browser-level XSS mitigation |
| Require HTTPS | Enabled | Prevents plaintext session cookies |

### High Assurance Session Requirements

High Assurance sessions are required before accessing certain sensitive areas
(certificates, Auth. Providers configuration, Connected App settings).

Configure which operations require High Assurance:
Path: Setup > Security > Session Settings > Session Security Levels

Recommended High Assurance operations:
- Manage Users
- Manage Connected Apps
- Manage Auth. Providers
- Manage Remote Sites
- Manage Certificates

---

## Connected App Network Controls

For each Connected App:
- Set IP Relaxation to "Enforce IP restrictions" (not "Relax IP restrictions").
- OAuth Token Timeout: set to minimum required for the integration use case.
- Review Callback URL — should be HTTPS only.

```sql
SELECT Id, Name, OptionsAllowAdminApprovedUsersOnly,
       MobileStartUrl, StartUrl
FROM ConnectedApplication
ORDER BY Name
```

Detailed OAuth policy review requires Metadata API retrieval of `ConnectedApp`
metadata type.

---

## Remote Site Settings

Path: Setup > Security > Remote Site Settings

Remote Site Settings control which external URLs Apex code and Visualforce
can make HTTP callouts to.

```sql
SELECT Id, EndpointUrl, IsActive, Description, DisableProtocolSecurity
FROM RemoteProxy
WHERE IsActive = true
ORDER BY EndpointUrl
```

**HIGH finding:** Any record where `DisableProtocolSecurity = true`.
This disables SSL certificate verification for that endpoint — equivalent to
`TrustManager.TRUST_ALL` in Java.

**Review:** All HTTP (non-HTTPS) endpoints. All endpoints on the pattern
`http://localhost` or `http://127.0.0.1` (only acceptable in developer orgs
for local Salesforce Functions development).
