# NetSuite Agent Ecosystem — Setup & Least-Privilege Role Configuration Guide

This guide walks you through deploying the NetSuite agent ecosystem and configuring least-privilege custom roles for every agent.

---

## Overview

The NetSuite agent ecosystem consists of:

- **1 Maestro (router)** — Classifies your matter and routes to specialists
- **1 Live-org mutation guard** — Gates all live-account changes
- **23 Specialist agents** — Domain-specific advisory roles (static review only)

Each specialist has a companion **LEAST-PRIVILEGES.md** file documenting:
- The recommended custom role to use
- Minimal permissions required
- Forbidden permissions
- Role creation steps

---

## Pre-requisites

1. **NetSuite account access** with Administrator or equivalent setup privileges
2. **Ability to create custom roles** in NetSuite
3. **Two-Factor Authentication (2FA)** configured on the account
4. **Sandbox environment** for testing custom roles before production deployment

---

## Phase 1: Understand the Architecture

### Static Review Only

All 25 agents are **static-review tier**. They:
- Never require a live NetSuite session token or API credentials
- Never hold authentication credentials
- Only review sanitized excerpts you provide
- Cannot execute live mutations

### Escalation Model

If a specialist recommends a live change (e.g., deploy to production, update a permission):
1. The specialist will route you to `netsuite-live-org-mutation-guard-agent`
2. The mutation guard requires **explicit named human approval**
3. Never bypass this gate — it is the primary firewall against unauthorized changes

### Evidence Hierarchy

Agents cite evidence at these levels:
- **LIVE_EVIDENCE** — Your own live account data (you provide)
- **REPOSITORY_EVIDENCE** — Your own GitHub/SDF code (you provide)
- **USER_PROVIDED** — Details you share
- **OFFICIAL_DOCUMENTATION** — NetSuite official sources (agents fetch via Context7 MCP)
- **INFERENCE** — Reasonable conclusions from official sources
- **UNVERIFIED** — Claims without strong source (agents will refuse)
- **BLOCKED** — Claims requiring credentials (agents will refuse)

Always ask agents to cite their evidence level. Prefer LIVE_EVIDENCE and OFFICIAL_DOCUMENTATION.

---

## Phase 2: Prepare Your Sandbox

Before creating custom roles in production, test them in a sandbox.

1. **Clone your production account to a sandbox** (or use an existing sandbox).
2. **Note the sandbox name** (e.g., "SB2").
3. **Refresh sandboxes** in NetSuite (`Setup → Sandbox Refresh`).
4. **Confirm OAuth apps are separate** — sandboxes have isolated OAuth authorization endpoints.

---

## Phase 3: Create Custom Roles

### Step 1: Open the role setup in your SANDBOX

1. Go to `Setup → Users/Roles → Manage Roles`
2. Click **+ New**

### Step 2: Choose your template

For each agent, the **LEAST-PRIVILEGES.md** file specifies a **standard role to copy from**. For example:

| Agent | Template Standard Role |
|---|---|
| Financial Foundations | Accountant |
| SuiteScript Secure Code Review | Developer |
| Identity Access Role Permission | Compliance Manager |
| SDF DevOps Release | System Administrator (read-only) |

Copy the standard role as your starting point.

### Step 3: Configure the role

For each agent:

1. Read the **LEAST-PRIVILEGES.md** file (e.g., `agents/netsuite/netsuite-financial-foundations-agent/LEAST-PRIVILEGES.md`)
2. **Recommended custom role name** — use the exact name specified (e.g., "NetSuite Financial Foundations Reviewer")
3. **Modules in scope** — see the LEAST-PRIVILEGES.md file for which modules apply
4. **Minimal permissions** — add ONLY the permissions listed under "Minimal permissions"
5. **Remove all others** — delete every permission not on the "Minimal permissions" list
6. **Forbidden** — ensure none of the "Forbidden" items are present
7. **Two-Factor Authentication** — enable if the agent touches privileged modules

### Step 4: Assign 2FA requirement

1. Scroll to **Authentication & Security**
2. Check **Require Two-Factor Authentication if Web Services access is enabled**
3. For sensitive roles (audit, SDF deploy, identity/access), also check **Require Two-Factor Authentication**

### Step 5: Save and test

1. Save the role in **Sandbox**
2. Assign it to a test user
3. Log in as that user and confirm:
   - You can access the permitted modules
   - You **cannot** access forbidden modules
   - No unexpected cross-module access is granted

### Step 6: Deploy to production

Once sandbox testing passes:

