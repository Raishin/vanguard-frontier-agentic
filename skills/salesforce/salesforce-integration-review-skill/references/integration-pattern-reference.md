# Integration Pattern Reference

Reference for Salesforce integration patterns covering REST, SOAP, Bulk API,
Platform Events, Change Data Capture, and OAuth flows.

---

## API Pattern Selection Matrix

| Use Case | Recommended API | Notes |
|----------|----------------|-------|
| Retrieve/update single or small record set | REST API | Low latency, JSON |
| Enterprise system integration, WSDLs | SOAP API | XML, strong typing |
| Bulk load/export (> 10,000 records) | Bulk API 2.0 | Async, CSV/JSON |
| Real-time event notification | Platform Events | Pub/Sub, async |
| Replicate CRM changes to external systems | Change Data Capture | Delta sync |
| User-facing OAuth flows | OAuth 2.0 Authorization Code | Browser-based |
| Server-to-server integration | OAuth 2.0 JWT Bearer | No user interaction |
| Legacy system callout from Apex | Named Credential + HttpRequest | Managed credentials |

---

## REST API

### Endpoint Structure
```
https://{instanceUrl}/services/data/v{apiVersion}/{resource}

Common resources:
  /sobjects/{sObjectType}/          -- CRUD on objects
  /sobjects/{sObjectType}/{Id}      -- Specific record
  /query/?q={SOQL}                  -- SOQL query
  /composite/                       -- Batch multiple requests
  /composite/tree/                  -- Create related records in one call
  /composite/batch                  -- Up to 25 subrequests in one HTTP call
```

### Composite API (minimize round trips)
```json
POST /services/data/v59.0/composite/batch
{
  "haltOnError": false,
  "batchRequests": [
    {
      "method": "GET",
      "url": "/services/data/v59.0/sobjects/Account/001XXXXXXXXXXXX"
    },
    {
      "method": "PATCH",
      "url": "/services/data/v59.0/sobjects/Contact/003XXXXXXXXXXXX",
      "richInput": { "Phone": "+1-555-0123" }
    }
  ]
}
```

### REST API Limits
- API requests per 24 hours: based on edition and user count.
- Concurrent API limits: 25 long-running requests per org.
- Composite batch: max 25 subrequests.
- Composite graph: max 500 nodes.
- SOQL: max 2,000 characters in query string via REST.

---

## SOAP API

### WSDL Types
- **Enterprise WSDL:** Org-specific, strongly typed, regenerated when metadata changes.
- **Partner WSDL:** Generic, works across orgs, uses `sObject` generic type.

Use Partner WSDL for multi-org integrations; Enterprise WSDL for single-org
tightly coupled systems.

### SOAP Session Management
```xml
<!-- Login request to get session ID -->
<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/"
                  xmlns:urn="urn:partner.soap.sforce.com">
  <soapenv:Body>
    <urn:login>
      <urn:username>user@example.com</urn:username>
      <urn:password>passwordSECURITYTOKEN</urn:password>
    </urn:login>
  </soapenv:Body>
</soapenv:Envelope>
```

**Security note:** SOAP login with username/password is deprecated in favor of
OAuth. Avoid using SOAP login in new integrations. If it exists, migrate to
OAuth JWT Bearer flow.

---

## Bulk API 2.0

### Job Lifecycle
```
1. Create Job:       POST /services/data/v59.0/jobs/ingest/
2. Upload Data:      PUT  /services/data/v59.0/jobs/ingest/{jobId}/batches
3. Close Job:        PATCH /services/data/v59.0/jobs/ingest/{jobId}  {"state":"UploadComplete"}
4. Poll Status:      GET  /services/data/v59.0/jobs/ingest/{jobId}
5. Get Results:      GET  /services/data/v59.0/jobs/ingest/{jobId}/successfulResults
                     GET  /services/data/v59.0/jobs/ingest/{jobId}/failedResults
6. Delete Job:       DELETE /services/data/v59.0/jobs/ingest/{jobId}
```

