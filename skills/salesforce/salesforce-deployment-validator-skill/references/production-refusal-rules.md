# Production Refusal Rules

Detection rules and refusal protocol for the deployment validator. A production
org target is the single most critical safety gate in this skill. These rules
must be evaluated before any Metadata API call is made.

---

## Why Production Refusal Exists

`sf project deploy validate` is non-committing by design, but:

1. Running a validation against a production org consumes Apex test execution
   time in the production org, increasing governor pressure.
2. A connected session to production widens the blast radius if the Connected
   App is compromised.
3. The validate-then-quick-deploy path (`sf project deploy quick`) could be
   invoked against the same validation ID to commit to production — this skill
   refuses to be the first step in that chain against a production target.
4. Org type verification at skill entry establishes a clean audit trail and
   prevents accidental production targeting due to alias misconfiguration.

---

## Step 1 — Run sf org display

```bash
sf org display --target-org <alias> --json
```

Extract the relevant fields from the JSON output:

```json
{
  "result": {
    "isSandbox": <true|false>,
    "loginUrl": "<string>",
    "instanceUrl": "<string>",
    "orgType": "<string or absent>"
  }
}
```

Strip `accessToken` and `refreshToken` from all logged output before further
processing.

---

## Step 2 — Evaluate Production Indicators

Evaluate ALL of the following indicators. If ANY is positive, treat the org
as production and refuse.

### Indicator 1: isSandbox field

| Value | Interpretation |
|---|---|
| `true` | Sandbox — continue |
| `false` | Production — REFUSE |
| absent / null | Ambiguous — treat as production until confirmed otherwise |

### Indicator 2: loginUrl domain

| loginUrl pattern | Interpretation |
|---|---|
| `https://test.salesforce.com` | Sandbox |
| `https://login.salesforce.com` | Production |
| `https://<MyDomain>.my.salesforce.com` (without `--sandbox` or `.sandbox.`) | Likely production — verify with isSandbox |
| `https://<MyDomain>.sandbox.my.salesforce.com` | Sandbox (sandbox MyDomain pattern) |
| `https://<MyDomain>.scratch.my.salesforce.com` | Scratch org — acceptable (non-production) |

### Indicator 3: instanceUrl domain pattern

| Pattern | Interpretation |
|---|---|
| `*.sandbox.my.salesforce.com` | Sandbox |
| `*.scratch.my.salesforce.com` | Scratch org |
| `*.develop.my.salesforce.com` | Developer org |
| `*.trailblaze.my.salesforce.com` | Trailhead Playground |
| `*.my.salesforce.com` (no sandbox/scratch/develop prefix) | Likely production — verify with isSandbox |
| `na*.salesforce.com`, `eu*.salesforce.com`, `ap*.salesforce.com` | Legacy production instance — REFUSE |

### Indicator 4: orgType field (when present)

| Value | Interpretation |
|---|---|
| `Production` | REFUSE |
| `Sandbox` | Continue |
| `Developer Edition` | Continue (acceptable for non-production validation) |
| `Scratch` | Continue |
| `Trial` | Acceptable (treat as sandbox-equivalent) |

### Indicator 5: Known production org ID prefix (when org ID is known)

Production org IDs start with `00D` followed by specific character sequences
that can be cross-referenced against an allowlist of sandbox org IDs maintained
by the org administrator. If the org ID is NOT in the sandbox allowlist, treat
as ambiguous and escalate to the org administrator before proceeding.

---

## Step 3 — Decision Tree

```
Is isSandbox == true AND loginUrl contains 'test.salesforce.com' ?
  YES → Likely sandbox; cross-check instanceUrl pattern
    instanceUrl matches *.sandbox.my.salesforce.com or *.scratch.my.salesforce.com ?
      YES → Proceed to Step 3 of the validation workflow
      NO  → Ambiguous — emit warning and request org administrator confirmation
  NO  → REFUSE — production org detected
```

When any indicator is ambiguous (e.g., `isSandbox` is absent, or domain
patterns are non-standard due to a custom domain), apply the most restrictive
interpretation and emit a refusal pending administrator confirmation.

---

