---
name: salesforce-apex-test-generator-skill
description: "Generates Apex test classes with TestDataFactory patterns, Assert class usage, bulkification (200+ records), positive/negative/bulk test method separation, async test patterns (Test.startTest/stopTest), and proper @TestSetup usage. T0 static generation — no org connection required. TRIGGER when: user asks to write Apex test classes, create @isTest code, generate test setup data, scaffold test coverage for a class, or add a test class for a .cls file. Trigger phrases: write apex tests, test class for, @isTest class, test setup data, generate test coverage. DO NOT TRIGGER when: user wants to execute tests against a live org (use salesforce-apex-test-runner-skill), generating production Apex logic (use salesforce-apex-generator-skill), debugging test failures from log output (use salesforce-apex-log-analyzer-skill)."
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

# salesforce-apex-test-generator-skill

T0 static code generation for Apex test classes. This skill authors test coverage
that is **bulkified, assertion-rich, and factory-patterned** — the three qualities
most often missing from hand-rolled Apex tests. No org connection required.

## When This Skill Owns the Task

Use `salesforce-apex-test-generator-skill` when the work requires **authoring new test classes**:

- "Write tests for the AccountService class"
- "Generate a @isTest class for my Queueable job"
- "Create test setup data using TestDataFactory"
- "Add positive, negative, and bulk test methods for OrderTriggerHandler"
- "Scaffold an @TestSetup method for the Contact selector tests"
- "Write async test patterns for my batch class"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| User wants to run the tests against a live org | `salesforce-apex-test-runner-skill` |
| User needs the production Apex class authored first | `salesforce-apex-generator-skill` |
| User is analyzing a test failure from logs | `salesforce-apex-log-analyzer-skill` |
| User needs to deploy validated code | `salesforce-deployment-validator-skill` |

---

## Required Context to Gather First

Before generating, confirm:

1. **Class under test** — the production Apex class name and its purpose
2. **Class type** — service, selector, domain, batch, queueable, schedulable, invocable, REST resource, trigger
3. **SObjects involved** — what objects the production class creates, reads, or updates
4. **TestDataFactory availability** — does the project already have a TestDataFactory class?
   If yes, use it. If no, scaffold a minimal factory or factory pattern.
5. **Key behaviors to test** — what are the main positive paths, exception paths, and
   governor-limit edge cases?
6. **API version** — default `62.0` minimum

If the user provides the production class source, read it to infer what test scenarios
are needed. If not, ask for the class name and its purpose.

---

## Recommended Workflow

### Step 1 — Read the production class (if available)

Use `Read` or `Grep` to locate the class under test. Identify:
- Public methods that need coverage
- Exception paths (try/catch blocks, explicit throws)
- DML operations that need TestDataFactory data
- Async paths (Queueable, Batch, Schedulable enqueue/execute calls)
- callout paths (HTTP callout mocks needed?)

### Step 2 — Design test structure

Map each method to test scenarios using the three-path model:

| Path | What to test |
|---|---|
| **Positive** | Happy path with valid data; asserts on expected outcome |
| **Negative** | Invalid input, missing required data, exception thrown; asserts on expected error |
| **Bulk** | 200+ record insert/update/delete; asserts governor-limit safety |

### Step 3 — Plan TestDataFactory usage

Consult `references/test-data-factory.md`. If a project factory exists, extend it.
If not, scaffold factory methods as static helpers in the test class or a companion
`TestDataFactory` class.

### Step 4 — Author the test class

Follow all hard-stop constraints (see Rules section). Generate:
- `@isTest` class declaration with `private` access
- `@TestSetup` for shared data (when multiple methods need the same records)
- Separate `@isTest` methods for each positive, negative, and bulk scenario
- `Test.startTest` / `Test.stopTest` wrapping all async enqueue or execute calls
- `Assert` class usage throughout (never bare `System.assert`)
- No `SeeAllData=true` unless explicitly required by an external tool pattern

### Step 5 — Generate metadata XML

Produce `{ClassName}Test.cls-meta.xml` with correct `apiVersion` and `status: Active`.

### Step 6 — Score against the quality rubric

Run the 100-point rubric (see below). If score is below 80, revise before presenting.

### Step 7 — Recommend test runner pairing

Always end with an explicit recommendation to execute the tests using
`salesforce-apex-test-runner-skill`, noting the test class name and expected coverage.

---

## Rules

### Hard-Stop Constraints (Must Enforce)

| Constraint | Rationale |
|---|---|
| Never use `@isTest(SeeAllData=true)` unless testing external tools that require live data | Test isolation; prevents reliance on org-specific state |
| Every test method must have at least one `Assert` statement | Tests without assertions give false confidence |
| Bulk test methods must use 200+ records | Triggers process in batches of 200; 201 records crosses the boundary |
| Use `Assert` class methods (`Assert.areEqual`, `Assert.isTrue`, etc.) not bare `System.assert` | Produces meaningful failure messages; follows Apex best practices |
| Use `Test.startTest` / `Test.stopTest` around all async enqueue or execute calls | Flushes async queue; required for governor-limit isolation in tests |
| All DML in test setup must use `insert` not `Database.insert` unless testing saveResults | Clarity; reserve `Database.insert` for tests specifically checking partial success |
| Do not hardcode record Ids in test assertions | Ids vary by org; use the returned record.Id |
| Separate positive, negative, and bulk methods — do not combine in one method | Maintainability and clarity of failure attribution |