1. Navigate to `Setup → Users/Roles → Manage Roles` in **Production**
2. Repeat Steps 1–5 for production
3. **Document the role** in your own wiki/runbook with:
   - Role name
   - Date created
   - Agent it supports
   - Minimal permissions summary
   - Assigned users

---

## Phase 4: Inventory All Agent Roles

Below is a summary of all 25 agents and their recommended custom roles. Create one custom role per agent in your sandbox, then migrate to production.

### Layer 1: Governance & Routing (5 agents)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-maestro-agent | None (static only) | NetSuite Maestro Reviewer | None | Per policy |
| netsuite-live-org-mutation-guard-agent | System Administrator (read-only) | NetSuite Live Org Mutation Guard | All (read-only) | **YES** |
| netsuite-evidence-release-drift-agent | Analyst | NetSuite Evidence Release Drift Reviewer | Setup, Customization, SDF | No |
| netsuite-enterprise-architecture-agent | System Administrator (read-only) | NetSuite Enterprise Architecture Reviewer | All (read-only) | No |
| netsuite-audit-controls-sox-agent | Internal Auditor | NetSuite SOX Audit & Controls Reviewer | Accounting, Financial Mgmt, Audit | **YES** |

### Layer 2: Domain Specialists (20 agents)

#### Financial & Accounting (3)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-financial-foundations-agent | Accountant | NetSuite Financial Foundations Reviewer | AP, AR, Accounting | No |
| netsuite-bi-reporting-agent | Analyst | NetSuite BI & Reporting Reviewer | Reporting, Analytics, Dashboards | No |
| netsuite-erp-consultant-agent | Consultant | NetSuite ERP Implementation Reviewer | Inventory, Purchasing, Sales | No |

#### Development & Integration (5)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-application-developer-agent | Developer | NetSuite Application Developer Reviewer | SuiteScript, SuiteFlow, UIF | **YES** |
| netsuite-suitescript-secure-code-review-agent | Developer | NetSuite SuiteScript Security Reviewer | SuiteScript, Deployments, Scripts | **YES** |
| netsuite-suitecloud-developer-agent | Developer | NetSuite SuiteCloud Developer Reviewer | SDF, SuiteScript 2.x | **YES** |
| netsuite-sdf-devops-release-agent | System Administrator (read-only) | NetSuite SDF DevOps Release Reviewer | SDF, Deployments, Bundles | **YES** |
| netsuite-web-services-integration-agent | Integration | NetSuite Web Services Integration Reviewer | SuiteTalk, REST, SOAP, OAuth | **YES** |

#### Security, Identity & Access (4)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-identity-access-role-permission-agent | Compliance Manager | NetSuite Identity Access Reviewer | Users, Roles, Permissions, SoD | **YES** |
| netsuite-sso-oauth-tba-agent | System Administrator (read-only) | NetSuite OAuth & SSO Reviewer | Setup, Customization, Security | **YES** |
| netsuite-ai-connector-mcp-agent | System Administrator (read-only) | NetSuite AI Connector Reviewer | Setup, SuiteCloud Developers, Customization | **YES** |
| netsuite-data-governance-privacy-agent | Compliance Manager | NetSuite Data Governance Reviewer | Field Security, Audit Trail, Preferences | **YES** |

#### Operations & Governance (5)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-suitefoundation-agent | Analyst | NetSuite SuiteFoundation Reviewer | Setup, Customization, Basic Modules | No |
| netsuite-administrator-agent | System Administrator (read-only) | NetSuite Administrator Reviewer | All (read-only) | **YES** |
| netsuite-sandbox-nonproduction-governance-agent | System Administrator (read-only) | NetSuite Sandbox Governance Reviewer | Setup, Sandbox Admin | No |
| netsuite-suiteflow-automation-agent | Process Manager | NetSuite SuiteFlow Automation Reviewer | SuiteFlow, Workflows, Approvals | No |
| netsuite-saved-searches-workbook-agent | Analyst | NetSuite Saved Searches Reviewer | SuiteAnalytics, Reporting, Workbooks | No |

#### Cross-functional (2)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-oneworld-multisubsidiary-agent | System Administrator (read-only) | NetSuite OneWorld Subsidiary Reviewer | All (read-only), Multi-subsidiary | **YES** |
| netsuite-integration-migration-agent | Integration | NetSuite Integration Migration Reviewer | SuiteTalk, REST, SOAP, Migration Tools | **YES** |

#### Foundation & AI (2)

| Agent | Template | Custom Role Name | Key Modules | 2FA Required |
|---|---|---|---|---|
| netsuite-ai-foundations-agent | Analyst | NetSuite AI Foundations Reviewer | Setup, Customization, AI Features | No |

