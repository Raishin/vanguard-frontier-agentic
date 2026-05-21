# Test Selection Strategy

Reference for choosing the correct Apex test level for deployment validation
and constructing an accurate test class list.

---

## Test Level Options

| Test Level | When to Use | Coverage Source |
|---|---|---|
| `RunSpecifiedTests` | Preferred for scoped changes — Apex classes or triggers changed | Specified test classes only |
| `RunLocalTests` | When broader confidence is needed — profile, permission set, or cross-class changes | All local (non-managed) test classes in the org |
| `RunAllTestsInOrg` | Only for major releases — full org regression | All test classes including managed packages |
| `NoTestRun` | Metadata-only changes with NO Apex content (labels, static resources, custom fields without triggers) | Not applicable — no coverage check |

**Default for this skill:** `RunSpecifiedTests`. Always prefer the narrowest
scope that still satisfies the 75% coverage requirement.

---

## 75% Coverage Requirement

Salesforce requires that each Apex class and trigger included in the
deployment has at least **75% code coverage** as measured by the tests
run during validation. This is an absolute requirement — deployments fail
if any component falls below this threshold.

**Coverage is per-class, not aggregate** for the purposes of deployment
validation. An org-wide aggregate of 75% does not override a single class
at 40%.

**Exception:** Classes with `@isTest` annotation are exempt — they are test
classes themselves and do not require coverage.

---

## RunSpecifiedTests — When and How

### When to use

Use `RunSpecifiedTests` when:
- The deployment touches specific Apex classes or triggers
- The test classes that cover those specific classes are known
- You want to minimize validation time and test org impact

### How to construct the test list

For each Apex class or trigger in the deployment manifest, identify the
test classes that cover it. The mapping can be derived from:

1. **Source code inspection** — search the test classes for methods that
   instantiate or call the production class:

   ```bash
   grep -r "MyProductionClass" force-app/main/default/classes/*Test*.cls
   grep -r "MyProductionClass" force-app/main/default/classes/*_Test.cls
   ```

2. **sf CLI coverage report** — run `sf apex run test --code-coverage` against
   the sandbox to see which test classes cover which production classes:

   ```bash
   sf apex run test \
     --class-names MyTest1 MyTest2 \
     --code-coverage \
     --result-format json \
     --target-org <sandbox_alias>
   ```

3. **Naming convention regex** — common patterns for test class names:

   ```
   # Convention 1: <ClassName>Test
   ^<ClassName>Test$

   # Convention 2: <ClassName>_Test
   ^<ClassName>_Test$

   # Convention 3: Test<ClassName>
   ^Test<ClassName>$

   # Convention 4: <ClassName>Tests (plural)
   ^<ClassName>Tests$
   ```

   Apply all four patterns to the available test class list to identify
   candidates, then verify coverage via `sf apex run test --code-coverage`.

### Constructing the --tests flag

```bash
sf project deploy validate \
  --manifest package.xml \
  --target-org <sandbox_alias> \
  --test-level RunSpecifiedTests \
  --tests MyClass_Test AnotherClass_Test ThirdClass_Test \
  --wait 30 \
  --json
```

Space-separate multiple test class API names. Do not use commas.

---

## RunLocalTests — When and How

### When to use

Use `RunLocalTests` when:
- The deployment includes profile changes, permission set changes, or
  custom object changes that could affect multiple classes
- The specific test classes covering changed components are unknown
- The deployment is a significant refactor touching many classes
- `RunSpecifiedTests` produced coverage warnings despite adding all known
  test classes (RunLocalTests may reveal additional covering tests)

### Caveats

- `RunLocalTests` runs ALL non-managed test classes in the org. For large
  orgs with extensive test suites, this can take 30–60+ minutes.
- Adjust `--wait` accordingly:

  ```bash
  sf project deploy validate \
    --manifest package.xml \
    --target-org <sandbox_alias> \
    --test-level RunLocalTests \
    --wait 60 \
    --json
  ```

