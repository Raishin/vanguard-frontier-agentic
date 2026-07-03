# Fixtures, Auth Setup, and storageState Security

Use this reference when reviewing or designing Playwright `storageState`/auth fixtures, a `setup` project with `dependencies`, `globalSetup`, or when a `storageState.json`/HAR file needs a security review before it's committed or shared. For sharding/parallelism interactions with per-project storage state, see `ci-sharding-and-parallelism.md`.

> Version note: `storageState()` option shapes (e.g. the `indexedDB` capture flag) have been added across Playwright releases. Verify exact option availability against the installed version via Context7 (`/microsoft/playwright`) or the official docs before citing a capability as available.

## Officially grounded patterns

Playwright's own docs describe two supported, non-exclusive ways to produce a reusable auth state:

- **Setup project + `dependencies`.** A dedicated project (e.g. `{ name: 'setup', testMatch: /.*\.setup\.ts/ }`) performs authentication once and writes `storageState` to a file (e.g. `playwright/.auth/user.json`); browser-specific projects (`chromium`, `firefox`, ...) declare `dependencies: ['setup']` and set `use: { storageState: 'playwright/.auth/user.json' }` to consume it. This is the documented pattern for reusing one authenticated session across multiple projects/browsers without re-authenticating per project.
- **`globalSetup`.** A `globalSetup` function (referenced from `playwright.config.ts`) launches a browser, performs the login flow, and calls `page.context().storageState({ path: storageState })` once before the whole run. Both the setup-project pattern and `globalSetup` are documented; the setup-project pattern is the one that composes with `dependencies` and per-project overrides, so prefer it when the project already uses Playwright's project model.
- **API-based auth**, skipping the UI entirely: a `setup` test can call `request.post(...)` against a login endpoint and then `request.storageState({ path: authFile })` to persist cookies/tokens without driving a browser -- faster and less flaky than a UI-driven login, when the app's auth flow supports it.
- `storageState({ path, indexedDB: true })` is the documented way to also persist IndexedDB-backed session data (relevant for apps that store tokens in IndexedDB rather than cookies/localStorage); omitting `indexedDB: true` for such an app silently drops part of the session and can cause the "auth setup ran but tests still see a logged-out state" failure mode.

## Non-negotiable design rules

1. **Never generate `storageState.json` against a real user's live session.** It must come from a dedicated, disposable test account with no access to production customer data. A file captured from a real session contains live cookies/tokens that are functionally equivalent to that user's credentials.
2. **Treat `storageState.json` as a secret artifact, not a build output.** Do not commit it to a public repository. If it must persist across CI runs, store it as a short-lived CI artifact/secret with restricted access, not a tracked file in version control.
3. **Scrub HAR-file fixtures before committing them.** HAR recordings used for network-mocking (route interception replay) capture full request/response headers, which commonly include `Authorization` headers, API keys, or session cookies. A HAR fixture that hasn't been reviewed for these is a credential-leak risk equivalent to committing a raw token.
4. **Re-authenticate per test-account tier, not per test.** If the suite exercises multiple roles/permission levels, use multiple named `storageState` files (one per role) produced by distinct `setup` tests, rather than one shared elevated-privilege session reused everywhere -- reusing an admin session for tests that only need a standard-user role overstates the privilege the test actually needs and can mask authorization bugs.
5. **Expire and rotate test-account credentials used to produce fixtures on a defined cadence.** A `storageState` fixture generated from a test account whose password/token never rotates is a long-lived credential sitting in CI infrastructure.

## Response discipline

When reviewing fixtures, state explicitly whether the `storageState`/HAR file was confirmed to originate from a dedicated test account (repo evidence: setup-test source, CI secret config) or whether that could not be confirmed from available evidence -- do not assume test-only provenance without a citation.
