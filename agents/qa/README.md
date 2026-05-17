# 🧪 QA Agents

QA and test-quality agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live execution |
|---|---|---|---|
| Review agents | Audit test suites, test artifacts, and CI test pipelines for reliability and meaning | read-only | not allowed |

## 📋 Test quality review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `playwright-e2e-suite-review-agent` | Review Playwright specs, config, and CI for flakiness, selector brittleness, isolation defects, retry masking | static-review | asked to run `npx playwright test` or contact a target app |
| `test-flakiness-triage-agent` | Triage flaky tests into root-cause categories and quarantine/fix paths; audit CI retry config | static-review | asked to re-run tests or contact CI |
| `test-coverage-quality-review-agent` | Detect coverage theater — assertion-free, tautological, over-mocked tests; weak coverage gates | static-review | asked to run the suite or a coverage tool |
| `ci-test-pipeline-review-agent` | Review CI test gating, sharding, fail-fast, artifacts, quarantine wiring, secret exposure | static-review | asked to trigger or dispatch a pipeline |

## 🛡️ Operating note

- These agents perform **static review only** — they read test specs, configuration, coverage reports, and CI workflow files. They never execute a test suite, launch a browser, run a coverage tool, or trigger a pipeline.
- A test step with a soft-failure escape hatch (`|| true`, `continue-on-error: true`) is the highest-impact defect in any QA pipeline — the suite runs, looks green, and gates nothing.
- A high coverage percentage with weak assertions (coverage theater) manufactures false confidence and is more dangerous than a low number.
- Flaky tests are a process failure: once re-running red builds becomes routine, the suite stops detecting real regressions. Quarantine needs an owner and a fix deadline, never silent deletion.
- None of these agents request live application URLs with credentials, CI secrets, auth tokens, or production data — they ask for sanitized snippets.

## 📦 Install

```bash
# Install the Playwright E2E suite review agent
npx vfa-export-agents --platform claude-code --agents playwright-e2e-suite-review-agent --repo .

# Install all QA test-quality review agents
npx vfa-export-agents --platform claude-code --agents playwright-e2e-suite-review-agent,test-flakiness-triage-agent,test-coverage-quality-review-agent,ci-test-pipeline-review-agent --repo .
```
