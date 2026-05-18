---
name: ci-test-pipeline-review
description: Use this skill when reviewing how a CI pipeline runs tests — gating, sharding, parallelism, fail-fast behavior, artifact retention, and flaky-test quarantine wiring. Trigger when a user provides a CI workflow file (GitHub Actions, GitLab CI, CircleCI, Jenkins), asks why CI is slow or unreliable as a merge gate, or wants to know whether their test pipeline actually blocks bad merges. This skill reviews CI configuration statically; it does not trigger or run pipelines.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: delivery
  lifecycle: experimental
---

# CI Test Pipeline Review

## Purpose
This skill reviews how a CI pipeline runs tests — not the tests themselves, but the pipeline that decides whether they block a merge. A test suite only protects the main branch if the pipeline runs it, runs it on the merge gate, fails the build when it fails, and finishes fast enough that developers do not route around it. The review catches non-blocking test steps, soft-failure escape hatches, missing required-check enforcement, un-sharded slow suites, fail-fast that hides parallel failures, missing artifacts, and quarantine lanes wired so that quarantined tests silently never run again.

## Lean operating rules
- Treat a test step that cannot fail the build — `|| true`, `continue-on-error: true`, `set +e`, an exit code swallowed, a non-blocking/optional check — as CRITICAL: the suite exists but gates nothing, and every "green" merge is unverified.
- Treat tests that run only post-merge (on `push` to main, nightly) and not on the pull-request merge gate as HIGH — regressions are caught after they are already on the main branch.
- Treat the test job not being a required status check for branch protection as HIGH — the run is advisory and a merge can proceed red. (Flag as inference if branch-protection config is not provided.)
- Treat `fail-fast: true` on a test matrix as MEDIUM — it cancels sibling shards on the first failure, hiding how many shards actually failed and forcing repeated partial runs.
- Treat a large suite in a single un-sharded job as HIGH when wall-clock time blocks merges — recommend a shard matrix sized to the suite.
- Treat the absence of test-result and failure-artifact upload (JUnit XML, traces, screenshots, logs) as HIGH — a CI-only failure is then undebuggable and engineers re-run blindly.
- Treat caching of dependencies/build but not keyed correctly (stale cache, no lockfile in the key) as MEDIUM — stale caches cause non-reproducible passes and failures.
- Treat a quarantine lane that excludes flaky tests from gating with no scheduled non-blocking run and no tracking as HIGH — quarantined tests then never run again and the coverage is silently lost.
- Treat secrets exposed to test jobs triggered by `pull_request_target` or to fork PRs as CRITICAL security exposure — flag and stop.
- Treat a missing concurrency/cancel-in-progress group on PR test runs as LOW — wasted runners, not a correctness issue.
- Do not recommend disabling or making a flaky check non-blocking as the fix — recommend quarantine with a scheduled run and an owner.
- Label every finding with evidence basis: CI config provided, branch-protection config provided, documentation-based, or inference.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Gating findings (non-blocking steps, soft-failure escape hatches, required-check enforcement)
- Merge-gate timing findings (PR vs. post-merge, sharding, parallelism)
- Fail-fast and matrix configuration findings
- Artifact and observability findings (test results, failure artifacts)
- Quarantine-lane wiring findings
- Security findings (secret exposure to test jobs)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
