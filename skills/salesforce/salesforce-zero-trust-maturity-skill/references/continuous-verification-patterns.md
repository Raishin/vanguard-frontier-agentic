# Continuous Verification Patterns Reference

Patterns for implementing continuous identity and session verification
in Salesforce to support zero trust principles.

---

## What Continuous Verification Means

In a Zero Trust model, authentication is not a one-time gate at login.
It is continuous:
- Session validity is re-checked at sensitive operations.
- Risk signals (new device, new location, anomalous behavior) trigger
  re-authentication or session downgrade.
- Tokens are short-lived and rotated.

---

## Adaptive Authentication Patterns

### Pattern 1: High-Assurance Session Gates

Salesforce's High Assurance session mechanism is the native implementation
of adaptive authentication. Users in a Standard session are challenged to
step up to High Assurance before accessing sensitive operations.

```
User logs in with SSO (Standard session)
  -> Accesses Salesforce CRM, creates Cases, updates Contacts (Standard session OK)
  -> Attempts to navigate to Setup > Connected Apps (High Assurance required)
     -> Salesforce presents MFA challenge
     -> User passes MFA
     -> Session elevated to High Assurance for 1 hour
  -> High Assurance session expires
     -> Next access to sensitive operation triggers re-challenge
```

**Configuring High Assurance requirements:**
```
Path: Setup > Security > Session Settings > Session Security Levels
Map permission to High Assurance level:
  - Manage Users
  - Manage Connected Apps
  - Manage Certificates
  - View Encrypted Data
  - Manage Remote Sites
```

### Pattern 2: Per-Permission-Set High Assurance

Require step-up authentication when a user activates a specific Permission Set:

```
Setup > Permission Sets > [Sensitive Permission Set]
  Session Activation Required = High Assurance
```

Users who try to exercise permissions in this set during a Standard session
are prompted to step up before the Permission Set activates.

---

## Contextual Risk Signals in Salesforce

Salesforce cannot natively evaluate all risk signals (device posture, geolocation
velocity) without integration with an external IdP or CASB. Here is the split:

| Risk Signal | Available in Salesforce Native | Requires Integration |
|------------|-------------------------------|---------------------|
| Login from new IP | Via LoginHistory monitoring | SIEM alert only |
| Login from new device | Salesforce Device Activation challenge | Partial native |
| Login outside normal hours | Login IP/hour restrictions on profile | SIEM for anomaly |
| Credential compromise | No | IdP or CASB required |
| Device compliance failure | No | MDM + IdP required |
| MFA bypass attempt | Via Failed LoginHistory | SIEM alert |
| Rapid record access (possible bot) | Event Monitoring | SIEM or CASB |

### Device Activation (Native)

When a user logs in from an unrecognized device, Salesforce sends a
verification email or SMS. This is a lightweight form of device signal.

```
Path: Setup > Security > Session Settings
Setting: Require Email Confirmation for Device Activation = Enabled
```

---

## Token Rotation Patterns

### OAuth Refresh Token Rotation

For connected apps using Authorization Code + refresh token flow:

```
Best practice OAuth token lifecycle:
1. Issue access token (short-lived: 15 minutes to 2 hours)
2. Issue refresh token (long-lived: configurable)
3. When access token expires: client uses refresh token to get new access token
4. Rotate: issue new refresh token with each token refresh (invalidate old)
5. If refresh token not used within expiry: force re-authentication
```

**Salesforce Connected App token policy:**
```
Setup > Apps > Connected Apps > [App] > OAuth Policies
  Access Token Valid For: 2 hours (recommended maximum)
  Refresh Token Policy: Expire refresh token after: 90 days
    OR: Expire refresh token if not used for: 30 days
```

**Rotate on use pattern** (most secure):
```
Refresh Token Policy: Immediately expire old refresh token
```
This means every token refresh issues a new refresh token and invalidates
the previous one — replay of captured refresh tokens fails.

### JWT Bearer Flow (No Refresh Token)

For server-to-server integrations, the JWT Bearer flow is preferred over
refresh tokens:
- No persistent token to steal.
- Short-lived JWT (max 5 minute validity window).
- Authentication re-occurs on every access token request.

```python
# JWT Bearer flow: no persistent token stored
import jwt, time, requests

def get_salesforce_token(client_id, private_key, username, audience):
    payload = {
        'iss': client_id,
        'sub': username,
        'aud': audience,
        'exp': int(time.time) + 300  # 5 minute validity
    }
    assertion = jwt.encode(payload, private_key, algorithm='RS256')

    response = requests.post(f'{audience}/services/oauth2/token', data={
        'grant_type': 'urn:ietf:params:oauth:grant-type:jwt-bearer',
        'assertion': assertion
    })
    return response.json['access_token']
    # Call this function each time a token is needed — no caching
```

---

## Event Monitoring for Continuous Verification

Event Monitoring
(a Salesforce Shield or add-on product)
provides real-time and log-based access to user activity events.

### Key Event Types for ZTA Monitoring

| Event Type | API Name | What to Alert On |
|------------|----------|-----------------|
| Login | `LoginEvent` | Failed logins > 5 per hour per user |
| Logout | `LogoutEvent` | Unexpected logouts (session hijack indicator) |
| API calls | `ApiEvent` | Volume spikes, unusual endpoints |
| Report export | `ReportExportEvent` | Any export by non-authorized user |
| List view access | `ListViewEvent` | Bulk queries on sensitive objects |
| Permission change | `PermissionSetAssignmentEvent` | Any assignment to high-privilege sets |
| SOQL query | `QueryEvent` | Queries touching regulated fields |
| Field history | `FieldHistoryEvent` | Changes to PII or financial fields |

### Real-Time Event Monitoring (Streaming API)

```apex
// Subscribe to real-time events via Salesforce Streaming API
// Example: Monitor permission set assignments as they happen
// Subscribe to: /event/PermissionSetAssignmentEvent (or equivalent)

// In an external monitoring tool (Node.js example):
const faye = require('faye');
const client = new faye.Client(instanceUrl + '/cometd/59.0', {
    timeout: 120,
    retry: 5
});

client.setHeader('Authorization', 'Bearer ' + accessToken);
client.subscribe('/event/PermissionSetAssignmentEvent', event => {
    const data = event.data.payload;
    if (data.PermissionSet.IsHighPrivilege__c) {
        sendAlert('High-privilege permission set assigned to ' + data.Assignee.Username);
    }
});
```

---

## Continuous Verification Checklist

- [ ] MFA enforced for all user logins (not just login page — also API).
- [ ] High Assurance sessions required for admin operations.
- [ ] Per-Permission-Set activation requires High Assurance for privileged sets.
- [ ] OAuth access tokens expire in <= 2 hours.
- [ ] Refresh tokens rotate on use (invalidate previous on refresh).
- [ ] Server-to-server integrations use JWT Bearer (no persistent refresh token).
- [ ] Event Monitoring enabled and logs exported to SIEM.
- [ ] Alert rule: 5+ failed logins per user per hour.
- [ ] Alert rule: Report export by user not in authorized-exporter list.
- [ ] Alert rule: High-privilege permission set assigned outside business hours.
- [ ] Alert rule: API volume spike (> 10x baseline per user per hour).
- [ ] Session lock-to-IP enabled to prevent token reuse from different IPs.
- [ ] Device activation challenge enabled for new device logins.
