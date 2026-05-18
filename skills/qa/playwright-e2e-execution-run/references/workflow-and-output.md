# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs (static mode)

Without running anything, gather:
- The Playwright project root (location of `playwright.config.ts/js` and the `tests/` directory).
- The target base URL the operator wants to test against.
- Whether browser binaries are already installed.
- Confirmation of whether the operator is opting into runtime execution this session.

If the operator has not explicitly opted into runtime execution, stay in static mode: report what would run and stop.

### Step 2 — Target safety gate

Before any execution, validate the target:
- Reject a base URL that names or resolves to production — `prod`, `production`, a bare customer apex domain, or `www.` on the public site. Require a staging, preview, QA, or ephemeral environment.
- Reject a base URL with embedded credentials (`https://user:pass@host`). Credentials belong in the environment, never the URL.
- Echo back only the **host** for confirmation (`staging.example.internal`), never the full URL with query string or token.

If the target cannot be confirmed as non-production, stay in static mode and state the refusal reason.

### Step 3 — Resolve the command

Construct the exact command from operator-supplied flags. Examples:

```bash
# Whole suite against a confirmed target
PLAYWRIGHT_BASE_URL=https://staging.example.internal npx playwright test

# A single project / shard
npx playwright test --project=chromium --shard=1/4

# A specific spec
npx playwright test tests/checkout.spec.ts
```

State the resolved command verbatim and get a final go-ahead.

### Step 4 — Ensure browsers (only if needed)

If browser binaries are missing:

```bash
npx playwright install --with-deps
```

If egress to the Playwright browser CDN (`cdn.playwright.dev`, `playwright.download.prss.microsoft.com`) is blocked, do not report a test failure — the run never started. Degrade to `manual-review` with reason `browser-install-blocked`.

### Step 5 — Execute (runtime mode only)

Run the resolved `npx playwright test` command. Use a machine-readable reporter so results can be parsed deterministically:

```bash
npx playwright test --reporter=json
```

Capture: exit code, total/passed/failed/skipped counts, tests that passed only on retry (flaky), the slowest tests, and the paths to `playwright-report/` and any `test-results/**/trace.zip`.

Do not re-run with raised timeouts or extra retries to force a green result. One run, reported as observed. A deliberate re-run for flakiness confirmation is allowed only if the operator asks, and both runs are reported.

### Step 6 — Emit the attestation

Produce a JSON attestation conforming to `schemas/attestation.schema.json`. Verdict rules:
- `pass` — exit code 0, zero failed tests.
- `fail` — one or more tests failed.
- `manual-review` — the run could not complete (browser install blocked, config error, target unreachable, egress denied). Never auto-`pass` an incomplete run.

### Step 7 — Produce the output

Format the response using the Output section below, with the attestation JSON included.

---

## Output

Return results in this structure:

```
## Mode
<static | runtime> — <one-line reason>

## Command
<the exact command executed, or that would be executed in static mode>

## Target
host: <host only>   playwright: <version>   duration: <wall-clock>

## Results
total: <n>   passed: <n>   failed: <n>   flaky: <n>   skipped: <n>

## Failures
- <test title> — <file:line> — trace: <path/to/trace.zip>

## Verdict
<pass | fail | manual-review> — <reasons>

## Attestation
```json
{
  "schema": "schemas/attestation.schema.json",
  "skill": "playwright-e2e-execution-run",
  "target_host": "<host>",
  "playwright_version": "<version>",
  "command": "<command>",
  "results": { "total": 0, "passed": 0, "failed": 0, "flaky": 0, "skipped": 0 },
  "verdict": "<pass|fail|manual-review>",
  "verdict_reasons": [],
  "artifacts": { "report": "playwright-report/", "traces": [] },
  "generated_at": "<ISO-8601>"
}
```

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring operator clarification>
```

---

## Security notes

- Default mode is static — the skill runs nothing until the operator explicitly opts into runtime execution in the current session.
- Runtime execution is gated on an operator-confirmed non-production target. A production target is an immediate refusal, not a warning.
- The Bash allowlist permits only `npx playwright test`, `npx playwright install`, and `npx playwright show-report`. Never run deploy, database migration, seed, registry, or `kubectl` commands under this skill.
- Never accept credentials, bearer tokens, or a `storageState` file inline or in the base URL. Test credentials are supplied through the operator-controlled environment and are never collected, echoed, or written into the attestation.
- E2E suites frequently create or modify data in the target application. That side effect is the operator's responsibility and is the reason a non-production target is mandatory — state this explicitly.
- An incomplete run degrades to `manual-review`, never to `pass`. A blocked browser CDN, an unreachable target, or a config error must not be reported as a test `fail`, which would misattribute the cause.
- Report failures as observed. Do not raise timeouts, add retries, or re-run selectively to manufacture a green verdict.
