---
name: vanguard-frontier-agentic-install
description: Install all Vanguard Frontier Agentic Codex agents and companion skills into the current user's ~/.codex home after adding or installing the plugin marketplace.
---

# Vanguard Frontier Agentic Codex Install

Use this skill when the user wants the Vanguard Frontier Agentic marketplace content installed into Codex as real user-level agents and skills.

## Reliable two-stage install

Run from any directory:

```bash
codex plugin marketplace add VincentChuWaiChow/vanguard-frontier-agentic
npx --yes -p @raishin/vanguard-frontier-agentic \
  vfa-export-agents --platform codex --all --repo "$HOME" --force
```

If working from a local checkout of the repository, prefer the local exporter so unpublished branch changes are used:

```bash
node scripts/export-marketplace-agents.mjs --platform codex --all --repo "$HOME" --force
```

## Verify

```bash
find "$HOME/.codex/agents" -maxdepth 1 -name '*.toml' | wc -l
find "$HOME/.codex/skills" -mindepth 1 -maxdepth 1 -type d | wc -l
```

Expected for this release: all Codex-capable VFA agents are installed under `$HOME/.codex/agents`, and companion skills are installed under `$HOME/.codex/skills`.

## Important limitation

`codex plugin marketplace add` installs/tracks the marketplace source. It does not, by itself, prove that Codex installed the plugin cache or exported repo-level agent TOML files. The exporter is the deterministic second stage for agents and companion skills.
