# Normalized Platform Matrix

Last reviewed: 2026-04-28

This matrix separates three things that are easy to blur:

1. the **normalized labels** this repo uses for cross-harness assets,
2. the **vendor product names** users recognize,
3. the **actual file or adapter shapes** needed for each platform.

Do not pretend these platforms are interchangeable. Some are Markdown-family adapters, some are structured configs, and some are broader agent products that this repo does **not** target yet.

## Repo-supported normalized labels

These are the labels currently used by this repository in metadata, docs, and harness variants.

| Normalized label | User-facing product | Adapter family | Typical repo artifact(s) | Current repo support | Notes |
| --- | --- | --- | --- | --- | --- |
| `codex` | OpenAI Codex | Structured config | `harnesses/codex.toml` | Supported | Native TOML adapter, not Markdown body. |
| `copilot` | GitHub Copilot / VS Code custom agent | Markdown + YAML frontmatter | `harnesses/copilot.agent.md` | Supported | Also related to Copilot Agent Skills, but skills are a separate portability layer. |
| `claude-code` | Claude Code | Markdown + YAML frontmatter | `harnesses/claude-code.agent.md` | Supported | Uses subagent-style Markdown definitions. |
| `cursor` | Cursor | Markdown + YAML frontmatter | `harnesses/cursor.agent.md` | Supported | Keep Cursor-specific fields out unless verified. |
| `gemini` | Gemini CLI | Markdown + YAML frontmatter | `harnesses/gemini.agent.md` | Supported | Treat Gemini CLI as the concrete executable target, not “Gemini” in the abstract. |
| `kiro` | Kiro | Split target: Markdown + JSON | `harnesses/kiro-ide.agent.md`, `harnesses/kiro-cli.agent.json` | Supported | Kiro IDE and Kiro CLI are related but not interchangeable. |
| `other` | Unclassified / future | Varies | none by default | Placeholder only | Use only when a platform is real but not yet normalized in repo conventions. |

## Repo-level instruction entrypoints

If the goal is to let a platform **open this repository and immediately inherit project guidance**, the runtime entrypoint matters as much as the agent adapter format.

| Platform | Repo-level guidance file or path | Status in this repo | Notes |
| --- | --- | --- | --- |
| Codex | `AGENTS.md` | Present | Use `AGENTS.md` for repo guidance; custom subagents are separate TOML files under `.codex/agents/` when needed. |
| Cursor | `AGENTS.md` | Present | Cursor docs explicitly support root `AGENTS.md` for project instructions. |
| Kiro IDE | `AGENTS.md` or `.kiro/steering/` | `AGENTS.md` present | Kiro docs support `AGENTS.md`; steering files are an additional option. |
| Claude Code | `CLAUDE.md` | Present | Claude Code uses `CLAUDE.md` for project memory and `.claude/agents/` for custom subagents. |
| Gemini CLI | `GEMINI.md` | Present | Gemini CLI uses hierarchical `GEMINI.md` memory and `.gemini/agents/` for custom subagents. |
| GitHub Copilot | `.github/copilot-instructions.md` | Present | Repo-level instructions are separate from `.github/agents/` custom agent profiles. |
| GitHub Copilot custom agents | `.github/agents/` | Not populated by default | This repo ships harness adapters under `agents/**/harnesses/`; consumers can copy selected adapters into `.github/agents/` when they want active repo-level custom agents. |
| Kiro CLI custom agents | `.kiro/agents/*.json` | Not populated by default | This repo stores Kiro CLI adapters under `agents/**/harnesses/kiro-cli.agent.json`; copy selected ones into `.kiro/agents/` in a consuming repo. |
| Claude Code custom subagents | `.claude/agents/*.md` | Not populated by default | This repo stores Claude adapters under `agents/**/harnesses/claude-code.agent.md`; copy selected ones into `.claude/agents/` in a consuming repo. |
| Gemini CLI custom subagents | `.gemini/agents/*.md` | Not populated by default | This repo stores Gemini adapters under `agents/**/harnesses/gemini.agent.md`; copy selected ones into `.gemini/agents/` in a consuming repo. |

The important distinction:

- **repo-level guidance files** tell the main agent how to behave in this repository
- **custom agent/subagent files** add specialized delegates that can be installed separately

## Broader agentic platform candidates

These matter for market awareness, but they are **not** normalized repo labels yet.

| Candidate product | Suggested future normalized label | Why it matters | Current repo status | Why not normalized yet |
| --- | --- | --- | --- | --- |
| Windsurf | `windsurf` | Agentic IDE with MCP-aware workflows | Not supported | No harness convention or repo adapter contract yet. |
| Cline | `cline` | Strong customization model with rules, skills, workflows, hooks, CLI | Not supported | Different customization surface than the current harness set. |
| Continue | `continue` | Agent mode plus hosted/shared agents | Not supported | Needs a clear adapter contract before adding to repo taxonomy. |

## Product names that users may mention but should be normalized carefully

| User says | Normalize to | Why |
| --- | --- | --- |
| “Claude” | `claude-code` only when they mean Claude Code subagents | Claude the model family is not the same thing as Claude Code the agent surface. |
| “Gemini” | `gemini` only when they mean Gemini CLI adapter support | Gemini as a model/vendor is broader than the executable harness target. |
| “Kiro” | `kiro` plus explicit IDE vs CLI distinction when generating files | Kiro has at least two materially different adapter shapes in this repo. |
| “Copilot” | `copilot` | This was an easy miss when listing agentic platforms; it is already a first-class harness here. |

## Recommended naming rule for this repo

When updating catalog entries, agent metadata, or skills:

- use the **normalized label** in machine-readable metadata,
- use the **user-facing product name** in prose,
- and keep the **exact adapter filename** specific to the executable target.

Example:

- metadata harness label: `kiro`
- prose name: `Kiro`
- executable adapters:
  - `harnesses/kiro-ide.agent.md`
  - `harnesses/kiro-cli.agent.json`

## What was missing from the original short list

The strongest miss was:

- **GitHub Copilot**

Additional serious candidates for later normalization:

- **Windsurf**
- **Cline**
- **Continue**

## Source grounding

Official references reviewed for this normalization work:

- Anthropic Claude Code subagents: <https://docs.anthropic.com/en/docs/claude-code/sub-agents>
- GitHub Copilot custom agents: <https://code.visualstudio.com/docs/copilot/customization/custom-agents>
- GitHub Copilot agent skills: <https://code.visualstudio.com/docs/copilot/customization/agent-skills>
- OpenAI Codex docs: <https://platform.openai.com/docs/codex>
- Cursor docs: <https://docs.cursor.com/>
- Kiro steering and hooks docs: <https://kiro.dev/docs/steering/> and <https://kiro.dev/docs/hooks/types/>
- Gemini CLI repo/docs entrypoint: <https://github.com/google-gemini/gemini-cli>
- Windsurf docs: <https://docs.windsurf.com/>
- Cline docs: <https://docs.cline.bot/home>
- Continue docs: <https://docs.continue.dev/intro>
