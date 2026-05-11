# Multi-Harness Adapter Pattern

**Audience:** contributors adding new agents or providers, and maintainers understanding how the cross-harness system works.

---

## The problem this solves

Different AI coding harnesses (Claude Code, Cursor, Copilot CLI, Kiro, Gemini, Codex) each define their own agent/skill file formats. A naïve approach would either:

- Maintain N separate copies of each agent (drift-prone), or
- Pick one format and exclude everyone else.

This repo uses a **canonical-spec + per-harness adapter** pattern. The canonical spec (`AGENT.md` at the agent root) defines the harness-neutral contract. A `harnesses/` directory alongside it holds thin adapter files that translate the canonical spec into each harness's native format.

---

## Directory layout for a single agent

```
agents/<provider>/<agent-id>/
├── AGENT.md                          # canonical harness-neutral spec
├── metadata.json                     # catalog metadata (schema: schemas/agent.schema.json)
└── harnesses/
    ├── claude-code.agent.md          # Claude Code subagent format
    ├── codex.toml                    # OpenAI Codex TOML format
    ├── copilot.agent.md              # GitHub Copilot agent format
    ├── cursor.agent.md               # Cursor agent format
    ├── gemini.agent.md               # Gemini CLI / Antigravity format
    ├── kiro-ide.agent.md             # Kiro IDE Markdown format
    └── kiro-cli.agent.json           # Kiro CLI JSON format
```

Every agent in the catalog ships all seven adapter files. The CI validator
(`validate:agent-schema`) enforces this.

---

## The canonical spec — AGENT.md

`AGENT.md` is the single source of truth for an agent's intent, behavior,
guarded response shape, and permission model. It is **not executed directly**
by any harness — it is the reference document that adapter files are derived
from.

Typical sections:

```markdown
---
name: aws-cost-optimization-agent
description: Identifies idle and undersized AWS resources and recommends cost reduction actions.
harnesses: [claude-code, codex, copilot, cursor, gemini, kiro]
provider: aws
domain: finops
---

# AWS Cost Optimization Agent

## Capabilities
...

## Guarded response shape
...

## Least-privilege permission model
...
```

---

## Adapter formats by harness

### Claude Code — `claude-code.agent.md`

YAML frontmatter + Markdown body. The frontmatter keys Claude Code reads are
`name`, `description`, `allowed-tools`, and optionally `model`.

```markdown
---
name: aws-cost-optimization-agent
description: Identifies idle and undersized AWS resources and recommends cost reduction actions.
allowed-tools: Read, Bash, Grep
---

You are the AWS Cost Optimization agent...
```

**Plugin manifest:** `.claude-plugin/plugin.json` declares each adapter's path
in the `agents[]` array. Regenerate with `npm run plugin-manifest:write`.

---

### OpenAI Codex — `codex.toml`

Structured TOML. Codex does not use Markdown-body agents.

```toml
[agent]
name = "aws-cost-optimization-agent"
description = "Identifies idle and undersized AWS resources."
version = "1.0.0"

[agent.permissions]
allowed_tools = ["read_file", "run_bash", "search_files"]

[agent.behavior]
system_prompt = """
You are the AWS Cost Optimization agent...
"""
```

---

### GitHub Copilot — `copilot.agent.md`

YAML frontmatter + Markdown body. Copilot uses `name` and `description`.

```markdown
---
name: aws-cost-optimization-agent
description: Identifies idle and undersized AWS resources.
---

You are the AWS Cost Optimization agent...
```

**Plugin manifest:** `.github/plugin/marketplace.json` declares the plugin.
Copilot's `extraKnownMarketplaces` is an array of URLs (not keyed object).

---

### Cursor — `cursor.agent.md`

Same shape as Claude Code adapters: YAML frontmatter + Markdown body.

```markdown
---
name: aws-cost-optimization-agent
description: Identifies idle and undersized AWS resources.
---

You are the AWS Cost Optimization agent...
```

**Plugin manifest:** `.cursor-plugin/plugin.json` enumerates all adapter paths
in the `agents[]` array. Regenerate with `npm run cursor-plugin:write`.

---

### Gemini CLI / Antigravity — `gemini.agent.md`

YAML frontmatter + Markdown body. Gemini reads `name` and `description`.

```markdown
---
name: aws-cost-optimization-agent
description: Identifies idle and undersized AWS resources.
---

You are the AWS Cost Optimization agent...
```

Adapters export to `.agent/skills/<name>/SKILL.md` (workspace) or
`~/.gemini/antigravity/skills/<name>/` (global) via `vfa-export-agents
--platform gemini`.

---

### Kiro IDE — `kiro-ide.agent.md`

Markdown with minimal frontmatter. Kiro IDE reads `name` and `description`.

```markdown
---
name: aws-cost-optimization-agent
description: Identifies idle and undersized AWS resources.
---

You are the AWS Cost Optimization agent...
```

---

### Kiro CLI — `kiro-cli.agent.json`

