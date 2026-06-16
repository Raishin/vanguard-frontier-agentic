# NetSuite Maestro — Real-World Examples & Routing Scenarios

This guide provides real-world examples of how to invoke the NetSuite maestro, what routing decisions it makes, and how each specialist agent handles your request.

---

## Example 1: Basic Accounts Payable Setup

**Scenario:** Your company is implementing NetSuite for the first time. You need help setting up Accounts Payable, including vendor records, payment terms, and GL account mappings.

### Input to the maestro

```json
{
  "task": "Help us set up NetSuite Accounts Payable. We need to configure vendor record defaults, payment term templates, and accounting period GL mappings for AP processing.",
  "tags": ["implementation"]
}
```

### Maestro classification & routing

```
Classification: financial-foundations domain
Routing mode: SINGLE
Specialist agent: netsuite-financial-foundations-agent
```

### What the specialist does

1. **Reviews** your sanitized AP configuration excerpts (e.g., "we use these 5 payment terms", "our GL chart has these AP liability accounts")
2. **Checks** against NetSuite best practices (e.g., "vendor records should have default payment method", "payment terms should use days overdue, not specific dates")
3. **Cites evidence** using OFFICIAL_DOCUMENTATION (NetSuite AP help docs via Context7 MCP)
4. **Produces** structured findings:
   - ✅ Vendor record setup is correct
   - ⚠️ Payment terms should use "Net 30" not "Due next month"
   - ⚠️ AP liability GL accounts are not segregated by payment method
5. **Recommends** a custom role for your AP team (see `LEAST-PRIVILEGES.md`)

### What NOT to do

❌ Do NOT ask the maestro to "log in and set up vendors in your live NetSuite account"
❌ Do NOT provide vendor bank account numbers, ACH account numbers, or credit card details
❌ Do NOT ask for the Administrator role to be used for AP

If you try any of these, the agent will refuse and ask for sanitization or escalation.

---

## Example 2: SuiteScript Security Code Review

**Scenario:** Your developer team wrote a custom SuiteScript to calculate sales commissions. Before deploying to production, you want a static security review.

### Input to the maestro

```json
{
  "task": "Static SuiteScript security review for a user event script calculating commission amounts. Check for OWASP vulnerabilities, injection risks, and unsafe input handling.",
  "code_excerpt": "// User Event Script\nconst commission = nlapiGetFieldValue('commission_calc');\nconst result = nlapiExecuteQuery('SELECT * FROM custom_commission WHERE id = ' + commission);\nnlapiSetFieldValue('commission_result', result);",
  "tags": ["security", "pre-deployment"]
}
```

### Maestro classification & routing

```
Classification: suitescript-secure-code-review domain
Routing mode: SINGLE
Specialist agent: netsuite-suitescript-secure-code-review-agent
```

### What the specialist does

1. **Analyzes** the code for security issues:
   - ❌ **SQL injection vulnerability**: `nlapiExecuteQuery()` concatenates user input directly (high-risk)
   - ❌ **Unsafe input**: `commission_calc` field is not validated before use
   - ✅ **No hardcoded credentials**: Good, no API keys or passwords in code
2. **Cites evidence**:
   - OFFICIAL_DOCUMENTATION: NetSuite SuiteScript security best practices (Context7 MCP)
   - INFERENCE: "SuiteQL injection is possible when user input is concatenated into queries"
3. **Produces findings**:
   ```
   [CRITICAL] SQL Injection Risk
   - nlapiExecuteQuery() with concatenated input allows query injection
   - Recommendation: Use SuiteQL parameterized queries or nlapiCreateSearch()
   
   [HIGH] Unsafe Input Handling
   - commission_calc not validated before use in query
   - Recommendation: Validate field type, range, and pattern before query
   ```
4. **Escalates** if you ask to deploy: "This code has vulnerabilities. To deploy, route to netsuite-live-org-mutation-guard-agent with your approval and a remediation plan."

### What NOT to do

❌ Do NOT ask the agent to deploy the script to production
❌ Do NOT provide credentials for the SuiteScript editor (OAuth tokens, etc.)
❌ Do NOT ask the agent to log in and modify your code in NetSuite

---

## Example 3: Cross-Domain Matter (Parallel Routing)

**Scenario:** Your company operates across three subsidiaries (US, EU, Asia) and needs to export a saved search containing customer data (including PII like email addresses) to a workbook for analysis.

### Input to the maestro

