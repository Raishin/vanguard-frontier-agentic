---
name: salesforce-deployment-validator-skill
description: "Runs `sf project deploy validate` against a SANDBOX or non-production org to surface deployment issues, test failures, and metadata dependency problems WITHOUT committing changes. T2 sandbox-mutating but reversible (dry-run only — no commit occurs). Production org targets are REFUSED — production deploys require human approval via salesforce-live-guard-agent. Feeds salesforce-change-impact-analyst-agent. TRIGGER when: user wants to validate a deployment package, dry-run a manifest, check pre-deploy test coverage, verify metadata dependencies, preview change impact in a sandbox. Trigger phrases: \"validate this deploy\", \"dry-run the deployment\", \"check deployment manifest\", \"validate package.xml\", \"test deploy to sandbox\", \"deployment preflight\". DO NOT TRIGGER when: user wants to commit to production (T3 PROHIBITED — route to salesforce-live-guard-agent for HITL); when only static review is needed (use salesforce-metadata-review-skill); when bulk data ops are needed (use salesforce-bulk-data-ops-skill)."
license: MIT
allowed-tools: Bash(sf project deploy validate:*) Bash(sf org display:*) Bash(sf apex run test:*) Read Grep Glob
metadata:
  author: "github: Raishin"
  version: 0.1.0
  updated: 2026-05-21
  category: operational
  lifecycle: experimental
  execution_tier: sandbox-mutating
  mcp_servers: []
  oauth_scopes: ["api", "refresh_token"]
  run_as_permissions:
    required: ["Deploy (Metadata API)", "View All Data (sandbox-only)"]
    denied: ["ModifyAllData (production)", "Customize Application (production)", "Manage Connected Apps"]
---

# salesforce-deployment-validator-skill

Sandbox-only deployment dry-run under T2 scope. This skill validates a
deployment package without committing any changes. It is a **preflight check**,
not a deployment. Production org targets are a hard refusal — any commit to
production must be routed through `salesforce-live-guard-agent` with explicit
human approval.

## When This Skill Owns the Task

Use `salesforce-deployment-validator-skill` when the work requires **pre-deploy
validation** in a non-production environment:

- "Validate this package.xml against our UAT sandbox before we ship"
- "Dry-run the deployment — I need to see test results before promoting"
- "Check whether the metadata dependencies are all satisfied in staging"
- "Preflight the change set — what will break?"
- "Give me the Apex test coverage delta for this deployment"

**This skill NEVER commits changes.** The `--check-only` flag is always
active. The `--dry-run` semantic is enforced structurally — no commit path
exists in this skill.

**Delegate elsewhere when:**

| Situation | Skill to use |
|---|---|
| Target is a production org (any commit) | T3 — `salesforce-live-guard-agent` (HITL required) |
| Only static metadata review needed (no org) | `salesforce-metadata-review-skill` |
| Bulk data operations in sandbox | `salesforce-bulk-data-ops-skill` |
| SCA / code quality review of Apex | `salesforce-apex-lwc-code-review-skill` |
| Change impact risk assessment after validation | `salesforce-change-impact-analyst-agent` |

---

## Required Context to Gather First

Before running any validation, confirm:

1. **Deployment package path or manifest** — local directory path containing
   `package.xml` and the `force-app/` tree, or a path to a bare `package.xml`
   manifest. Accept only local filesystem paths, not org-to-org migration paths.
2. **Target sandbox alias** — the `--target-org` value recognized by
   `sf org list`. Never accept a raw instance URL or session token as input.
3. **Expected test classes** — list of Apex test class API names to run with
   `--test-level RunSpecifiedTests`. If the deployment touches Apex, a test
   specification is mandatory.
4. **Change impact summary** (optional but preferred) — the output of
   `salesforce-change-impact-analyst-agent` if it has already run. This
   pre-populates the risk context in the audit envelope.
5. **Validation scope** — which metadata types are included? (Apex classes,
   triggers, flows, profiles, custom objects, etc.) Affects test selection
   strategy.

If any of these are missing, ask before proceeding. Do not guess the target
org alias — a wrong guess could validate against the wrong environment.

---

## Recommended Workflow

### Step 1 — Verify target org is NOT production

```bash
sf org display --target-org <alias>
```

Inspect the output for production indicators:

- `instanceUrl` domain matches a known production pattern (see
  `references/production-refusal-rules.md`)
- `isSandbox` field is `false` or absent
- `trailblazer.salesforce.com` or `login.salesforce.com` in the OAuth
  endpoint (non-sandbox pattern)
- `orgType` indicates `Production` or the org ID prefix pattern matches a
  known production org

**If any indicator suggests production: STOP. Emit a refusal. Do not proceed.**

### Step 2 — Refuse if target is production

If Step 1 reveals a production org, emit this refusal immediately:

```yaml
refusal:
  reason: "production_org_detected"
  target_org_alias: "<alias>"
  detected_indicators: ["<indicator1>", "<indicator2>"]
  instruction: "Route to salesforce-live-guard-agent for HITL approval before any production deployment."
  audit_timestamp: "<ISO-8601-UTC>"
```

