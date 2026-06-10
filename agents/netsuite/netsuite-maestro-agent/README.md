# NetSuite Maestro Agent

Entry point for the NetSuite domain. Classifies a NetSuite matter and routes it to the right specialist agent, or gates it to the live-operation guard. Classification and routing only — never executes or recommends executing a live NetSuite mutation.

---

## How routing works

### Required skills

- `skills/cross-functional/netsuite-routing-protocol/SKILL.md`

### Routing modes

- `single` — one specialist owns the matter.
- `parallel (N)` — multiple domains co-own; escalate conflicts.
- `live-guard-gate` — any live-account mutation intent; routes to the live-operation guard.
- `unclassified` — insufficient signal; ask for sanitized evidence.

### Escalation gates

- Financial close / posting / revenue recognition impact → audit-controls-sox agent.
- Cross-subsidiary (OneWorld) boundary risk → oneworld-multisubsidiary agent.
- AI Connector / MCP tool scope → ai-connector-mcp agent.
- Any live mutation → netsuite-live-org-mutation-guard-agent (named human owner).

---

## The NetSuite domain taxonomy

| Domain | Primary agent | Typical signals |
|---|---|---|
| `administrator` | `netsuite-administrator-agent` | netsuite administration, account setup, user provisioning, email preferences, tax configuration, accounting preferences |
| `ai-connector-mcp` | `netsuite-ai-connector-mcp-agent` | AI Connector, MCP, AI Service Connector, MCP Server Connection, tool allowlist, prompt injection |
| `ai-foundations` | `netsuite-ai-foundations-agent` | AI Foundations, NetSuite AI, AI Connector, generative AI, AI bill matching, AI anomaly detection |
| `application-developer` | `netsuite-application-developer-agent` | SuiteScript, SuiteFlow, SuiteBuilder, UIF, SPA, client script |
| `audit-controls-sox` | `netsuite-audit-controls-sox-agent` | SOX, separation of duties, SoD, posting period, period close, revenue recognition |
| `bi-reporting` | `netsuite-bi-reporting-agent` | report, dashboard, KPI, financial narrative, chart, pivot |
| `data-governance-privacy` | `netsuite-data-governance-privacy-agent` | PII, data retention, privacy, field-level access, export controls, data classification |
| `enterprise-architecture` | `netsuite-enterprise-architecture-agent` | NetSuite architecture, SuiteCloud platform, integration architecture, OneWorld design, multi-subsidiary, SDF architecture |
| `erp-consultant` | `netsuite-erp-consultant-agent` | erp implementation, order to cash, procure to pay, inventory management, item setup, pricing rules |
| `evidence-release-drift` | `netsuite-evidence-release-drift-agent` | evidence label, release drift, SOAP deprecation, NetSuite release, 2026.1, 2027.1 |
| `financial-foundations` | `netsuite-financial-foundations-agent` | accounts payable, accounts receivable, AP, AR, chart of accounts, accounting periods |
| `identity-access-role-permission` | `netsuite-identity-access-role-permission-agent` | roles, permissions, segregation of duties, SoD, custom role, least privilege |
| `integration-migration` | `netsuite-integration-migration-agent` | SOAP to REST migration, integration architecture, migration program, SOAP sunset, REST migration plan, integration inventory |
| `oneworld-multisubsidiary` | `netsuite-oneworld-multisubsidiary-agent` | oneworld, subsidiary, intercompany, multi-currency, legal entity, tax jurisdiction |
| `sandbox-nonproduction-governance` | `netsuite-sandbox-nonproduction-governance-agent` | sandbox, non-production environment, release preview, sandbox refresh, OAuth re-authorization, sandbox isolation |
| `saved-searches-workbook` | `netsuite-saved-searches-workbook-agent` | saved search, SuiteAnalytics, workbook, search criteria, results columns, pivot table |
| `sdf-devops-release` | `netsuite-sdf-devops-release-agent` | SuiteCloud Development Framework, SDF, deployment, environment promotion, release pipeline, SDF project |
| `sso-oauth-tba` | `netsuite-sso-oauth-tba-agent` | OAuth 2.0, TBA, token-based authentication, SSO, SAML, sandbox reauthorization |
| `suitecloud-developer` | `netsuite-suitecloud-developer-agent` | SuiteCloud Development Framework, SDF, SuiteScript 2.x, SuiteScript 2.1, SuiteScript upgrade, SuiteApp |
| `suiteflow-automation` | `netsuite-suiteflow-automation-agent` | SuiteFlow, workflow automation, NetSuite workflow, workflow action, workflow condition, approval routing |
| `suitefoundation` | `netsuite-suitefoundation-agent` | suitefoundation, suite foundation, netsuite basics, record types, transaction forms, saved searches |
| `suitescript-secure-code-review` | `netsuite-suitescript-secure-code-review-agent` | SuiteScript security, OWASP SuiteScript, injection SuiteScript, SuiteQL injection, unsafe input SuiteScript, XSS SuiteScript |
| `web-services-integration` | `netsuite-web-services-integration-agent` | SuiteTalk, REST web services, SOAP web services, integration record, OAuth 2.0 REST, RESTlet |

Structural roles (excluded from keyword routing):

| Role | Agent | Function |
|---|---|---|
| Maestro | `netsuite-maestro-agent` | Classify + route only |
| Live Guard | `netsuite-live-org-mutation-guard-agent` | Gate all live mutations |

---

## What the maestro will refuse

- Requests for account credentials, tokens, or the Administrator role.
- Direct execution of any live NetSuite mutation.
- Claiming a Coming-Soon certification is available.

---

## How to use the maestro

### Step 1: Prepare your input

