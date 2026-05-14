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

| Agent | Purpose | Companion skill(s) |
|-------|---------|--------------------|
| [finops-maestro-agent](finops-maestro-agent/) | Route FinOps tasks to the narrowest specialist or parallel team (max 4); FOCUS-aware classification; never auto-dispatches mutating specialists | [finops-maestro](../../skills/finops/finops-maestro/) |
| [finops-ai-economist-agent](finops-ai-economist-agent/) | AI workload economics across foundation-model providers and GPU instance families: token economics, $/GPU-hour-utilized, cross-provider comparison, training-vs-inference TCO | [fetch-foundation-model-pricing](../../skills/finops/fetch-foundation-model-pricing/), [carbon-cost-pair](../../skills/finops/carbon-cost-pair/) |
| [finops-kubernetes-rightsizer-agent](finops-kubernetes-rightsizer-agent/) | Pod request/limit recommendations from supplied p50/p95/p99 metrics, idle scan, Karpenter consolidation eligibility, OpenCost-compatible allocation mapped to FOCUS | [rightsize-recommendation](../../skills/finops/rightsize-recommendation/), [kubernetes-allocation-report](../../skills/finops/kubernetes-allocation-report/), [carbon-cost-pair](../../skills/finops/carbon-cost-pair/) |
| [finops-cloud-price-advisor-agent](finops-cloud-price-advisor-agent/) | Fetch live public prices from AWS, Azure, and OCI pricing APIs; produce cost estimates for live environments and prototypes; default currency USD | [finops-cloud-price-advisor](../../skills/finops/finops-cloud-price-advisor/) |

### Shared posture

All FinOps agents operate in read-only mode:

- **Public unauthenticated pricing APIs only.** No cloud credentials, billing account IDs, API keys, kubeconfig, bearer tokens, service-account JWTs, or cost-management access are accepted. Refusal is unconditional.
- **Provenance labels mandatory**: every numeric output is labeled `live-price` / `live-evidence` / `documentation-based` / `assumed` / `excluded` with source URL + ISO 8601 timestamp where applicable.
- **FOCUS v1.2-mapped output** where the domain admits it (BilledCost, EffectiveCost, ServiceCategory, ServiceName, ChargeCategory, SkuPriceId, ResourceId, etc.).
- **Currency**: USD by default; other currencies on explicit request via public exchange rate APIs (no auth required).
- **On-demand list prices only** unless the user explicitly requests committed, reserved, or savings-plan pricing.
- **Carbon pairing** available via `carbon-cost-pair` for CSRD/SEC climate disclosure (Scope 2 market-based default).
- **No auto-mutation**: the maestro never dispatches a mutating specialist without an explicit human approval gate and handoff packet (specialist name, blast-radius, rollback path).

### Maestro routing

The maestro routes across three specialists today. Fixture set: `tests/fixtures/finops-maestro-routing/`. Validation gate: `npm run validate:maestro-routing`. No live-guard agents exist in v1; future mutating specialists must be added to `live_guards` in `taxonomy.json` before dispatch is permitted.

## Rules
- Keep skill links pointed at `skills/finops/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent`.
- Do not invent authentication requirements for public pricing APIs.
- Do not introduce mutating specialists without wiring the live-guard gate in the maestro taxonomy.
- Run `npm run validate` after changes.
