# Cross-Harness Skill Adapter Design

Last reviewed: 2026-05-05

## Purpose

Today the marketplace exporter (`scripts/export-marketplace-agents.mjs`) only
bundles `SKILL.md` companion content for the `claude-code` platform via the
`SKILLS_PLATFORM_CONFIG` map. Other harnesses we already export agents for —
`cursor`, `codex`, `copilot`, `gemini`, `kiro` — are silently skipped with a
"not yet supported" notice.

This document records, with citations, what each harness actually offers as a
skill / reusable-instruction primitive, and proposes the smallest correct
adapter shape per harness. It is **design-only**. No bulk content rewrites
and no new dependencies are proposed here.

The conclusion up front:

- Three harnesses already accept the **same `SKILL.md` shape** we ship today
  (`name`, `description` frontmatter, Markdown body, optional
  `scripts/`/`references/`/`assets/` siblings): **Claude Code (current),
  Gemini CLI, GitHub Copilot, OpenAI Codex CLI**.
- Two harnesses do **not** have a `SKILL.md` primitive but do have a
  closely related instruction primitive: **Cursor** (`.cursor/rules/*.mdc`)
  and **Kiro** (`.kiro/steering/*.md`). For these, an export adapter is
  feasible but lossy.
- The recommended next harness to ship is **Gemini CLI**, because the file
  contract is byte-for-byte compatible with the existing Claude Code path —
  the diff is one entry in `SKILLS_PLATFORM_CONFIG`. Copilot is an equally
  small change and a valid second target.

## Per-harness assessment

### Claude Code — current baseline

- Skill primitive: yes. `SKILL.md` with `name` + `description` YAML
  frontmatter, optional `scripts/` / `references/` / `assets/`.
- Path: `.claude/skills/<skill-id>/SKILL.md`.
- Status: shipped (`SKILLS_PLATFORM_CONFIG["claude-code"] = ".claude/skills"`
  in `scripts/export-marketplace-agents.mjs`).
- Source: Anthropic Claude Code subagents and skills docs
  <https://docs.anthropic.com/en/docs/claude-code/sub-agents>.

### Gemini CLI — ready to ship

- Skill primitive: **yes, native, identical contract**.
- File: `SKILL.md` with YAML frontmatter `name`, `description`. Optional
  `scripts/`, `references/`, `assets/`.
- Workspace path: `.gemini/skills/<skill-id>/SKILL.md`.
  Alias also accepted: `.agents/skills/<skill-id>/SKILL.md`.
- User path: `~/.gemini/skills/` or `~/.agents/skills/`.
- Discovery precedence (lowest to highest): built-in → extension →
  user → workspace.
- Activation requires per-session permission.
- Adapter approach: **byte-identical copy** of `skills/<provider>/<id>/`
  trees from this repo into `.gemini/skills/<id>/`. No frontmatter
  transformation needed.
- Source: Gemini CLI docs `creating-skills.md`, `using-agent-skills.md`
  in <https://github.com/google-gemini/gemini-cli/tree/main/docs/cli>.

### GitHub Copilot (VS Code) — ready to ship

- Skill primitive: **yes, native, identical contract**.
- File: `SKILL.md` with YAML frontmatter `name` (must match the parent
  directory name) + `description`.
- Workspace paths accepted by Copilot: `.github/skills/`, `.claude/skills/`,
  `.agents/skills/`. Personal: `~/.copilot/skills/`, `~/.claude/skills/`,
  `~/.agents/skills/`. Additional locations configurable via
  `chat.agentSkillsLocations`.
- Adapter approach: **byte-identical copy** to `.github/skills/<id>/`
  (recommended canonical Copilot project location) **or** reuse the
  `.claude/skills/` path that Copilot also reads (which would let one
  exported tree serve both Claude Code and Copilot).
- Note: Copilot Agent Skills are distinct from
  `.github/copilot-instructions.md` (project-wide always-on instructions)
  and from `.github/prompts/*.prompt.md` (slash-invokable prompts) and
  from custom chat modes. Skills are the right primitive for our content.
- Source: VS Code Copilot customization docs, "Use Agent Skills in VS Code"
  page; project skills directory list cited from Context7-mirrored
  <https://code.visualstudio.com/docs/copilot/customization/agent-skills>.

### OpenAI Codex CLI — ready to ship (with one caveat)

- Skill primitive: **yes, native, contract is a strict superset of ours**.
- File: `SKILL.md` with YAML frontmatter. Required: `name` (lowercase,
  digits, hyphens, ≤ 64 chars), `description`. Optional Codex-specific
  fields: `argument-hint`, `disable-model-invocation`, `user-invocable`,
  `allowed-tools`, `context`, `agent`, `model`. The Codex docs explicitly
  forbid additional unknown fields.
- User path documented: `${CODEX_HOME:-$HOME/.codex}/skills/<id>/SKILL.md`.
- Workspace/project path: **not clearly documented in the public docs**
  reachable from <https://github.com/openai/codex/blob/main/docs/skills.md>
  (the page redirects to <https://developers.openai.com/codex/skills>,
  which is gated). The `skills/list` RPC accepts a `cwd` argument and
  `perCwdExtraUserRoots`, implying per-cwd discovery exists, but a
  documented project-level directory name has not been confirmed in this
  research pass.
