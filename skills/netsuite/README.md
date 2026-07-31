# Oracle NetSuite Skills

Companion skill library for the 25 NetSuite agents. Each skill provides procedures, decision logic, safety checklists, and least-privilege guidance.

## Portfolio Overview

**25 skills** (1:1 with agents):
- Layer 1 Governance & Routing (5)
- Layer 2 Domain Specialists (20)

Each skill includes:
- `SKILL.md` – Description, when-to-use, workflow, safety rules, decision logic
- `metadata.json` – Catalog metadata (id, name, provider, harnesses, allowed-tools, etc.)
- `references/` – Official sources, safety checklists, least-privilege notes, release drift, topic-specific guides

## Key Features

### Evidence Hierarchy
Skills declare evidence quality per step:
1. LIVE_EVIDENCE (NetSuite live API, current state)
2. REPOSITORY_EVIDENCE (project git, version control)
3. USER_PROVIDED (context from engineer)
4. OFFICIAL_DOCUMENTATION (NetSuite/Oracle docs + date)
5. INFERENCE (agent reasoning)
6. UNVERIFIED (claims without source)
7. BLOCKED (denied—credentials, live mutations, fabrications)

### Least Privilege by Default
Each skill's `references/least-privilege.md` provides:
- Standard role baseline
- Permission removal checklist
- Module requirements
- 2FA mandate for privileged roles
- Forbidden permission combinations

### Release Sensitivity
Skills track NetSuite's biannual release timeline:
- 2026.1 – OAuth2 default, SOAP deprecation
- 2027.1 – New SOAP integrations disabled
- 2028.2 – SOAP sunset (estimated)

Claims impacting future releases are labeled explicitly.

### Safety Checklists
Each `references/safety-checklist.md` covers:
- Refusal triggers (credentials, live mutations, admin role)
- Escalation gates (cross-domain, segregation of duties, etc.)
- Evidence quality gates
- Compliance and audit trails

## Companion to Agents

Each skill is paired with its agent:
- Agent performs judgment and review
- Skill provides step-by-step procedure and safety rules
- Together they form a complete advisory capability

Example: `netsuite-sso-oauth-tba-agent` + `netsuite-sso-oauth-tba-skill`
- Agent: "This design uses TBA. Is that optimal given your integrations?"
- Skill: "TBA use cases, OAuth2 preference, token management, 2FA requirements, release timeline…"

## Routing & Cross-Domain

The cross-functional `netsuite-routing-protocol` skill coordinates:
- Multi-domain escalations (saved-search + oneworld + data-governance, etc.)
- Conflict resolution when agents disagree
- Escalation gates (segregation-of-duties, irreversible-deploy, finance/revenue, etc.)

Skills reference this protocol when matters cross boundaries.

## Official Sources

All skills cite official NetSuite/Oracle sources with verification dates:
- NetSuite Certification Resource Center
- Oracle NetSuite Help Portal
- SuiteTalk API documentation
- SuiteCloud SDK github
- Release notes and breaking-change bulletins

No fabrication. All facts trace back to official docs.

## Files

- `netsuite/<agent-id>-skill/` – Individual skill directories
  - `SKILL.md` – Main skill spec (allowed-tools in frontmatter)
  - `metadata.json` – Catalog metadata
  - `references/official-sources.md` – Citation list with URLs and dates
  - `references/safety-checklist.md` – Refusal triggers, escalation gates, evidence quality
  - `references/least-privilege.md` – Role, permission, 2FA, module guidance
  - `references/release-drift.md` – NetSuite release timeline and impacts
  - `references/<topic>.md` – Topic-specific guides (e.g., sandbox-oauth-isolation.md)

## No Live Mutations

Skills are advisory and procedural ONLY. They do NOT:
- Deploy to production
- Modify permissions or roles in live accounts
- Execute workflows
- Edit data
- Activate features

Live actions require `netsuite-live-org-mutation-guard-agent` + separate human authorization.

## Least-Privilege Baseline

Skills enforce:
- Never Administrator unless scoping requires it (then document removal plan)
- 2FA on all privileged custom roles
- OAuth2 for REST/RESTlets/SuiteAnalytics; TBA only as fallback
- No user credentials; service accounts with OAuth2 tokens
- Minimum-necessary modules and permissions per role

## Escalation & Conflict Resolution

When matters cross domains or agents disagree:
1. netsuite-routing-protocol applies conflict-resolution protocol
2. Escalation gate fires (e.g., segregation-of-duties, irreversible-deploy)
3. Matter pauses and escalates to human owner
4. Decision logged in audit trail

Skills document escalation triggers and gates explicitly.

## Tools & Boundaries

Each skill declares `allowed-tools` in SKILL.md frontmatter:
- Read – read project files, official docs
- Grep – search code and specs
- Glob – find files by pattern
- **NOT** allowed by default: Bash, GitHub mutation, live NetSuite API calls

This enforces the advisory boundary: judgment without destructive capability.

---

**Version:** 0.1.0  
**Provider:** netsuite  
**Source:** github: VincentChuWaiChow  
**Last updated:** 2026-06-09
