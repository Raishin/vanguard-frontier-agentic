# NetSuite Build — Parallel Workflow Coordination

Persistent build state lives in this folder (`tmp/netsuite-build/`). Survives context
summarization. Full plan: `PLAN.md`.

## Phase A — Foundation (PARALLEL, read-only + web research) — IN PROGRESS
Three independent Sonnet agents, each writes ONE file. No repo mutation, no conflicts.

| Stream | Agent | Output file | Blocks |
|---|---|---|---|
| A1 Conventions | conventions-scout | `findings/conventions.md` | all content writing |
| A2 Evidence | evidence-researcher | `evidence/evidence-matrix.md` | all agent/skill prose |
| A3 Upstream reuse | upstream-mapper | `evidence/upstream-reuse-matrix.md` | skill design |

## Phase B — Template + vertical slice (SEQUENTIAL after A)
Build maestro + live-mutation-guard + one finance specialist + skills. Run `npm run validate`.
Lock the proven template into `findings/TEMPLATE.md`.

## Phase C — Mass production (PARALLEL by layer, after B validated)
Partition the 25 agents into non-overlapping batches; each agent-batch worker writes only
its own `agents/netsuite/<id>/` + `skills/netsuite/<id>/` dirs (disjoint paths → safe).
Re-run validate after each batch.

## Phase D — Regenerate + adversarial board + commit
manifest:write:all, asset-integrity --write, 20-scenario suite, score, verdict, push.

## Status log
- [in progress] Phase A launched (3 parallel Sonnet agents).
