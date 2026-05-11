# Golden fixtures — `nvidia-maestro` routing

EDD harness for the NVIDIA Maestro routing skill. Each fixture is a pair:

- `inputs/NN-name.json` — `{ "name", "task" }` (operator-style natural-language task)
- `expected/NN-name.json` — `{ "route": [agent_id, ...], "mode": "single" | "parallel (N)" | "runtime-evidence-gate" }`

`tests/validate-nvidia-maestro-routing.py` is the deterministic grader: it scores every domain in the taxonomy (which mirrors `skills/nvidia/nvidia-maestro/references/workflow-and-output.md`) by keyword overlap with the task, resolves to one or more agents, and diffs against `expected/`.

## Why deterministic, not LLM-as-judge

A keyword-taxonomy grader is fast, free, reproducible, and forces the routing table to be the executable spec. If a future task pattern needs a new domain, the taxonomy in `validate-nvidia-maestro-routing.py` and `workflow-and-output.md` must be updated together — that coupling is the point.

## Regression guards

- Every agent referenced by the routing table must exist in `catalog/agents.json`.
- `nvidia-model-promotion-gatekeeper-agent` is **never** routed in `single` or `parallel` modes — only `runtime-evidence-gate`. Any other path trips the live-agent auto-dispatch guard.
- Every domain has at least one mapped agent.

## Adding a fixture

1. Drop a new `inputs/NN-shortname.json` with `{name, task}`.
2. Add `expected/NN-shortname.json` with the expected `route` (sorted) and `mode`.
3. Run `python3 tests/validate-nvidia-maestro-routing.py`.
