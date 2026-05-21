# CLI Commands Reference — Deployment Validator

Reference for the sf CLI commands used by this skill. All commands are
read-only or validate-only — no commit path exists in this skill's scope.

---

## sf project deploy validate

The primary command. Validates a deployment package against a target org
without committing any changes. Equivalent to the legacy `sfdx force:source:deploy --checkonly`.

### Full invocation pattern

```bash
sf project deploy validate \
  --manifest package.xml \
  --target-org <sandbox_alias> \
  --test-level RunSpecifiedTests \
  --tests <TestClass1> <TestClass2> \
  --wait 30 \
  --json
```

### Flag reference

| Flag | Short | Description | Required |
|---|---|---|---|
| `--manifest <path>` | `-x` | Path to `package.xml`. Preferred over `--source-dir` for explicit scope control | Yes (or `--source-dir`) |
| `--source-dir <path>` | `-d` | Path to source directory. Less explicit — prefer `--manifest` for validation | Alternative |
| `--target-org <alias>` | `-o` | Target org alias from `sf org list`. Never inferred. Always explicit | Yes |
| `--test-level <level>` | `-l` | Test level: `NoTestRun`, `RunSpecifiedTests`, `RunLocalTests`, `RunAllTestsInOrg` | Yes (when Apex changed) |
| `--tests <classes>` | `-t` | Space-separated list of Apex test class API names. Required with `RunSpecifiedTests` | When `RunSpecifiedTests` |
| `--wait <minutes>` | `-w` | Minutes to wait for async validation result (default: 33). Adjust to test suite size | Recommended |
| `--json` | | Emit JSON output for structured parsing | Recommended |
| `--verbose` | | Include detailed component and test output | Optional |
| `--coverage-formatters` | | Output code coverage in additional formats (e.g., `json-summary`) | Optional |

### Important: no commit flag

`sf project deploy validate` is **inherently non-committing**. There is no
`--commit` flag. Committing requires the separate command `sf project deploy
start`, which this skill never calls. Do not confuse `--check-only` (a legacy
MDAPI concept) with the validate subcommand — they are equivalent in effect
but the CLI uses the subcommand separation.

### Output structure (JSON)

```json
{
  "status": 0,
  "result": {
    "id": "<deploymentId>",
    "status": "Succeeded",
    "success": true,
    "done": true,
    "numberComponentsTotal": 12,
    "numberComponentsDeployed": 12,
    "numberComponentErrors": 0,
    "numberTestsTotal": 45,
    "numberTestsCompleted": 45,
    "numberTestsFailed": 0,
    "runTestResult": {
      "numTestsRun": 45,
      "numFailures": 0,
      "totalTime": 18234.0,
      "successes": [],
      "failures": [],
      "codeCoverageWarnings": []
    },
    "details": {
      "componentSuccesses": [],
      "componentFailures": [],
      "runTestResult": {}
    }
  }
}
```

Key fields to extract:

| Field | Meaning |
|---|---|
| `result.id` | Validation deployment ID (transient, not a commit) |
| `result.status` | `Succeeded`, `Failed`, `InProgress`, `Canceled` |
| `result.numberComponentErrors` | Number of component-level failures |
| `result.numberTestsFailed` | Number of test method failures |
| `result.runTestResult.failures` | Array of individual test failures |
| `result.runTestResult.codeCoverageWarnings` | Classes below 75% threshold |
| `result.details.componentFailures` | Metadata component errors with type and problem |

---

## sf org display

Used to verify the target org type before any deployment API call.

```bash
sf org display --target-org <alias> --json
```

### Output structure (JSON)

```json
{
  "result": {
    "id": "<orgId>",
    "username": "<username>",
    "instanceUrl": "https://<instance>.salesforce.com",
    "orgId": "<orgId>",
    "accessToken": "<NEVER LOG THIS>",
    "isSandbox": true,
    "loginUrl": "https://test.salesforce.com"
  }
}
```

**Production detection fields (see `production-refusal-rules.md` for full
decision tree):**

