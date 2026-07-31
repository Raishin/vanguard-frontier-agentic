---
name: salesforce-validation-rule-writer-skill
description: "Converts business rules described in plain English into deployable Salesforce validation rule formula syntax, including error message copy, profile bypass logic, null handling, and formula compilation safety. TRIGGER when: user says write a validation rule, create validation rule formula, block user from saving if, require field when, prevent save when, enforce that field must be filled. Trigger phrases: validation rule for, formula to block, required field logic, save condition. DO NOT TRIGGER when: reviewing existing validation rules for quality (use salesforce-platform-admin-review-agent), deploying validation rules (use salesforce-deployment-validator-skill), bulk updating multiple rules across objects (use salesforce-bulk-data-ops-skill)."
license: MIT
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-21"
  category: generation
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# salesforce-validation-rule-writer-skill

Pure-generation T0 skill that translates plain-English business rules into
production-ready Salesforce validation rule formula syntax. Outputs the
formula, a clear error message, profile bypass guidance, null handling,
and a formula compilation checklist. Nothing is deployed — this skill
generates text only.

## When This Skill Owns the Task

Use `salesforce-validation-rule-writer-skill` when the request is to **write
or design** a validation rule formula:

- "Write a validation rule that requires Billing Country when the account type is Partner"
- "Block reps from saving an Opportunity with Stage = Closed Won if Amount is blank"
- "Create a formula to enforce that Close Date cannot be in the past on new records"
- "Require phone number when Lead Source is Outbound"
- "Prevent picklist selection of Status = Inactive if there are open Cases"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Reviewing or auditing existing validation rule logic | `salesforce-platform-admin-review-agent` |
| Deploying the generated formula to an org | `salesforce-deployment-validator-skill` |
| Bulk enabling/disabling many rules across objects | `salesforce-bulk-data-ops-skill` |
| Need to check what existing rules are on an object | `salesforce-metadata-fetcher-skill` |

---

## Required Context to Gather First

Before generating a formula, confirm:

1. **Target object** — standard (Account, Contact, Opportunity, Lead, Case)
   or custom (`My_Object__c`).
2. **Field API names** — the exact API names of every field referenced in the
   rule. Ask if only labels are provided.
3. **Trigger condition** — precisely when should the save be blocked? Describe
   the AND/OR logic in English.
4. **Record types in scope** — does the rule apply to all record types or only
   specific ones? (Affects `RecordType.DeveloperName` branching.)
5. **New vs. edit context** — should the rule fire on new records only, edits
   only, or both? (`ISNEW`, `ISCHANGED` implications.)
6. **Profile bypass requirement** — which profiles (if any) should be exempt?
   (System Admin bypass is almost always required.)
7. **Error message placement** — field-level error or page-level error?
8. **Picklist field types** — if picklist fields are used, confirm the exact
   API values (not labels) of the controlling values.

If any critical context is missing, ask before generating.

---

## Recommended Workflow

### Step 1 — Confirm and rephrase the business rule

Restate the rule in your own words to confirm understanding:

> "This rule fires when: [condition]. It blocks save with: [error text].
> It is bypassed for: [profile/record type]. It applies on: [new/edit/both]."

Ask for confirmation before proceeding.

### Step 2 — Decompose into atomic conditions

Break the rule into atomic boolean conditions. Map each condition to
the appropriate Salesforce formula function:

| Condition type | Formula pattern |
|---|---|
| Field is blank | `ISBLANK(Field__c)` for text; `ISNULL(Field__c)` for number/date |
| Field equals picklist value | `TEXT(Field__c) = "API_Value"` |
| Field changed | `ISCHANGED(Field__c)` |
| New record | `ISNEW` |
| Prior value comparison | `PRIORVALUE(Field__c)` |
| Record type check | `RecordType.DeveloperName = "RecordTypeName"` |
| Date in past | `Field__c < TODAY` |

### Step 3 — Compose the formula with AND/OR/NOT