- **Caveat for our existing SKILL.md content**: many of our skills carry
  rich frontmatter (e.g. `metadata`, `version`, `tags`, `companion_skills`).
  Codex frontmatter is closed-set; unknown keys may be rejected. An adapter
  must either (a) strip our extension fields when writing to the Codex
  destination, or (b) confirm Codex tolerates unknown YAML keys. This
  needs an empirical pass (`codex skills list` against an exported tree)
  before flipping the switch.
- Adapter approach: copy the directory tree, then run a frontmatter
  filter that retains only Codex-known keys. Body content is unchanged.
- Source: <https://github.com/openai/codex/blob/main/docs/skills.md>
  (top-level reference) and the in-repo
  `codex-rs/skills/src/assets/samples/skill-creator/SKILL.md` template.

### Cursor — design only, not viable as a 1:1 skill export

- Skill primitive: **no**. Cursor has **Project Rules**, not skills.
- File: `.cursor/rules/<name>.mdc`. Frontmatter fields: `description`,
  `globs`, `alwaysApply`. No `name` field. Rule types are derived from
  the combination of those fields:
  - `Always` (`alwaysApply: true`) — always loaded
  - `Auto Attached` (`globs: <pattern>`) — loaded when matching files
    are in context
  - `Agent Requested` (`description` set, `alwaysApply: false`) —
    Agent picks based on description
  - `Manual` — user invokes via `@rule`
- Mismatch with our `SKILL.md`:
  - Our skills are large, multi-section operating playbooks. Cursor rules
    are intended to be small, focused style/architecture guides.
  - We rely on `scripts/`, `references/`, and `assets/` siblings. Cursor
    rules support an `@file` reference syntax in the body but do not bundle
    sibling resource directories the way SKILL packages do.
  - Our content is platform-aware (cloud, security, compliance). Cursor
    rules tend to be language- and codebase-aware. The "when to use"
    semantics overlap but do not match.
- Recommendation: **do not export skills to Cursor**. Print an explicit
  notice (not a silent skip) so consumers know the content was intentionally
  withheld. If Cursor users want our content they should rely on agents
  (already exported via `harnesses/cursor.agent.md`) and on the project's
  `AGENTS.md` which Cursor honors.
- Source: Cursor docs, "Rules" page <https://cursor.com/docs/rules>.

### Kiro — design only, not viable as a 1:1 skill export

- Skill primitive: **no, not under that name**. Kiro has **Steering files**
  and **Hooks**. Steering is the closest analog to skills.
- File: `.kiro/steering/<name>.md`. Frontmatter declares an inclusion mode
  via YAML — modes are `always` (default), `fileMatch` (with a pattern,
  conditional on edited file), `manual` (registered as a slash command),
  and an auto-mode driven by `description`.
- Mismatch with `SKILL.md`:
  - No bundled-resources convention (no first-class `scripts/` or
    `references/` siblings; steering is single-file guidance).
  - Inclusion semantics differ from our description-as-trigger contract.
  - Steering is plural-by-default ("loaded into every interaction" or by
    file pattern), not one-skill-per-task.
- Hooks are not a skill substitute — hooks are deterministic event
  handlers, not model-invoked workflows.
- Recommendation: **do not export skills to Kiro as steering**. The
  semantic mismatch is large enough that doing so would lower Kiro UX,
  not raise it. Continue exporting Kiro IDE / Kiro CLI agents only.
  Print a notice when `--platform kiro` is used with skills enabled.
- Source: Kiro docs, "Steering" <https://kiro.dev/docs/steering> and
  `multi-root-workspaces` page.

## Normalization table

This is the canonical mapping from our `SKILL.md` frontmatter to each
harness's expected file shape.

| Our field                       | Claude Code         | Gemini CLI        | Copilot (VS Code)   | Codex CLI                 | Cursor (rules)         | Kiro (steering)         |
| ------------------------------- | ------------------- | ----------------- | ------------------- | ------------------------- | ---------------------- | ----------------------- |
| `name`                          | `name`              | `name`            | `name` (must match dir) | `name`                | (drop; filename used)  | (drop; filename used)   |
| `description`                   | `description`       | `description`     | `description`       | `description`             | `description`          | `description` (auto)    |
| Markdown body                   | body                | body              | body                | body                      | body                   | body                    |
| `scripts/` siblings             | copied              | copied            | copied              | copied                    | not supported          | not supported           |
| `references/` siblings          | copied              | copied            | copied              | copied                    | not supported          | not supported           |
| `assets/` siblings              | copied              | copied            | copied              | copied                    | not supported          | not supported           |
| `allowed-tools`                 | passthrough         | passthrough       | passthrough         | passthrough (Codex-known) | drop                   | drop                    |
| `metadata.*` (our extension)    | passthrough         | passthrough       | passthrough         | **strip** (closed schema) | drop                   | drop                    |
| `companion_skills`, `tags`, etc | passthrough         | passthrough       | passthrough         | **strip**                 | drop                   | drop                    |
| Trigger model                   | description-based   | description-based | description-based   | description-based         | desc + globs/always    | inclusion mode          |
| Destination root (workspace)    | `.claude/skills/`   | `.gemini/skills/` | `.github/skills/`   | `.codex/skills/` (assumed; needs verify) | `.cursor/rules/`       | `.kiro/steering/`       |

