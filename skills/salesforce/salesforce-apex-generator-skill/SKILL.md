---
name: salesforce-apex-generator-skill
description: "Generates production-grade Apex classes with Service-Selector-Domain layering, correct sharing models (with sharing / without sharing / inherited sharing per class type), async patterns (Queueable, Batchable, Schedulable), and governor-limit awareness. T0 static generation — no org connection required. TRIGGER when: user asks to write an Apex class or trigger, generate a service/selector/domain layer, create an async job, implement a REST resource, scaffold a .cls file, or port business logic to Apex. Trigger phrases: write apex class, generate apex trigger, create apex service, .cls file, implement queueable, create batch job. DO NOT TRIGGER when: live test execution needed (use salesforce-apex-test-runner-skill), generating test classes specifically (use salesforce-apex-test-generator-skill), debugging an existing class from log evidence (use salesforce-apex-log-analyzer-skill), or the user needs a live deployment (use salesforce-deployment-validator-skill)."
license: MIT
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
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

# salesforce-apex-generator-skill

T0 static code generation for production-grade Apex. This skill is a **forge**, not a
flashlight — it authors correct, deployable Apex with the right sharing model, governor-limit
safety, and security defaults for each class type. No org connection required.

## When This Skill Owns the Task

Use `salesforce-apex-generator-skill` when the work requires **authoring new Apex code**:

- "Write an AccountService class that queries all Accounts by owner"
- "Generate a Queueable that sends platform events for Order updates"
- "Create a Schedulable wrapper for a batch job"
- "Scaffold the Selector layer for the Contact object"
- "Implement a Domain class with trigger logic for Opportunity"
- "Write a REST resource endpoint for an external integration"
- "Create a .cls file for our deduplication logic"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| User needs test classes specifically | `salesforce-apex-test-generator-skill` |
| User wants to run tests against a live org | `salesforce-apex-test-runner-skill` |
| User is debugging a log file or stack trace | `salesforce-apex-log-analyzer-skill` |
| User needs to deploy and validate | `salesforce-deployment-validator-skill` |
| Static code review of existing Apex | `salesforce-apex-lwc-code-review-skill` |

---

## Required Context to Gather First

Before generating, confirm:

1. **Class type** — service, selector, domain, batch, queueable, schedulable, invocable,
   trigger, trigger-handler, REST resource, DTO, utility, interface, exception
2. **Target object(s)** — the SObject API names involved (e.g., `Account`, `Opportunity__c`)
3. **Business goal** — what the class must accomplish
4. **Class name** — derive from naming conventions if not provided
5. **Existing patterns** — does the project use a trigger framework (TAF, MetadataTriggerHandler)?
   Service-Selector-Domain already in place?
6. **API version** — default `62.0` minimum
7. **Test class expected?** — always recommend pairing with `salesforce-apex-test-generator-skill`

If the user provides a clear, complete request, generate immediately without unnecessary
back-and-forth. Apply defaults (see Rules section) for anything not specified.

---

## Recommended Workflow

### Step 1 — Identify class type and infer defaults

Map the request to a class type using the type table below. Set defaults:
- Sharing: `with sharing` unless type-specific exception applies
- Access modifier: `public` unless `global` is required
- ApexDoc: always include
- API version meta XML: always generate

### Step 2 — Choose the minimal correct pattern

Consult `references/apex-patterns.md` for Service-Selector-Domain conventions,
async patterns, and type-specific templates.

### Step 3 — Apply all hard-stop constraints

Before writing a single line, verify every constraint in the Rules section would be
satisfied. If a constraint would be violated by the user's request, explain the problem
and propose the correct approach.

### Step 4 — Generate the class file

Produce `{ClassName}.cls` with:
- Correct `with sharing` / `without sharing` / `inherited sharing` declaration
- ApexDoc comment block (purpose, author placeholder, version)
- All SOQL outside loops
- All DML outside loops
- No `@future` methods — use Queueable
- Bind variables for any dynamic SOQL
- Proper exception handling
- No hardcoded Ids or credentials

### Step 5 — Generate the metadata XML

Produce `{ClassName}.cls-meta.xml` using the correct `apiVersion` and `status: Active`.

### Step 6 — Score against the quality rubric

Run the 100-point scoring rubric (see below). If the score is below 80, revise before
presenting. Document score and notes in the output.

