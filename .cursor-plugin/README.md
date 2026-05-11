# `.cursor-plugin/` — Cursor plugin manifest

This directory holds the **Cursor** plugin manifest for `vanguard-frontier-agentic`.
Cursor loads any directory with `.cursor-plugin/plugin.json` as a plugin.

## What's in here

| File | Purpose |
|------|---------|
| [`plugin.json`](plugin.json) | Plugin manifest. Enumerates **331** Cursor agent adapter paths under `agents/<provider>/<agent>/harnesses/cursor.agent.md` via the explicit `agents[]` array. **Generated** by `scripts/generate-cursor-plugin.mjs` — do not hand-edit. |

## How users install

```bash
# Clone the repo, then register it as a plugin directory in Cursor:
git clone https://github.com/Raishin/vanguard-frontier-agentic
```

In Cursor: **Settings → Plugins → Add Plugin Directory** → pick the cloned repo path.

Or via the Cursor Extension API:

```ts
vscode.cursor.plugins.registerPath("/absolute/path/to/vanguard-frontier-agentic");
```

## How to update

```bash
# After adding/removing agents, regenerate the manifest:
npm run cursor-plugin:write

# Then verify everything is in sync:
npm run validate:multi-harness-marketplace
```

The `validate` chain runs `validate:multi-harness-marketplace` automatically.

## Schema references (official Cursor docs)

- **Plugins overview:** <https://cursor.com/docs/plugins>
- **Plugins reference (manifest schema):** <https://cursor.com/docs/reference/plugins>
- **Rules (`.cursor/rules/*.mdc`):** <https://cursor.com/docs/rules>
- **MCP servers (`mcp.json`):** <https://cursor.com/docs/mcp>
- **Extension API (`vscode.cursor.plugins.registerPath`):** <https://cursor.com/docs/extension-api>

## Design notes

- **Plugin root = repo root.** A Cursor plugin is any directory containing `.cursor-plugin/plugin.json`. We place ours at the repo root so existing `rules/`, `skills/`, `agents/` directories are auto-discoverable.
- **Explicit `agents[]` paths** — Cursor's manifest schema accepts custom component paths. We enumerate cursor adapter files explicitly so the multi-harness directory structure (`agents/<provider>/<agent>/harnesses/cursor.agent.md`) stays intact.
- **Full 331/331 coverage** — every catalog agent ships a cursor adapter. The validator catches silent drift if the count ever skews.
