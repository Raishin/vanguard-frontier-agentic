# Validation Rule Patterns

## Pattern 1: Required Field When Another Field Has a Value

**Use case:** Require field B when field A is filled.

```
AND(
  $Profile.Name <> "System Administrator",
  NOT(ISBLANK(Field_A__c)),
  ISBLANK(Field_B__c)
)
```

**Error message:** "Field B is required when Field A has a value."

---

## Pattern 2: Required Field When Picklist Equals Value

**Use case:** Require phone when Lead Source is "Outbound Call".

```
AND(
  $Profile.Name <> "System Administrator",
  TEXT(LeadSource) = "Outbound Call",
  ISBLANK(Phone)
)
```

**Error message:** "Phone is required for Outbound Call leads."

---

## Pattern 3: Prevent Bypass by Profile

**Use case:** Allow only specific profiles to leave a field blank.

```
AND(
  NOT(OR(
    $Profile.Name = "System Administrator",
    $Profile.Name = "Data Migration User",
    $Profile.Name = "Integration User"
  )),
  TEXT(Stage) = "Closed Won",
  ISBLANK(Win_Reason__c)
)
```

**Guidance:** Always include System Administrator. Add Integration User if
automated processes (Apex, Flow, REST API) should bypass the rule.

---

## Pattern 4: Conditional Requirement by Record Type

**Use case:** Require additional fields only on "Enterprise" record type.

```
AND(
  $Profile.Name <> "System Administrator",
  RecordType.DeveloperName = "Enterprise",
  ISBLANK(Executive_Sponsor__c)
)
```

**Guidance:** Use `DeveloperName` not `Name`. DeveloperName is stable across
org refreshes; Name is not.

---

## Pattern 5: Prevent Date in the Past on New Records

**Use case:** Close Date cannot be in the past when creating a new Opportunity.

```
AND(
  $Profile.Name <> "System Administrator",
  ISNEW,
  NOT(ISNULL(CloseDate)),
  CloseDate < TODAY
)
```

**Error message:** "Close Date must be today or a future date."

---

## Pattern 6: Prevent Stage Demotion

**Use case:** Once Opportunity reaches "Contract Sent", prevent moving back
to an earlier stage.

```
AND(
  $Profile.Name <> "System Administrator",
  NOT(ISNEW),
  ISCHANGED(StageName),
  OR(
    AND(PRIORVALUE(StageName) = "Contract Sent", StageName = "Proposal/Price Quote"),
    AND(PRIORVALUE(StageName) = "Contract Sent", StageName = "Needs Analysis"),
    AND(PRIORVALUE(StageName) = "Closed Won", StageName <> "Closed Won")
  )
)
```

**Error message:** "Stage cannot be moved backwards once Contract Sent. Contact Sales Ops."

---

## Pattern 7: Dependent Picklist Validation (Not Native Dependent Picklist)

**Use case:** Sub-type field must match parent type value combinations.

```
AND(
  $Profile.Name <> "System Administrator",
  NOT(AND(
    TEXT(Type__c) = "Hardware",
    OR(
      TEXT(Sub_Type__c) = "Hardware - Standard",
      TEXT(Sub_Type__c) = "Hardware - Premium",
      ISBLANK(Sub_Type__c)
    )
  )),
  NOT(AND(
    TEXT(Type__c) = "Software",
    OR(
      TEXT(Sub_Type__c) = "Software - SaaS",
      TEXT(Sub_Type__c) = "Software - On-Prem",
      ISBLANK(Sub_Type__c)
    )
  ))
)
```

**Guidance:** This pattern validates combinations not covered by native
dependent picklists. Prefer native dependent picklists where possible;
use formula validation only when combination logic is complex.

---

## Pattern 8: Prevent Save if Related Records Exist (Cross-Object)

**Use case:** Cannot set Account Status to "Inactive" if related open
Opportunities exist.

```
AND(
  $Profile.Name <> "System Administrator",
  TEXT(Status__c) = "Inactive",
  PRIORVALUE(TEXT(Status__c)) <> "Inactive",
  Account_Open_Opp_Count__c > 0
)
```

**Guidance:** Requires a roll-up summary field (`Account_Open_Opp_Count__c`)
on the Account object. Direct cross-object formula fields cannot aggregate
child record counts in validation rules.

---

## Pattern 9: Enforce Required Field on Edit When Stage Advances

**Use case:** Require Forecast Category to be set when Stage moves to
"Proposal/Price Quote" or beyond.

```
AND(
  $Profile.Name <> "System Administrator",
  NOT(ISNEW),
  ISCHANGED(StageName),
  OR(
    TEXT(StageName) = "Proposal/Price Quote",
    TEXT(StageName) = "Value Proposition",
    TEXT(StageName) = "Id. Decision Makers",
    TEXT(StageName) = "Perception Analysis",
    TEXT(StageName) = "Closed Won"
  ),
  ISBLANK(ForecastCategoryName)
)
```

---

## Pattern 10: Multi-Select Picklist Validation

**Use case:** Block save if "Sensitive Data" is selected in a multi-select
picklist without the DPA checkbox being checked.

```
AND(
  $Profile.Name <> "System Administrator",
  INCLUDES(Data_Categories__c, "Sensitive Data"),
  Data_Processing_Agreement_Signed__c = FALSE
)
```

**Error message:** "DPA must be signed before selecting Sensitive Data categories."

---

## Anti-Patterns to Avoid

| Anti-pattern | Why it fails | Fix |
|---|---|---|
| `Field__c = ""` | Does not catch null — only catches empty string on text | Use `ISBLANK(Field__c)` |
| `Field__c = null` | Syntax error in Salesforce formula | Use `ISNULL(Field__c)` |
| `ISCHANGED` without `NOT(ISNEW)` | Fires on every new record as if field changed | Gate with `NOT(ISNEW)` |
| `PRIORVALUE` without `NOT(ISNEW)` | Returns null on new records; unexpected behavior | Gate with `NOT(ISNEW)` |
| No profile bypass | Blocks admins during data migrations and sandbox refreshes | Always include System Administrator bypass |
| `Text_Picklist__c = "Value"` without TEXT | Syntax error — picklists cannot be compared directly | Use `TEXT(Text_Picklist__c) = "Value"` |
| Hardcoding Profile Names | Breaks when profile is renamed | Use Custom Permissions for production-grade bypass |
