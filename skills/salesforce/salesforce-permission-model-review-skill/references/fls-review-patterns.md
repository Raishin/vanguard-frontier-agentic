# FLS Review Patterns Reference

Field-Level Security review patterns for identifying and remediating
unauthorized access to PII, encrypted fields, and regulated data in Salesforce.

---

## What Field-Level Security Controls

Field-Level Security (FLS) determines which fields a user can read or edit
on an object. It operates independently from record sharing:
- **Record sharing** controls whether a user can see the record at all.
- **FLS** controls which fields within an accessible record the user can see or edit.

A user might have read access to a Contact record but FLS may hide the
SSN__c field for that user's profile/permission set configuration.

---

## FLS Enforcement Gaps in Apex

The most common FLS gap is Apex code that queries and returns fields without
checking whether the running user has FLS access to those fields.

### Why Apex Bypasses FLS By Default

SOQL executed in Apex system context returns all fields regardless of FLS.
Even `with sharing` only enforces row-level access, not field-level access.

### Detection: Find @AuraEnabled Methods Returning Sensitive Fields

```bash
# Find Apex methods that return Contact with SSN or financial fields
grep -rn "SSN__c\|CreditScore__c\|BankAccount__c\|TaxId__c" \
  --include="*.cls" \
  force-app/main/default/classes/
```

Then check each result for the presence of FLS enforcement:
```bash
# Verify Security.stripInaccessible or WITH SECURITY_ENFORCED nearby
grep -A5 -B5 "SSN__c" force-app/main/default/classes/ContactController.cls | \
  grep -E "stripInaccessible|SECURITY_ENFORCED|isAccessible"
```

---

## FLS Enforcement Methods

### Method 1: WITH SECURITY_ENFORCED in SOQL

```apex
// Throws QueryException if user lacks FLS read on any field in SELECT
@AuraEnabled
public static List<Contact> getSensitiveContacts {
    return [
        SELECT Id, Name, Email, SSN__c, TaxId__c
        FROM Contact
        WITH SECURITY_ENFORCED
        LIMIT 100
    ];
}
```

**Limitation:** If the field is in the SELECT clause and the user lacks access,
the entire query throws an exception. This is all-or-nothing — you cannot
selectively strip one field and return the rest with this approach.

### Method 2: Security.stripInaccessible

```apex
@AuraEnabled
public static List<Contact> getSensitiveContacts {
    // Query without FLS enforcement first
    List<Contact> rawContacts = [
        SELECT Id, Name, Email, SSN__c, TaxId__c
        FROM Contact
        LIMIT 100
    ];

    // Strip fields the running user cannot read
    SObjectAccessDecision decision = Security.stripInaccessible(
        AccessType.READABLE,
        rawContacts
    );

    // Returns records with inaccessible fields removed
    return (List<Contact>) decision.getRecords;
}
```

`stripInaccessible` strips fields silently rather than throwing an exception.
The returned records simply do not have the restricted field populated.

### Method 3: WITH USER_MODE (API 57.0+)

```apex
// Runs query entirely in user context: sharing rules + FLS + CRUD all enforced
@AuraEnabled
public static List<Contact> getSensitiveContacts {
    return [
        SELECT Id, Name, Email, SSN__c
        FROM Contact
        WITH USER_MODE
        LIMIT 100
    ];
}
```

`WITH USER_MODE` is the most comprehensive enforcement — use this when possible
in new code (Salesforce API version 57.0 and above).

### Method 4: Manual FLS Check

```apex
public static Boolean canReadField(SObjectType objType, String fieldApiName) {
    return objType.getDescribe
        .fields
        .getMap
        .get(fieldApiName)
        ?.getDescribe
        .isAccessible ?? false;
}

// Usage
if (!canReadField(Contact.SObjectType, 'SSN__c')) {
    throw new AuraHandledException('You do not have access to this information.');
}
```

---

## PII Field Classification for FLS Review