### Bulk API 2.0 Limits
- Max records per job: 150 million (via multiple batches)
- Max file size: 100 MB per upload call
- Max parallel jobs: varies by edition
- Use `query` operation type for exports: `POST /services/data/v59.0/jobs/query/`

### Bulk API vs SOQL for Exports

| Volume | Method |
|--------|--------|
| < 50,000 rows | REST API + SOQL with LIMIT/OFFSET |
| 50,000–10M rows | Bulk API 2.0 query job |
| > 10M rows | Bulk API 2.0 + PK chunking (for certain objects) |

---

## Platform Events

Platform Events provide a publish/subscribe messaging system within Salesforce.
They are defined as Salesforce objects with `__e` suffix.

### Publishing from Apex
```apex
// Publish a Platform Event
Shipment_Status__e event = new Shipment_Status__e(
    OrderId__c = '1234',
    Status__c = 'Delivered',
    Timestamp__c = DateTime.now
);
Database.SaveResult result = EventBus.publish(event);
if (!result.isSuccess) {
    System.debug('Event publish failed: ' + result.getErrors[0].getMessage);
}
```

### Subscribing (Trigger on Platform Event)
```apex
trigger ShipmentStatusTrigger on Shipment_Status__e (after insert) {
    for (Shipment_Status__e event : Trigger.new) {
        // Process event
        System.debug('Order ' + event.OrderId__c + ' status: ' + event.Status__c);
    }
}
```

### Platform Event Limits
- Event delivery: at least once (idempotent subscribers required).
- Retention: 72 hours (standard events); 1 day to 90 days (high-volume events).
- Max event message size: 1 MB.
- Publishing limits: governed by API request limits.

---

## Change Data Capture (CDC)

CDC publishes change events when Salesforce records are created, updated,
deleted, or undeleted. External systems subscribe to receive delta changes.

```
Supported objects: Standard and custom objects enabled for CDC.
Event type suffix: __ChangeEvent (e.g., AccountChangeEvent)
```

### Subscribing via CometD (External)
```python
# Python example using salesforce-cometd-client
from salesforce.cometd import SalesforceCometDClient

client = SalesforceCometDClient(
    login_url="https://login.salesforce.com",
    client_id=CLIENT_ID,
    client_secret=CLIENT_SECRET,
    username=USERNAME,
    password=PASSWORD + SECURITY_TOKEN
)
client.subscribe("/data/AccountChangeEvent", callback=handle_account_change)
client.start

def handle_account_change(event):
    header = event['data']['schema']
    body = event['data']['payload']
    print(f"Change type: {body['ChangeEventHeader']['changeType']}")
    print(f"Changed fields: {body['ChangeEventHeader']['changedFields']}")
```

---

## OAuth Flows Reference

### OAuth 2.0 Authorization Code Flow (User-Facing)
```
1. Redirect user to:
   https://login.salesforce.com/services/oauth2/authorize?
     response_type=code&
     client_id={CLIENT_ID}&
     redirect_uri={CALLBACK_URL}&
     scope=api+refresh_token

2. User logs in and approves.

3. Salesforce redirects to CALLBACK_URL with ?code={AUTH_CODE}

4. Exchange code for token:
   POST https://login.salesforce.com/services/oauth2/token
   Body: grant_type=authorization_code&code={AUTH_CODE}&
         client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&
         redirect_uri={CALLBACK_URL}

5. Store access_token and refresh_token securely.
```

### OAuth 2.0 JWT Bearer Flow (Server-to-Server)
```
1. Generate JWT signed with your Connected App's private key:
   Header: {"alg":"RS256","typ":"JWT"}
   Payload: {
     "iss": "{CLIENT_ID}",
     "sub": "{SALESFORCE_USERNAME}",
     "aud": "https://login.salesforce.com",
     "exp": {current_epoch + 300}
   }

2. POST to token endpoint:
   POST https://login.salesforce.com/services/oauth2/token
   Body: grant_type=urn:ietf:params:oauth:grant-type:jwt-bearer&
         assertion={SIGNED_JWT}

3. Receive access_token (no refresh_token in JWT flow).
```

JWT Bearer is the recommended server-to-server pattern. No user credentials
are transmitted; access is pre-authorized by admin.