Wrap atomic conditions using:
- `AND(cond1, cond2)` — both must be true
- `OR(cond1, cond2)` — either must be true
- `NOT(cond)` — negation

Prefer explicit `AND`/`OR` over `&&`/`||` operators for readability
and tooling compatibility. Nest carefully — Salesforce formulas have a
5,000-character compiled size limit.

### Step 4 — Add profile bypass

Unless the customer explicitly opts out, wrap the core logic with a
System Administrator bypass:

```
AND(
  $Profile.Name <> "System Administrator",
  <your_core_formula>
)
```

For additional profiles, use:
```
AND(
  NOT(OR(
    $Profile.Name = "System Administrator",
    $Profile.Name = "Sales Ops"
  )),
  <your_core_formula>
)
```

### Step 5 — Apply null handling

Validate that every text, date, and number field access guards against
null where needed:

- Text fields: use `ISBLANK` not `= ""`
- Number/currency fields: use `ISNULL` or `BLANKVALUE(Field__c, 0)` before
  arithmetic
- Date fields: use `ISNULL` or compare after null guard
- Picklist fields: use `TEXT(Field__c) = "Value"` not `Field__c = "Value"`
- Multi-select picklists: use `INCLUDES(Field__c, "Value")`

### Step 6 — Write the error message

Apply these principles (see `references/error-message-style.md`):

- 15 words or fewer for the primary instruction
- Start with what the user must do, not what they did wrong
- Name the specific field or value required
- Include the condition context when space allows
- No jargon: avoid "null", "API name", "formula"

Good: `"Close Date cannot be in the past. Set a future date to save."`
Bad: `"Validation error: CloseDate field value failed validation formula."`

### Step 7 — Compile safety check

Before emitting, verify mentally:

- [ ] Formula evaluates to TRUE when the rule should block (not the inverse)
- [ ] No division by zero if arithmetic is used (use `IF(divisor = 0, ...)`)
- [ ] PRIORVALUE only used on edits (wrap with `NOT(ISNEW)`)
- [ ] ISCHANGED only used on edits (same)
- [ ] All referenced fields exist on the stated object
- [ ] No cross-object formula deeper than 10 relationship hops
- [ ] No circular reference risk (field does not reference itself)
- [ ] Picklist values use TEXT wrapper

### Step 8 — Emit structured output

Produce the full output block per the Output Format section below.

---

## Quality Scoring Rubric (100-point)

Score every output before emitting. Threshold: 85+ ship, 70–84 ship with
caveat, below 70 reject and revise.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Formula correctness** | 30 | Formula evaluates to TRUE when it should block; correct operators; picklist TEXT wrapper; ISBLANK/ISNULL appropriately placed |
| **Error message clarity** | 20 | ≤ 15 words, action-oriented, names the required field or condition, no technical jargon |
| **Profile bypass awareness** | 15 | System Admin bypass present unless explicitly opted out; additional profiles listed; bypass wrapper syntactically correct |
| **Null handling** | 15 | Every field access null-guarded; no runtime errors on blank fields; BLANKVALUE used where arithmetic is involved |
| **Governor limit safety** | 10 | Compiled size < 5,000 chars; no cross-object formula chain deeper than 5 hops; no expensive SOQL-like patterns in formulas |
| **Formula compilation safety** | 10 | PRIORVALUE/ISCHANGED gated by NOT(ISNEW); no division by zero; no circular references; all functions used with correct argument counts |

**Scoring penalties:**
- Formula evaluates to FALSE when it should block (inverted logic): -30
- Missing null guard on a field that can be blank in the target object: -10 per field (max -20)
- Missing System Admin bypass when not explicitly opted out: -10
- Error message exceeds 30 words: -10
- PRIORVALUE used without ISNEW guard: -15

---

## T0 Contract

This skill is **static-review (T0)** only:

- No Salesforce org connection required or permitted.
- No CLI commands are executed.
- No metadata is fetched from a live org.
- Output is a text formula and supporting notes only.
- The user must copy the formula into Setup → Object Manager → Validation Rules
  or deploy it via `salesforce-deployment-validator-skill`.