### High-Priority Fields Requiring Restrictive FLS

| Object | Field API Name Pattern | Classification |
|--------|----------------------|----------------|
| Contact, Lead, Individual | `SSN__c`, `NationalId__c`, `TaxId__c` | Restricted PII |
| Contact | `BirthDate` | Sensitive PII |
| Contact, Account | `CreditScore__c`, `BankAccount__c` | Financial PII |
| HealthCloudGA__EhrPatient__c | All fields | PHI (HIPAA) |
| FinServ__FinancialAccount__c | `FinServ__Balance__c`, account number fields | Financial PII |
| Contact | `Password__c`, `SecurityAnswer__c` | Credential (should not exist in Salesforce) |
| Contact | `PassportNumber__c`, `DriversLicense__c` | Government ID |

### Standard Fields Requiring FLS Review

| Object | Field | Caution |
|--------|-------|---------|
| User | `Username`, `Email` | Identity data; restrict from non-admin users |
| Contact | `Email` | Core PII; verify FLS aligns with consent model |
| Lead | `Email`, `Phone` | Contact data; restrict edit to owners |

---

## FLS Audit Queries

### Find Fields Without FLS Restrictions (All Profiles Can Read)

```apex
// Check FLS for a specific field across all active profiles
String objectApiName = 'Contact';
String fieldApiName = 'SSN__c';

List<FieldPermissions> fps = [
    SELECT Id, Parent.Label, Field, PermissionsRead, PermissionsEdit
    FROM FieldPermissions
    WHERE SobjectType = :objectApiName
      AND Field = :(objectApiName + '.' + fieldApiName)
    ORDER BY Parent.Label
];

for (FieldPermissions fp : fps) {
    if (fp.PermissionsRead) {
        System.debug('READ ACCESS: ' + fp.Parent.Label + ' -> ' + fp.Field);
    }
}
```

### Find Profiles with Broad Field Access

```sql
-- Via SOQL: identify Permission Sets with read access to sensitive fields
SELECT Parent.Label, Field, PermissionsRead, PermissionsEdit
FROM FieldPermissions
WHERE SobjectType = 'Contact'
  AND Field IN ('Contact.SSN__c', 'Contact.TaxId__c', 'Contact.BankAccount__c')
  AND PermissionsRead = true
ORDER BY Parent.Label
```

---

## FLS Review Checklist by Data Type

### For Each Regulated/Sensitive Object

- [ ] Default FLS for sensitive fields is Read=false, Edit=false on all profiles.
- [ ] Access to sensitive fields granted only via named Permission Sets.
- [ ] Permission Sets granting field access have documented business justification.
- [ ] Number of users with field access is documented and reviewed annually.
- [ ] Apex code querying sensitive fields uses `stripInaccessible`, `WITH SECURITY_ENFORCED`, or `WITH USER_MODE`.
- [ ] LWC components receiving sensitive field data enforce FLS at the Apex layer.
- [ ] Reports using sensitive fields are in restricted-access report folders.

### For @AuraEnabled Methods Returning SObjects

- [ ] Method uses `with sharing` on the class.
- [ ] Method uses at least one FLS enforcement mechanism (see Method 1-3 above).
- [ ] Method does not log sensitive field values to `System.debug`.
- [ ] Method does not include sensitive fields in error messages returned to the UI.

---

## FLS and Report Access

FLS applies to reports: users without FLS read access to a field cannot add
that field to a report. However, if a report was saved by an admin who had
access, and then shared with a user who lacks FLS access, the behavior varies
by report type.

**Recommendation:** Store reports containing sensitive fields in folders with
restricted access matching the FLS grant.

```sql
-- Find report folders with their access levels
SELECT Id, Name, AccessType, DeveloperName
FROM Folder
WHERE Type = 'Report'
ORDER BY AccessType, Name
```

`AccessType = 'Public'` means all users can access reports in this folder —
review whether any report in a public folder contains restricted fields.