```json
{
  "task": "We need to export a saved search with customer contact info (names, emails, phone numbers) across all three subsidiaries to a shared workbook for analysis. What are the data governance, privacy, and subsidiary boundary considerations?",
  "subsidiary_count": 3,
  "data_types": ["customer_email", "customer_phone", "customer_name"],
  "export_target": "shared_workbook",
  "tags": ["cross-subsidiary", "pii", "analysis"]
}
```

### Maestro classification & routing

```
Classification: data-governance + oneworld-multisubsidiary + saved-searches-workbook domains
Routing mode: PARALLEL (3 agents)
Specialist agents:
  1. netsuite-data-governance-privacy-agent
  2. netsuite-oneworld-multisubsidiary-agent
  3. netsuite-saved-searches-workbook-agent
Escalation gate: cross-subsidiary-data
```

### What the specialists do (in parallel)

**Agent 1: Data Governance & Privacy**
- ✅ Customer email/phone is PII under GDPR, CCPA, etc.
- ✅ Exporting PII across borders requires data transfer agreements
- ✅ Shared workbooks may not be appropriate if workbook is accessible to non-subsidiary users
- Recommendation: Use saved search filters to restrict by subsidiary, apply field-level encryption, or use a secure export container

**Agent 2: OneWorld Multi-Subsidiary**
- ✅ Saved searches can be scoped per subsidiary to prevent cross-subsidiary data leakage
- ❌ If the search is not scoped, it will return data from all subsidiaries (including restricted ones)
- ✅ Recommend filtering by subsidiary in the search criteria
- ✅ Recommend assigning the workbook to subsidiary-specific roles (not global)

**Agent 3: Saved Searches & Workbooks**
- ✅ Saved search structure is valid, columns are correct
- ❌ Workbook may be over-permissioned if it's shared across subsidiaries
- ⚠️ PII in workbooks is cached; if workbook is published, it may be visible to more users than intended
- Recommendation: Keep workbook in sandbox until data governance review is complete

### Coordination & Escalation

If all three agents agree on a path forward, you can proceed. If they disagree (e.g., "subsidiary agent says filter by subsidiary, but privacy agent says don't export at all"), the maestro escalates to `netsuite-live-org-mutation-guard-agent` for a named human decision.

### What NOT to do

❌ Do NOT ask agents to deploy the workbook without reviewing all three perspectives
❌ Do NOT export customer PII without understanding GDPR/CCPA implications
❌ Do NOT assume that because one agent says "yes" it's safe (wait for all three to agree)

---

## Example 4: SDF Deployment with Live-Org Gate

**Scenario:** Your SDF project is ready to deploy to production. You've already code-reviewed it, and now you want approval before going live.

### Input to the maestro

```json
{
  "task": "We're ready to deploy our SuiteCloud Development Framework project to production. The project includes: 3 custom scripts, 2 saved searches, 1 custom record type, and 5 script deployments. Can you review the deployment plan and give us the go-ahead?",
  "project_files_count": 11,
  "deployment_target": "production",
  "tags": ["deployment", "live-mutation"]
}
```

### Maestro classification & routing

```
Classification: ANY + live-mutation intent detected
Routing mode: LIVE-GUARD-GATE
Specialist agent: netsuite-live-org-mutation-guard-agent
Reason: Your task mentions "deploy to production", which is a live mutation.
```

### What the mutation guard does

1. **Acknowledges** your request: "You want to deploy an SDF project to production. This is a live mutation and requires explicit approval."
2. **Asks** for a structured case capsule:
   ```
   [REQUIRED] Deployment approval form
   - Project name: ___
   - Deployment date/time: ___
   - Change control ticket: ___
   - Named approver (human name, email): ___
   - Rollback plan: ___
   - Testing evidence (e.g., sandbox deployment passed, scripts work, no errors): ___
   ```
3. **Reviews** your evidence:
   - ✅ Sandbox testing was completed
   - ✅ Change control ticket is approved by ops
   - ✅ Named approver is authorized
4. **Gates** the deployment: "Deployment is approved contingent on:
   - [ ] Backup of current production code
   - [ ] Deployment window (after business hours)
   - [ ] Rollback plan is tested
   - [ ] Monitoring is enabled for new scripts"
5. **Produces** the decision: "APPROVED for production deployment [timestamp] by [approver name]"

### What NOT to do

❌ Do NOT ask the mutation guard to "just deploy it without approvals"
❌ Do NOT provide credentials or expect the agent to log in
❌ Do NOT bypass the mutation guard if you "think it's safe"

