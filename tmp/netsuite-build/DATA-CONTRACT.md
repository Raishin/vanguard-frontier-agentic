# NetSuite Build — Data Contract (v1)

Parallel content agents each write **one JSON file per agent** into
`scripts/netsuite_data/agents/<agent-id>.json`. Disjoint files → zero conflict.
The generator `scripts/gen_netsuite_agents.py` globs these and emits all agent +
companion-skill + harness files deterministically (modeled on
`scripts/gen_azure_live_guards.py`, but using salesforce **static-review (T0)**
conventions: `LEAST-PRIVILEGES.md`, `sandbox_mode = "read-only"`, full metadata).

## JSON schema for `scripts/netsuite_data/agents/<agent-id>.json`

```json
{
  "id": "netsuite-<slug>-agent",
  "name": "NetSuite <Title> Agent",
  "layer": 1,                          // 1=governance, 2=specialist
  "domain_key": "<routing-domain-key>",// kebab; used in maestro taxonomy
  "routing_keywords": ["...", "..."],  // 6-10 distinct keywords for routing
  "summary": "<20-300 chars; what it reviews; ends with 'static review only, never mutates a NetSuite account'>",
  "focus": "<1-2 sentences, the review remit>",
  "mission": "<1 paragraph>",
  "scope_owned": ["...", "..."],       // 4-8 bullets
  "out_of_scope": ["...", "..."],      // 3-6 bullets, name the agent to use instead
  "cert_alignment": "<cert/role name + status from evidence-matrix, or 'Enterprise role: <x>'>",
  "required_inputs": ["...", "..."],   // sanitized config excerpts the agent needs
  "operating_rules": ["...", "..."],   // 5-8; MUST include static-review + evidence-before-assertion + least-privilege
  "evidence_requirements": ["...", "..."],
  "refusal_triggers": ["...", "..."],  // MUST include: credentials/tokens; Administrator-role; live mutation w/o approval; coming-soon cert claimed available
  "escalation_triggers": ["...", "..."],
  "least_privilege": {
    "custom_role_name": "NetSuite <X> Reviewer (custom)",
    "based_on_standard_role": "<standard role to copy>",
    "permissions": [{"name": "<perm>", "level": "View|Create|Edit|Full", "why": "<rationale>"}],
    "modules": ["..."],
    "requires_2fa": true,
    "forbidden": ["Administrator role", "<others>"],
    "notes": "<custom-role-from-standard, sandbox-test-first, etc.>"
  },
  "companion_skill": {
    "id": "netsuite-<slug>-skill",
    "name": "NetSuite <Title> Skill",
    "category": "<one of skill category enum: security|compliance|platform|data|finance|architecture|operational|generation|...>",
    "description": "<50-1500 chars; includes TRIGGER phrases and DO NOT TRIGGER cases inline (salesforce style)>",
    "when": ["...", "..."],            // 3-5 'use when' bullets
    "workflow_steps": ["Step 1 — ...", "Step 2 — ..."],  // 4-7 steps
    "safety_checklist": ["...", "..."],
    "evidence_hierarchy_note": "LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED",
    "references": [                     // each becomes references/<file>.md
      {"file": "official-sources.md", "purpose": "..."},
      {"file": "safety-checklist.md", "purpose": "..."},
      {"file": "least-privilege.md", "purpose": "..."},
      {"file": "release-drift.md", "purpose": "..."},
      {"file": "<topic-specific>.md", "purpose": "..."}
    ]
  },
  "official_docs": ["https://...", "https://..."],   // REAL Oracle/NetSuite URLs from evidence-matrix.md ONLY
  "security_notes": "<>=20 chars; static review only; no creds; no mutation>",
  "source_type": "original | adapted | reference-only",
  "source_attribution": "<required iff adapted: name upstream skill + UPL-1.0 + added value>",
  "upstream_reuse": "<REFERENCE|DEPENDENCY|ADAPTED_WRAPPER|NO_ACTION + upstream skill id, from upstream-reuse-matrix.md>"
}
```

## Hard rules for content authors
- **No fabricated facts.** Every NetSuite claim and every `official_docs` URL must trace
  to `tmp/netsuite-build/evidence/evidence-matrix.md`. If not in the matrix, mark inside the
  text as `[UNVERIFIED]` and do NOT put it in `official_docs`.
- **Coming-soon certs** (AI Specialist/Professional, BI & Reporting Professional) must NEVER
  be described as available. State status explicitly.
- **Least privilege**: never Administrator. Custom role copied from a standard role. Note 2FA.
  AI Connector agent: role is NOT Administrator; perms `MCP Server Connection` +
  `Log in using OAuth 2.0 Access Tokens` (exact strings).
- **Integration posture**: OAuth 2.0 for REST/RESTlets/SuiteAnalytics Connect; SOAP =
  migration risk (2026.1 REST+OAuth2 default; 2027.1 new SOAP blocked; per evidence matrix).
- **Adapted skills** must set `source_type:"adapted"` + `source_attribution` citing the Oracle
  upstream skill (UPL-1.0, attribution required) per `upstream-reuse-matrix.md`.
- Companion skill `description` MUST be 50-1500 chars and embed TRIGGER / DO NOT TRIGGER inline.
- Keep summaries/security_notes within length mins (summary ≥20, security_notes ≥20).
