# EVAL DEFINITION: eu-cloud-providers

Branch: `claude/add-eu-cloud-providers-6NGhv`
Last updated: 2026-05-10
**Status: COMPLETE — all gates green as of 2026-05-10**

## Scope

Add EU cloud provider agent suites for: OVHcloud, IONOS Cloud, Scaleway, Hetzner Cloud, Contabo.

## Capability Evals (per provider)

Each provider must satisfy ALL of the following before being marked complete:

### CE-1: Filesystem layout

- [x] `agents/<provider>/README.md` exists — all 5 providers
- [x] `agents/<provider>/<provider>-maestro-agent/AGENT.md` exists — all 5 providers
- [x] `agents/<provider>/<provider>-maestro-agent/metadata.json` valid against `schemas/agent.schema.json`
- [x] At least 4 advisory agents exist (maestro + 3 specialists) — 6 agents per provider
- [x] At least 1 live-guard agent exists — OVHcloud (KMS), IONOS (DBaaS), Scaleway (Kapsule), Hetzner (firewall + server), Contabo (instance + storage)
- [x] Each agent has `harnesses/claude-code.agent.md` and `harnesses/codex.toml`

### CE-2: Companion skills

- [x] Each agent has a 1:1 companion skill under `skills/<provider>/<skill-id>/SKILL.md` — 30 skills total
- [x] Every `SKILL.md` declares a valid YAML frontmatter (passes `validate:skill-schema`)
- [x] Every `SKILL.md` declares `allowed-tools` as least-privilege
- [x] Every `SKILL.md` has `name`, `description` (50–1500 chars), and `metadata.author/version`

### CE-3: Schema/catalog validity

- [x] `provider` value is one of `ovhcloud`, `ionos`, `scaleway`, `hetzner`, `contabo`
- [x] All `metadata.json` files pass `validate-agent-frontmatter-schema.py`
- [x] `catalog/agents.json` and `catalog/skills.json` updated with new entries
- [x] `catalog/skill-manifest.json` regenerated (`manifest:check` passes)

### CE-4: Doc grounding (Context7-backed)

- [x] `official_docs[]` field references official URLs (vendor docs, Terraform registry, or CLI docs)
- [x] `last_verified` is `2026-05-10`
- [x] AGENT.md operating rules cite Context7 fallback where MCP tooling is unavailable

### CE-5: Security posture

- [x] No hardcoded credentials, API keys, account IDs, customer IDs, tenants — verified in PR #18 security audit
- [x] `security_notes` field is non-trivial (≥ 20 chars, calls out provider-specific risks)
- [x] Live-guard agents declare approval-gated posture and rollback requirement

### CE-6: Content quality

- [x] AGENT.md follows the canonical 5-section response shape (verdict, evidence, blockers, next actions, open questions) — or equivalent
- [x] AGENT.md references its companion skill via `skills/<provider>/<skill-id>/SKILL.md`
- [x] SKILL.md follows progressive disclosure (references loaded only when needed)

### CE-7: Role-based install coverage (added post-implementation)

- [x] All EU agents added to `catalog/install-roles.json` across appropriate roles
- [x] `cloud-security-engineer`: OVHcloud IAM/KMS, IONOS security, Scaleway IAM, Contabo hardening
- [x] `cloud-platform-engineer`: OVHcloud k8s/network, IONOS k8s/DCD, Scaleway k8s/rollout/network, Hetzner infra/firewall/server, Contabo instance/storage
- [x] `cloud-finops-analyst`: all 5 providers' cost and capacity agents

### CE-8: Taxonomy and documentation (added post-implementation)

- [x] `docs/taxonomy.md` lists all 5 EU providers
- [x] `README.md` provider table, agent counts, directory tree, `--provider` arg updated
- [x] Provider README files corrected — only existing agents listed, no phantom references
- [x] `harnesses` field in metadata.json matches actual `harness_variants` (codex + claude-code only)

## Regression Evals

- [x] `npm run validate:catalog` — pass
- [x] `npm run validate:skill-schema` — pass
- [x] `npm run validate:allowed-tools` — pass
- [x] `npm run validate:agent-schema` — pass (applies to new EU agents)
- [x] `npm run manifest:check` — pass (after `manifest:write`)
- [x] `npm run validate:links --offline` — pass

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
