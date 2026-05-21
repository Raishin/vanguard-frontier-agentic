# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized test source (no secrets, no connection strings, no tokens, no tenant identifiers, no customer data — replace with placeholders):
- The test classes and fixtures under review (xUnit, NUnit, or MSTest).
- The mock or fake setups (Moq, NSubstitute, FakeItEasy) used by those tests.
- The integration-test harness: `WebApplicationFactory` setup, Testcontainers configuration, or shared database fixtures.
- The coverage configuration (`coverlet` settings, `.runsettings`, `ExcludeFromCodeCoverage` usage) and any coverage gate.
- The solution file or the CI test command, to confirm which test projects actually run.

If the solution file or CI test command is not provided, suite-inclusion findings are stated as `assumption (source absent)` — say so and ask for them.

### Step 2 — Assertion-quality audit

Confirm each test actually asserts behavior.

- A test method with no assertion — no `Assert.*`, no FluentAssertions `Should`, no `mock.Verify`, no `[ExpectedException]` / `Assert.Throws` — → HIGH. It proves nothing and inflates coverage.
- A test that asserts only a mock's own configured return (set up `mock.Setup(x => x.Get).Returns(v)` then asserts the result equals `v`, with the real code stubbed away) → HIGH tautological test: it passes regardless of the system under test.
- A test whose only assertion is `Assert.True(true)` or equivalent → HIGH.

### Step 3 — Mocking audit

Review what is mocked and what is verified.

- Mocking a type the team owns that carries real logic (a domain service, a calculator, a mapper) instead of exercising it → MEDIUM over-mocking: the test verifies a stub.
- Assertions made only against `mock.Verify(...)` with no assertion on the system's observable output, where output assertions are possible → HIGH (mock-only assertion).
- Recommend mocking only true external boundaries (clock, network, third-party SDK) and exercising owned logic for real.

### Step 4 — Coverage-gate audit

Review whether the coverage number reflects tested behavior.

- A coverage gate that counts generated code, migrations, or `[ExcludeFromCodeCoverage]`-marked code toward the percentage, or excludes whole assemblies to lift the number → HIGH coverage theater.
- No coverage gate at all, where the team treats a coverage number as a quality signal → HIGH.
- Recommend a gate scoped to hand-written production code, with exclusions justified and visible.

### Step 5 — Isolation audit

Review whether tests are independent.

- Integration tests sharing a mutable database with no per-test isolation (no transaction rollback, no respawn/reset, no fresh container per test class) → HIGH: tests pollute each other and pass or fail by run order.
- Tests sharing static or singleton mutable state across test classes → HIGH.
- Tests dependent on execution order, or on data left by a prior test → HIGH flaky pattern.
- `Thread.Sleep`-based waits in async or integration tests → MEDIUM flaky pattern; recommend deterministic waits.

### Step 6 — Suite-inclusion audit

Confirm every test project runs on the CI test gate.

- A test project present in the repo but not referenced by the solution's test run or the CI test command → HIGH: those tests never execute on the merge gate and the coverage they claim is fictional.
- Recommend including every test project in the CI test run, or removing it.

### Step 7 — Negative- and security-test audit

Review whether the dangerous paths are tested.

- Only happy-path tests, with no tests for unauthorized (401), forbidden (403), invalid-input (400), not-found (404), or concurrency-conflict paths → HIGH: defects hide in the paths nobody asserts.
- No tests asserting that an unauthenticated or under-privileged caller is rejected on protected endpoints → HIGH security-test gap.
- Recommend explicit negative tests for each guarded path.

### Step 8 — Brittleness audit

- Tests asserting on private fields, internal structure, or exact log strings → MEDIUM: they break on safe refactors and train the team to ignore red.
- Recommend asserting observable behavior through the public surface.

### Step 9 — Produce the output

Format findings using the Output contract section below.

---

## Evidence checklist

Before writing findings, confirm which inputs were actually provided:
- [ ] Test classes and fixtures
- [ ] Mock / fake setups
- [ ] Integration-test harness (WebApplicationFactory, Testcontainers, DB fixtures)
- [ ] Coverage configuration and gate
- [ ] Solution file or CI test command

Each unchecked item downgrades the related findings to `inference (partial source)` or `assumption (source absent)`.

---

## Findings rubric

| Severity | Criteria |
|----------|----------|
| critical | Reserved for a confirmed false-confidence pattern that demonstrably ships a known defect class with no test coverage and an explicitly disabled or excluded gate. |
| high | Assertion-free tests; tautological mock-only tests; coverage theater or no coverage gate; shared-mutable-state integration tests; test projects excluded from the CI run; missing negative and security tests. |
| medium | Over-mocking owned logic; brittle tests on internal structure; `Thread.Sleep`-based waits. |
| low | Minor naming, organization, or readability issues in otherwise sound tests. |

Every finding carries an evidence-basis label: `confirmed (test source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

---

## Output contract

Return findings in this structure:

```
## Verdict
<pass | pass-with-conditions | block>

## Evidence level
<full test source provided | partial source | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding> — <evidence basis> — <description> — <remediation>

### HIGH
- [H1] <finding> — <evidence basis> — <description> — <remediation>

### MEDIUM
- [M1] <finding> — <evidence basis> — <description> — <remediation>

### LOW
- [L1] <finding> — <evidence basis> — <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept secrets, connection strings, tokens, tenant identifiers, or customer data. Ask for test source with placeholders.
- This is a static review: never run the test suite, a coverage tool, or a test container; never contact live systems.
- An assertion-free or tautological test is the highest-impact finding possible — the suite looks green and verifies nothing. Lead with it.
- A test project excluded from the CI run is invisible lost coverage; treat it as HIGH and tell the user the claimed coverage is fictional until the project runs on the gate.
- Never recommend raising coverage with assertion-free tests; never recommend `[Skip]`/`[Ignore]`/`[Fact(Skip=...)]` on a failing test, or disabling a failing gate, as the fix — that converts a known problem into an invisible one.