## Step 4 — Emit Refusal Envelope

If the target is production (or ambiguous with no administrator confirmation),
emit this structured refusal immediately and stop:

```yaml
refusal_envelope:
  skill_id: "salesforce-deployment-validator-skill"
  skill_version: "0.1.0"
  verdict: "REFUSED"
  reason: "production_org_detected"
  target_org_alias: "<alias>"
  detected_indicators:
    - isSandbox: "<false|absent>"
    - loginUrl: "<value>"
    - instanceUrl: "<value>"
    - orgType: "<value if present>"
  instruction: >
    Production org targets are not permitted for this skill.
    Route to salesforce-live-guard-agent for HITL approval before any
    production deployment. The salesforce-live-guard-agent will manage
    the change promotion workflow with appropriate human approval gates.
  routing:
    next_agent: "salesforce-live-guard-agent"
    reason: "Production deployment requires human approval (T3 path)"
  audit_timestamp: "<ISO-8601-UTC>"
```

---

## Step 5 — Audit Trail on Refusal

Every refusal must be recorded in the audit envelope regardless of whether
the validation proceeded. The refusal itself is an auditable event.

Fields to populate in the audit envelope on refusal:

```yaml
audit_envelope:
  skill_id: "salesforce-deployment-validator-skill"
  skill_version: "0.1.0"
  target_org_alias: "<alias>"
  target_org_type: "unknown-refused"
  package_manifest_path: "<path if provided>"
  package_manifest_hash: "<sha256 if computable>"
  validation_id: null                   # no validation ran
  test_level_used: null
  tests_specified: []
  test_results_summary: null
  coverage_delta: null
  deployment_proposed: false
  component_errors: []
  refusal_triggers_fired: ["production_org_detected"]
  audit_user_role: "<Connected App OAuth username if available>"
  timestamp: "<ISO-8601-UTC>"
```

---

## Connected App Allowlist Enforcement

The Connected App used by the Run As service account must maintain an explicit
allowlist of sandbox org IDs. This is the primary enforcement mechanism —
the skill's org-type check is a secondary belt-and-suspenders control.

**Configuration guidance for org administrators:**

1. In the Connected App settings, restrict OAuth policies to specific org IDs
   using the IP relaxation and Connected App allowlist features.
2. Maintain a named list of approved sandbox aliases in your deployment pipeline
   configuration. Compare the target alias against this list before running
   `sf org display`.
3. Do not add production org IDs to the Connected App allowlist — this is the
   hardware-level enforcement that makes the skill's refusal durable.
4. Rotate the service account refresh token after any suspected compromise or
   after an org is decommissioned.

---

## Handling Ambiguous Orgs

Some orgs present ambiguous signals (non-standard MyDomain, shared sandbox
pools, Trailhead Playgrounds with production-like URLs). In these cases:

1. Emit a warning, not a refusal.
2. Request explicit confirmation from the user or org administrator that the
   target is a non-production org.
3. Include the ambiguity in the audit envelope under `assumptions`.
4. Do not proceed until unambiguous confirmation is received.

Example warning:

```yaml
warning:
  reason: "ambiguous_org_type"
  target_org_alias: "<alias>"
  ambiguous_indicators:
    - "isSandbox field absent from sf org display output"
    - "instanceUrl does not match known sandbox URL pattern"
  action_required: >
    Confirm with the org administrator that '<alias>' is a non-production
    sandbox before validation proceeds. Do not proceed without confirmation.
```

---

## Handoff to salesforce-live-guard-agent

When a production org is detected and a deployment is genuinely needed,
route to `salesforce-live-guard-agent` with this payload:

```yaml
live_guard_handoff:
  source_skill: "salesforce-deployment-validator-skill"
  reason: "production_org_targeted"
  target_org_alias: "<alias>"
  deployment_package: "<path>"
  requested_action: "production_deployment_with_hitl"
  validation_completed_in_sandbox: "<sandbox_alias or 'not yet'>"
  audit_envelope: <refusal_envelope from above>
  notes: >
    Deployment must first be validated in a sandbox using this skill,
    then promoted to production via salesforce-live-guard-agent with
    human approval.
```
