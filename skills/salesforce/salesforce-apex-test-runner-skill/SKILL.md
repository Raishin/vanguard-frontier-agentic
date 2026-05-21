---
name: salesforce-apex-test-runner-skill
description: "Executes Apex tests against a connected SANDBOX org via sf apex run test, parses results and coverage delta, identifies failures with stack traces, and suggests fixes. T1 read-only runtime (sandbox-only). Production org targets are HARD REFUSED before any API call. TRIGGER when: user wants to run Apex tests, execute a test class, check test coverage, diagnose test failures, or validate coverage before deployment. Trigger phrases: run apex tests, execute test class, test my changes, check test coverage, why is my test failing. DO NOT TRIGGER when: user needs to generate test classes (use salesforce-apex-test-generator-skill), debug without running tests (use salesforce-apex-log-analyzer-skill), static code review of test code (use salesforce-apex-lwc-code-review-skill), or user needs a deployment (use salesforce-deployment-validator-skill)."
license: MIT
allowed-tools: Bash(sf apex run test:*) Bash(sf org display:*) Bash(sf apex get test:*) Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-21"
  category: operational
  lifecycle: experimental
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token"]
  run_as_permissions:
    required: ["View Setup and Configuration", "View All Data (sandbox-only)"]
    denied: ["ModifyAllData", "ViewEncryptedData", "ModifyMetadata", "AuthorApex", "ManageConnectedApps"]
---

# salesforce-apex-test-runner-skill

T1 read-only runtime skill for Apex test execution against a connected **sandbox** org.
This skill runs tests, reads coverage, and diagnoses failures — it does not write code,
deploy metadata, or touch production orgs. The **View All Data** permission is required
by `sf apex run test` per Salesforce platform behavior; this permission must be granted
only on the sandbox-only service account and NEVER on a production-eligible account.

## When This Skill Owns the Task

Use `salesforce-apex-test-runner-skill` when the work requires **live test execution**:

- "Run the AccountServiceTest class in my sandbox"
- "Execute all local tests before deployment"
- "My test is failing with a governor limit error — run it and show me the stack trace"
- "Check code coverage for the Order trigger after my last change"
- "Run specific test methods: AccountServiceTest.testBulkSuccess"

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| User needs test classes authored | `salesforce-apex-test-generator-skill` |
| User needs production Apex authored | `salesforce-apex-generator-skill` |
| Debug log analysis without running | `salesforce-apex-log-analyzer-skill` |
| Static code review of test code | `salesforce-apex-lwc-code-review-skill` |
| Sandbox deploy and validate | `salesforce-deployment-validator-skill` |

---

## Required Context to Gather First

Before running any tests, confirm:

1. **Target org alias** — the `--target-org` value from `sf org list`. Never accept a raw
   instance URL or session token.
2. **Org type** — must be sandbox. If production, **stop immediately** (see HARD REFUSAL).
3. **Test scope** — specific class names, specific method names, or test level
   (`RunLocalTests`, `RunAllTestsInOrg`, `RunSpecifiedTests`).
4. **Coverage threshold** — default 75% per Salesforce deployment requirement; note if higher.
5. **Goal** — diagnosis only, or diagnosis + fix loop?

---

## Recommended Workflow

### Step 1 — Verify org alias and type

```bash
sf org display --target-org <alias>
```

Parse output for `instanceUrl` and check for sandbox indicators in the domain
(`.sandbox.`, `.cs`, `.scratch.`, `.scratch.salesforce.com`). If `instanceUrl` contains
`login.salesforce.com` or `<companyname>.my.salesforce.com` without sandbox indicators,
**treat as production and stop**.

### Step 2 — Confirm Connected App allowlist

The Connected App used by the Run As service account must have the target org alias
explicitly in its allowlist. If the allowlist cannot be confirmed, stop and request
the Connected App administrator verify the configuration.

### Step 3 — Run the targeted test set

```bash
sf apex run test \
  --test-level RunSpecifiedTests \
  --tests <TestClassName1> <TestClassName2> \
  --target-org <alias> \
  --result-format json \
  --wait 10
```

For broader coverage checks:

```bash
sf apex run test \
  --test-level RunLocalTests \
  --target-org <alias> \
  --result-format json \
  --wait 30
  --code-coverage
```

### Step 4 — Retrieve results if async

If the run was async (no `--wait` or timeout exceeded):

```bash
sf apex get test \
  --test-run-id <id> \
  --target-org <alias> \
  --result-format json \
  --code-coverage
```

### Step 5 — Parse results

