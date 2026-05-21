# sf apex run test — CLI Commands Reference

Adapted from forcedotcom/sf-skills running-apex-tests references (Apache-2.0).

## Core Command: sf apex run test

```bash
# Run specific test classes
sf apex run test \
  --test-level RunSpecifiedTests \
  --tests AccountServiceTest ContactServiceTest \
  --target-org <alias> \
  --result-format json \
  --wait 10

# Run specific test methods
sf apex run test \
  --test-level RunSpecifiedTests \
  --tests AccountServiceTest.testBulkUpdate AccountServiceTest.testNullIdThrows \
  --target-org <alias> \
  --result-format json \
  --wait 10

# Run all local tests (pre-deploy coverage check)
sf apex run test \
  --test-level RunLocalTests \
  --target-org <alias> \
  --result-format json \
  --code-coverage \
  --wait 30

# Run all tests including managed package tests (rarely needed)
sf apex run test \
  --test-level RunAllTestsInOrg \
  --target-org <alias> \
  --result-format json \
  --code-coverage \
  --wait 60
```

## --test-level Options

| Level | What runs | When to use |
|---|---|---|
| `RunSpecifiedTests` | Only the named test classes/methods | Targeted debugging, single-class coverage |
| `RunLocalTests` | All tests in the org (not managed package tests) | Pre-deploy coverage check, full regression |
| `RunAllTestsInOrg` | All tests including managed packages | Rarely needed; very slow |

## Result Format Options

| Format | Use case |
|---|---|
| `json` | Machine-parseable; use for scripted analysis |
| `tap` | TAP protocol; use for CI integration |
| `junit` | JUnit XML; use for CI reporting systems |
| `human` | Readable; use for interactive sessions |

## Async Run (no --wait)

For long test suites, omit `--wait` to run asynchronously and retrieve results later:

```bash
# Start async run
sf apex run test \
  --test-level RunLocalTests \
  --target-org <alias> \
  --result-format json
# Output includes: "testRunId": "707Xx000001ABCDEF"

# Retrieve results when ready
sf apex get test \
  --test-run-id 707Xx000001ABCDEF \
  --target-org <alias> \
  --result-format json \
  --code-coverage
```

## Org Verification Commands

```bash
# Verify org alias and type
sf org display --target-org <alias>
# Check: instanceUrl (sandbox vs prod), connectedStatus, username

# List all available orgs
sf org list
# Shows all authenticated orgs with type and alias

# Check org limits
sf org display --target-org <alias> --json
```

## Parsing JSON Results

Key fields in the JSON output:

```json
{
  "result": {
    "summary": {
      "outcome": "Passed | Failed",
      "testsRan": 42,
      "passing": 40,
      "failing": 2,
      "skipped": 0,
      "passRate": "95%",
      "failRate": "5%",
      "testStartTime": "2026-05-21T12:00:00.000Z",
      "testExecutionTime": "12345 ms",
      "testTotalTime": "12500 ms",
      "orgId": "00DXx000001ABCDEF",
      "username": "deploy@myorg.sandbox"
    },
    "tests": [
      {
        "id": "07MXx000001ABCDEF",
        "queueItemId": "7091X000002ABCDEF",
        "stackTrace": "Class.AccountService.updateIndustry: line 45, column 1\nClass.AccountServiceTest.testBulkUpdate: line 22, column 1",
        "message": "System.LimitException: Too many SOQL queries: 101",
        "asyncApexJobId": "707Xx000001ABCDEF",
        "methodName": "testBulkUpdate",
        "outcome": "Fail",
        "apexClass": {
          "id": "01pXx000001ABCDEF",
          "name": "AccountServiceTest",
          "namespacePrefix": null
        },
        "runTime": 1234,
        "testTimestamp": "2026-05-21T12:00:05.000Z",
        "fullName": "AccountServiceTest.testBulkUpdate"
      }
    ],
    "codecoverage": [
      {
        "id": "01pXx000001ABCDEF",
        "name": "AccountService",
        "type": "Class",
        "numLinesCovered": 45,
        "numLinesUncovered": 5,
        "coveredPercent": 90
      }
    ]
  }
}
```

## Coverage Analysis via jq

```bash
# Extract overall pass/fail summary
sf apex run test --test-level RunLocalTests --target-org <alias> --result-format json \
  | jq '.result.summary | {outcome, passing, failing, passRate}'

# Find classes below 75% coverage
sf apex run test --test-level RunLocalTests --target-org <alias> --result-format json --code-coverage \
  | jq '[.result.codecoverage[] | select(.coveredPercent < 75) | {name, coveredPercent}]'

# Extract failing test details
sf apex run test --test-level RunSpecifiedTests --tests MyClass --target-org <alias> \
  --result-format json \
  | jq '[.result.tests[] | select(.outcome == "Fail") | {fullName, message, stackTrace}]'
```
