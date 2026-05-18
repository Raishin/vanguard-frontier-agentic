# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized snippets (no credentials, no real customer data in fixtures, no production DB snapshots):
- Test files for the modules under review
- A coverage report (text summary, `lcov`, or HTML summary numbers)
- The source of the code under test, so assertions can be checked against actual behavior
- The CI coverage-gate configuration (threshold, scope)

If only a coverage report is supplied without test source, the review can only flag *suspiciously high coverage* and cannot judge assertion quality — say so and ask for test source.

### Step 2 — Assertion presence audit

For every test, confirm it makes at least one assertion that can fail.

```js
// HIGH — no assertion; passes as long as nothing throws
test('processes the order', async () => {
  await processOrder(sampleOrder);
});

// CORRECT — asserts the observable outcome
test('processes the order', async () => {
  const result = await processOrder(sampleOrder);
  expect(result.status).toBe('confirmed');
  expect(result.total).toBe(149.97);
});
```
Flag every assertion-free test as HIGH. These are pure coverage inflation: the lines execute, the percentage rises, nothing is verified.

### Step 3 — Assertion strength audit

Grade each assertion.

| Assertion pattern | Grade | Note |
|---|---|---|
| exact value (`toBe(149.97)`, `toEqual({...})`) | strong | fails on any wrong value |
| `toBeDefined()`, `not.toBeNull()`, `toBeTruthy()` | weak (MEDIUM) | passes for wrong values; use when an exact value is unknowable only |
| `toBeGreaterThan(0)`, `length > 0` | weak (MEDIUM) | passes for `[wrong, wrong]` |
| `expect(true).toBe(true)`, `expect(x).toBe(x)` | tautological (HIGH) | cannot fail |
| auto-updated snapshot of logic output | weak (MEDIUM) | detects change, not correctness |

Flag tautological assertions as HIGH and shape-only assertions (where an exact value is knowable) as MEDIUM.

### Step 4 — Mock usage audit

Review tests that use mocks, stubs, or spies.

**4a. Call-assertion-only**
```js
// HIGH — asserts the mock was called, never the behavior
test('sends the email', async () => {
  await notifyUser(user);
  expect(emailService.send).toHaveBeenCalled();
});
```
This verifies wiring, not outcome. It should also assert *what* was sent and the result the caller depends on.

**4b. Over-mocking**
```js
// HIGH — every collaborator mocked; assertions restate the setup
const repo = { find: jest.fn().mockReturnValue({ id: 1, price: 10 }) };
const tax = { calc: jest.fn().mockReturnValue(2) };
test('total', () => {
  const t = new Cart(repo, tax).total(1);
  expect(t).toBe(12); // 10 + 2 — but both came from mocks
});
```
When every input is mocked and the assertion is arithmetic over mock return values, the test is a mirror of its own setup. It cannot catch a bug in `find`, `calc`, or their integration. Recommend testing the real collaboration, or moving the guarantee to an integration test.

### Step 5 — Branch coverage gap audit

For code under test that contains error paths, boundaries, or empty-input branches, confirm each has a test.

- Error path (`catch`, thrown error, rejected promise) with no test → HIGH.
- Boundary (zero, empty array, max value, off-by-one) with no test → HIGH.
- Only the happy path tested while the source has multiple branches → HIGH: the coverage percentage is inflated by the easy path while real failure modes ship untested.

Distinguish *line* coverage from *branch* coverage in the report — a function can be 100% line-covered with half its branches unexercised.

### Step 6 — Coverage gate audit

Review the CI coverage gate.

- A global line-percentage threshold as the sole quality signal → MEDIUM. It is satisfied by assertion-free tests; the number does not measure verification.
- No per-changed-file / diff coverage → MEDIUM: new untested code hides behind a large tested codebase. Recommend diff coverage on changed lines.
- 100% coverage presented as the goal → MEDIUM: it incentivizes theater. The goal is meaningful assertions on behavior that matters.
- Recommended posture: keep a modest line threshold as a floor, **add diff coverage** on changed lines, and review assertion quality in code review — the percentage is a floor, not a target.

### Step 7 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: tests verify behavior / coverage theater detected / mixed>

## Evidence level
<test source + coverage report provided | coverage report only | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding>: <description> — <remediation>

### HIGH
- [H1] <finding>: <description> — <remediation>

### MEDIUM
- [M1] <finding>: <description> — <remediation>

### LOW
- [L1] <finding>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request credentials, fixtures containing real customer data, or production database snapshots. Ask for sanitized test code.
- This is a static review: do not run the test suite or a coverage tool.
- Do not recommend raising the coverage percentage threshold as a quality improvement — a higher threshold is satisfied by more assertion-free tests. Recommend assertion strength and diff coverage instead.
- A high coverage number with weak assertions is more dangerous than a low number, because it manufactures false confidence. Say so explicitly when the evidence shows it.
