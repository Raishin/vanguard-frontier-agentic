---
name: dotnet-testing-quality-review
description: Use this skill when statically reviewing .NET test suites for false confidence — assertion-free and tautological tests, over-mocking, coverage theater, weak test isolation, flaky patterns, and missing negative or security tests across xUnit, NUnit, and MSTest. Trigger when a user provides .NET test source (test classes, fixtures, mock setups, coverage configuration), asks why their green test suite still ships bugs, or wants to know whether their tests actually verify the system instead of inflating a coverage number. This skill reads test source only; it never runs the test suite, a coverage tool, or a test container.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-19"
  category: delivery
  lifecycle: experimental
---

# .NET Testing Quality Review

## Purpose
This skill statically reviews .NET test suites for false confidence — tests that pass but prove nothing. A green suite only protects a release if its tests assert real behavior, exercise the system under test rather than the mocks, isolate from each other, run as part of the CI test set, and cover the negative and security paths that defects actually hide in. The review catches assertion-free tests, tautological tests that assert a mock's own configured behavior, over-mocking, coverage theater, integration tests that share mutable state, test projects excluded from the CI run, and missing unauthorized/forbidden/invalid-input tests across xUnit, NUnit, and MSTest.

## Trigger conditions
- A user provides .NET test source: test classes, fixtures, mock or fake setups (Moq, NSubstitute, FakeItEasy), `WebApplicationFactory` or Testcontainers harnesses, or coverage configuration.
- A user asks why a green suite still lets bugs reach production, or why coverage is high but quality is low.
- A user wants a static review of their test suite before merge or release.
- A user asks whether their mocks, isolation, or coverage gate are meaningful.

## Lean operating rules
- HIGH — treat a test method with no assertion (no `Assert`, no `Should()`, no `Verify`, no expected-exception attribute) as a defect; it proves nothing and inflates the coverage number.
- HIGH — treat a test that asserts only a mock's own configured behavior (tautological — it asserts the mock, not the system under test) as a defect; the test passes regardless of the real code.
- HIGH — treat a coverage gate that counts generated or excluded code, or the absence of any coverage gate, as coverage theater; the number does not reflect tested behavior.
- HIGH — treat integration tests sharing a mutable database with no per-test isolation or reset as a defect; tests pollute each other and pass or fail by run order.
- HIGH — treat a test project not referenced by the CI test run as a silent gap; those tests never execute on the merge gate.
- HIGH — treat missing negative and security tests (unauthorized, forbidden, invalid-input paths) as a defect; defects hide in the paths nobody asserts.
- MEDIUM — treat over-mocking (mocking types you own that carry real logic) as a defect; the test exercises a stub instead of the behavior.
- MEDIUM — treat brittle tests asserting on internal or private structure as a maintainability defect; they break on safe refactors and erode trust in the suite.
- Never recommend raising coverage with assertion-free tests; never recommend `[Skip]`/`[Ignore]`/`[Fact(Skip=...)]` on a failing test as the fix; never recommend disabling a failing gate as the fix.
- Static review only: never run the test suite, a coverage tool, or a test container; never contact live systems. Never request secrets, connection strings, tokens, tenant identifiers, or customer data.
- Label every finding with an evidence-basis label: `confirmed (test source provided)`, `inference (partial source)`, `assumption (source absent)`, or `unknown`.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- A verdict (pass / pass-with-conditions / block)
- An evidence level
- Assertion-quality findings (assertion-free and tautological tests)
- Mocking findings (over-mocking, mock-only assertions)
- Coverage-gate findings (coverage theater, excluded code)
- Isolation findings (shared mutable state, run-order dependence)
- Suite-inclusion findings (test projects excluded from the CI run)
- Negative- and security-test gap findings
- A severity-labelled finding list (critical / high / medium / low), each with an evidence-basis label
- Safe next actions
- Open questions