### Step 7 — Recommend test class pairing

Always end with an explicit recommendation to generate a companion test class using
`salesforce-apex-test-generator-skill`, noting the class name and key scenarios to cover.

---

## Class Type Reference

| Class Type | Sharing Model | Access | Key Rules |
|---|---|---|---|
| Service | `with sharing` | `public` | No SOQL/DML in loops; inject Selector |
| Selector | `with sharing` | `public` | Returns typed lists; no business logic |
| Domain | `with sharing` | `public` | Trigger-context aware; delegates to Service |
| Trigger | N/A (handler delegates) | — | One trigger per object; delegates to Domain/handler |
| Batchable | `without sharing` or `with sharing` | `public` | Implements `Database.Batchable`; `Database.Stateful` only if required |
| Queueable | `with sharing` | `public` | Implements `Queueable`; use `System.Finalizer` for chaining; replaces `@future` |
| Schedulable | `with sharing` | `public` | Implements `Schedulable`; delegates to Queueable or Batch |
| Invocable | `with sharing` | `public` | `@InvocableMethod`; typed inner request/result classes |
| REST Resource | `without sharing` (framework handles sharing) | `global` | `@RestResource`; `@HttpGet`/`@HttpPost` etc. |
| DTO / Wrapper | N/A | `public` | No methods with side effects; serializable |
| Utility | `with sharing` | `public` | Static methods only; no instance state |
| Exception | N/A | `public` | Extends `Exception`; no override of `getMessage` |

---

## Rules

### Hard-Stop Constraints (Must Enforce)

| Constraint | Rationale |
|---|---|
| Place all SOQL outside loops | Avoid query governor limit (100 queries per transaction) |
| Place all DML outside loops | Avoid DML statement governor limit (150 per transaction) |
| Declare sharing keyword on every class | Prevent unintended `without sharing` defaults and data leakage |
| Use Custom Metadata / Labels / describe calls instead of hardcoded Ids | Portability across orgs |
| Always handle exceptions (log, rethrow, or recover) | Prevent silent failures |
| Use bind variables for all dynamic SOQL accepting user input | Prevent SOQL injection |
| Use Apex-native collection types | Prevent compile errors from Java-style types |
| Never use `@future` methods | Use Queueable with `System.Finalizer`; `@future` cannot chain or accept non-primitive types |
| No `System.debug()` in main code paths | Debug statements consume CPU; use a logging framework |
| Security: use `WITH USER_MODE` on SOQL in classes that touch user-controlled data | Enforces FLS/CRUD at query time |
| Apply `Security.stripInaccessible()` when constructing SObjects from user input | Strips fields the user cannot access |

### Naming Conventions

| Type | Pattern | Example |
|---|---|---|
| Service | `{SObject}Service` | `AccountService` |
| Selector | `{SObject}Selector` | `AccountSelector` |
| Domain | `{SObject}Domain` | `AccountDomain` |
| Handler | `{SObject}TriggerHandler` | `AccountTriggerHandler` |
| Batch | `{Purpose}Batch` | `AccountDeduplicationBatch` |
| Queueable | `{Purpose}Queueable` | `OrderEventQueueable` |
| Schedulable | `{Purpose}Scheduler` | `NightlyCleanupScheduler` |
| DTO | `{Purpose}Dto` or `{Purpose}Wrapper` | `OrderResponseDto` |

---

## Quality Scoring Rubric (100-point)

Score the generated class before presenting. Threshold: 80+ pass, 60–79 caveat with explanation, below 60 revise before presenting.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Sharing model correctness** | 25 | Every class has an explicit sharing declaration; `with sharing` is used wherever FLS/CRUD enforcement is appropriate; `without sharing` is justified with a comment |
| **Governor-limit safety** | 25 | No SOQL in loops; no DML in loops; bulkified with List/Map patterns; Collections used correctly |
| **Security defaults** | 20 | `WITH USER_MODE` or `Security.stripInaccessible()` applied where appropriate; no hardcoded Ids or credentials; no SOQL injection vectors |
| **Naming conventions** | 15 | Class and method names follow conventions table; ApexDoc present; XML meta generated |
| **Test class recommendation** | 15 | Companion test class explicitly recommended with key scenarios listed |