Gather a sanitized description of your NetSuite matter. No credentials, no account IDs, no PII.

```json
{
  "name": "finance-ap-setup",
  "task": "Help us configure AP/AR and accounting setup in NetSuite.",
  "tags": ["happy-path"]
}
```

### Step 2: Invoke the maestro

Provide the task description. The maestro will classify and route.

**Example 1: Single-domain routing**

Input:
```
"task": "Help us configure AP/AR and accounting setup in NetSuite."
```

Routing outcome:
```json
{
  "route": ["netsuite-financial-foundations-agent"],
  "mode": "single"
}
```

Then invoke: `netsuite-financial-foundations-agent`

---

**Example 2: Static SuiteScript security review**

Input:
```
"task": "Static SuiteScript secure code review for security vulnerabilities and OWASP best practices."
```

Routing outcome:
```json
{
  "route": ["netsuite-suitescript-secure-code-review-agent"],
  "mode": "single"
}
```

Then invoke: `netsuite-suitescript-secure-code-review-agent`

---

**Example 3: Cross-domain matter (parallel dispatch)**

Input:
```
"task": "We need to export saved search results with PII across subsidiaries. Ensure data governance, subsidiary access, and workbook safety."
```

Routing outcome:
```json
{
  "route": [
    "netsuite-data-governance-privacy-agent",
    "netsuite-oneworld-multisubsidiary-agent",
    "netsuite-saved-searches-workbook-agent"
  ],
  "mode": "parallel (3)",
  "escalation_gate": "cross-subsidiary-data"
}
```

Then invoke all three agents in parallel, with escalation coordination via `netsuite-live-org-mutation-guard-agent` if any live action is proposed.

---

**Example 4: Live mutation gate**

Input:
```
"task": "We need to deploy our SDF project to production now."
```

Routing outcome:
```json
{
  "route": ["netsuite-live-org-mutation-guard-agent"],
  "mode": "live-guard-gate"
}
```

The live-org-mutation-guard agent requires explicit named human approval before proceeding.

---

**Example 5: Unclassified matter**

Input:
```
"task": "Can you help with something?"
```

Routing outcome:
```json
{
  "route": [],
  "mode": "unclassified"
}
```

The maestro will ask for more specific evidence to classify the matter.

---

### Step 3: Receive agent review

Each routed agent will produce:
- **Summary** — one-line classification of the matter
- **Findings** — structured review output (e.g., security issues, config gaps, role design recommendations)
- **Evidence labels** — LIVE_EVIDENCE, REPOSITORY_EVIDENCE, OFFICIAL_DOCUMENTATION (see evidence hierarchy below)
- **Escalation advice** — who should approve, what approvals are needed, what guardrails apply

---

## Evidence hierarchy

Agents use this hierarchy when citing sources:

1. **LIVE_EVIDENCE** — verified facts from your own live NetSuite account (e.g., saved-search results, role configurations you provided)
2. **REPOSITORY_EVIDENCE** — code or config from your own GitHub/SDF repository
3. **USER_PROVIDED** — details you shared in the request (verified by you, not fetched)
4. **OFFICIAL_DOCUMENTATION** — NetSuite help docs, release notes, SuiteCloud API reference (fetched via Context7 MCP or published sources)
5. **INFERENCE** — reasonable conclusions from official sources (e.g., "SOAP is deprecated at 2027.1 based on release notes, so…")
6. **UNVERIFIED** — claims without strong source (agents will refuse or escalate)
7. **BLOCKED** — claims requiring credentials or live mutation without approval (agents will refuse)

Always ask agents to cite evidence level when making a recommendation. Prefer LIVE_EVIDENCE and OFFICIAL_DOCUMENTATION.

---

## Refusal contract

All specialist agents enforce these refusals (via their LEAST-PRIVILEGES.md):

- ❌ Credentials, tokens, session cookies, client secrets
- ❌ Administrator role as a dependency
- ❌ Direct execution of live mutations (must route through live-org-mutation-guard)
- ❌ Claims that Coming-Soon certifications are available (e.g., "AI Specialist is available now" — it is not; only AI Foundations Associate is available)
- ❌ PII (SSN, credit card, bank account numbers)

If a specialist agent receives any of these, it will refuse and ask for sanitization before resubmission.

---

## Quick reference: Specialist agent domains

See the **Domain Taxonomy** table above for a complete mapping. Common quick routes:

| You need help with… | Route to… |
|---|---|
| Accounts Payable / Accounts Receivable setup | `netsuite-financial-foundations-agent` |
| SuiteScript security review | `netsuite-suitescript-secure-code-review-agent` |
| OAuth 2.0 / SAML / SSO configuration | `netsuite-sso-oauth-tba-agent` |
| SDF deploy and environment promotion | `netsuite-sdf-devops-release-agent` |
| OneWorld / multi-subsidiary design | `netsuite-oneworld-multisubsidiary-agent` |
| Role design and least-privilege custom roles | `netsuite-identity-access-role-permission-agent` |
| Data governance and PII controls | `netsuite-data-governance-privacy-agent` |
| Saved searches and workbooks | `netsuite-saved-searches-workbook-agent` |
| SuiteFlow automation review | `netsuite-suiteflow-automation-agent` |
| SOAP deprecation and REST migration | `netsuite-integration-migration-agent` |
| SOX and audit trail design | `netsuite-audit-controls-sox-agent` |
| NetSuite AI Connector and MCP governance | `netsuite-ai-connector-mcp-agent` |

---

## Eval coverage

Routing is covered by `tests/fixtures/netsuite-maestro-routing/`. Run `npm run validate:maestro-routing`.

---

Part of the Vanguard Frontier Agentic NetSuite portfolio.
