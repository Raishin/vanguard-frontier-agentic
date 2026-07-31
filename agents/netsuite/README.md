# Oracle NetSuite Agents

Enterprise-grade advisor roles for NetSuite architecture, operations, and compliance review.

**25 agents** across 3 layers: maestro routing, governance (5), and domain specialists (20).

## Quick Start

- **Routing:** `netsuite-maestro-agent` classifies matters via `skills/cross-functional/netsuite-routing-protocol`
- **Docs:** See `AGENTS.md` for portfolio overview, operating principles, and refusal triggers
- **Skills:** `skills/netsuite/` contains companion skills with procedures, safety checklists, and least-privilege guides
- **Fixtures:** `tests/fixtures/netsuite-maestro-routing/` contains routing test cases and expected outputs

## Key Principles

- **Least privilege:** Custom roles from standards, never Administrator
- **Static review only:** No live mutations without separate authorization
- **Evidence-based:** Official NetSuite/Oracle docs only; no fabrication
- **OAuth2 priority:** REST/OAuth2 over legacy SOAP (deprecation timeline: 2026.1→2028.2)
- **Cross-domain:** Maestro coordinates multi-domain and escalation gates

## Operating Stance

This portfolio is **advisory only**. Agents provide judgment and recommendations on architecture, compliance, security, and operations—but do NOT:
- Deploy code or configuration changes to production
- Modify permissions or roles in live accounts
- Execute workflows or automated tasks
- Edit customer data
- Publish reports or saved searches

Those require separate authorization via `netsuite-live-org-mutation-guard-agent` + human approval.

## Refusal Triggers

All agents refuse:
- Credential or token requests
- Administrator role dependency
- Live mutations without explicit out-of-band approval
- "Coming Soon" certification claims (unverified)
- Regulated PII without jurisdiction
- Broad unvetted MCP tool grants

## Certification Alignment

Agents track NetSuite's 5 certification paths: SuiteFoundation, Administrator, Developer, OpenAir, Advanced Developer. See `AGENTS.md` for details.

## Release Sensitivity

NetSuite ships biannual releases. Critical timeline:
- 2026.1 – OAuth2 default; SOAP deprecation begins
- 2027.1 – New SOAP integrations disabled
- 2028.2 – SOAP sunset (estimated)

Agents label release-sensitive claims explicitly.

## Files

- `AGENTS.md` – Detailed agent portfolio, routing, escalation, refusal rules
- `agents/netsuite/<agent-id>/` – Individual agent specs (AGENT.md, metadata.json, LEAST-PRIVILEGES.md, harnesses/)
- `skills/netsuite/<agent-id>-skill/` – Companion skills (SKILL.md, references/)
- `tests/fixtures/netsuite-maestro-routing/` – Routing test taxonomy and test cases
- `catalog/agents.json` – Machine-readable agent manifest
- `catalog/skills.json` – Machine-readable skills manifest

---

**Version:** 0.1.0  
**Provider:** netsuite  
**Source:** github: VincentChuWaiChow  
**Last updated:** 2026-06-09
