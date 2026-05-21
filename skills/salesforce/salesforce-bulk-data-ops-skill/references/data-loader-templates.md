# Data Loader Templates Reference

## Data Loader Operation Overview

| Operation | When to use | Requires Id field? |
|---|---|---|
| Insert | Create new records | No (Id auto-generated) |
| Update | Modify existing records by Salesforce Id | Yes (`Id` column required) |
| Upsert | Insert or update based on external ID | External ID field required |
| Delete | Soft-delete records (Recycle Bin) | Yes (`Id` column required) |
| Export | Query records to CSV | N/A |
| Export All | Query including soft-deleted records | N/A |

---

## Template 1: Mass Owner Reassignment (Update)

**Operation:** Update
**Object:** Opportunity (or any object with OwnerId)

**CSV header row:**
```
Id,OwnerId
```

**Example data:**
```
Id,OwnerId
006Xx000001REPLACE1,005Xx000001NEWOWNER
006Xx000001REPLACE2,005Xx000001NEWOWNER
```

**Notes:**
- `OwnerId` must be the 18-character Salesforce User ID of the new owner
- Pre-query new owner ID: `SELECT Id FROM User WHERE Username = 'newowner@company.com'`
- Generate the Id column via export query:
  ```
  SELECT Id FROM Opportunity WHERE OwnerId = '<old_owner_id>'
  ```
- Run in sandbox first to verify ownership transfer

---

## Template 2: Mass Field Update (Update)

**Operation:** Update
**Object:** Account

**CSV header row (example — update Rating and Industry):**
```
Id,Rating,Industry
```

**Example data:**
```
Id,Rating,Industry
001Xx000001REPLACE1,Hot,Technology
001Xx000001REPLACE2,Warm,Healthcare
```

**Notes:**
- Only include the fields being changed plus `Id`
- Do not include fields you want to preserve unchanged (they will remain unchanged)
- Picklist values must match exact API values (case-sensitive)

---

## Template 3: Upsert with External ID (Upsert)

**Operation:** Upsert
**Object:** Contact
**External ID field:** `HubSpot_Contact_ID__c`

**CSV header row:**
```
HubSpot_Contact_ID__c,FirstName,LastName,Email,Phone
```

**Example data:**
```
HubSpot_Contact_ID__c,FirstName,LastName,Email,Phone
HS-001,Jane,Smith,jane.smith@example.com,415-555-0101
HS-002,John,Doe,john.doe@example.com,415-555-0102
```

**Notes:**
- The external ID field must have "External ID" checked in Salesforce Setup
- New records (no matching external ID) are inserted
- Existing records (matching external ID) are updated
- Do not include `Id` column — it is ignored in upsert mode
- For relationship lookups in upsert: use `AccountId:Account:External_ID__c`

---

## Template 4: Bulk Soft Delete (Delete)

**Operation:** Delete
**Object:** Lead

**CSV header row:**
```
Id
```

**Example data:**
```
Id
00QXx000001REPLACE1
00QXx000001REPLACE2
```

**Notes:**
- Deleted records go to the Recycle Bin (15-day retention by default)
- Hard delete (emptyRecycleBin) is T3 prohibited — do not run without explicit approval
- Pre-export the records to a backup CSV before deleting
- Max 10,000 records per Data Loader batch recommended

---

## Template 5: Lead Conversion (Insert + Update combination)

Lead conversion is not directly supported by Data Loader. Use the
Anonymous Apex template from `anonymous-apex-patterns.md`.

For Data Loader lead status update (marking leads as "Converted" is
read-only in Salesforce — proper conversion requires API call):

**Alternative:** Export unconverted leads, use the Anonymous Apex
lead conversion script, verify results.

---

## Data Loader Configuration Notes

### Batch Size

| Volume | Recommended batch size |
|---|---|
| < 1,000 records | 200 |
| 1,000 – 50,000 records | 2,000 |
| > 50,000 records | 5,000 (Bulk API v2 mode) |

Configure in Data Loader: Settings → Batch Size

### Bulk API v2 (Recommended for Large Volumes)

Enable Bulk API v2 for operations over 10,000 records:
- Settings → Use Bulk API: Enable
- Settings → Bulk API v2 for query: Enable
- Advantages: Parallel processing, higher throughput, better error reporting

### Error Log

Data Loader generates an error log CSV after each operation:
- Success file: `<operation>_success_<timestamp>.csv`
- Error file: `<operation>_error_<timestamp>.csv`

Always review the error file after operations. Common errors:
- `REQUIRED_FIELD_MISSING` — required field not in CSV
- `INVALID_FIELD_FOR_INSERT_UPDATE` — read-only field included
- `STRING_TOO_LONG` — value exceeds field max length
- `INVALID_ID_FIELD` — malformed record ID

---

## Data Loader Export Query Templates

### Export before mass update (backup)

```
SELECT Id, OwnerId, Name, StageName, CloseDate, Amount, LastModifiedDate
FROM Opportunity
WHERE OwnerId = '<target_user_id>'
ORDER BY Id
```

### Export stale records before close

```
SELECT Id, Name, StageName, CloseDate, OwnerId, Amount, Account.Name
FROM Opportunity
WHERE IsClosed = false
AND CloseDate < LAST_N_DAYS:90
ORDER BY CloseDate ASC
```

### Export duplicate leads by email

```
SELECT Id, FirstName, LastName, Email, Company, Status, CreatedDate
FROM Lead
WHERE Email != null
AND IsConverted = false
ORDER BY Email, CreatedDate
```

---

## Relationship Field Mapping in Data Loader

To set a relationship field using an external ID (not Salesforce ID):

| Lookup field | External ID syntax | Example |
|---|---|---|
| AccountId | `AccountId:Account:External_ID__c` | `AccountId:Account:HubSpot_Acct_ID__c` |
| OwnerId | `OwnerId:User:Username` | `OwnerId:User:Username` (use username string) |
| RecordTypeId | `RecordTypeId:RecordType:DeveloperName` | `RecordTypeId:RecordType:DeveloperName` |

This syntax is supported in Data Loader upsert operations only.
