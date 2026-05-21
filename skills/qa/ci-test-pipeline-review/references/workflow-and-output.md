# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized files (no CI secrets, no deploy keys, no registry tokens — replace with placeholders):
- The CI workflow file(s) that run tests (`.github/workflows/*.yml`, `.gitlab-ci.yml`, `.circleci/config.yml`, `Jenkinsfile`)
- The branch-protection / merge-rule configuration, if available (which checks are required to merge)
- Any reusable workflow or composite action the test job calls
- Optional: a recent pipeline run summary showing job durations

If branch-protection configuration is not provided, required-check findings are stated as `inference` — say so and ask for it.

### Step 2 — Gating audit

Confirm the test step can actually fail the build.

```yaml
# CRITICAL — test failures are swallowed; the job is always green
- run: npm test || true

# CRITICAL — step failure does not fail the job
- run: npm test
  continue-on-error: true
```
Scan for every escape hatch: `|| true`, `continue-on-error: true`, `set +e`, `; exit 0`, a test command piped so its exit code is lost (`npm test | tee log`), or a soft/optional/advisory check label. Any of these on the test step is CRITICAL — the suite runs, looks green, and verifies nothing.

### Step 3 — Merge-gate placement audit

Confirm tests run on the pull-request merge gate, not only after merge.

- Tests triggered only on `push` to main, on a schedule, or in a nightly job → HIGH. Regressions are then detected after they are already on the protected branch.
- Tests run on `pull_request` but the job is not in the repo's required status checks → HIGH (or `inference` if branch protection is not provided). The run is advisory; a red PR can still merge.
- Recommended: the test job runs on `pull_request` and is a required status check; merges queue behind a green run.

### Step 4 — Speed and sharding audit

Review wall-clock time on the merge gate.

- A large suite in a single job with no sharding, where the job duration is long enough that developers complain or route around it → HIGH. Recommend a shard matrix:
```yaml
strategy:
  fail-fast: false
  matrix:
    shard: [1, 2, 3, 4]
steps:
  - run: npx playwright test --shard=${{ matrix.shard }}/4
```
- `workers`/parallelism pinned to 1 with no reason → MEDIUM.
- Dependency or build cache missing, or keyed without the lockfile hash → MEDIUM: stale caches produce non-reproducible results.

### Step 5 — Fail-fast and matrix audit

- `fail-fast: true` (the default on GitHub Actions matrices) on a test shard matrix → MEDIUM. The first shard failure cancels the others, so a developer sees "1 shard failed" when 3 did, fixes one cause, re-runs, and discovers the next. Set `fail-fast: false` for test matrices so every shard reports.
- No `concurrency` group with `cancel-in-progress` on PR runs → LOW: superseded commits keep burning runners.

### Step 6 — Artifact and observability audit

- No upload of test results (JUnit XML) and failure artifacts (traces, screenshots, videos, logs) → HIGH. A CI-only failure is then undebuggable; engineers re-run blindly hoping for green.
- Artifacts uploaded only on success, or retention too short to investigate → MEDIUM.
- Recommended: upload JUnit XML always, and traces/screenshots/logs `if: failure`.

### Step 7 — Quarantine-lane audit

If a flaky-test quarantine mechanism exists in CI:
- Quarantined tests excluded from the gate but with **no scheduled non-blocking run** → HIGH: the tests never execute again and the coverage is silently lost.
- Quarantine with no tracking issue and no owner → HIGH (consistent with the flakiness-triage skill).
- Recommended: quarantined tests run in a separate non-blocking job on every PR or on a schedule, their results visible, each with an owner and a fix deadline.

### Step 8 — Security audit

- Test jobs triggered by `pull_request_target` that check out and execute PR-author code with secrets in scope → CRITICAL. A fork PR can exfiltrate secrets. Flag and stop.
- Secrets passed to test jobs that run on fork PRs → CRITICAL.
- Long-lived credentials where OIDC / short-lived tokens would work → MEDIUM.

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pipeline gates merges / suite runs but gates nothing / mixed>

## Evidence level
<CI config + branch protection provided | CI config only | documentation-based | inference>

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

- Never request or accept CI secrets, deploy keys, or registry tokens. Ask for workflow files with placeholders.
- This is a static review: do not trigger pipelines, dispatch workflows, or contact CI.
- A test step with a soft-failure escape hatch (`|| true`, `continue-on-error`) is the highest-impact finding possible — the entire suite is decorative. Lead with it.
- `pull_request_target` running PR-author code with secrets in scope is a real exfiltration path; treat it as CRITICAL and tell the user to stop merging through that pipeline until it is fixed.
- Do not recommend making a flaky check non-blocking as the fix — that converts a known problem into an invisible one. Recommend quarantine with a scheduled run and an owner.
