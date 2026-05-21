# Error Message Style Guide

## Core Principles

Salesforce validation rule error messages are the user's only guidance when
a save is blocked. A good error message tells the user exactly what to do
in plain language. A bad error message tells the user what they did wrong
in technical language.

**Golden rule:** Start with the action required, not the error that occurred.

---

## Length and Placement

| Attribute | Guideline |
|---|---|
| Optimal length | 10–20 words |
| Maximum length | 40 words (beyond this, users stop reading) |
| Placement | Field-level for single-field errors; page-level for multi-field or complex logic |
| Casing | Sentence case — capitalize only the first word and proper nouns |

---

## Message Templates by Pattern

### Required field
```
[Field name] is required to save this [record type].
```
Example: `"Billing Country is required to save this Partner account."`

### Required when another field has a value
```
[Field B] is required when [Field A] is [value].
```
Example: `"Close Date is required when Stage is Proposal or later."`

### Date cannot be in the past
```
[Field] must be today or a future date.
```
Example: `"Expected Close Date must be today or a future date."`

### Stage / status cannot regress
```
[Object] cannot move from [from state] to [to state]. Contact [team] to change.
```
Example: `"Opportunity stage cannot move backwards once Contract Sent. Contact Sales Ops to change."`

### Conditional block based on related data
```
[Action] is not allowed while [condition]. [How to unblock].
```
Example: `"Account cannot be marked Inactive while open Opportunities exist. Close or reassign open deals first."`

### Dependent field validation
```
[Sub-field] must match a valid option for [controlling field] = [value].
```
Example: `"Sub-type must match a valid option for Type = Hardware."`

---

## Words to Avoid

| Avoid | Replace with |
|---|---|
| "null" | "blank", "empty", or "missing" |
| "API name" | the field's display label |
| "formula" | (never mention formulas) |
| "validation error" | state what needs to change |
| "field value" | the field's display label |
| "boolean" / "true" / "false" | "checked" / "unchecked" for checkboxes |
| "exception" | (do not expose technical exceptions in error messages) |
| "required by" | "required to" or just state the requirement |

---

## Jargon-Free Examples

| Bad (technical) | Good (actionable) |
|---|---|
| "CloseDate field validation formula returned TRUE." | "Close Date must be today or a future date." |
| "Type_Mismatch_Exception: null value for non-null field Billing_Country__c." | "Billing Country is required for Partner accounts." |
| "Validation rule StageLock_NoRegress fired." | "Stage cannot move backwards once Contract Sent." |
| "AND condition failed: missing required fields." | "Phone and Email are both required for Outbound leads." |
| "RecordType.DeveloperName validation blocked save." | "Executive Sponsor is required on Enterprise accounts." |

---

## Multi-Field Error Messages

When the rule enforces multiple fields simultaneously, list them:

```
Phone and Email are required for Outbound Call leads.
```

Not:
```
Required fields are missing for this lead type.
```

If three or more fields are involved, use a page-level error to provide
enough context. Keep the message to two sentences maximum.

---

## Pointing to the Fix

The best error messages tell the user not just what is wrong but exactly
how to fix it. Add a second sentence for complex conditions:

| Condition | Fix pointer |
|---|---|
| Related record blocks save | "Close or reassign open Opportunities first." |
| Permission required | "Contact your Sales Ops team to change this." |
| Date range issue | "Set a date in the current or next quarter." |
| DPA required | "DPA must be signed in the Agreements tab before selecting this." |

---

## Testing Your Message

Before shipping, ask three questions:

1. Can a non-admin understand this message without opening a help article?
2. Does it tell the user exactly what field or value to change?
3. Would a user misinterpret what "fix" means from this message?

If any answer is "no", rewrite.
