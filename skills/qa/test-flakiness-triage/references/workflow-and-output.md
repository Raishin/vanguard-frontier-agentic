# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized excerpts (no CI credentials, no dashboard API tokens, no production data inside logs):
- A flaky-test report or dashboard export (test name, pass/fail counts, recent failure rate)
- CI rerun history — which tests failed then passed on re-run, with timestamps
- The source of the suspected flaky tests
- The CI configuration that controls retries and job re-runs
- Optional: failure logs / stack traces from intermittent failures

If only failure counts are available without test source, triage stays at the category level and remediation is directional — say so.

### Step 2 — Assign a root-cause category

Place every flaky test into exactly one **primary** category. Note secondary contributors separately.

| Category | Signature in evidence | Typical fix direction |
|---|---|---|
| Async / timing race | fails under load or on slower CI runners; passes locally; "element not found", "undefined" | wait on a deterministic signal, not a sleep; await the actual condition |
| Test interdependence | fails only in a specific order; passes when run alone; passes with `--shuffle` off | per-test fixtures; isolate DB rows / files / state; remove cross-test writes |
| Environment coupling | fails at certain times of day, in CI timezone, or on a different locale | inject and freeze the clock; pin timezone and locale; no real `Date.now()` in assertions |
| Non-deterministic data | fails when random/seeded data hits an edge; date-dependent assertions | seed RNG; use fixed fixtures; freeze "today" |
| Resource contention | fails under parallelism; port/DB/file conflicts; "address in use", deadlock | unique ports/schemas per worker; bound parallelism; isolate resources |
| External dependency | fails on third-party outage, rate limit, or latency; un-mocked network | mock at the boundary in unit/component tests; contract-test the integration separately |

### Step 3 — Decide quarantine vs. fix-in-place

For each flaky test, assign one disposition:
- **Keep gating** — flakiness is rare and the test is high-value; fix-in-place is fast. Only if failure rate is low and a fix is in hand.
- **Fix in place now** — root cause is clear and small (a sleep to replace, a fixture to isolate).
- **Quarantine** — the test fails often enough to cost the team re-runs, and the fix is non-trivial. Move it out of the gating set into a non-blocking lane **with an owner and a fix deadline**.

A flaky test left gating with no owner and no fix is HIGH — quantify the cost: failure rate × team size × runs per day.

### Step 4 — Audit CI retry configuration

Review how CI handles failures.

- Automatic whole-job re-runs, unlimited test retries, or `|| true` on the test step with no flaky tracking → HIGH. This makes flakiness invisible and unbounded; a test that fails 40% of the time still "passes" and nobody knows.
- Retries with no diff between attempts surfaced (no annotation, no flaky reporter) → HIGH.
- Recommended posture: bounded retries (e.g. 2) **plus** a report that lists every test that passed only on retry, fed into the quarantine process.

### Step 5 — Audit quarantine policy

If a quarantine mechanism exists, check each quarantined test has:
- An **owner** (a person or team, not "the team").
- An **expiry / fix deadline** — quarantine without an exit date becomes permanent coverage loss.
- A **tracking issue** linking the root cause.
- A **visible count** — if the quarantine list grows unbounded, escalate; it means flakes are entering faster than they are fixed.

Quarantine without owner + expiry + tracking → MEDIUM.

### Step 6 — Audit flake intake

Check whether new flakes can enter the suite undetected:
- Is there a pre-merge signal that a new test is flaky (e.g. running new/changed tests multiple times)?
- Absence of any intake gate → MEDIUM: the suite's flakiness only ever grows.

### Step 7 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: suite healthy / flakiness contained / flakiness eroding signal>

## Evidence level
<rerun history + source provided | failure counts only | documentation-based | inference>

## Flaky test triage
| Test | Primary category | Disposition | Owner needed |
|------|------------------|-------------|--------------|
| ...  | ...              | ...         | ...          |

## Findings

### CRITICAL
- [C1] <finding>: <description> — <remediation>

### HIGH
- [H1] <finding>: <description> — <remediation>

### MEDIUM
- [M1] <finding>: <description> — <remediation>

### LOW
- [L1] <finding>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request CI credentials, test-dashboard API tokens, or production data that may be embedded in failure logs. Ask for sanitized excerpts.
- This is a static triage: do not re-run tests, do not execute the suite, do not contact CI.
- Do not recommend deleting a flaky test as the default fix — it usually covers a real path. Quarantine with an owner and a fix deadline preserves the coverage intent while removing the team-wide cost.
- Do not recommend a sleep, a raised timeout, or an added retry as the fix for a root cause — flag it as masking, not remediation.
- If the quarantine list is growing faster than it is drained, treat that as the headline finding: the team is losing the flakiness race.