- Failures in unrelated test classes will cause the validation to fail even
  if the deployment components themselves are correct. Investigate whether
  pre-existing test failures exist in the sandbox before running RunLocalTests.

---

## RunAllTestsInOrg — When and How

### When to use

Use `RunAllTestsInOrg` only when:
- Preparing for a major release that touches core shared utilities
- The org has managed packages with test classes that must be included
- Compliance or audit requirements mandate full-suite regression

### Caveats

- This is the slowest option — expect 60–120+ minutes in large orgs.
- Managed package test failures can block validation even when unrelated
  to the deployment. Document known pre-existing managed package test
  failures before running.
- Set `--wait` to at least 120 minutes:

  ```bash
  sf project deploy validate \
    --manifest package.xml \
    --target-org <sandbox_alias> \
    --test-level RunAllTestsInOrg \
    --wait 120 \
    --json
  ```

---

## NoTestRun — Metadata-Only Changes

Use `NoTestRun` only when the deployment manifest contains NO Apex classes
or triggers — for example:

- Custom labels, custom metadata records
- Static resources, documents
- Custom fields without associated triggers
- Page layouts, list views
- Email templates, dashboards, reports (when no Apex is involved)

**Do not use `NoTestRun` if there is any doubt about whether the changed
metadata triggers Apex.** Flows that invoke Apex actions, validation rules
with custom formula references, and workflow rules can indirectly involve
Apex — use `RunLocalTests` when uncertain.

---

## Coverage Delta Analysis

After a validation run, compare the coverage result against the baseline
to identify regressions.

### Extracting coverage from validation output

```bash
sf project deploy validate --manifest package.xml --target-org <alias> --json \
  | jq '.result.runTestResult.codeCoverageWarnings[] | {class: .name, warning: .message}'
```

### Coverage delta calculation

1. **Baseline:** The most recent successful RunLocalTests or RunAllTestsInOrg
   result in the sandbox (stored as a JSON file in your CI pipeline).
2. **Current:** The coverage percentages from this validation run.
3. **Delta:** Current − Baseline per class.

Flag any class where:
- Coverage drops below 75% (deployment blocker)
- Coverage drops by more than 10 percentage points (regression risk)
- A previously-100%-covered class drops below 90% (regression signal)

Include this analysis in the `coverage_delta` field of the skill output.

### Coverage formula

```
coverage_percent = (total_lines - uncovered_lines) / total_lines * 100
```

Salesforce reports this per class in the `codeCoverageResult` array within
the test run output.

---

## Test Class List Construction Checklist

Before running `RunSpecifiedTests`, verify:

- [ ] All Apex classes in the deployment manifest have at least one test class
      mapped to them
- [ ] All Apex triggers in the manifest have at least one test class mapped
- [ ] The test classes mapped actually cover the production classes (verify
      via `sf apex run test --code-coverage` if uncertain)
- [ ] Test class API names are correct (case-sensitive match to the class
      name in the org)
- [ ] No test classes are in the list that are themselves managed packages
      (managed package test classes cannot be run via RunSpecifiedTests in
      most orgs)

If any Apex class in the manifest cannot be mapped to a test class, escalate
to `RunLocalTests` rather than proceeding with incomplete coverage.

---

## Common Coverage Failures and Remediation

| Failure | Likely Cause | Remediation |
|---|---|---|
| Class at 0% coverage | No test class covers it; test class not included in `--tests` | Add the covering test class to the `--tests` list |
| Class at 60–74% coverage | Test class exists but doesn't cover all branches | Add more test methods; or use RunLocalTests to find broader coverage |
| Previously-passing class now at 0% | Test class was deleted or renamed | Identify the replacement test class |
| Managed package class coverage warning | RunAllTestsInOrg picked up managed test classes | Switch to RunLocalTests to exclude managed tests |
| Validation timeout | Test suite too large for `--wait` value | Increase `--wait`; switch to RunSpecifiedTests to reduce scope |
