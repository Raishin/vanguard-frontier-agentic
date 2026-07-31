---
name: netsuite-live-operation-safety-skill
description: "Evaluates live NetSuite mutation requests against a structured authorization checklist covering blast-radius, rollback, human decision ownership, and integration posture. T0 static evaluation — no org connection required. TRIGGER when: a request involves activating a workflow, deploying an SDF project, editing live records, publishing a saved search to new roles, changing permissions, rotating OAuth certificates, issuing or revoking TBA tokens, or any other operation that writes to or configures a live NetSuite account. Trigger phrases: deploy to production, activate workflow, change permissions in NetSuite, rotate cert, publish saved search, edit live record, SDF deploy, grant role. DO NOT TRIGGER when: the request is purely a static design review with no live-op intent (use the appropriate domain specialist); request is about reading or querying live data without mutation (use netsuite-saved-searches-workbook-agent or netsuite-bi-reporting-agent); request is about architecture design only (use netsuite-enterprise-architecture-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-09"
  category: security
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Live Operation Safety Skill

## Purpose

Act as the mandatory approval gate for all live-org mutation paths in the NetSuite domain. Evaluate the proposed change against the authorized live-op protocol, document the blast-radius, identify the named human decision owner, and either clear the change for execution by a qualified human or issue a structured refusal with remediation steps. T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- User presents a change request targeting a live NetSuite production, sandbox, or release-preview account
- SDF deploy checklist review is needed before a human executes the deploy
- Workflow activation or deactivation in any NetSuite environment requires authorization posture review
- Permission or role assignment change requires blast-radius and SoD pre-check
- OAuth 2.0 certificate rotation or TBA token lifecycle event requires protocol verification

## Recommended Workflow

1. Step 1 — Extract change metadata: target environment tier, change type, named human decision owner, ticket/protocol reference
2. Step 2 — Blast-radius mapping: identify affected subsidiaries, integrations, roles, and record types
3. Step 3 — Authorization posture check: verify authorized live-op protocol is present and complete; default to refusal if any required field is absent
4. Step 4 — Least-privilege verification: confirm the change does not require or grant Administrator role; check 2FA trigger permissions per evidence item 5c
5. Step 5 — Rollback plan validation: confirm a documented rollback path exists and a named rollback owner is identified
6. Step 6 — Integration posture check: flag any SOAP-based change as migration-risk per evidence items 2a-2d; flag new TBA-for-SOAP post-2027.1 per evidence item 4d
7. Step 7 — Emit structured clearance or refusal with all required fields and remediation steps for any refusal

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or secrets were supplied in the request input
- Administrator role is not required by the proposed change
- Authorized live-op protocol reference is present and cites a named human decision owner
- Rollback plan is documented and a rollback owner is named
- For SDF deploys: sandbox validation evidence is present
- For OAuth/TBA changes: confirms REST+OAuth2 path (not new SOAP TBA post-2027.1)
- For permission changes: target role is a custom copy of a standard role, not Administrator
- For AI Connector changes: confirms account is not a healthcare BAA-restricted account

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request supplies credentials, tokens, OAuth client secrets, TBA token values, or session cookies — hard refuse, do not echo or log
- Request asks for or implies use of the Administrator role for any automated or scripted operation
- No authorized live-op protocol or change-management ticket reference is present
- No named human decision owner is identified
- No rollback plan is provided for production-bound changes
- Request proposes building a new SOAP integration after the 2026.1 release (REST+OAuth2 is required for new builds per evidence item 2a)
- Request proposes new TBA for SOAP, REST, or RESTlets after 2027.1 (hard block per evidence item 4d)
- Proposed change would grant permissions that mandate 2FA (Access Token Management, OAuth 2.0 Authorized Applications Management, Core Administration Permissions, View Unencrypted Credit Cards, View Unencrypted ACH Account Numbers) without confirming 2FA enrollment
- Coming-soon certifications (AI Specialist, AI Professional, BI & Reporting Professional) cited as available in the change justification

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only. The live guard never executes mutations in NetSuite. It operates from sanitized text inputs and never requests, stores, echoes, or logs credentials, OAuth tokens, TBA token values, client secrets, or session cookies. Default posture is refusal absent a fully documented authorized live-op protocol. All clearances require a named human decision owner and a documented rollback path.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Oracle/NetSuite official documentation URLs for authentication, role management, SOAP removal plans, and 2FA requirements
- [safety-checklist.md](references/safety-checklist.md) — Expanded live-op authorization checklist with per-change-type decision trees
- [least-privilege.md](references/least-privilege.md) — Custom role construction guidance and forbidden permission enumeration for live-guard posture
- [release-drift.md](references/release-drift.md) — SOAP removal timeline (2026.1, 2027.1, 2028.2) and TBA deprecation milestones for integration posture checks
- [blast-radius-guide.md](references/blast-radius-guide.md) — Blast-radius assessment framework for multi-subsidiary and multi-integration change scopes
