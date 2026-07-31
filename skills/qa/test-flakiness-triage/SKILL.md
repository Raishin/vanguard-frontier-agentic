---
name: test-flakiness-triage
description: Use this skill when triaging flaky tests across any test framework — analyzing a flaky-test report, CI rerun history, or a set of intermittently failing tests to assign each a root-cause category and a remediation or quarantine path. Trigger when a user reports tests that pass on re-run, asks why CI is unreliable, provides a flaky-test dashboard export, or wants a quarantine policy. This skill reviews evidence statically; it does not execute or re-run tests.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-17"
  category: delivery
  lifecycle: experimental
---

# Test Flakiness Triage

## Purpose
This skill triages flaky tests — tests that pass and fail without a code change — into root-cause categories and assigns each a remediation or quarantine path. Flakiness is a process failure, not a test-by-test annoyance: once a suite flakes, engineers re-run red builds reflexively, a re-run becomes the default response to any failure, and the suite stops detecting real regressions. The skill is framework-agnostic (Playwright, Cypress, Jest, JUnit, pytest, Go) and converts a pile of "sometimes fails" into categorized causes, a quarantine decision per test, and a policy that stops new flakes from entering the suite.

## Lean operating rules
- Triage every flaky test into exactly one primary root-cause category: async/timing race, test interdependence (shared state / order), environment coupling (clock, timezone, locale, network), non-deterministic data (random, current date, unseeded faker), resource contention (ports, DB, parallelism), or external dependency (third-party API, un-mocked network).
- Treat a flaky test left in the gating suite with no quarantine and no owner as HIGH — every run it costs the whole team a re-run and erodes signal.
- Treat "re-run until green" CI configuration (automatic unlimited retries, `|| true`, retry-the-whole-job) with no flaky tracking as HIGH — it converts flakiness from visible to invisible and unbounded.
- Treat a flaky test "fixed" only by adding a sleep, raising a timeout, or adding a retry as HIGH — the root cause is still present and will resurface.
- Treat tests sharing a mutable fixture, database row, file, or port without per-test isolation as HIGH for interdependence flakiness.
- Treat assertions on wall-clock time, `Date.now`, timezone, or locale without injection/freezing as HIGH for environment coupling.
- Treat un-mocked calls to third-party services in a unit or component test as HIGH — network and rate limits make them flaky by construction.
- Treat a quarantine with no expiry, no owner, and no tracking issue as MEDIUM — quarantine is a holding cell, not a graveyard; without an exit it becomes permanent coverage loss.
- Treat the absence of any flaky-detection mechanism (no rerun-diff, no flaky reporter, no quarantine list) as MEDIUM — flakiness is then discovered only by frustrated engineers.
- Do not recommend deleting a flaky test as the default fix — a flaky test usually covers a real path; quarantine with an owner and a fix deadline preserves the coverage intent.
- Label every finding with evidence basis: failure logs / rerun history provided, test source provided, documentation-based, or inference.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full triage or formatting the final answer.

## Response minimum
Return, at minimum:
- Per-test root-cause category assignment
- Quarantine decision per test (quarantine now / fix in place / keep gating)
- Root-cause remediation direction per category
- Quarantine policy assessment (owner, expiry, tracking)
- CI retry-configuration findings
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
