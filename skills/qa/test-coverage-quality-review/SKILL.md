---
name: test-coverage-quality-review
description: Use this skill when reviewing a test suite for assertion quality rather than coverage percentage — detecting coverage theater, where line/branch coverage is high but the tests would not catch a regression. Trigger when a user provides test files, a coverage report, or asks whether their tests are actually meaningful, why bugs ship despite high coverage, or how to set a coverage gate. This skill reviews test artifacts statically; it does not execute tests or run a coverage tool.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-17"
  category: delivery
  lifecycle: experimental
---

# Test Coverage Quality Review

## Purpose
This skill reviews a test suite for whether its tests would actually catch a regression — not whether a coverage tool reports a high percentage. High line coverage with weak assertions is *coverage theater*: code runs during the test, the number looks good in CI, and the test still passes when the behavior breaks. The review separates exercised code from verified behavior, surfaces assertion-free and tautological tests, finds mock over-specification that tests the test instead of the system, and recommends a coverage gate that measures meaning rather than line execution.

## Lean operating rules
- Treat a test with no assertion — it calls the code, no error is thrown, the test passes — as HIGH. Line coverage counts it; it verifies nothing.
- Treat tautological assertions (`expect(true).toBe(true)`, `expect(result).toBe(result)`, snapshot tests auto-updated on every change without review) as HIGH — they cannot fail when behavior changes.
- Treat assertions that only check shape, not value (`expect(result).toBeDefined`, `expect(res.status).toBeTruthy`, `expect(arr.length).toBeGreaterThan(0)`) where an exact value is knowable as MEDIUM — they pass for wrong values.
- Treat tests that assert the mock was called but never assert the result computed from it as HIGH — they test the wiring, not the behavior.
- Treat over-mocked unit tests where every collaborator is mocked and the assertions only restate the mock setup as HIGH — the test is a mirror of itself and proves nothing about integration.
- Treat the absence of error-path, empty-input, and boundary tests for code that has those branches as HIGH — the happy path inflates the coverage number while real failure modes are untested.
- Treat a coverage **percentage gate** as the sole quality signal as MEDIUM — a line-percentage threshold is easily satisfied by assertion-free tests; recommend pairing it with assertion-density and changed-line coverage.
- Treat snapshot tests as the primary verification for logic-bearing output as MEDIUM — snapshots detect change, not correctness, and decay into rubber-stamped updates.
- Treat coverage measured only as a global percentage with no per-changed-file or diff coverage as MEDIUM — new untested code hides behind a large tested codebase.
- Treat 100% coverage as a target presented as a goal as MEDIUM — it incentivizes theater; the goal is meaningful assertions on behavior that matters.
- Do not recommend raising the coverage threshold as a quality improvement — recommend assertion strength and diff coverage instead.
- Label every finding with evidence basis: test source provided, coverage report provided, documentation-based, or inference.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Assertion quality findings (assertion-free, tautological, shape-only)
- Mock usage findings (call-assertion-only, over-mocking)
- Branch coverage gap assessment (error paths, boundaries, empty inputs)
- Coverage gate assessment (percentage-only vs. diff/assertion-aware)
- Snapshot test reliance assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
