# EVAL: qa-cluster

Golden evaluation harness for the QA skill/agent cluster (`skills/qa/*`,
`agents/qa/*`). Eval-driven development artifact — defines pass/fail
criteria for reference grounding and agent wiring, graded deterministically
by `tests/eval-qa-cluster.mjs` and gated in CI via `npm run validate:qa-cluster`.

## Capability Evals

Per QA skill (7) and QA agent (7):

- **REF-1** — skill metadata declares >= 3 `official_docs` entries
- **REF-2** — no stale dated version pins (e.g. `/2023.10/`) in `official_docs`;
  vendor docs must point at current/`latest` paths
- **REF-3** — `SKILL.md` links its progressive-disclosure
  `references/workflow-and-output.md`
- **REF-4** — agent `companion_skills` resolve to an existing skill `SKILL.md`
- **HEUR-1** — static-review skills enumerate HIGH/MEDIUM severity heuristics;
  the read-only-runtime execution skill defines the
  `pass`/`fail`/`manual-review` verdict shape instead
- **HEUR-2** — the output contract carries all four severity buckets
  (review skills) or the verdict shape (execution skill)
- **HARNESS-1** — every `harness_variants` file exists on disk
- **HARNESS-2** — harness count matches the execution tier
  (`static-review` = 7, `read-only-runtime` = 2)

## Regression Evals

- `npm run validate` — full marketplace gate chain stays green
- `npm run validate:qa-cluster` — this eval is wired as a release gate

## Success Metrics

- Capability evals: pass^1 = 100% (56/56 deterministic checks)
- Regression evals: pass^1 = 100% for the release-critical gate chain

## Continuous-Loop Gate

`validate:qa-cluster` is the loop gate for QA-cluster changes. Pattern:
**sequential** (quality-gate eval loop) — run the gate, fix the failing
unit, re-run until green.

Convergence record:
- Iteration 1 — 54/56 (the runtime-tier execution skill mis-graded against
  review-skill severity heuristics)
- Fix — made HEUR-1/HEUR-2 tier-aware: verdict shape for `read-only-runtime`,
  severity buckets for `static-review`
- Iteration 2 — 56/56; converged
- Stability — pass^3 = 100% (deterministic grader, three consecutive runs)

## Recovery Controls

If the loop churns without progress:
- Freeze the loop; do not re-run the same failing root cause.
- Reduce scope to the single failing check ID (e.g. `HEUR-2:<skill>`).
- Replay with explicit acceptance criteria from the capability eval above.
- A grader false positive is a grader defect — fix the grader, not the asset.

## Run History

See `.claude/evals/qa-cluster.log`.