---

## Refusal Triggers

Stop and decline if:

- The request is to review or audit an existing formula (route to
  `salesforce-platform-admin-review-agent`).
- The request is to deploy the formula to a live org (route to
  `salesforce-deployment-validator-skill`).
- The formula requested would bypass all validation for all profiles
  permanently (security risk — flag and decline).
- The target object cannot be determined after two clarification attempts.

---

## Output Format

```yaml
verdict: "ship | ship-with-caveat | reject"
quality_score: <0-100>
quality_notes: "<what drove the score>"

validation_rule:
  object_api_name: "<SObjectApiName>"
  rule_name_suggestion: "<DescriptiveRuleName_NoSpaces>"
  active: true
  description: "<one-sentence plain English description>"
  error_condition_formula: |
    <multiline formula text>
  error_message: "<≤15 word actionable message>"
  error_display_field: "<FieldApiName | blank for page-level>"

profile_bypass:
  included: <true|false>
  profiles_bypassed: ["System Administrator"]
  notes: "<any bypass caveats>"

null_handling_notes: "<summary of null guards applied>"

compilation_checklist:
  formula_is_true_when_blocking: <true|false>
  priorvalue_gated_by_isnew: <true|false|not_applicable>
  ischanged_gated_by_isnew: <true|false|not_applicable>
  no_division_by_zero: <true|false>
  all_picklists_use_text_wrapper: <true|false|not_applicable>
  compiled_size_estimate: "<characters>"

deployment_notes: "<instructions for deploying in Setup UI or via Change Set>"

handoff_suggestions:
  - "<any recommended follow-up actions>"

assumptions:
  - "<explicit list of assumptions made>"
```

---

## Redaction Rules

This is a T0 generation skill with no live org connection. Apply:

1. Never emit actual production org IDs, user IDs, or credentials even if
   the user includes them in their prompt.
2. Sanitize any pasted metadata that includes Org IDs (18-char starting `00D`)
   or User IDs (18-char starting `005`) before incorporating into output.
3. Do not include field values that appear to be real customer data (e.g.,
   email addresses, account names, SSNs) in example output.

---

## Handoff Rules

| Situation | Hand off to |
|---|---|
| User wants to deploy the generated rule | `salesforce-deployment-validator-skill` |
| User wants to review existing rules on the object | `salesforce-metadata-fetcher-skill` → `salesforce-platform-admin-review-agent` |
| Formula involves complex record types or permission topology | `salesforce-permission-model-review-skill` |
| User needs bulk enable/disable of rules | `salesforce-bulk-data-ops-skill` |

---

## Stop Conditions

- Requested formula would unconditionally block all saves on a standard
  critical object (Account, Contact, Opportunity) without any bypass — stop
  and require explicit confirmation plus bypass design.
- Formula references fields that do not exist on the stated object — stop,
  list the missing fields, ask for correct API names.
- Rule logic contradicts itself (e.g., require field X when X is already
  required by a page layout rule that cannot be changed) — stop and
  surface the conflict.

---

## Security Notes

- **T0 static generation only:** No org connection, no CLI, no MCP calls.
- **No credential handling:** Skill never requests or emits OAuth tokens,
  session IDs, or Connected App credentials.
- **Profile bypass required by default:** Unconditional validation rules
  without System Admin bypass are a common data-migration blocker and
  governance risk — always include unless explicitly opted out with
  documented reason.
- **Formula compilation safety:** Inverted formula logic (evaluates FALSE
  when it should block) is the most common validation rule bug. The
  compilation checklist catches this before emission.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/formula-syntax-quickref.md` | Operators, ISBLANK/ISNULL, AND/OR/NOT, TEXT/VALUE/DATEVALUE, BLANKVALUE, PRIORVALUE |
| `references/validation-patterns.md` | Required-when patterns, prevent-bypass-by-profile, conditional require by record type, dependent picklist validation |
| `references/error-message-style.md` | Concise actionable errors, no jargon, point to fix |
