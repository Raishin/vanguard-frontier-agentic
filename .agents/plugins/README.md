# `.agents/plugins/` — OpenAI Codex CLI marketplace registry

This directory holds the **OpenAI Codex CLI** plugin marketplace registry
for `vanguard-frontier-agentic`. Codex looks for marketplace JSON at this
canonical path when you run `codex plugin marketplace add owner/repo`.

## What's in here

| File | Purpose |
|------|---------|
| [`marketplace.json`](marketplace.json) | Declares this repo as a Codex marketplace. Lists two plugins: `vanguard-frontier-agentic` (main) and `cross-platform-agent-template` (scaffold). Hand-maintained. |

## How users install

```bash
codex plugin marketplace add Raishin/vanguard-frontier-agentic
/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic
```

After `codex plugin marketplace add`, Codex writes the following into
`~/.codex/config.toml` (this is the local user config — not in this repo):

```toml
[marketplaces.vanguard-frontier-agentic]
last_updated = "2026-05-11T06:46:00Z"
last_revision = "<sha>"
source_type = "git"
source = "https://github.com/Raishin/vanguard-frontier-agentic.git"

[plugins."vanguard-frontier-agentic@vanguard-frontier-agentic"]
enabled = true
```

Other commands:

```bash
codex plugin marketplace upgrade vanguard-frontier-agentic
codex plugin marketplace remove vanguard-frontier-agentic
```

## How to update

This file is hand-maintained because Codex marketplaces are small. Required
fields per Codex spec on every plugin entry: `policy.installation`,
`policy.authentication`, `category`. The validator
(`validate:codex-marketplace`) enforces:

- `name` is `vanguard-frontier-agentic`
- every plugin has kebab-case `name`
- plugin folder name **equals** plugin name (Codex strict rule)
- `source.source` is `"local"` and `source.path` resolves to a real directory
- every referenced plugin has `.codex-plugin/plugin.json`
- `policy.{installation, authentication}` and `category` are present
- main plugin's `version` matches `package.json` `version`

```bash
npm run validate:codex-marketplace
```

## Schema references (official OpenAI Codex docs)

- **OpenAI Codex CLI repo:** <https://github.com/openai/codex>
- **Plugin marketplace command (`codex plugin marketplace`):**
  <https://github.com/openai/codex/blob/main/codex-rs/app-server/README.md>
- **Plugin manifest spec (`.codex-plugin/plugin.json`):**
  <https://github.com/openai/codex/blob/main/codex-rs/skills/src/assets/samples/plugin-creator/references/plugin-json-spec.md>
- **Plugin creator skill (best practices, naming rules):**
  <https://github.com/openai/codex/blob/main/codex-rs/skills/src/assets/samples/plugin-creator/SKILL.md>
- **Context7 mirror (llms.txt):** <https://context7.com/openai/codex/llms.txt>

## Design notes

- **Canonical path** — Codex docs are explicit: repo plugin marketplaces live
  at `<repo-root>/.agents/plugins/marketplace.json`. Earlier in this PR's
  history I mistakenly deleted this file thinking it was a precursor to the
  Claude Code marketplace; the official docs corrected that and the file is
  now back at its canonical location.
- **Two plugins, not one** — `vanguard-frontier-agentic` is the main entry
  point for cloud/security/compliance work. `cross-platform-agent-template`
  is a scaffold for authors who want to build new cross-platform agents
  using this repo's adapter pattern.
- **Codex plugins bundle skills/hooks/MCP servers, not agents** — Codex
  agents are loaded from `.codex/agents/*.toml` at the workspace or user
  level, not from plugin manifests. The plugin's role is to bootstrap the
  marketplace presence; agent adapter files are still written via
  `npx vfa-export-agents --platform codex --all --repo .`. This two-step
  model is documented in the README's Codex install dropdown.
