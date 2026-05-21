# Apex Security Defaults Reference

Adapted from forcedotcom/sf-skills generating-apex references (Apache-2.0).

## WITH SHARING Default Policy

Every Apex class must declare an explicit sharing model. The default when no keyword is
present is `without sharing`, which bypasses record-level access — this is a security risk.

**Rule:** Always declare `with sharing` unless there is a documented system-operation
reason to bypass sharing rules. Never rely on the implicit default.

```apex
// CORRECT
public with sharing class AccountService { ... }

// WRONG — implicit without sharing, data exposure risk
public class AccountService { ... }
```

---

## USER_MODE for SOQL

`WITH USER_MODE` enforces FLS (Field-Level Security) and CRUD permissions at query time.
Apply to all SOQL in classes that operate in a user-facing context.

```apex
// Enforces FLS — user cannot see fields they lack access to
List<Account> accounts = [
    SELECT Id, Name, AnnualRevenue
    FROM Account
    WHERE OwnerId = :userId
    WITH USER_MODE
];
```

`WITH SYSTEM_MODE` bypasses FLS — only use when the class intentionally operates with
elevated system privileges (e.g., a background sync class that must read all fields
regardless of the running user's FLS settings). Document the reason.

---

## Security.stripInaccessible()

Use `Security.stripInaccessible()` when constructing SObjects from user-supplied data
or when returning records to untrusted callers. Strips fields the running user cannot
access according to their profile/permission sets.

```apex
// Strip inaccessible fields from a user-constructed SObject before DML
SObjectAccessDecision decision = Security.stripInaccessible(
    AccessType.CREATABLE,
    recordsFromUserInput
);
insert decision.getRecords();

// Strip on query results before returning to caller
List<Account> rawAccounts = [SELECT Id, Name, SSN__c FROM Account WITH USER_MODE];
SObjectAccessDecision readDecision = Security.stripInaccessible(
    AccessType.READABLE,
    rawAccounts
);
return readDecision.getRecords();
```

`AccessType` values: `READABLE`, `CREATABLE`, `UPDATABLE`, `UPSERTABLE`.

---

## SOQL Injection Prevention

Never concatenate user input directly into a SOQL string. Always use bind variables.

```apex
// VULNERABLE
String query = 'SELECT Id FROM Account WHERE Name = \'' + userInput + '\'';
List<SObject> results = Database.query(query);

// SAFE — bind variable
String safeName = userInput;
List<Account> results = [SELECT Id FROM Account WHERE Name = :safeName];

// SAFE — dynamic SOQL with bind
String fieldName = 'Name'; // must be from an allowlist, not raw user input
String query = 'SELECT Id FROM Account WHERE ' + fieldName + ' = :safeName';
List<SObject> results = Database.query(query, AccessLevel.USER_MODE);
```

For dynamic SOQL where field/object names come from user input:
1. Validate against a Schema.getGlobalDescribe() allowlist — never trust raw strings
2. Use `Database.query(query, AccessLevel.USER_MODE)` for FLS enforcement in dynamic SOQL

---

## No Hardcoded IDs or Credentials

Never hardcode Salesforce record IDs, profile IDs, permission set IDs, or credentials
in Apex code. They break cross-org portability and create maintenance nightmares.

**Prohibited patterns:**
```apex
// WRONG
if (userId == '0053a00000AbCdEfG') { ... }
String apiKey = 'sk-abc123';
Id profileId = '00e3a00000AbCdEfG';
```

**Correct approaches:**
```apex
// Custom Metadata Type for config
My_Config__mdt config = [SELECT API_Key__c FROM My_Config__mdt WHERE Label = 'Integration' LIMIT 1];

// Named Credential for external credentials
HttpRequest req = new HttpRequest();
req.setEndpoint('callout:My_Named_Credential/api/endpoint');

// Schema describe for object/field metadata
Schema.DescribeFieldResult fieldDesc = Schema.SObjectType.Account.fields.getMap().get('Industry').getDescribe();

// Label for user-visible strings
String message = Label.Account_Updated_Success;
```

---

## Sensitive Data Handling

- Never log PII (email, phone, SSN, financial data) via `System.debug()`.
- Never store sensitive data in Custom Settings visible to non-admin profiles.
- Use Named Credentials for external system credentials — never string literals.
- Use Shield Platform Encryption or Classic Encryption for fields requiring at-rest encryption.
- Apply `stripInaccessible(AccessType.READABLE)` before returning record collections to
  any method that may pass data to an untrusted layer.

---

## Permission Checks in Apex

For explicit CRUD checks before DML (in classes that cannot rely on WITH USER_MODE):

```apex
// Check before insert
if (!Schema.SObjectType.Account.isCreateable()) {
    throw new AuraHandledException('Insufficient permissions to create Account records.');
}

// Check before update
if (!Schema.SObjectType.Account.isUpdateable()) {
    throw new AuraHandledException('Insufficient permissions to update Account records.');
}
```

Prefer `WITH USER_MODE` on SOQL and `Security.stripInaccessible()` on DML over
manual permission checks — they are more reliable and less verbose.