From the JSON output, extract:
- **Pass count / fail count / skip count**
- **Overall code coverage percentage** (target: >= 75%)
- **Per-class coverage** — identify classes below threshold
- **Failing test methods** — extract `methodName`, `message`, `stackTrace`
- **Governor limit hits** — scan message for "Limit" patterns

### Step 6 — Diagnose failures

Consult `references/failure-diagnosis.md` for common patterns:
- `DML in @TestSetup` restrictions
- Governor limit errors in test context
- `CalloutException` — missing mock
- `QueryException` — data isolation issue (SeeAllData mis-use)
- Async result not visible — missing `Test.startTest` / `Test.stopTest`

### Step 7 — Emit sanitized results with audit envelope

Apply redaction rules (see Redaction Rules section) before emitting any output.
Emit the full audit envelope regardless of pass/fail outcome.

### Step 8 — Recommend next steps

If failures found: recommend `salesforce-apex-test-generator-skill` for test corrections
or `salesforce-apex-log-analyzer-skill` for deeper log analysis.
If coverage below threshold: identify which classes are uncovered and recommend targeted
test generation.

---

## Quality Scoring Rubric (100-point)

Score the test run and analysis before presenting. Threshold: 80+ acceptable.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Test selection appropriateness** | 25 | Scope matches the user's goal; RunSpecifiedTests used for targeted runs; RunLocalTests for pre-deploy coverage |
| **Sandbox-only enforcement** | 25 | **CRITICAL gate** — org type verified as sandbox before any API call; HARD REFUSAL triggered for production targets |
| **Coverage analysis** | 20 | Coverage percentage computed and compared to 75% threshold; per-class breakdown provided; uncovered classes named |
| **Failure diagnosis quality** | 20 | Failures categorized by type; stack trace excerpted; remediation suggested from failure-diagnosis reference |
| **Audit envelope** | 10 | All required audit fields present; timestamp accurate; org type verified field populated |

**Scoring penalties:**
- Sandbox check skipped: score voided (immediate HARD REFUSAL required)
- No coverage analysis when `--code-coverage` was available: -20
- Failures reported without diagnosis: -15
- Missing audit envelope: -15 (automatic caveat regardless of total)
- Org type not verified before execution: -25

---

## T1 Least-Privilege Contract

This skill operates at T1 — read-only runtime (sandbox only).

- **OAuth scopes:** `api` and `refresh_token` only. No `full`, `web`, `sfap_api`, or other scopes.
- **Run As account permissions:**
  - REQUIRED: `View Setup and Configuration`, `View All Data` (**sandbox service account only**)
  - DENIED: `ModifyAllData`, `ViewEncryptedData`, `ModifyMetadata`, `AuthorApex`, `ManageConnectedApps`
- **View All Data — important note:** The `sf apex run test` command requires the `View All Data`
  system permission for the running user per Salesforce platform behavior. This is why the Run As
  service account for this skill must be a **sandbox-only** account. This permission must NEVER
  be granted on an account used for production org access.
- **No code mutation:** This skill reads test results and coverage data. It does not write,
  deploy, modify metadata, or create records outside of test context.
- **Org allowlist:** Connected App enforces which aliases are reachable. Skill verifies org
  type before executing any `sf apex` command.

---

## HARD REFUSAL — Production Org Targets

**If the target org is identified as production, stop immediately and do not execute any
`sf apex` command.**

Production refusal response:

```
HARD REFUSAL: Apex test execution is not permitted on production orgs via this skill.

Reason: salesforce-apex-test-runner-skill operates at T1 sandbox-only scope. The
View All Data system permission required by sf apex run test must never be granted
on a production-eligible service account. Production Apex test runs also carry risk
of long-running test locks affecting live users.

Action required:
1. Identify a sandbox org alias for this test run.
2. Verify the Connected App allowlist includes the sandbox alias.
3. Re-invoke this skill with the sandbox alias.

If a production test run is genuinely required (e.g., for a post-deployment verification),
route through the human approval path via salesforce-live-guard-agent.
```

---

## Audit Envelope Schema

Every execution emits an audit envelope. Emit even on HARD REFUSAL.

```yaml
audit_envelope:
  matter_id: "<caller-provided-or-generated-uuid>"
  skill_id: "salesforce-apex-test-runner-skill"
  skill_version: "0.1.0"
  target_org_alias: "<alias>"
  run_as_user_id: "<user_id_placeholder>"
  org_type_verified: "sandbox | production | unknown"
  test_run_id: "<sf-apex-run-test-id or 'refused'>"
  test_level: "<RunSpecifiedTests | RunLocalTests | RunAllTestsInOrg | refused>"
  tests_requested: ["<ClassName.methodName or ClassName>"]
  pass_count: <integer>
  fail_count: <integer>
  skip_count: <integer>
  overall_coverage_pct: <float or null>
  timestamp: "<ISO-8601-UTC>"
  refusal_reason: "<reason if refused | null>"
```