The mutation guard is the safety net. Use it.

---

## Example 5: OAuth 2.0 Configuration Review

**Scenario:** You're migrating from Token-Based Authentication (TBA) to OAuth 2.0 for your REST API integrations. You need guidance on OAuth scope, client setup, and sandbox vs. production differences.

### Input to the maestro

```json
{
  "task": "We're migrating from TBA to OAuth 2.0 for our REST API integrations. We have 3 RESTlets and 2 SuiteAnalytics Connect queries. What OAuth scopes do we need, and what's different between sandbox and production auth?",
  "integration_count": 5,
  "integration_types": ["RESTlet", "SuiteAnalytics"],
  "source_auth": "TBA",
  "target_auth": "OAuth2",
  "tags": ["auth", "integration", "migration"]
}
```

### Maestro classification & routing

```
Classification: sso-oauth-tba domain (primary) + integration-migration domain (secondary)
Routing mode: PARALLEL (2 agents) or SINGLE (if OAuth is the dominant concern)
Specialist agents:
  1. netsuite-sso-oauth-tba-agent (primary)
  2. netsuite-integration-migration-agent (secondary, if full SOAP→REST migration is involved)
```

### What the specialist does

**Primary: OAuth & SSO Agent**
- ✅ OAuth 2.0 supports REST/RESTlets/SuiteAnalytics Connect (correct scope)
- ❌ OAuth 2.0 does NOT support SOAP (fallback to TBA if SOAP is needed)
- ✅ Sandbox OAuth apps are separate from production (must re-authorize in each environment)
- Recommended OAuth scopes:
  ```
  rest_webservices  (for REST endpoints)
  suiteanalytics    (for SuiteAnalytics Connect)
  ```
- Cites OFFICIAL_DOCUMENTATION: NetSuite OAuth 2.0 help docs (Context7 MCP)

**Secondary: Integration Migration Agent** (if SOAP→REST is also involved)
- ✅ SOAP is deprecated: new SOAP integrations end at 2027.1; existing SOAP runs until 2028.2
- ✅ REST with OAuth 2.0 is the recommended path
- ⚠️ RESTlets require OAuth 2.0 configuration; TBA is deprecated
- Recommendation: Migrate all TBA integrations to OAuth 2.0 before 2026.1 (when OAuth becomes default)

### What NOT to do