Structured JSON (not Markdown). Kiro CLI uses a JSON contract.

```json
{
  "name": "aws-cost-optimization-agent",
  "description": "Identifies idle and undersized AWS resources.",
  "version": "1.0.0",
  "permissions": {
    "allowed_tools": ["read_file", "run_bash"]
  },
  "behavior": {
    "system_prompt": "You are the AWS Cost Optimization agent..."
  }
}
```

---

## metadata.json — the catalog contract

Every agent must have a `metadata.json` that conforms to
`schemas/agent.schema.json`. Key fields:

```json
{
  "id": "aws-cost-optimization-agent",
  "name": "AWS Cost Optimization Agent",
  "provider": "aws",
  "domain": "finops",
  "summary": "Identifies idle and undersized AWS resources and recommends cost reduction actions.",
  "harnesses": ["claude-code", "codex", "copilot", "cursor", "gemini", "kiro"],
  "harness_variants": {
    "claude-code": "harnesses/claude-code.agent.md",
    "codex": "harnesses/codex.toml",
    "copilot": "harnesses/copilot.agent.md",
    "cursor": "harnesses/cursor.agent.md",
    "gemini": "harnesses/gemini.agent.md",
    "kiro": "harnesses/kiro-ide.agent.md"
  },
  "companion_skills": ["aws-cost-optimization"]
}
```

**`harnesses`** is the authoritative list. Downstream tooling (catalog
generators, plugin manifest generators, validators) reads this field to know
which adapters exist. Do not omit harnesses that have adapter files on disk.

---

## Generated artifacts that depend on metadata.json

When you add or update an agent's `metadata.json`, the following generated
files must be refreshed:

| Generated file | Regenerate command |
|---------------|-------------------|
| `catalog/agents.json` | `npm run manifest:write` (re-indexes all agents) |
| `.claude-plugin/plugin.json` | `npm run plugin-manifest:write` |
| `.cursor-plugin/plugin.json` | `npm run cursor-plugin:write` |
| `powers/vanguard-<provider>/POWER.md` | `npm run kiro-powers:write` |

Run `npm run validate` to confirm all generated files are in sync with disk.

---

## Adding a new provider

1. **Create the provider directory:**
   ```
   agents/<new-provider>/
   ```

2. **Add at least one agent** with all seven harness adapters:
   ```
   agents/<new-provider>/<agent-id>/
   ├── AGENT.md
   ├── metadata.json
   └── harnesses/
       ├── claude-code.agent.md
       ├── codex.toml
       ├── copilot.agent.md
       ├── cursor.agent.md
       ├── gemini.agent.md
       ├── kiro-ide.agent.md
       └── kiro-cli.agent.json
   ```

3. **Declare all harnesses** in `metadata.json`'s `harnesses[]` array.
   Missing harnesses cause CI to fail the `validate:agent-schema` gate.

4. **Add a Kiro Power** (if the provider warrants one):
   ```
   powers/vanguard-<new-provider>/POWER.md
   ```
   Strict-5 frontmatter only: `name`, `displayName`, `description`, `keywords`,
   `author`. Maximum three sentences in `description`. Run
   `npm run validate:kiro-powers` to verify.

5. **Add a provider README:**
   ```
   agents/<new-provider>/README.md
   ```
   List all agents in an advisory table (see `agents/aws/README.md` for the
   canonical format).

6. **Regenerate and validate:**
   ```bash
   npm run manifest:write
   npm run plugin-manifest:write
   npm run cursor-plugin:write
   npm run kiro-powers:write
   npm run validate
   ```

---

## Validator gates that enforce the adapter pattern

| Gate | What it checks |
|------|---------------|
| `validate:agent-schema` | Every agent in `catalog/agents.json` has required fields, valid harnesses, and `harness_variants` paths that exist on disk |
| `validate:plugin-manifest` | `.claude-plugin/plugin.json` lists exactly the right number of Claude Code adapters (currently 331) |
| `validate:multi-harness-marketplace` | `.cursor-plugin/plugin.json` + `.github/plugin/marketplace.json` are in sync with catalog |
| `validate:codex-marketplace` | `.agents/plugins/marketplace.json` + `plugins/*/\.codex-plugin/plugin.json` are valid |
| `validate:kiro-powers` | All 14 `powers/*/POWER.md` files have strict-5 frontmatter with ≤3 sentences |
| `validate:catalog` | `catalog/agents.json` + `catalog/skills.json` are structurally valid |
| `validate:allowed-tools` | All adapter files declare only permitted tools |

Run the full suite with `npm run validate`.

---

## See also

- [`docs/integrations/installation-guide.md`](./installation-guide.md) — end-user install steps for each harness
- [`docs/compatibility.md`](../compatibility.md) — which harnesses are supported and what each ships
- [`docs/normalized-platform-matrix.md`](../normalized-platform-matrix.md) — repo-level instruction files per harness
- [`CONTRIBUTING.md`](../../CONTRIBUTING.md) — full contributor onboarding guide