Do not proceed. Do not run any Metadata API call against a production org.

### Step 3 — Verify Run As account permissions

Confirm the connected user has:
- `Deploy` (Metadata API) permission — required for validation
- **Does not** have `ModifyAllData` in production scope — enforced at the
  Connected App allowlist and profile level

If the Connected App allowlist does not include the target sandbox org ID,
stop and escalate to the org administrator.

### Step 4 — Run the validation

```bash
sf project deploy validate \
  --manifest package.xml \
  --target-org <sandbox_alias> \
  --test-level RunSpecifiedTests \
  --tests <TestClass1> <TestClass2> \
  --wait 30 \
  --json
```

Key flags:
- `--manifest package.xml` — always use a manifest (not `--source-dir` for
  validation; manifest gives explicit scope control)
- `--target-org <alias>` — never inferred; always explicit
- `--test-level RunSpecifiedTests` — preferred for scoped changes (see
  `references/test-selection-strategy.md` for when to use RunLocalTests)
- `--wait 30` — wait up to 30 minutes for async validation; adjust to org
  test suite size
- `--json` — structured output for downstream parsing

**Do NOT pass `--dry-run` with intent to remove it — this flag is not how
Salesforce CLI controls commit vs. validate.** The `validate` subcommand
is inherently non-committing. The deploy commit path is a separate command
(`sf project deploy start`) which this skill never calls.

### Step 5 — Capture validation result and test outcomes

From the JSON output, extract:

- `status` — `Succeeded` / `Failed` / `InProgress`
- `deploymentId` — the validation deployment ID (not a commit; does not
  persist changes)
- `numberTestsTotal`, `numberTestsCompleted`, `numberTestsFailed`
- `numberComponentErrors`
- `runTestResult.codeCoverageWarnings` — classes below 75% threshold
- `runTestResult.failures` — individual test failure messages
- `details.componentFailures` — specific metadata component errors and
  dependency issues

### Step 6 — Emit audit envelope

Populate and emit the audit envelope (see Audit Envelope Schema). The
envelope must be emitted regardless of validation outcome — including on
refusals.

### Step 7 — Hand off to change-impact-analyst-agent

Package the validation result, test outcomes, and audit envelope as the
input payload for `salesforce-change-impact-analyst-agent` for risk
assessment. Required handoff fields:

- `audit_envelope`
- `validation_verdict`
- `test_results`
- `coverage_delta`
- `dependency_issues`
- `refusal_triggers_fired`

### Step 8 — Stop conditions

Stop immediately and do not proceed if any stop condition is met (see Stop
Conditions section).

---

## Quality Scoring Rubric (100-point)

Score the validation execution quality before emitting results.
The production-refusal gate must pass — failure voids the score.

| Dimension | Points | What earns full marks |
|---|---|---|
| **Production refusal gate** | CRITICAL | Sandbox verified before any API call; production target causes immediate refusal |
| **Test coverage** | 25 | All changed Apex classes covered by specified tests; aggregate coverage ≥ 75%; no coverage warnings |
| **Validation result clarity** | 25 | JSON output parsed; all component errors enumerated with type and detail; no silent failures |
| **Dependency analysis** | 20 | All `componentFailures` with `problemType: Missing` enumerated; missing dependencies identified by name |
| **Audit envelope** | 15 | All required audit fields present; package manifest hash computed; timestamp accurate |
| **Handoff routing** | 15 | Structured payload prepared for `salesforce-change-impact-analyst-agent`; refusal triggers enumerated |

**Scoring penalties:**
- Production refusal gate not checked before API call: score voided (CRITICAL failure)
- Test specification missing for Apex changes: −25
- `--wait` timeout expired with inconclusive result: −15
- Audit envelope incomplete or missing: −20
- Component failures not enumerated in output: −15

---

## Refusal Triggers

Stop immediately and emit a refusal envelope when any of these conditions are
detected:

| Trigger | Action |
|---|---|
| Target org identified as production | Hard refusal — do not proceed; route to `salesforce-live-guard-agent` |
| Validation request implies commit (`sf project deploy start`) | Refuse — this skill never calls the commit command |
| Test specification missing for deployment containing Apex changes | Stop — require test class list before proceeding |
| Cross-org deploy from a production sandbox refresh (prod-sourced sandbox with prod data) | Escalate — treat as production-equivalent for data privacy |
| Connected App allowlist does not include target sandbox | Stop — escalate to org administrator |
| `--check-only false` equivalent intent expressed | Refuse — this skill is validate-only |

---

## Audit Envelope Schema

Every validation emits an audit envelope. The envelope travels with the
validation result to any downstream skill or agent.

