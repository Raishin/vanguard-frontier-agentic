# Compatibility

This repository targets multiple agent harnesses without pretending they are identical.

## Supported harness labels

- `codex`
- `copilot`
- `claude-code`
- `cursor`
- `gemini`
- `kiro`
- `other`

## Compatibility expectations

- Skills should be plain Markdown where possible.
- Harness-specific rules belong under `rules/<harness>/`.
- Do not put runtime-only configuration in generic prose.
- MCP references should show generic configuration concepts and link to official setup docs instead of hardcoding unstable client-specific syntax.

## Cross-harness rule

If an asset depends on a specific tool, CLI, MCP server, or model behavior, say so in metadata or the asset body. Silent dependencies are defects.

## Skill bundling per harness

Skill (`SKILL.md`) export is currently shipped only for `claude-code`. Per-harness viability, citations, and the proposed adapter shape for Gemini CLI, GitHub Copilot, OpenAI Codex CLI, Cursor, and Kiro live in [`docs/cross-harness-skills.md`](./cross-harness-skills.md).