---

## Phase 5: Test Each Agent

For each custom role, verify it works correctly:

### Verification Checklist

For each agent:

1. ✅ Open `agents/netsuite/<agent-id>/LEAST-PRIVILEGES.md`
2. ✅ Copy the standard role specified → new custom role
3. ✅ Add only the minimal permissions listed
4. ✅ Confirm no forbidden permissions are present
5. ✅ Enable 2FA requirement if specified
6. ✅ Assign to a test user
7. ✅ Log in as that user, verify permissions work
8. ✅ Document the role in your internal runbook

### Test with Companion Skill

If the agent has a companion skill, also check:

1. ✅ Open `skills/netsuite/<agent-id>-skill/SKILL.md`
2. ✅ Review the `allowed-tools` field (least-privilege baseline)
3. ✅ Verify the skill is callable from the agent harness (Claude Code, Copilot, Codex, etc.)

---

## Phase 6: Monitor for Drift

After roles are created, periodically audit:

1. **Permission creep** — Did anyone add unnecessary permissions? Run `Setup → Users/Roles → Manage Roles` and spot-check.
2. **Assignment drift** — Did unauthorized users get assigned to sensitive roles? Check `Setup → Users → Manage Users`.
3. **2FA compliance** — Are all 2FA-required roles actually enforcing it? Check `Setup → Authentication → Two-Factor Authentication`.

---

## Refusal Triggers (all agents)

All agents enforce these refusals:

- ❌ **Credentials, tokens, session cookies** — agents will refuse and not log/echo them
- ❌ **Administrator role as a dependency** — agents will cite least-privilege principle
- ❌ **Direct execution of live mutations** — agents will route to live-org-mutation-guard
- ❌ **Coming-soon certifications** — agents will refuse claims like "AI Specialist is available" (it is not; only AI Foundations Associate is available)
- ❌ **PII (SSN, credit card, bank account numbers)** — agents will refuse and ask for sanitization

---

## Quick Start: Three-Role Deployment

If you want to start small, deploy these three critical roles first:

1. **NetSuite Maestro Reviewer** — for classifying matters (no NetSuite permissions required)
2. **NetSuite Live Org Mutation Guard** — for approving live changes (System Administrator read-only, 2FA required)
3. **NetSuite Financial Foundations Reviewer** — for accounting/AP/AR review (Accountant template)

Once those three are working, expand to the full 25-agent portfolio at your own pace.

---

## Troubleshooting

### "Agent refuses to proceed with missing credentials"

**Cause:** You may have included credentials (OAuth tokens, session cookies, etc.) in your request.

**Fix:** Sanitize your input. Remove all authentication material and resubmit.

---

### "Agent says the role doesn't have permission to view this module"

**Cause:** The custom role you created is missing a permission listed in LEAST-PRIVILEGES.md.

**Fix:**
1. Open `agents/netsuite/<agent-id>/LEAST-PRIVILEGES.md`
2. Check the "Minimal permissions" section
3. In NetSuite, edit the custom role and add the missing permission at the stated access level (View, Create, Full, etc.)
4. Save and re-test

---

### "Agent requests the Administrator role"

**Cause:** Some tasks genuinely require Administrator access, but agents are forbidden from using it.

**Fix:**
1. Use `netsuite-live-org-mutation-guard-agent` for Administrator-tier changes
2. Or create a new custom role with only the minimum permissions needed (not Administrator)
3. Never enable Administrator unless absolutely unavoidable, and do so only in a controlled, audited way

---

### "Agent mentions a certification as 'available' but I think it's Coming Soon"

**Cause:** Agent may not have fetched the latest NetSuite certification catalog.

**Fix:**
1. Tell the agent: "Please verify in the official NetSuite Certification Resource Center that this cert is available (not Coming Soon)."
2. Agents will use Context7 MCP to fetch current documentation.

---

## Support & Escalation

- **Questions about agent behavior?** Open an issue in the repository.
- **Found a least-privilege gap?** Check the agent's LEAST-PRIVILEGES.md file, then propose a fix.
- **Need to add a new agent?** Follow the pattern in an existing agent's LEAST-PRIVILEGES.md file.

---

## Related Documentation

- `agents/netsuite/README.md` — overview of all 25 agents
- `agents/netsuite/AGENTS.md` — detailed agent remits and operating principles
- `agents/netsuite/netsuite-maestro-agent/README.md` — maestro routing guide with examples
- `skills/netsuite/README.md` — skill portfolio overview
- `catalog/install-roles.json` — which agents are available in each practitioner role

---

Part of the Vanguard Frontier Agentic NetSuite portfolio.