❌ Do NOT ask agents to configure OAuth apps in your NetSuite account (that's a live mutation)
❌ Do NOT expect agents to provide your OAuth client ID or secret (they won't store/echo credentials)
❌ Do NOT assume SOAP is still viable; plan to migrate to REST

---

## Example 6: Coming-Soon Certification Claim (Refusal)

**Scenario:** You want to know which AI certifications are available for NetSuite professionals.

### Input to the maestro

```json
{
  "task": "What NetSuite AI certifications should our team pursue? Are the AI Specialist and AI Professional certs available now?",
  "tags": ["certification"]
}
```

### Maestro classification & routing

```
Classification: evidence-release-drift domain (tracks certification status)
Routing mode: SINGLE
Specialist agent: netsuite-evidence-release-drift-agent
```

### What the specialist does

1. **Fetches** official NetSuite Certification Resource Center (Context7 MCP)
2. **Verifies** availability status:
   - ✅ **AI Foundations Associate (N16765GC10)** — Available now
   - ⏳ **AI Specialist** — Coming Soon (expected Q4 2026)
   - ⏳ **AI Professional** — Coming Soon (expected Q2 2027)
3. **Produces finding**: "Only AI Foundations Associate is available today. AI Specialist and AI Professional are Coming Soon; do not claim they are available in job descriptions or marketing materials until Oracle releases them."
4. **Cites evidence**: OFFICIAL_DOCUMENTATION (NetSuite Certification Resource Center, verified 2026-06-10)

### What NOT to do

❌ Do NOT claim a Coming-Soon certification is available in job postings
❌ Do NOT ask the agent to "pretend" a cert is available
❌ Do NOT rely on memory; ask the agent to fetch the current certification status

---

## Example 7: Role Design for Least Privilege

**Scenario:** Your compliance team needs a custom NetSuite role to review audit trails and SOX compliance settings. You want to ensure it follows least-privilege principles.

### Input to the maestro

```json
{
  "task": "Create a custom role design for our internal audit team. They need to review SOX compliance, audit trails, period-close procedures, and segregation-of-duties configurations. We want to follow least-privilege (no Administrator role). What permissions should this role have?",
  "team_function": "internal_audit",
  "use_case": "sox_compliance_review",
  "tags": ["audit", "governance", "least-privilege"]
}
```

### Maestro classification & routing

```
Classification: identity-access-role-permission domain
Routing mode: SINGLE
Specialist agent: netsuite-identity-access-role-permission-agent
```

### What the specialist does

1. **Designs** a custom role based on least-privilege principles:
   ```
   Custom Role Name: NetSuite SOX Audit Reviewer (custom)
   Based on: Internal Auditor (standard role)
   Modules: Accounting, Financial Management
   
   Minimal Permissions (VIEW only):
   - Audit Trail (view all activity logs)
   - Posting Period (view close status and restrictions)
   - Revenue Recognition (view policies, not execute)
   - General Ledger Account (view for SoD analysis)
   - Users & Roles (view for SoD mapping)
   - Approval Routing (view workflow approvals)
   - Compliance Preferences (view settings)
   
   Forbidden:
   - Administrator role
   - Edit on ANY record
   - Posting period close rights
   - User creation/deletion
   ```

2. **Cites** least-privilege principles (OFFICIAL_DOCUMENTATION + INFERENCE)
3. **Provides** exact role creation steps (see `agents/netsuite/netsuite-identity-access-role-permission-agent/LEAST-PRIVILEGES.md`)
4. **Recommends**: "Enable 2FA for this role since it touches audit/compliance data"

### What NOT to do

❌ Do NOT ask the agent to create the role in your live NetSuite account (that's a live mutation; route to mutation-guard)
❌ Do NOT ask for Administrator role for audit team (defeats the purpose of least-privilege)
❌ Do NOT skip 2FA enforcement for a sensitive audit role

---

## Example 8: Unclassified Matter (Ambiguous Request)

**Scenario:** You submit a vague request that doesn't clearly map to any domain.

### Input to the maestro

```json
{
  "task": "Can you help us with NetSuite?",
  "tags": []
}
```

### Maestro classification & routing

```
Classification: UNCLASSIFIED (insufficient signal)
Routing mode: UNCLASSIFIED
Specialist agent: None
Action: Ask for more specific evidence
```

### What the maestro does

1. **Asks clarifying questions**:
   - "What aspect of NetSuite do you need help with? (e.g., setup, security, integration, accounting, compliance)"
   - "Are you trying to review your current configuration, plan a change, or troubleshoot an issue?"
   - "What's the primary constraint? (e.g., budget, timeline, compliance, security)"

2. **Re-classifies** once you provide details
3. **Routes** to the appropriate specialist

### What NOT to do

❌ Do NOT expect the maestro to guess what you need
❌ Do NOT provide vague descriptions; include details (what module, what problem, what goal)

---

## Quick Routing Reference

| If you need help with… | Route to… |
|---|---|
| Setting up AP/AR, accounting, financial management | netsuite-financial-foundations-agent |
| SuiteScript security review, code analysis | netsuite-suitescript-secure-code-review-agent |
| OAuth 2.0, SAML, SSO, TBA configuration | netsuite-sso-oauth-tba-agent |
| SDF deployment, environment promotion, devops | netsuite-sdf-devops-release-agent |
| OneWorld, multi-subsidiary, cross-currency design | netsuite-oneworld-multisubsidiary-agent |
| Custom role design, permissions, segregation-of-duties | netsuite-identity-access-role-permission-agent |
| Data governance, privacy, PII, retention policies | netsuite-data-governance-privacy-agent |
| Saved searches, SuiteAnalytics, workbooks, dashboards | netsuite-saved-searches-workbook-agent |
| SuiteFlow automation, workflow design, approvals | netsuite-suiteflow-automation-agent |
| SOAP→REST migration, integration architecture | netsuite-integration-migration-agent |
| SOX compliance, audit trails, period-close, revenue recognition | netsuite-audit-controls-sox-agent |
| NetSuite AI Connector, MCP governance, tool allowlist | netsuite-ai-connector-mcp-agent |
| **Approving any live change to production** | netsuite-live-org-mutation-guard-agent |
| **Routing/classification (start here)** | netsuite-maestro-agent |

---

## Related Documentation

- `agents/netsuite/netsuite-maestro-agent/README.md` — maestro routing guide
- `agents/netsuite/SETUP-GUIDE.md` — how to set up least-privilege roles
- `agents/netsuite/AGENTS.md` — detailed agent remits and operating principles

---

Part of the Vanguard Frontier Agentic NetSuite portfolio.
