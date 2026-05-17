# 🧪 QA Agents

QA, test-quality, and automation-resilience agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live execution |
|---|---|---|---|
| Review agents | Audit test suites, automation workflows, control logic, and CI pipelines for reliability, safety, and meaning | read-only | not allowed |
| Execution agents | Run an existing test suite against an operator-confirmed non-production target and emit an attestation | read-only-runtime | per-session opt-in only |

## 📋 Test quality review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `playwright-e2e-suite-review-agent` | Review Playwright specs, config, and CI for flakiness, selector brittleness, isolation defects, retry masking | static-review | asked to run `npx playwright test` or contact a target app |
| `test-flakiness-triage-agent` | Triage flaky tests into root-cause categories and quarantine/fix paths; audit CI retry config | static-review | asked to re-run tests or contact CI |
| `test-coverage-quality-review-agent` | Detect coverage theater — assertion-free, tautological, over-mocked tests; weak coverage gates | static-review | asked to run the suite or a coverage tool |
| `ci-test-pipeline-review-agent` | Review CI test gating, sharding, fail-fast, artifacts, quarantine wiring, secret exposure | static-review | asked to trigger or dispatch a pipeline |

## 🏭 Automation and control-logic review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `plc-control-logic-safety-review-agent` | Review exported IEC 61131-3 PLC logic for E-stop correctness, unsafe states, unresolved latches, scan races, forced I/O | static-review | asked to connect to a live PLC or weaken a safety interlock |
| `rpa-workflow-resilience-review-agent` | Review exported RPA workflows for hardcoded credentials, brittle selectors, missing exception handling, non-idempotency | static-review | asked to run a bot or supply orchestrator credentials |

## ▶️ Test execution agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `playwright-e2e-execution-run-agent` | Execute an existing Playwright suite against an operator-confirmed non-production target; emit a run attestation | read-only-runtime (static by default) | target is production, or no in-session runtime opt-in |

## 🛡️ Operating note

- The **review agents** perform static review only — they read test specs, configuration, control logic, workflow definitions, coverage reports, and CI files. They never execute a suite, launch a browser, run a coverage tool, trigger a pipeline, or connect to a PLC or RPA orchestrator.
- The **execution agent** is read-only-runtime: its default mode is static and runs nothing. Runtime execution is a per-session opt-in gated on an operator-confirmed non-production target; a production target is an immediate refusal.
- A test step with a soft-failure escape hatch (`|| true`, `continue-on-error: true`) is the highest-impact defect in any QA pipeline — the suite runs, looks green, and gates nothing.
- A high coverage percentage with weak assertions (coverage theater) manufactures false confidence and is more dangerous than a low number.
- PLC review is OT/ICS work — a defect injures people or destroys equipment. These agents never advise modifying running logic or bypassing an E-stop or safety function.
- None of these agents request live application URLs with credentials, CI secrets, auth tokens, PLC controller access, RPA runner credentials, or production data — they ask for sanitized snippets.

## 📦 Install

```bash
# Install the Playwright E2E suite review agent
npx vfa-export-agents --platform claude-code --agents playwright-e2e-suite-review-agent --repo .

# Install the full QA role (all review and execution agents)
npx vfa-export-agents --platform claude-code --role qa-test-quality-engineer --repo .
```
