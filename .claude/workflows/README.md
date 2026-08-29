# Executable workflows

Scripts here are run by the `Workflow` tool (multi-agent orchestration), not read
by a human. They are the executable counterpart to `.claude/skills/` — a skill
tells one agent how to behave, a workflow deterministically orchestrates many.

Do not confuse this directory with `.claude/workflow/` (singular), which holds
per-board **planning documents** (`m365-d365/`, `typescript-board/`). Those are
prose plans for humans; these are JavaScript pipelines for agents.

## `agentic-delegation.js`

Runs `.claude/skills/agentic-delegation/SKILL.md` as a pipeline, with Context7
MCP verification wired in as a first-class phase.

```text
Recon      Haiku Explore agents, one narrow question each, citations required
Context7   resolve + query per library surface, adjudicated CONFIRMED /
           CONTRADICTED / NOT_COVERED / NOT_AVAILABLE
Author     Sonnet writers against exact file-scoped specs, no commits
Verify     an independent refuter reads each authored change
Gates      the repo gate suite, raw failure output, asset-integrity last
Synthesis  an orchestrator-facing report
```

Recon and Context7 run as independent branches. Author and Verify run as a
`pipeline()`, so a spec's refuter starts as soon as that spec is written rather
than waiting for every other spec.

### Invoking it

```text
Workflow({ name: "agentic-delegation", args: { ... } })
```

With no `args` it runs a recon plus gate health check. Every field is optional:

| Field | Shape | Meaning |
|---|---|---|
| `task` | string | One line of context passed to every writer. |
| `questions` | string[] | Recon questions, one agent each. Max 5. |
| `libraries` | object[] | Context7 surfaces: `{name, query, claims[]}`. Max 5. |
| `specs` | object[] | Writing tasks: `{paths[], instruction, conventions, acceptance}`. Max 6. |
| `gates` | boolean | Set `false` to skip the gate phase. Defaults on. |

Anything beyond a cap is dropped **loudly** via `log()` — a truncated run must
never read as full coverage.

### Why Context7 is a phase and not a footnote

Service documentation describes features; library documentation pins call
signatures. This repo has already shipped two wrong MLflow API claims that
Databricks' own documentation pages did not catch — a non-existent
`evaluate(eval_data, ..., prediction_fn)` keyword pair, and built-in judges
imported from `mlflow.genai.judges` rather than `mlflow.genai.scorers`. Both
would have failed at runtime for anyone following the agent. Context7 caught
both.

The adjudication rule the phase enforces matters as much as the lookup:

- **`contradicted`** means Context7 positively shows something different. That is
  a defect, and the `actual` value is recorded.
- **`notCovered`** means Context7 returned no evidence. Context7 serves retrieved
  snippets, not a complete API inventory, so absence is **uncorroborated, never
  disproven**.

Conflating the two is not a harmless imprecision: it causes correct, documented
content to be "corrected" into incorrectness. A delegate that reports absence as
contradiction is wrong, and the schema is shaped to make that mistake hard.

If the Context7 tools cannot be reached, an agent returns `NOT_AVAILABLE` and
the affected claims stay labelled unknown. It never substitutes memory or a web
fetch for the MCP in that phase, and it never fabricates MCP evidence.

### What the workflow will not do

It never commits, never pushes, and never treats a green gate as sufficient. The
final report ends with an `orchestratorMustDo` list, and reading the actual diff
is always on it — a delegate's self-report is not verification.
