# `.claude-plugin/` — Claude Code plugin manifests

This directory holds the **Claude Code** plugin marketplace and plugin manifest
for `vanguard-frontier-agentic`. Do not move these files — Claude Code looks
for them at exactly these paths.

## What's in here

| File | Purpose |
|------|---------|
| [`marketplace.json`](marketplace.json) | Declares this repo as a Claude Code plugin marketplace. Lists one plugin (`vanguard-frontier-agentic`) sourced at `./` so the repo root is the plugin root. |
| [`plugin.json`](plugin.json) | Plugin manifest. Enumerates all 331 Claude Code agent adapter paths under `agents/<provider>/<agent>/harnesses/claude-code.agent.md`. **Generated** by `scripts/generate-plugin-manifest.mjs` — do not hand-edit. |

## How users install

```bash
/plugin marketplace add Raishin/vanguard-frontier-agentic
/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic
```

Or via `~/.claude/settings.json`:

```json
{
  "extraKnownMarketplaces": {
    "vanguard-frontier-agentic": {
      "source": { "source": "github", "repo": "Raishin/vanguard-frontier-agentic" }
    }
  },
  "enabledPlugins": {
    "vanguard-frontier-agentic@vanguard-frontier-agentic": true
  }
}
```

## How to update

```bash
# After adding/removing agents, regenerate the plugin manifest:
npm run plugin-manifest:write

# Then verify everything is in sync:
npm run validate:plugin-manifest
```

The `validate` chain runs `validate:plugin-manifest` automatically.

## Schema references (official Anthropic docs)

- **Plugin marketplaces overview:** <https://code.claude.com/docs/en/plugin-marketplaces>
- **Plugins reference (manifest schema):** <https://code.claude.com/docs/en/plugins-reference>
- **Discover & install plugins:** <https://code.claude.com/docs/en/discover-plugins>
- **`extraKnownMarketplaces` / `enabledPlugins` settings:** <https://code.claude.com/docs/en/settings>
- **Agent SDK plugin integration:** <https://code.claude.com/docs/en/agent-sdk/plugins>

## Design notes

- **Plugin source is `"./"`** so the repo root is the plugin root. This lets us keep agents at their existing nested paths (`agents/<provider>/<agent>/...`) instead of restructuring to the conventional flat `agents/<name>.md` layout.
- **Custom paths via `agents[]` array** — Claude Code's plugin spec explicitly supports an array of file paths for the `agents` field, which we use to enumerate every claude-code adapter file. This avoids forcing a flatten of the multi-harness directory structure.
- **Skills are omitted** from this manifest because the repo nests skills as `skills/<provider>/<skill>/SKILL.md`, one level deeper than Claude Code's flat `skills/<skill>/SKILL.md` convention. Skills remain available via `npm install @raishin/vanguard-frontier-agentic` + the `vfa-export-agents` CLI.