| Field | Production indicator |
|---|---|
| `isSandbox` | `false` = production |
| `loginUrl` | `https://login.salesforce.com` = production (not test.salesforce.com) |
| `instanceUrl` | Production instance URL pattern (not `.sandbox.my.salesforce.com`) |

**Never log or emit `accessToken` or `refreshToken`** — strip these fields
before including any org display output in audit envelopes or skill output.

---

## sf apex run test

Used when re-running specific Apex tests independently of a deployment
validation, or to gather a coverage baseline.

```bash
sf apex run test \
  --class-names <TestClass1> <TestClass2> \
  --target-org <sandbox_alias> \
  --result-format json \
  --wait 10
```

### Flag reference

| Flag | Description |
|---|---|
| `--class-names <names>` | Space-separated Apex test class API names to run |
| `--suite-names <names>` | Run a named test suite instead of individual classes |
| `--target-org <alias>` | Target sandbox alias |
| `--result-format <format>` | `human` (default), `tap`, `junit`, `json` |
| `--code-coverage` | Collect and display code coverage data |
| `--wait <minutes>` | Wait for async test result (default: 0 — async) |
| `--synchronous` | Run tests synchronously (only for single class) |

### When to use independently

- **Coverage baseline:** Run tests before a deployment validation to establish
  the current coverage baseline for comparison.
- **Targeted re-run:** After a failing validation, fix a test class and
  re-run only the failing test methods.
- **Post-validation smoke test:** Confirm a specific integration test passes
  after a validation-then-commit cycle (only in sandbox).

---

## sf org list

Used to enumerate connected orgs and confirm the target alias exists.

```bash
sf org list --json
```

Output includes all connected orgs with their `alias`, `username`,
`instanceUrl`, and `isSandbox` fields. Use to confirm:
- The target alias is recognized
- The alias maps to a sandbox org (`isSandbox: true`)

---

## Package Manifest Formats

### package.xml — full format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
    <types>
        <members>MyApexClass</members>
        <name>ApexClass</name>
    </types>
    <types>
        <members>MyLightningWebComponent</members>
        <name>LightningComponentBundle</name>
    </types>
    <types>
        <members>My_Custom_Object__c</members>
        <name>CustomObject</name>
    </types>
    <version>61.0</version>
</Package>
```

### Wildcard manifest (retrieve all of a type — validation only)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
    <types>
        <members>*</members>
        <name>ApexClass</name>
    </types>
    <version>61.0</version>
</Package>
```

Use wildcards with caution for validation — they expand to all components
of that type in the org, which may pull in more than intended.

### API version

Use the current API version for the org (as of 2026-05-21, Salesforce API
version 61.0 corresponds to Spring '25). Mismatched API versions cause
component-level failures for metadata types introduced after the specified
version.

---

## Useful jq Patterns for Output Parsing

Extract test failures:

```bash
sf project deploy validate --manifest package.xml --target-org <alias> --json \
  | jq '.result.runTestResult.failures[] | {class: .name, method: .methodName, message: .message}'
```

Extract coverage warnings:

```bash
sf project deploy validate --manifest package.xml --target-org <alias> --json \
  | jq '.result.runTestResult.codeCoverageWarnings[] | {class: .name, coverage: .message}'
```

Extract component failures:

```bash
sf project deploy validate --manifest package.xml --target-org <alias> --json \
  | jq '.result.details.componentFailures[] | {type: .componentType, name: .fullName, problem: .problem}'
```

---

## Command Not Used by This Skill

| Command | Why excluded |
|---|---|
| `sf project deploy start` | Commits changes — T3, requires HITL via `salesforce-live-guard-agent` |
| `sf project deploy cancel` | Cancels an in-progress deploy — not needed for validate-only |
| `sf project retrieve start` | Retrieves metadata from org — use `salesforce-metadata-fetcher-skill` |
| `sf data upsert bulk` | DML — T3, prohibited |
| `sf project deploy quick` | Quick-deploys a previously validated ID — T3, requires HITL |
