# `.github/plugin/` — GitHub Copilot CLI marketplace

This directory holds the **GitHub Copilot CLI** plugin marketplace registry
for `vanguard-frontier-agentic`. Copilot CLI looks for marketplace JSON at
this path when you run `copilot plugin marketplace add owner/repo`.

## What's in here

| File | Purpose |
|------|---------|
| [`marketplace.json`](marketplace.json) | Declares this repo as a Copilot CLI single-plugin marketplace. The lone plugin `vanguard-frontier-agentic` uses `"source": "./"` so the repo root is the plugin root. Hand-maintained; values rarely change. |

## How users install

```bash
# Add the marketplace, then install the bundled plugin
copilot plugin marketplace add Raishin/vanguard-frontier-agentic
/plugin install vanguard-frontier-agentic
```

Or wire it into `.github/copilot/settings.json` for repo-wide trust:

```json
{
  "extraKnownMarketplaces": [
    "https://raw.githubusercontent.com/Raishin/vanguard-frontier-agentic/master/.github/plugin/marketplace.json"
  ]
}
```

Note that for Copilot CLI, `extraKnownMarketplaces` is an **array of URLs**
(not an object keyed by name as in Claude Code). This is a Copilot-specific
quirk; the validator enforces this shape.

## How to update

This file is hand-maintained because it's small and rarely changes. Bump
`version` when `package.json` `version` bumps. The validator
(`validate:multi-harness-marketplace`) enforces version parity.

```bash
npm run validate:multi-harness-marketplace
```

## Schema references (official GitHub docs)

- **GitHub Copilot CLI repo:** <https://github.com/github/copilot-cli>
- **Plugin marketplace command (`/plugin marketplace add`):** see the CLI's
  built-in help: `copilot plugin marketplace --help`
- **Settings (`extraKnownMarketplaces`, `enabledSkills`):** documented in the
  CLI's `llms.txt` at <https://context7.com/github/copilot-cli/llms.txt>
- **GitHub Docs entry "Creating a plugin marketplace for GitHub Copilot CLI":**
  <https://docs.github.com/en/copilot/how-tos/use-copilot-features/use-copilot-cli>

## Design notes

- **Source is `"./"`** — the repo root is the plugin root, matching the
  Claude Code pattern. Copilot CLI's marketplace.json plugins[] supports
  relative source paths the same way Claude Code's marketplace.json does.
- **Single-plugin marketplace** — we ship one plugin (`vanguard-frontier-agentic`)
  rather than per-provider plugins because Copilot CLI's marketplace.json
  schema (per the screenshot in GitHub Docs: `id`, `source`, `description`)
  is lightweight; per-provider scoping is better done via Copilot's
  `disabledSkills` settings on the user side.
