# Named Credential Design Reference

Reference for designing secure Named Credentials and External Credentials
in Salesforce for API callout authentication management.

---

## Named Credential vs External Credential

Salesforce introduced a split between Named Credential and External Credential
<!-- verify-before-merge:2026-05-21 --> in recent API versions. Understanding the distinction is
required for correct configuration.

| Concept | Purpose | Stores |
|---------|---------|--------|
| External Credential | Authentication method and credentials | Client ID, client secret, certificate, or OAuth token |
| Named Credential | URL and HTTP settings | Base URL, protocol, auth scheme reference |
| Principal | User or permission set mapped to External Credential | Per-user or per-permission-set auth |

In the legacy model (prior to split), a single Named Credential stored both
the URL and the credentials.

### When to Use Each

- **Legacy Named Credential (URL + auth in one):** Acceptable for simple integrations.
  Deprecated path; migrate to split model when possible.
- **External Credential + Named Credential (split):** Use for:
  - Per-user authentication (each user's own credentials used for callouts).
  - OAuth flows where token must be refreshed per user.
  - Environments where multiple endpoints share the same auth scheme.

---

## Configuring OAuth External Credentials

### OAuth 2.0 Client Credentials (Server-to-Server)
```
External Credential:
  Authentication Protocol: OAuth 2.0
  Authentication Flow Type: Client Credentials
  Client Id: {CLIENT_ID}
  Client Secret: {CLIENT_SECRET}  (stored encrypted, not visible after save)
  Token URL: https://api.vendor.example.com/oauth/token
  Scope: api:read api:write
```

### OAuth 2.0 JWT Bearer (Server-to-Server, Certificate-Based)
```
External Credential:
  Authentication Protocol: OAuth 2.0
  Authentication Flow Type: JWT Bearer Token
  Client Id: {CLIENT_ID}
  Certificate: {CERT_UNIQUE_NAME}  (Salesforce Certificate, not a file upload)
  Token URL: https://login.salesforce.com/services/oauth2/token
  Audience: https://login.salesforce.com
  Subject: integration-user@myorg.salesforce.com
```

---

## Named Credential URL Configuration

```
Named Credential:
  Label: Vendor API Production
  URL: https://api.vendor.example.com/v2
  External Credential: VendorAPI_ExternalCred
  Generate Authorization Header: Enabled
  Allow Formula Fields in HTTP Header: Disabled (unless needed)
  Allow Merge Fields in HTTP Body: Disabled (unless needed)
```

**Security note on Merge Fields:**
- `Allow Merge Fields in HTTP Body` and `Allow Merge Fields in HTTP Header`
  let Apex inject field values from Salesforce records into callout requests.
- Enable only when strictly necessary. When enabled, ensure the injected values
  are validated and bounded — arbitrary field injection can exfiltrate data.

---

## mTLS (Mutual TLS) Setup

For endpoints that require client certificate authentication:

### Step 1: Generate or upload certificate
```
Setup > Security > Certificate and Key Management
  -> Generate Self-Signed Certificate (for testing)
  -> Upload Signed Certificate (for production)
  Note the Certificate Unique Name (e.g., VendorMTLSCert)
```

### Step 2: Configure External Credential with certificate
```
External Credential:
  Authentication Protocol: Custom Header
  Custom Headers:
    X-Client-Cert: {!$Credential.Certificate}   (if vendor requires cert in header)
  Certificate: VendorMTLSCert
```

For true mTLS (TLS handshake-level):
```
Named Credential:
  Callout Options: Use Client Certificate
  Certificate: VendorMTLSCert
```

### Step 3: Verify endpoint accepts Salesforce certificate
Share the public certificate (exported from Certificate and Key Management)
with the integration vendor for allowlisting on their server.

---

## Custom Header Injection

Named Credentials support injecting custom headers into every callout:

```
Named Credential > Custom Headers:
  Name: X-API-Key
  Value: {!$Credential.Password}   (references External Credential stored secret)

  Name: X-Tenant-Id
  Value: MyTenantId123             (static value; acceptable for non-secret metadata)

  Name: X-Request-Id
  Value: {!CASESAFEID($Api.Organization_Id)}  (dynamic if merge fields enabled)
```

**Security review checklist for custom headers:**
- [ ] No secret values stored as static strings (must reference `{!$Credential.X}`).
- [ ] No internal org IDs or user IDs exposed in headers to external systems
  unless required by the vendor.
- [ ] Header names match vendor's documented API specification.

---

## Named Credential Security Review

### Access Control

Named Credentials are accessible to any Apex code that references them.
Access is not restricted at the Named Credential level. Restrict access at
the Apex class level using `with sharing` and appropriate caller validation.

```apex
// WHO can invoke this callout? Restrict at the class level.
public with sharing class VendorAPIClient {
    private static final String NC_NAME = 'callout:VendorAPI_Prod';

    @AuraEnabled
    public static String fetchData(String resourcePath) {
        // Validate resourcePath to prevent path traversal
        if (!resourcePath.startsWith('/allowed/path/')) {
            throw new AuraHandledException('Invalid resource path.');
        }
        HttpRequest req = new HttpRequest();
        req.setEndpoint(NC_NAME + resourcePath);
        req.setMethod('GET');
        req.setTimeout(30000);
        HttpResponse res = new Http().send(req);
        if (res.getStatusCode() != 200) {
            throw new AuraHandledException('Callout failed: ' + res.getStatusCode());
        }
        return res.getBody();
    }
}
```

### Listing All Named Credentials (SOQL)
```sql
SELECT Id, DeveloperName, Endpoint, PrincipalType, AuthorizationStatus
FROM NamedCredential
ORDER BY DeveloperName
```

### Audit Questions

1. Does every Named Credential point to an HTTPS endpoint? (`http://` endpoints
   should not exist in production.)
2. Does every Named Credential have a corresponding External Credential with
   stored secrets — or is it using legacy merged fields with secrets?
3. Are credentials scoped to a specific integration user/service account, not
   a named human's credentials?
4. Is there a rotation plan documented for credentials stored in External
   Credentials?

---

## Credential Rotation Procedure

```
1. Obtain new credentials from the integration vendor.
2. Navigate to: Setup > Named Credentials > External Credentials > [Credential]
3. Update the Client Secret or Password field with the new value.
4. Save.
5. Test the Named Credential (Setup > Named Credentials > Test button if available,
   or run a test Apex script in anonymous Apex window against the NC).
6. Document rotation date in the integration runbook.
7. Revoke the old credentials with the vendor within 24 hours of successful rotation.
```

Rotation schedule guidance:

| Credential Type | Rotation Frequency |
|----------------|-------------------|
| API keys (long-lived) | 90 days |
| OAuth client secrets | 180 days |
| Certificates | Before expiry date; minimum annual |
| Username/password (if used) | 30 days |