Empty cell == not applicable. "passthrough" == leave the YAML key in
place, body unchanged. "drop" == remove for that target. "strip" ==
remove unknown keys to satisfy a closed-set schema.

## Staged rollout proposal

Order of preference for enabling new harnesses, from lowest to highest
risk:

1. **Gemini CLI** (smallest delta).
   - Adapter is a path addition: `SKILLS_PLATFORM_CONFIG.gemini = ".gemini/skills"`.
   - Zero content transformation. Same `SKILL.md` shape.
   - Validation: install path also accepts `.agents/skills/` if we want a
     single export to serve other harnesses too — defer that until we have
     a reason.

2. **GitHub Copilot**.
   - Adapter is also a path addition:
     `SKILLS_PLATFORM_CONFIG.copilot = ".github/skills"`.
   - Zero content transformation.
   - Independent test: each `SKILL.md` `name` field must match its parent
     directory name. Our existing skills satisfy this by repository
     convention; add a one-line check in `validate:skill-schema`
     downstream if we ship this.

3. **OpenAI Codex CLI**.
   - Requires a frontmatter filter step: keep only the Codex-known key
     set. We must verify the project-level destination path against
     official docs before shipping (the in-repo doc redirects to a gated
     OpenAI portal that this research pass could not reach).
   - Recommend gating this behind a flag (`--codex-skills-experimental`)
     until verified end-to-end against `codex skills list`.

4. **Cursor / Kiro** — do not enable. Document the decision and make the
   exporter print an explicit notice when a user requests skills for
   those targets. This is more useful than a silent skip.

## What changes in this PR

This PR is **design-only**. The following are intentionally **not**
modified:

- `scripts/export-marketplace-agents.mjs` — `SKILLS_PLATFORM_CONFIG` is
  unchanged. Even though Gemini and Copilot are byte-compatible and could
  be enabled today by a one-line change each, gating that on this design
  review keeps the PR scoped. The follow-up enablement PR per harness is
  ~5 lines of code plus tests.
- `skills/**` content — not touched.
- `schemas/skill.frontmatter.schema.json` — not touched. Codex's
  closed-set frontmatter rule means we may want a "codex export filter"
  list, but that is a follow-up implementation concern.
- `catalog/skill-manifest.json` — not touched.

The follow-up adapter PR for Gemini CLI is approximately:

```js
const SKILLS_PLATFORM_CONFIG = {
  "claude-code": ".claude/skills",
  gemini: ".gemini/skills",
};
```

plus a corresponding update to the `--help` output and to
`docs/integrations/skills-cli.md` Path 2.

## Honest assessment

We are **ready** to enable Gemini CLI and GitHub Copilot today on the
basis of file-format compatibility documented in the official Gemini CLI
and VS Code Copilot docs. The reason this design recommends not flipping
the switch in the same PR is purely process: the exporter has not been
exercised against either harness end-to-end inside this repo's CI, and
shipping a skills surface to a new harness is a support contract, not a
file copy.

We are **not ready** to enable Codex CLI without a verification pass that
confirms (a) the project-level path Codex uses for workspace skills, and
(b) whether Codex's YAML loader rejects or silently ignores unknown keys
present in our existing `SKILL.md` files.

We should **not pretend Cursor and Kiro have a skill primitive**. They do
not. Forcing our content into `.cursor/rules/*.mdc` or `.kiro/steering/*.md`
would degrade the consumer experience on those harnesses and create the
appearance of feature parity that does not exist. Honest opt-out, with an
explicit notice, is the right behavior.

## References

- Anthropic Claude Code subagents:
  <https://docs.anthropic.com/en/docs/claude-code/sub-agents>
- Gemini CLI creating skills:
  <https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/creating-skills.md>
- Gemini CLI using agent skills:
  <https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/using-agent-skills.md>
- VS Code Copilot Agent Skills:
  <https://code.visualstudio.com/docs/copilot/customization/agent-skills>
- VS Code Copilot custom instructions:
  <https://code.visualstudio.com/docs/copilot/customization/custom-instructions>
- OpenAI Codex skills (top-level redirect page):
  <https://github.com/openai/codex/blob/main/docs/skills.md>
- OpenAI Codex skill-creator template (in-repo source of truth for
  frontmatter contract):
  <https://github.com/openai/codex/blob/main/codex-rs/skills/src/assets/samples/skill-creator/SKILL.md>
- Cursor Rules:
  <https://cursor.com/docs/rules>
- Kiro Steering:
  <https://kiro.dev/docs/steering>
- Kiro multi-root workspaces (steering scope):
  <https://kiro.dev/docs/editor/multi-root-workspaces>
