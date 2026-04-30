# AGENTS.md

## Purpose
- Store multi-cloud FinOps agents focused on pricing, cost estimation, and financial operations across AWS, Azure, and OCI.

## Patterns
- `agents/finops/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/finops/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/finops/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/finops/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/finops/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/finops/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/finops/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants.
- `agents/finops/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## FinOps Agents

| Agent | Purpose | Skill |
|-------|---------|-------|
| [finops-cloud-price-advisor-agent](finops-cloud-price-advisor-agent/) | Fetch live public prices from AWS, Azure, and OCI pricing APIs; produce cost estimates for live environments and prototypes; default currency USD | [finops-cloud-price-advisor](../../skills/finops/finops-cloud-price-advisor/) |

### FinOps price advisor posture

The FinOps Cloud Price Advisor operates in read-only mode only:

- **All three pricing APIs are public and unauthenticated.** No cloud credentials, billing account IDs, or cost management access are required or accepted.
- **Two modes**: live-environment (enumerate running resources → line-item estimate) and prototype (planned architecture spec → pre-provisioning estimate).
- **Currency**: USD by default; other currencies available via public exchange rate APIs (no auth required).
- **On-demand list prices only** unless the user explicitly requests committed/reserved pricing.
- **Label every value**: `live-price` (fetched this session), `documentation-based` (fallback), `assumed` (user did not specify), `excluded` (out of scope).

## Rules
- Keep skill links pointed at `skills/finops/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent`.
- Do not invent authentication requirements for public pricing APIs.
- Run `npm run validate` after changes.