**Scoring penalties:**
- SOQL inside a loop: -20 (immediate caveat)
- DML inside a loop: -20 (immediate caveat)
- Missing sharing declaration: -15
- `@future` used instead of Queueable: -10
- Hardcoded Salesforce Id literal: -10
- No exception handling in a method with DML: -10
- Missing ApexDoc: -5

---

## T0 Contract

This skill operates exclusively at T0 — static generation only.

- **No org connection:** No `sf` CLI calls, no MCP tool calls, no live query or execution.
- **No OAuth required:** Zero-scope, zero-credential, zero-network.
- **Output is draft code:** All generated Apex is a starting point for human review and
  org-specific adaptation. No generated file should be deployed without a human reviewing
  the sharing model, naming conventions, and test coverage.
- **Handoff to T1/T2 for validation:** Pair with `salesforce-apex-test-generator-skill` for
  test authoring, then `salesforce-apex-test-runner-skill` (T1) for execution, then
  `salesforce-deployment-validator-skill` (T2) for sandbox dry-run before any deployment.

---

## Refusal Triggers

Stop and do not generate if:

- The requested logic would require hardcoding a production org Id or session token —
  explain and ask for Custom Metadata or Label approach instead.
- The request is to generate Apex that intentionally bypasses FLS/CRUD for user-visible
  data without a documented justification — explain the security risk and require
  explicit acknowledgment before generating `without sharing` on a user-data class.
- The user requests `@future` — explain the Queueable alternative and generate Queueable
  instead unless the user explicitly overrides with a documented reason.

---

## Output Format

```yaml
verdict: "pass | caveat | reject"
quality_score: <0-100>
quality_notes: "<scoring rationale>"

generated_files:
  - path: "{ClassName}.cls"
    content: |
      <apex code>
  - path: "{ClassName}.cls-meta.xml"
    content: |
      <meta xml>

sharing_model_used: "with sharing | without sharing | inherited sharing"
sharing_model_rationale: "<why this model was chosen>"

governor_limit_notes: "<any limit-adjacent patterns noted>"

security_notes: "<WITH USER_MODE / stripInaccessible usage>"

test_class_recommendation:
  companion_skill: "salesforce-apex-test-generator-skill"
  class_to_test: "{ClassName}"
  key_scenarios:
    - "<positive path>"
    - "<negative/exception path>"
    - "<bulk path (200+ records)>"

assumptions:
  - "<list of assumptions made>"

missing_context:
  - "<what the user should provide to improve the output>"
```

---

## Handoff Rules

| Output | Hand off to |
|---|---|
| Generated Apex needing test class | `salesforce-apex-test-generator-skill` |
| Generated Apex ready for test execution | `salesforce-apex-test-runner-skill` (T1) |
| Ready for sandbox validation | `salesforce-deployment-validator-skill` (T2) |
| Code review of existing/generated Apex | `salesforce-apex-lwc-code-review-skill` |

---

## Stop Conditions

Stop and do not continue if:

- The requested class type is not in the class type table and cannot be safely categorized —
  ask for clarification.
- The request would produce Apex that violates a hard-stop constraint and the user insists
  on the violating pattern after explanation — document the refusal and stop.
- The request requires live org data (field names, object relationships, current Apex) that
  the user has not provided — ask for the specific context needed.

---

## Security Notes

- **T0 static generation only:** No org connection, no OAuth, no secrets.
- **Sharing by default:** Every generated class has an explicit sharing declaration. `with sharing`
  is the default; `without sharing` only where explicitly required by class type (REST resource,
  certain batch contexts) and documented.
- **No credential generation:** This skill never generates hardcoded credentials, org Ids,
  user Ids, or session tokens in Apex code. All external references use Custom Metadata,
  Custom Labels, or Named Credentials.
- **Security-first patterns:** `WITH USER_MODE`, `Security.stripInaccessible()`, and
  bind variables are applied by default to all user-data-touching code.
- **Draft status:** All generated code is a starting point. Human review of sharing model,
  FLS patterns, and business logic correctness is required before deployment.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/apex-patterns.md` | Service-Selector-Domain pattern, sharing models, async patterns, type templates |
| `references/governor-limits.md` | Governor limit values, bulkification strategies, async fallback patterns |
| `references/security-defaults.md` | WITH SHARING default policy, USER_MODE, stripInaccessible, no hardcoded credentials |
