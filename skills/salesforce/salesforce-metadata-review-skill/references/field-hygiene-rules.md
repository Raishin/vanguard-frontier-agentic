# Field Hygiene Rules Reference

Standards for naming, cleaning, and managing custom fields in Salesforce
to prevent metadata bloat and maintain org health.

---

## Naming Conventions

### Custom Field API Names

| Object Type | Convention | Example |
|-------------|-----------|---------|
| Standard field extension | Descriptive suffix | `Account.PreferredChannel__c` |
| Boolean field | `Is` prefix | `Contact.IsVerified__c` |
| Date/DateTime fields | Verb + Date | `Opportunity.CloseRescheduledDate__c` |
| Count fields | `Count` suffix | `Account.ActiveCasesCount__c` |
| External ID | `ExtId` or system name | `Contact.SFMCContactId__c` |
| Currency fields | No currency symbol in name | `Opportunity.EstimatedBudget__c` |
| Formula fields | Indicate computed nature | `Opportunity.DaysOpenFormula__c` |

### Label Naming

- Labels are user-visible; API names are developer-visible. They may differ.
- Labels should be human-readable and title-cased.
- Avoid technical jargon in labels.
- Avoid label changes after the field is in use — label changes affect reports,
  list views, and page layouts.

### Namespace Conventions (Managed Packages)

All fields in managed packages include the namespace prefix:
`namespace__FieldName__c`

In unmanaged orgs, avoid creating field API names that conflict with common
managed package namespaces.

---

## Custom Field Bloat Indicators

Field bloat makes orgs harder to maintain, slows schema lookups, and confuses
developers and admins.

### Quantitative Thresholds

| Object | Custom Field Count | Health Status |
|--------|------------------|--------------|
| Account | < 50 | Good |
| Account | 50-100 | Review — identify unused |
| Account | > 100 | Bloated — cleanup required |
| Contact | < 40 | Good |
| Opportunity | < 60 | Good |
| Any custom object | > 100 | Bloated |

These are guidelines, not hard rules. Objects with complex data models may
legitimately exceed these counts.

### Detecting Unused Fields

```apex
// Run this in anonymous Apex to get field counts per object
Map<String, Integer> fieldCounts = new Map<String, Integer>();
for (Schema.SObjectType objType : Schema.getGlobalDescribe().values()) {
    String objName = objType.getDescribe().getName();
    Integer fieldCount = objType.getDescribe().fields.getMap().size();
    if (fieldCount > 50 && objName.endsWith('__c')) {
        System.debug(objName + ': ' + fieldCount + ' fields');
    }
}
```

### Using Field Usage Reports

Salesforce Setup provides field usage reports:
Path: Setup > Schema Builder > (select object) > Fields tab > Sort by "Last Used"
<!-- verify-before-merge:2026-05-21 -->

Or use the Salesforce Optimizer:
Path: Setup > Salesforce Optimizer > Run (generates field usage report)

Fields with no usage in the last 90 days in production are candidates for review.

### Before Deleting a Field

1. Check all Apex code for references: `grep -rn "FieldName__c" force-app/`
2. Check all Flows referencing the field via Metadata API.
3. Check all Reports using the field.
4. Check all List Views.
5. Check all Page Layouts.
6. Check all Validation Rules.
7. Export field data for archival before deletion.
8. Delete the field in a sandbox first and run all tests.

---

## Formula Field Complexity Limits

Salesforce enforces formula size limits:
<!-- verify-before-merge:2026-05-21 -->

| Limit | Value |
|-------|-------|
| Characters in formula (compiled size) | 5,000 characters |
| Characters in formula (uncompiled source) | 3,900 characters |
| Cross-object references per formula | 10 |
| Date/time function calls per formula | 10 |

### Formula Optimization Patterns

**Anti-pattern: Deeply nested IIF/IF chains**
```
IF(Status__c = 'A', 'Alpha', IF(Status__c = 'B', 'Beta', IF(Status__c = 'C', 'Gamma', 'Other')))
```

For more than 3-4 branches, use a custom Apex trigger to compute the value
and store it in a regular field. Formulas cannot be easily extended and hit
complexity limits.

**Pattern: Text-based CASE equivalent**
```
CASE(Status__c,
  'A', 'Alpha',
  'B', 'Beta',
  'C', 'Gamma',
  'Other'
)
```

CASE formulas are more readable and compile smaller than nested IFs for
picklist-matching logic.

**Cross-object formula depth limit:**
Formulas can reference fields up to 10 hops in a relationship chain.
`Opportunity.Account.Owner.Profile.Name` is 4 hops — within limit.
Deeply nested formulas referencing many cross-object fields increase query
complexity when Salesforce resolves them.

---

## Field Types and Data Integrity

| Field Type | When to Use | Common Misuse |
|------------|-------------|--------------|
| Text | Short free-form text | Using for structured data (dates, IDs, amounts) |
| Text Area | Multi-line free text | Using for structured JSON (use LongTextArea or a related object) |
| Long Text Area | Descriptions, notes | Storing binary-encoded data |
| Rich Text | Formatted HTML content | Storing user-supplied HTML without sanitization |
| Number | Integer or decimal amounts | Storing phone numbers (use Phone type) |
| Currency | Monetary amounts | Multi-currency amounts need currency field type, not number |
| Picklist | Fixed-value categorical data | Using free-text where picklist provides governance |
| Multi-select Picklist | Multiple values from fixed set | Using for data that has many legitimate values (use related object) |
| External ID | External system reference ID | Creating duplicate external IDs |
| Formula | Computed read-only values | Attempting to store mutable state |

### Multi-Select Picklist Cautions

Multi-select picklist fields are stored as semicolon-delimited strings.
They cannot be:
- Used in SOQL with `=` operator (use `INCLUDES` or `LIKE` with `%value%`).
- Reliably indexed.
- Efficiently queried when values need to be joined.

```sql
-- Correct SOQL for multi-select picklist
SELECT Id, Name, Tags__c
FROM Account
WHERE Tags__c INCLUDES ('Enterprise', 'Partner')
```

---

## Field Hygiene Checklist

- [ ] Custom field count per object is within healthy thresholds.
- [ ] All custom fields have populated Description fields.
- [ ] Fields not used in the last 6 months flagged for review.
- [ ] Formula fields do not exceed 5,000 compiled characters.
- [ ] Cross-object formula references are minimized (< 5 hops when possible).
- [ ] Multi-select picklists used only where appropriate (not as a substitute for related objects).
- [ ] External ID fields are indexed.
- [ ] Boolean fields use the `Is` prefix naming convention.
- [ ] No duplicate fields covering the same business concept.
- [ ] Legacy integration fields from decommissioned systems are removed or documented.
