# Apex Code Coverage Analysis Reference

Adapted from forcedotcom/sf-skills running-apex-tests references (Apache-2.0).

## The 75% Rule

Salesforce requires 75% aggregate code coverage across all Apex classes and triggers
(excluding test classes) for deployment to production. This is a deployment gate —
failing this threshold blocks all production deployments.

**Key facts:**
- 75% applies to the **aggregate** across all classes, not each class individually
- However, any single class below 75% may still block if that class is in the deployment
- Triggers must have at least 1% coverage (at least one line covered) to deploy
- Classes with 0% coverage block deployment if they are included in the deployment package

## Coverage Report Interpretation

From `sf apex run test --code-coverage --result-format json`:

```json
{
  "codecoverage": [
    {
      "name": "AccountService",
      "numLinesCovered": 45,
      "numLinesUncovered": 5,
      "coveredPercent": 90,
      "uncoveredLines": [23, 45, 67, 89, 102]
    }
  ]
}
```

- `coveredPercent` = `numLinesCovered / (numLinesCovered + numLinesUncovered) * 100`
- `uncoveredLines` — line numbers not executed by any test; these are the gaps to fill

## Coverage Gap Triage

When a class is below threshold, identify the gap type:

| Gap type | Description | Fix strategy |
|---|---|---|
| **Exception path uncovered** | Try/catch blocks with no test for the catch | Add negative test that triggers the exception |
| **Bulk path uncovered** | Code only reached with > 200 records | Add bulk test method |
| **Conditional branch uncovered** | else/if branch never executed | Add test for each conditional branch |
| **Async path uncovered** | Queueable/Batch execute never called | Add async test with Test.startTest/stopTest |
| **Early return uncovered** | Guard clause never triggered | Add test with invalid/null input |

## Line vs Branch Coverage

Salesforce reports **line coverage** (not branch coverage). A line is "covered" if it
was executed at least once by any test. This means:

- An if/else on one line may show as covered even if only one branch was taken
- 100% line coverage does not guarantee all code paths are tested
- Aim for explicit positive + negative + bulk tests to cover both branches even if
  line coverage shows 100%

## Per-Class Coverage Thresholds

| Coverage | Status |
|---|---|
| >= 90% | Excellent — deploy with confidence |
| 75–89% | Acceptable — at or above deployment threshold |
| 50–74% | Warning — below deployment threshold; gaps likely in exception/bulk paths |
| < 50% | Critical — significant coverage gaps; deployment will likely fail |
| 0% | Blocker — class will prevent deployment if included in the package |

## Identifying Uncovered Lines

After a test run, retrieve specific uncovered lines:

```bash
sf apex get test \
  --test-run-id <id> \
  --target-org <alias> \
  --result-format json \
  --code-coverage \
  | jq '[.result.codecoverage[] | {name, coveredPercent, uncoveredLines}]'
```

Then read the class file to identify which code paths the uncovered lines correspond to:

```bash
# Find the class file and check uncovered lines
grep -n "." force-app/main/default/classes/AccountService.cls | sed -n '23p;45p;67p'
```

## Coverage Strategy Recommendations

For each class below 75%:

1. **Check test class exists** — if no test class exists, generate one with
   `salesforce-apex-test-generator-skill`
2. **Check for async paths** — if Queueable/Batch methods are present, verify
   `Test.startTest` / `Test.stopTest` wraps the enqueue call in tests
3. **Check exception paths** — add tests that trigger catch blocks
4. **Check bulk path** — add a test with 201+ records if trigger-context logic exists
5. **Check conditional branches** — identify if/else and add tests for each branch

## Coverage in CI/CD Context

For pre-deployment validation with `salesforce-deployment-validator-skill`:
- Use `--test-level RunLocalTests --code-coverage` to measure full org coverage
- Set a stricter internal threshold (80-85%) to provide buffer above the 75% hard limit
- Track per-class coverage trends across deployments to catch regressions early