```yaml
audit_envelope:
  skill_id: "salesforce-deployment-validator-skill"
  skill_version: "0.1.0"
  target_org_alias: "<alias>"
  target_org_type: "sandbox"          # must be 'sandbox'; never 'production'
  package_manifest_path: "<local path to package.xml>"
  package_manifest_hash: "<sha256 of package.xml content>"
  validation_id: "<deploymentId from sf CLI JSON output>"
  test_level_used: "RunSpecifiedTests | RunLocalTests | RunAllTestsInOrg"
  tests_specified: ["<TestClass1>", "<TestClass2>"]
  test_results_summary:
    total: <integer>
    passed: <integer>
    failed: <integer>
    failures: ["<ClassName.methodName: failure reason>"]
  coverage_delta:
    classes_below_threshold: ["<ClassName>"]
    aggregate_coverage_percent: <0-100>
  deployment_proposed: false           # always false — this skill never commits
  component_errors:
    - component: "<MetadataType>.<ApiName>"
      problem_type: "<Missing|Error|Warning>"
      problem: "<error message>"
  refusal_triggers_fired: ["<trigger name or 'none'>"]
  audit_user_role: "<Connected App OAuth username>"
  timestamp: "<ISO-8601-UTC>"
```

---

## Output Format

```yaml
validation_verdict: "Succeeded | Failed | Refused | Inconclusive"

test_results:
  total: <integer>
  passed: <integer>
  failed: <integer>
  coverage_warnings: ["<ClassName: X% coverage>"]
  failures:
    - class: "<TestClassName>"
      method: "<testMethodName>"
      message: "<failure detail>"

coverage_delta:
  aggregate_coverage_percent: <0-100>
  classes_below_75_percent: ["<ClassName>"]
  notes: "<coverage trend vs. last known baseline if available>"

dependency_issues:
  - type: "<MetadataType>"
    api_name: "<ApiName>"
    problem: "<what is missing or broken>"

refusal_triggers_fired:
  - "<trigger name or 'none'>"

change_impact_handoff:
  target_agent: "salesforce-change-impact-analyst-agent"
  payload_ready: true | false
  notes: "<what the analyst agent should focus on>"

missing_evidence:
  - "<what would strengthen the validation — e.g., missing test classes, missing dependencies>"

assumptions:
  - "<explicit list of assumptions made>"

audit_envelope:
  <see Audit Envelope Schema>
```

---

## Handoff Rules

| Situation | Hand off to |
|---|---|
| Validation complete — risk assessment needed | `salesforce-change-impact-analyst-agent` |
| Static Analysis / SCA findings from Apex code | `salesforce-apex-lwc-code-review-skill` or `salesforce-devsecops-pipeline-skill` |
| Production commit approved after validation | `salesforce-live-guard-agent` (HITL required) |
| Metadata review (static, no org) | `salesforce-metadata-review-skill` |

Required handoff fields for all outbound payloads: `audit_envelope`,
`validation_verdict`, `refusal_triggers_fired`, `missing_evidence`,
`assumptions`.

---

## Stop Conditions

Stop immediately and do not continue if:

- **Production org detected** — `sf org display` indicates `isSandbox: false`
  or production URL pattern. Stop, emit refusal envelope, route to
  `salesforce-live-guard-agent`.
- **Validation implies commit** — user requests `sf project deploy start` or
  equivalent. This skill does not call the commit command. Refuse and explain.
- **Apex changes with no test specification** — deployment manifest contains
  Apex classes or triggers and no test class list has been provided. Stop and
  require test specification before proceeding.
- **Connected App allowlist mismatch** — target org alias is not in the
  authorized sandbox allowlist. Stop and escalate to org administrator.
- **`--wait` timeout exceeded with no result** — emit an `Inconclusive` verdict
  with the partial output and note the timeout. Do not re-run automatically.
- **Manifest hash mismatch** — if the package.xml changes between the pre-hash
  step and the validation call (filesystem race), stop and require re-confirmation.

---

## Security Notes

- **T2 sandbox-only enforcement:** The skill structurally refuses any production
  org target. The `sf project deploy validate` command is inherently non-committing
  but production refusal is enforced at the org-type check level before any API
  call is made.
- **No commit possible:** This skill calls `sf project deploy validate` only —
  never `sf project deploy start`. The commit path does not exist in this skill's
  allowed-tools scope.
- **Org allowlist:** The Connected App restricts which sandbox orgs can be
  targeted. This skill verifies via `sf org display` before proceeding.
- **Audit emitted on every execution:** Including on refusals. The audit envelope
  records the target org type, the refusal reason (if any), and the timestamp.
- **Reversible:** `sf project deploy validate` makes no persistent changes to
  the target org. The validation deployment ID is transient and does not alter
  metadata in the org.
- **Explicit production refusal:** Any production org target triggers an
  immediate structured refusal with routing to `salesforce-live-guard-agent`.
- **No credential echo:** OAuth tokens, refresh tokens, and session IDs are
  never included in output or audit envelopes.
- **Run As permissions scoped:** The connected service account has Deploy
  (Metadata API) permission but is blocked from ModifyAllData (production),
  Customize Application (production), and Manage Connected Apps by profile
  and Connected App policy.

---

## Reference File Index

| File | When to read |
|---|---|
| `references/cli-commands.md` | sf CLI deploy validate command flags, sf apex run test, sf org display, manifest formats |
| `references/production-refusal-rules.md` | Detection rules for production orgs, refusal audit emission, live-guard routing |
| `references/test-selection-strategy.md` | RunSpecifiedTests vs. RunLocalTests vs. RunAllTestsInOrg; 75% coverage; test list construction |
