# CI Sharding and Parallelism

Use this reference when configuring or reviewing Playwright's `--shard` CLI flag, a CI matrix strategy, blob-reporter merging, `fullyParallel`, or `workers` tuning. For auth-state interactions across sharded/parallel projects, see `fixtures-and-auth-setup.md`.

> Version note: the `blob` reporter and `npx playwright merge-reports` command are the documented mechanism for combining sharded results into one report. Verify exact reporter name and CLI flags against the installed version via Context7 (`/microsoft/playwright`) or the official docs before citing them in a report.

## Officially grounded design points

- **`fullyParallel` and sharding solve different bottlenecks.** `fullyParallel: true` in `playwright.config.ts` parallelizes tests *within a single file* in addition to across files (by default, Playwright already parallelizes across files using `workers`). CI-level sharding (`--shard=<index>/<total>`) instead splits the *set of test files* across separate CI jobs/machines. A suite bottlenecked by one huge file with many tests needs `fullyParallel`; a suite bottlenecked by total file count/CI wall-clock time needs sharding (or both).
- **`test.describe.parallel()`** is the documented, file-scoped alternative to global `fullyParallel` -- it opts a specific `describe` block into within-file parallel execution without changing the setting for the whole suite.
- **Sharding requires a report-merge step to remain useful.** The documented CI pattern is: a matrix job runs `npx playwright test --shard=${{ matrix.shardIndex }}/${{ matrix.shardTotal }}` with `reporter: 'blob'` configured, each shard uploads its `blob-report` directory as a CI artifact, and a separate downstream job downloads all shard artifacts and runs `npx playwright merge-reports --reporter html ./all-blob-reports` to produce one combined report. Sharding without this merge step means each shard produces an isolated, easy-to-miss report instead of one gating artifact.
- **`workers`** controls the number of parallel worker processes *within* a single Playwright invocation (i.e., within one shard/CI job); it is a separate lever from both `fullyParallel` and `--shard` and is commonly constrained by the CI runner's available CPU/memory, not just desired speed.

## Non-negotiable design rules

1. **Do not recommend sharding to fix flakiness.** Sharding distributes work across CI jobs; it does not change whether an individual test is flaky and can even change failure order/timing in ways that mask a root cause. If the underlying request is "the suite is unreliable," diagnose flakiness first (locators, waits, isolation) and treat sharding purely as a wall-clock-time lever.
2. **Never introduce `--shard` without also configuring `reporter: 'blob'` and a merge job.** A sharding change that lands without the corresponding merge-reports job in CI produces N disconnected reports and typically means nobody actually reviews the combined result -- treat this as an incomplete change, not a matter of taste.
3. **Justify sharding with a measured wall-clock baseline, not a guess.** Require the current CI suite duration before recommending a specific shard count; splitting a suite that already finishes in 3 minutes into 8 shards adds CI queueing/setup overhead without meaningful benefit and increases the merge-job's own failure surface.
4. **Tune `workers` to the CI runner's actual resources, not a fixed "more is faster" assumption.** Over-provisioning workers on a memory-constrained runner is a documented source of resource contention that manifests as flaky, not slow, tests -- which is easy to misdiagnose as an application bug.
5. **`fail-fast: false` in a CI matrix is the documented choice for shard jobs** so that one failing shard doesn't cancel the others and hide additional failures that would otherwise surface in the same run.

## Response discipline

When reviewing a sharding/parallelism setup, state the current shard count and worker configuration as found (repo/CI-config evidence), the measured baseline duration if available (or explicitly flag it as missing), and whether a `merge-reports` step exists -- an incomplete shard-without-merge config is a hard finding, not a style note.
