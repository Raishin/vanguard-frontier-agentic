# Formula Syntax Quick Reference

## Logical Operators

| Function | Syntax | Notes |
|---|---|---|
| AND | `AND(cond1, cond2)` | Prefer over `&&` for readability |
| OR | `OR(cond1, cond2)` | Prefer over `\|\|` |
| NOT | `NOT(cond)` | Wraps a single boolean expression |
| IF | `IF(condition, true_result, false_result)` | Conditional branching |

## Blank / Null Checks

| Function | Use when | Notes |
|---|---|---|
| `ISBLANK(field)` | Text, email, phone, URL fields | Returns TRUE if field is empty string or null |
| `ISNULL(field)` | Number, currency, date, datetime fields | Returns TRUE if field has no value |
| `BLANKVALUE(field, default)` | Any field where you want a fallback | Avoids runtime null errors in arithmetic |

**Rule:** Never use `= ""` or `= null` in formulas. Always use `ISBLANK()` or `ISNULL()`.

## Picklist Handling

```
TEXT(Picklist_Field__c) = "API_Value"
```

- Always wrap picklist fields in `TEXT()` before string comparison
- Use the **API value** (not the display label) — they may differ
- Multi-select picklists: `INCLUDES(MSP_Field__c, "Value")`
- Multi-select exclusion: `NOT(INCLUDES(MSP_Field__c, "Value"))`

## Type Conversion Functions

| Function | Input | Output | Example |
|---|---|---|---|
| `TEXT(value)` | Picklist, number, date | String | `TEXT(Stage)` |
| `VALUE(text)` | String | Number | `VALUE("42")` |
| `DATEVALUE(text)` | String (ISO date) | Date | `DATEVALUE("2026-01-01")` |
| `DATETIMEVALUE(text)` | String (ISO datetime) | Datetime | `DATETIMEVALUE("2026-01-01 00:00:00")` |

## Record Context Functions

| Function | Returns | Use case |
|---|---|---|
| `ISNEW()` | Boolean | TRUE on record creation; FALSE on edit |
| `ISCHANGED(field)` | Boolean | TRUE if field value changed during this save |
| `PRIORVALUE(field)` | Field's prior value | Compare current vs. previous value |
| `TODAY()` | Date | Current date (no time) |
| `NOW()` | Datetime | Current datetime |

**Critical:** `ISCHANGED()` and `PRIORVALUE()` are only meaningful on edit
operations. Always gate them with `NOT(ISNEW())`:

```
AND(
  NOT(ISNEW()),
  ISCHANGED(Close_Date__c),
  CloseDate < TODAY()
)
```

## Cross-Object References

Access parent (lookup/master-detail) field values:

```
Account.BillingCountry
OpportunityLineItem.Opportunity.Account.Industry
```

**Limits:**
- Max 10 levels of relationship traversal
- Cannot span more than 5 distinct object hops in a single formula
- Each cross-object merge field counts toward the 5,000-character compiled limit

## Profile and Permission Checks

```
$Profile.Name = "System Administrator"
$Profile.UserLicense.Name = "Salesforce"
$Permission.My_Custom_Permission = true
```

For bypass patterns, negate the check:
```
AND(
  $Profile.Name <> "System Administrator",
  <core_condition>
)
```

Multiple profile bypass:
```
AND(
  NOT(OR(
    $Profile.Name = "System Administrator",
    $Profile.Name = "Integration User",
    $Profile.Name = "Sales Ops"
  )),
  <core_condition>
)
```

## Record Type Check

```
RecordType.DeveloperName = "Enterprise_Account"
```

Note: Use `DeveloperName` (no spaces, no special characters), not `Name`
(display label). DeveloperName is stable across sandbox/production refreshes.

## BLANKVALUE Pattern (Null-Safe Arithmetic)

```
BLANKVALUE(Amount, 0) < 1000
```

Avoids null pointer errors when arithmetic fields may be empty. Always use
`BLANKVALUE(field, 0)` for numeric and currency fields that can be blank.

## Formula Compilation Limits

| Limit | Value |
|---|---|
| Compiled formula size | 5,000 bytes |
| Uncompiled formula characters | 3,900 characters |
| Cross-object references | 10 per formula |
| Nested functions | No explicit limit, but readable depth ≤ 5 recommended |

If you approach the limit, refactor by:
1. Using Custom Fields with pre-computed values to reduce cross-object hops
2. Splitting complex AND/OR chains into intermediate helper formulas
3. Using Custom Permissions instead of Profile Name checks

## Common Formula Patterns

### Require field on new record
```
AND(
  ISNEW(),
  ISBLANK(Required_Field__c)
)
```

### Require field on edit only
```
AND(
  NOT(ISNEW()),
  ISBLANK(Required_Field__c)
)
```

### Block if date is in past (new records only)
```
AND(
  ISNEW(),
  NOT(ISBLANK(Close_Date__c)),
  Close_Date__c < TODAY()
)
```

### Conditional requirement by picklist value
```
AND(
  TEXT(Status__c) = "Active",
  ISBLANK(Billing_Contact__c)
)
```

### Prevent stage regression
```
AND(
  NOT(ISNEW()),
  ISCHANGED(StageName),
  OR(
    AND(PRIORVALUE(StageName) = "Closed Won", StageName <> "Closed Won"),
    AND(PRIORVALUE(StageName) = "Closed Lost", StageName <> "Closed Lost")
  )
)
```