### Naming Conventions

| Element | Pattern | Example |
|---|---|---|
| Test class | `{ProductionClass}Test` | `AccountServiceTest` |
| Positive test | `test{Method}Success` or `test{Scenario}` | `testGetAccountByOwnerSuccess` |
| Negative test | `test{Method}ThrowsWhenInvalid` or `test{Scenario}Error` | `testGetAccountByOwnerThrowsWhenNullId` |
| Bulk test | `test{Method}Bulk` or `test{Scenario}Bulk` | `testGetAccountByOwnerBulk` |
| @TestSetup | `setup` (conventional) | `static void setup` |

---

## Quality Scoring Rubric (100-point)

Score the test class before presenting. Threshold: 80+ pass, 60–79 caveat, below 60 revise.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Bulkification** | 25 | At least one bulk test method with 200+ records; asserts on all bulk records not just first |
| **Positive/negative/bulk coverage** | 25 | All three paths present and distinct; each method has a clear purpose |
| **TestDataFactory used** | 15 | Factory methods used or scaffolded; no inline SObject construction scattered across methods |
| **Assert class used** | 15 | `Assert.areEqual`, `Assert.isTrue`, `Assert.isNotNull` used throughout; no bare `System.assert(condition)` |
| **Governor-limit aware** | 10 | `Test.startTest` / `Test.stopTest` wraps async paths; no unnecessary SOQL in each test method |
| **No SeeAllData** | 10 | `@isTest(SeeAllData=true)` is absent unless explicitly justified with a comment |

**Scoring penalties:**
- Missing bulk test: -20
- Bare `System.assert` without message: -10 per occurrence (max -20)
- `SeeAllData=true` without justification: -15
- No `Test.startTest` on async path: -10
- Assertions only on first record in bulk test: -10
- Missing `@TestSetup` when multiple methods share data setup: -5

---

## T0 Contract

This skill operates exclusively at T0 — static generation only.

- **No org connection:** No `sf` CLI calls, no MCP tool calls, no live execution.
- **No OAuth required:** Zero-scope, zero-credential, zero-network.
- **Output is draft code:** All generated test classes are starting points for review.
  Coverage percentages are estimated, not measured. Use `salesforce-apex-test-runner-skill`
  for actual execution and coverage measurement.

---

## Refusal Triggers

Stop and do not generate if:

- The user requests `SeeAllData=true` for a standard use case — explain the isolation risk
  and generate test-isolated factory data instead. Only emit `SeeAllData=true` if the user
  provides a documented reason (e.g., testing a legacy package that requires live data).
- The user requests test methods that assert on hardcoded record Ids — explain the org-portability
  risk and generate assertions using returned record Ids instead.

---

## Output Format

```yaml
verdict: "pass | caveat | reject"
quality_score: <0-100>
quality_notes: "<scoring rationale>"

generated_files:
  - path: "{ClassName}Test.cls"
    content: |
      <apex test code>
  - path: "{ClassName}Test.cls-meta.xml"
    content: |
      <meta xml>

test_scenarios_covered:
  positive: ["<method name: scenario description>"]
  negative: ["<method name: scenario description>"]
  bulk: ["<method name: scenario description>"]

factory_approach: "existing-factory | scaffolded-factory | inline-helpers"
factory_notes: "<what factory methods were used or created>"

estimated_coverage_areas:
  - "<class/method: coverage area>"

async_patterns_used: ["Test.startTest/stopTest", "callout mock", "schedulable mock"]

runner_recommendation:
  companion_skill: "salesforce-apex-test-runner-skill"
  test_class: "{ClassName}Test"
  expected_coverage_target: ">=75%"

assumptions:
  - "<list of assumptions made>"

missing_context:
  - "<what the user should provide to improve coverage>"
```

---

## Handoff Rules

| Output | Hand off to |
|---|---|
| Generated test class ready for execution | `salesforce-apex-test-runner-skill` (T1) |
| Production class still needed | `salesforce-apex-generator-skill` |
| Test failures need log analysis | `salesforce-apex-log-analyzer-skill` |
| Ready for sandbox deploy | `salesforce-deployment-validator-skill` (T2) |

---

## Stop Conditions

Stop and do not continue if:

- The production class under test cannot be located and the user has not provided its
  source or purpose — ask for the class content or a description of its methods.
- The requested test patterns would require live org data not expressible via factory
  records — explain the limitation and recommend an approach.

---

## Security Notes

- **T0 static generation only:** No org connection, no OAuth, no secrets.
- **No SeeAllData by default:** Test isolation is enforced. Live org data access in tests
  is a security and reliability anti-pattern.
- **No credential generation:** Test classes never contain hardcoded credentials, org Ids,
  or session tokens.
- **Factory data only:** All test data is generated via factory methods in the test class
  or `TestDataFactory`. No reliance on existing org records.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/test-data-factory.md` | TestDataFactory pattern, builder pattern, field overrides, lazy init, duplicate rule handling |
| `references/assertion-patterns.md` | Assert class methods, error messages, multi-assert vs single, bulk assertion patterns |
| `references/async-testing.md` | Test.startTest/stopTest, mocking Schedulable/Batchable, HTTP callout mocks |
