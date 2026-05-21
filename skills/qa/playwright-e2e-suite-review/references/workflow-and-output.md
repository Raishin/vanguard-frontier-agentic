# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized snippets (no live URLs with embedded credentials, no auth tokens, no real `storageState` JSON, no `.env` contents):
- Playwright spec files (`*.spec.ts`, `*.spec.js`, `tests/**`)
- `playwright.config.ts` / `playwright.config.js`
- Page object / fixture files (`fixtures.ts`, `pages/**`)
- The CI workflow step that runs Playwright (GitHub Actions, GitLab CI, etc.)
- Optional: a recent CI failure log or flaky-test report

If only a partial set is provided, note which inputs are absent and scope findings accordingly. A config without specs, or specs without a config, each leaves a blind spot — say so.

### Step 2 — Flakiness audit

Scan every spec for time-based and non-retrying synchronization.

**2a. Hard waits**
```ts
// HIGH — fixed sleep races the application
await page.waitForTimeout(2000);
await page.click('#submit');
```
`waitForTimeout` is for debugging only. It either fires before the app is ready (flake) or pads every run (slow). Replace with an action or web-first assertion that auto-waits:
```ts
// CORRECT — auto-waits for the element to be actionable
await page.getByRole('button', { name: 'Submit' }).click;
```

**2b. Manual non-retrying assertions**
```ts
// HIGH — snapshots one instant, no auto-retry
expect(await page.getByText('welcome').isVisible).toBe(true);
```
Web-first assertions retry until the condition holds or the timeout expires:
```ts
// CORRECT
await expect(page.getByText('welcome')).toBeVisible;
```
Flag any `expect(await ...)` wrapping `isVisible`, `textContent`, `innerText`, `count`, `getAttribute` as HIGH.

**2c. Network-idle as a synchronization crutch**
```ts
// MEDIUM — fragile under analytics, polling, websockets
await page.waitForLoadState('networkidle');
```
`networkidle` is discouraged for general synchronization. Wait on the specific signal instead:
```ts
await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible;
// or
await page.waitForResponse(r => r.url.includes('/api/orders') && r.ok);
```

### Step 3 — Selector brittleness audit

Review the locator strategy in every spec and page object.

| Locator pattern | Verdict | Why |
|---|---|---|
| `getByRole`, `getByLabel`, `getByText`, `getByTestId` | preferred | resilient to refactor; user- or contract-facing |
| `data-testid` CSS (`[data-testid="x"]`) | acceptable | stable contract, but `getByTestId` is clearer |
| deep CSS chain (`div > div:nth-child(3) .btn`) | HIGH | breaks on any layout change |
| hashed/generated class (`.css-1a2b3c`, `.MuiBox-root`) | HIGH | regenerated on every build |
| raw XPath (`//div[2]/span`) | HIGH | brittle, hard to read |
| `nth` / index-based selection on dynamic lists | MEDIUM | breaks when list order or length changes |

Flag each HIGH locator with the spec file and the recommended role/label/test-id replacement.

### Step 4 — Test isolation audit

Verify each test is independent and order-free.

Check for:
- Module-level mutable variables written by one `test` and read by another → HIGH
- A test that creates a record (user, order) consumed by a later test → HIGH (breaks under sharding and `--shuffle`)
- `test.describe.serial` used to paper over a shared-state dependency rather than for a genuine sequential flow → HIGH
- `beforeAll` performing mutable setup that tests then modify without reset → MEDIUM
- Shared `storageState` file written to by tests → MEDIUM (cross-test auth contamination)

```ts
// HIGH — test B depends on test A's side effect
let createdOrderId;
test('creates order', async  => { createdOrderId = await createOrder; });
test('views order', async  => { await page.goto(`/orders/${createdOrderId}`); });

// CORRECT — each test owns its data via a fixture
test('views order', async ({ orderFixture }) => {
  await page.goto(`/orders/${orderFixture.id}`);
});
```

### Step 5 — Retry and observability audit

Review `retries`, `trace`, `screenshot`, `video` in `playwright.config`.

- `retries > 0` in CI with no flaky surfacing (no `trace: 'on-first-retry'`, no flaky reporter, no quarantine list) → HIGH. Retries are a buffer to *fix* flakes, not to *hide* them. A test that only passes on retry must be visible and tracked.
- `trace`, `screenshot`, and `video` all `'off'` for the CI project → HIGH. A CI-only failure with zero artifacts is undebuggable; engineers re-run blindly.
- Recommended CI baseline:
```ts
export default defineConfig({
  retries: process.env.CI ? 2 : 0,
  use: {
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  reporter: [['html'], ['github']],
});
```

### Step 6 — CI configuration audit

Review parallelism, sharding, and timeouts.

- `fullyParallel: false` without a stated reason → MEDIUM (serial execution blocks deploys).
- A large suite running in a single CI job with no `--shard` matrix → MEDIUM. Recommend a shard matrix:
```yaml
strategy:
  matrix:
    shard: [1/4, 2/4, 3/4, 4/4]
steps:
  - run: npx playwright test --shard=${{ matrix.shard }}
```
- Global `timeout` or `expect.timeout` raised far above default to force a pass → MEDIUM. The raised timeout masks a real slow path or race; flag the underlying cause.
- `workers` pinned to 1 in CI without justification → MEDIUM.
- No `--forbid-only` (or equivalent) in CI → MEDIUM: a stray `test.only` silently skips the rest of the suite.

### Step 7 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<spec and config provided | partial artifacts | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security notes

- Never request or accept live application URLs with embedded credentials, bearer tokens, real `storageState.json`, or `.env` contents. Ask for sanitized snippets.
- This is a static review: do not run `npx playwright test`, launch browsers, or contact the application under test.
- Do not recommend `.skip` or deleting a flaky test as the fix — every flaky test needs a root-cause category (race, hard wait, shared state, brittle selector) and a quarantine/tracking path so it is fixed, not buried.
- Do not recommend raising timeouts or adding retries to make a suite "go green" — both mask defects the review exists to surface.
