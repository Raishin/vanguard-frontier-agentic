# EVAL DEFINITION: eu-cloud-providers

Branch: `claude/add-eu-cloud-providers-6NGhv`
Last updated: 2026-05-10

## Scope

Add EU cloud provider agent suites for: OVHcloud, IONOS Cloud, Scaleway, Hetzner Cloud, Contabo.

## Capability Evals (per provider)

Each provider must satisfy ALL of the following before being marked complete:

### CE-1: Filesystem layout

- [ ] `agents/<provider>/README.md` exists
- [ ] `agents/<provider>/<provider>-maestro-agent/AGENT.md` exists
- [ ] `agents/<provider>/<provider>-maestro-agent/metadata.json` valid against `schemas/agent.schema.json`
- [ ] At least 4 advisory agents exist (maestro + 3 specialists)
- [ ] At least 1 live-guard agent exists (where applicable — Hetzner/Contabo may skip if no Terraform)
- [ ] Each agent has `harnesses/claude-code.agent.md` and `harnesses/codex.toml`

### CE-2: Companion skills

- [ ] Each agent has a 1:1 companion skill under `skills/<provider>/<skill-id>/SKILL.md`
- [ ] Every `SKILL.md` declares a valid YAML frontmatter (passes `validate:skill-schema`)
- [ ] Every `SKILL.md` declares `allowed-tools` as least-privilege
- [ ] Every `SKILL.md` has `name`, `description` (50–1500 chars), and `metadata.author/version`

### CE-3: Schema/catalog validity

- [ ] `provider` value is one of `ovhcloud`, `ionos`, `scaleway`, `hetzner`, `contabo`
- [ ] All `metadata.json` files pass `validate-agent-frontmatter-schema.py`
- [ ] `catalog/agents.json` and `catalog/skills.json` updated with new entries
- [ ] `catalog/skill-manifest.json` regenerated (`manifest:check` passes)

### CE-4: Doc grounding (Context7-backed)

- [ ] `official_docs[]` field references official URLs (vendor docs, Terraform registry, or CLI docs)
- [ ] `last_verified` is `2026-05-10`
- [ ] AGENT.md operating rules cite Context7 fallback where MCP tooling is unavailable

### CE-5: Security posture

- [ ] No hardcoded credentials, API keys, account IDs, customer IDs, tenants
- [ ] `security_notes` field is non-trivial (≥ 20 chars, calls out provider-specific risks)
- [ ] Live-guard agents declare approval-gated posture and rollback requirement

### CE-6: Content quality

- [ ] AGENT.md follows the canonical 5-section response shape (verdict, evidence, blockers, next actions, open questions) — or equivalent
- [ ] AGENT.md references its companion skill via `skills/<provider>/<skill-id>/SKILL.md`
- [ ] SKILL.md follows progressive disclosure (references loaded only when needed)

## Regression Evals

- [ ] `npm run validate:catalog` — pass
- [ ] `npm run validate:skill-schema` — pass
- [ ] `npm run validate:allowed-tools` — pass
- [ ] `npm run validate:agent-schema` — pass (applies to new EU agents)
- [ ] `npm run manifest:check` — pass (after `manifest:write`)
- [ ] `npm run validate:links --offline` — pass

## Graders

### G-1: Code grader (deterministic)

```bash
# Per-provider asset count
for provider in ovhcloud ionos scaleway hetzner contabo; do
  agents=$(find agents/$provider -name AGENT.md 2>/dev/null | wc -l)
  skills=$(find skills/$provider -name SKILL.md 2>/dev/null | wc -l)
  echo "$provider: agents=$agents skills=$skills"
done

# Schema gates
npm run validate:catalog && npm run validate:skill-schema && \
  npm run validate:allowed-tools && npm run validate:agent-schema && \
  npm run manifest:check
```

### G-2: Rule grader (regex / schema)

- All `metadata.json` files match: `"provider":\s*"(ovhcloud|ionos|scaleway|hetzner|contabo)"`
- No file contains: `(AKIA[0-9A-Z]{16}|client_secret\s*=\s*"[^"]+")` (real secrets)

### G-3: Model grader (manual sweep)

- Spot-check 1 agent per provider:
  - Does AGENT.md state when to load the agent?
  - Does it cite an official documentation URL?
  - Does the live-guard agent (if any) declare a hard-stop condition?

## Pass thresholds

- **CE-1 to CE-6**: pass^3 = 100% (must succeed every time)
- **Regression evals**: pass^3 = 100% (zero tolerance for breakage)
- **Manual model grader**: 5/5 providers must pass spot-check

## Anti-patterns to refuse

- Inventing services that don't exist (e.g., "OVHcloud Lambda")
- Copying AWS/GCP boilerplate without adapting region/IAM model
- Recommending Terraform for Hetzner/Contabo (no official provider)
- Adding agents that overlap perfectly with existing ones (no value)
- Including secrets, account IDs, or customer-specific identifiers