---

## Output Format

```yaml
verdict: "pass | fail | refused"
quality_score: <0-100>
quality_notes: "<scoring rationale>"

test_summary:
  org_alias: "<alias>"
  org_type: "sandbox | production | unknown"
  test_level: "<level used>"
  pass_count: <integer>
  fail_count: <integer>
  skip_count: <integer>
  overall_coverage_pct: <float>
  coverage_threshold_met: true | false

failing_tests:
  - test: "<ClassName.methodName>"
    message: "<error message>"
    stack_trace_excerpt: "<first 3-5 lines>"
    diagnosis: "<category from failure-diagnosis.md>"
    suggested_fix: "<specific remediation>"

coverage_gaps:
  - class: "<ClassName>"
    coverage_pct: <float>
    suggested_action: "<add tests for X method>"

audit_envelope:
  <see Audit Envelope Schema>

next_steps:
  - "<if failures: salesforce-apex-test-generator-skill for test fixes>"
  - "<if coverage gaps: salesforce-apex-test-generator-skill for additional tests>"
  - "<if ready: salesforce-deployment-validator-skill for sandbox deploy>"

assumptions:
  - "<explicit list>"
```

---

## Redaction Rules

Apply in order before emitting any output:

1. **OAuth tokens, refresh tokens, session IDs:** Never include in any output. Strip from CLI output.
2. **Salesforce Org IDs (18-char starting with `00D`):** Replace with `<org_id_placeholder>`.
3. **Salesforce Record IDs (15/18-char):** Replace with `<record_id_placeholder>` in output.
4. **User IDs (OwnerId, CreatedById, RunningUserId, User.Id):** Replace with `<user_id_placeholder>`.
5. **Test run IDs:** Retain in audit envelope (non-sensitive); redact from narrative output.
6. **Stack trace class names and line numbers:** Retain — needed for diagnosis. Do not redact.
7. **Exception messages:** Retain — needed for diagnosis. If message contains a Salesforce ID,
   replace the ID but retain the rest of the message.
8. **Instance URLs and API endpoints:** Omit from output; reference only the org alias.

---

## Handoff Rules

| Finding | Hand off to |
|---|---|
| Test failures needing code fixes | `salesforce-apex-test-generator-skill` |
| Coverage gaps needing new test methods | `salesforce-apex-test-generator-skill` |
| Log-level failure requiring debug analysis | `salesforce-apex-log-analyzer-skill` |
| All tests pass with coverage met | `salesforce-deployment-validator-skill` |
| Production target attempted | `salesforce-live-guard-agent` (human approval path) |

---

## Stop Conditions

Stop and do not continue if:

- Org type cannot be determined from `sf org display` output — stop, request clarification.
- Org type is production — HARD REFUSAL (see above), stop.
- The Run As account is missing `View Setup and Configuration` — stop and escalate to org administrator.
- The audit envelope cannot be populated (missing matter_id, org alias unresolvable) — stop.
- The user requests redaction to be skipped — stop and explain the policy.

---

## Security Notes

- **Sandbox-only:** Production org targets are hard-refused before any API call. The Connected App
  allowlist enforces this at the platform level as a second control.
- **View All Data — sandbox-only account:** The `View All Data` system permission required by
  `sf apex run test` must exist only on the sandbox-only Run As service account. Separation from
  any production-eligible account is mandatory.
- **Read-only runtime:** No DML, no metadata mutation, no code deployment, no record creation
  outside Apex test context (which is isolated and rolled back by the platform).
- **Sanitized output:** All Salesforce IDs, user IDs, and OAuth tokens redacted before emission.
- **Structured audit:** Every execution produces a complete audit envelope, including refusals.
- **Revocable:** Rotating the Run As sandbox account's refresh token immediately revokes all access.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/cli-commands.md` | sf apex run test variants, --test-level options, --result-format, --wait, async retrieval |
| `references/coverage-analysis.md` | Coverage percentage interpretation, 75% requirement, line vs branch coverage, per-class gap analysis |
| `references/failure-diagnosis.md` | Common Apex test failure patterns: DML on setup, governor limits, callouts in tests, async test gotchas |
