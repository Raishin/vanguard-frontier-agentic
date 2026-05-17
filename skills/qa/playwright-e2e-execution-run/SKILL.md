---
name: playwright-e2e-execution-run
description: Use this skill when an operator wants to actually execute an existing Playwright end-to-end suite against a confirmed non-production target and receive a structured, attested run report — pass/fail counts, flaky tests, durations, and trace artifacts. Trigger when the user asks to "run the e2e suite", "execute the Playwright tests against staging", or hands the agent a Playwright project plus a target base URL. This is the live-execution counterpart to the static-review skill `playwright-e2e-suite-review`. Default mode is static and runs nothing; runtime execution is a per-session opt-in that requires explicit target confirmation.
allowed-tools: Read Grep Glob Bash(npx playwright test*) Bash(npx playwright install*) Bash(npx playwright show-report*)
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: delivery
  lifecycle: experimental
  execution_tier: read-only-runtime
  required_egress:
    - operator-confirmed-target-host
    - cdn.playwright.dev
    - playwright.download.prss.microsoft.com
  requires_credentials: []
  output_attestation:
    schema: schemas/attestation.schema.json
    signed_with: none
---

# Playwright E2E Execution Run

## Purpose
This skill executes an existing Playwright end-to-end suite against an operator-confirmed non-production target and emits a structured run attestation: total/passed/failed/flaky counts, slowest tests, retry-only passes, and the location of trace and screenshot artifacts. It is the live-execution counterpart to `playwright-e2e-suite-review` (which is static-review only and never runs anything). The skill runs the suite as authored — it does not write the tests, deploy the application, or mutate infrastructure — and it refuses to run against a production target.

## Execution modes
- **Static (default).** The skill runs nothing. It inspects `playwright.config`, enumerates the project and target, states exactly which command it would execute, and asks the operator for explicit runtime opt-in plus target confirmation.
- **Runtime (per-session opt-in).** Only after the operator explicitly opts in and confirms a non-production base URL does the skill invoke `npx playwright test`. Runtime mode is never assumed from the request alone.

## Lean operating rules
- Never execute the suite without an explicit, in-session runtime opt-in AND an operator-confirmed base URL — absent either, stay in static mode and ask.
- Refuse to run if the target base URL resolves to, or is named like, a production environment (`prod`, `www.`, a customer-facing apex domain). Require a staging, preview, or ephemeral target; state the refusal reason.
- Never accept credentials, bearer tokens, or a `storageState` file inline. Test credentials must come from the environment or a config the operator already controls; the skill never collects, echoes, or logs their values.
- Run only the allowlisted commands: `npx playwright test` (with operator-supplied flags), `npx playwright install` (browser binaries), `npx playwright show-report`. Never run deploy, migration, seed, or registry commands.
- Treat the suite's own side effects as the operator's responsibility — state plainly that E2E tests may create or modify data in the target, which is why a non-production target is mandatory.
- Do not retry a failed run with raised timeouts or added retries to manufacture a green result — report the failure as observed.
- Emit the run attestation as JSON conforming to `schemas/attestation.schema.json`; the verdict is one of `pass`, `fail`, or `manual-review`.
- If browser binaries are missing, run `npx playwright install` only with operator awareness; if egress to the browser CDN is blocked, degrade to `manual-review` rather than reporting a false `fail`.
- Label the run: command executed, target host (host only, never the full credentialed URL), Playwright version, and wall-clock duration.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the run or formatting the attestation.

## Response minimum
Return, at minimum:
- The execution mode used (static or runtime) and why
- The exact command executed (runtime) or that would be executed (static)
- The confirmed target host and Playwright version
- Run results: total / passed / failed / flaky (retry-only pass) counts
- Trace and screenshot artifact locations for any failure
- A `pass` / `fail` / `manual-review` verdict with reasons
- Safe next actions
