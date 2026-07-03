# Task-Graph Review (Turborepo / Nx)

Use this reference for reviewing `turbo.json` or `nx.json` task-graph configuration to catch false-green CI: a task that reports success/cache-hit without actually re-running against a real upstream change.

> Version note: Turborepo renamed the top-level `pipeline` key to `tasks` (current schema uses `tasks`). Nx task defaults live in `nx.json` under `targetDefaults`, or per-project under an `nx` block in `package.json`/`project.json`. Verify which key the installed major version expects via Context7 before proposing a diff — do not silently rewrite `pipeline` to `tasks` (or vice versa) without confirming the installed version.

## What people get wrong

The naive story is:

> "The task graph is just for parallelism/ordering; caching is a separate, unrelated performance feature."

Wrong. In both Turborepo and Nx, the task graph and the cache key are the same mechanism: a task's cache hit/miss is computed from its declared `inputs` (Nx) or the combination of source files plus `env`/`globalEnv` (Turborepo), plus the recursive hash of everything in its `dependsOn` chain. A gap in either the dependency edges or the cache-key inputs does not just cause wasted rebuilds — it causes **wrongly cached passes**: CI reports green using output from a version of the code that no longer exists in the branch being tested.

## Officially grounded shape

From Turborepo docs (`configuring-tasks`, `caching`, `reference/configuration`):

- `tasks.<name>.dependsOn` — declares task ordering and cross-package dependency. `"^build"` means "the `build` task of every package this package depends on must run first." Omitting this for a task that reads another package's build output is the single most common false-green cause.
- `tasks.<name>.outputs` — file globs that get cached. Without `outputs`, nothing is cached for that task (Turborepo does not silently assume `dist/**`).
- `tasks.<name>.inputs` — restricts which source files affect the cache hash for that task specifically; if omitted, Turborepo hashes the package's full file set by default (verify current default behavior against docs for the installed version, since defaults have changed across majors).
- `tasks.<name>.env` / top-level `globalEnv` — environment variables that affect the cache hash. A task whose output depends on an env var not listed here can produce a cache **hit** even though the env var changed — a classic false-green pattern (e.g., `API_SERVICE_KEY` changed but `build` still serves the stale cached bundle).

From Nx docs (`concepts/task-pipeline-configuration`, `reference/project-configuration`, `features/cache-task-results`):

- `targetDefaults.<target>.dependsOn` — same role as Turborepo's `dependsOn`; `"^build"` again means upstream-project build first.
- `namedInputs` + `targetDefaults.<target>.inputs` — Nx's cache-key input set. A `production` named input that excludes test files is common; if a target's `inputs` accidentally excludes a file that genuinely affects its output (e.g., a shared `tsconfig.json` not covered by any named input), that target can cache-hit on a real change.
- `outputs` — same caching role as Turborepo; must match what the target actually writes.
- Per-project `nx` blocks in `package.json` or `project.json` can override `targetDefaults` — always check both locations before concluding a target has no cache configuration.

## Non-negotiable design rules

### 1. Every cross-package read needs a graph edge

If package B's task reads package A's build output (imports `A/dist`, reads a generated type file, etc.), B's task must declare `dependsOn: ["^build"]` (or the Nx equivalent) referencing that relationship. Do not accept "it happens to work because of task ordering in CI scripts" as a substitute — that is fragile and breaks under `--parallel`/affected-only runs.

### 2. Cache key completeness is a correctness property, not an optimization

Treat a missing `env` entry or an under-scoped `inputs`/`namedInputs` pattern as a bug report, not a suggestion. Ask: "if I change only this file/env var and nothing else, does the declared cache key change?" If the answer is no and the file/var genuinely affects output, that is a blocking finding.

### 3. Do not conflate `outputs` misconfiguration with `inputs` misconfiguration

`outputs` gaps cause **missing** cached artifacts (annoying, but safe — forces a rebuild). `inputs`/`env` gaps cause **wrongly reused** cached artifacts (unsafe — false green). Report these as distinct severities; do not lump them into one "caching issue" finding.

### 4. Legacy `pipeline` key requires a version check, not a silent rewrite

If a `turbo.json` uses `pipeline` instead of `tasks`, do not assume it is simply outdated syntax to "fix." Confirm the installed Turborepo major version via `package.json`/lockfile before recommending the rename, since `pipeline` may still be the correct key for an older pinned version.

### 5. Remote cache tokens are CI secrets, not repo config

`TURBO_TOKEN`/`TURBO_TEAM` (Turborepo Remote Cache) or Nx Cloud access tokens must be sourced from CI secret stores (e.g., referenced as `${{ secrets.TURBO_TOKEN }}` in a GitHub Actions workflow), never hardcoded in `turbo.json`, `nx.json`, or committed CI YAML. Any literal token-shaped string in these files is an automatic blocking finding.

## Minimal safe review flow

1. Identify the tool (`turbo.json` present → Turborepo; `nx.json` present → Nx; both possible in migration).
2. For the task(s) in scope, list every `dependsOn`/`targetDefaults.dependsOn` edge and cross-check against actual cross-package imports/reads in the source (grep for imports of other workspace packages' build output paths).
3. For each task with caching enabled, list declared `inputs`/`env`/`globalEnv` and cross-check against files/env vars the task's underlying command actually reads (config files, `.env` references, shared tsconfig/eslint config).
4. Check for any remote-cache token or registry auth token literal in `turbo.json`, `nx.json`, or CI workflow files.
5. Report gaps with exact file:line citations and the specific missing key/edge — do not apply the fix; propose the diff.

## Adversarial checklist

Before signing off on a task-graph config, answer these:

- If I changed one source file in an upstream package and nothing else, would every downstream task that reads it show a cache miss?
- If I changed one environment variable that affects a task's build output, would that task show a cache miss?
- Does every `outputs` glob match what the underlying build command actually writes — no more, no less?
- Is there a task that depends on another package's output without declaring `dependsOn` for it, relying instead on incidental script ordering?
- Is a remote-cache or registry token present as a literal anywhere in this config or adjacent CI YAML?

If you cannot answer all of these from repo evidence, the review is incomplete — say so rather than approving.

## When to push back

Push back if the user asks to:

- "just add `dependsOn: []` everywhere to speed things up" — this removes correctness guarantees, not just performance overhead,
- silently rewrite `pipeline` to `tasks` (or the reverse) without checking the installed version first,
- disable caching entirely as a workaround for a false-green bug instead of fixing the missing `inputs`/`env` entry — that hides the symptom without fixing the cache-key gap, and reintroduces it the moment caching is re-enabled,
- commit a remote-cache token directly into `turbo.json`/`nx.json`/CI YAML "just to get CI working."

Those are not shortcuts. They trade a diagnosable correctness bug for either wasted CI time or a live credential exposure.
