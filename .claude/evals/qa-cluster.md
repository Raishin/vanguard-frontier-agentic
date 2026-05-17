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

## Run History

See `.claude/evals/qa-cluster.log`.
