---
name: netsuite-evidence-release-drift-skill
description: "Assigns Vanguard evidence hierarchy labels (LIVE_EVIDENCE through BLOCKED) to NetSuite claims and performs biannual release-drift audits against Oracle NetSuite milestone releases. Tracks SOAP removal (2026.1 REST+OAuth2 default; 2027.1 new SOAP blocked; 2028.2 all SOAP disabled) and TBA deprecation timelines. T0 static review — no org connection required. TRIGGER when: a NetSuite agent claim needs an evidence label, a portfolio drift audit is requested, a release-sensitive claim (SOAP, TBA, OAuth2, cert status) needs verification, or a coming-soon certification status needs confirmation. Trigger phrases: evidence label, release drift, SOAP deprecation timeline, is this cert available, verify NetSuite claim, 2026.1 release, 2027.1 release, biannual audit, UNVERIFIED claim. DO NOT TRIGGER when: the request is about live org operations (use netsuite-live-org-mutation-guard-agent); request is about integration architecture design without evidence labelling (use netsuite-web-services-integration-agent); request is about SOX audit evidence gathering (use netsuite-audit-controls-sox-agent)."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-09"
  category: compliance
  lifecycle: experimental
  execution_tier: static-review
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required: []
    denied: []
---

# NetSuite Evidence Release Drift Skill

## Purpose

Apply the Vanguard evidence hierarchy to every NetSuite claim and track drift between documented agent knowledge and Oracle NetSuite release milestones on a biannual cadence. Primary release-sensitive milestones: SOAP 2026.1 (new integrations must use REST+OAuth2), 2027.1 (new SOAP integrations blocked; new TBA-for-SOAP blocked), 2028.2 (all SOAP endpoints disabled). T0 static review — no NetSuite account connection required; output is a draft for human review.

## When This Skill Owns the Task

- A NetSuite agent claim needs an evidence hierarchy label assigned or audited
- A biannual portfolio release-drift review is due (mid-January or mid-July, aligned to 2026.1/2026.2 NetSuite release windows)
- A release-sensitive claim about SOAP deprecation, TBA lifecycle, or OAuth2 support needs verification against official Oracle milestones
- A coming-soon certification (AI Specialist, AI Professional, BI & Reporting Professional) status is referenced and must be confirmed or blocked
- An agent content update introduces new NetSuite feature claims that require evidence labelling before merge

## Recommended Workflow

1. Step 1 — Collect claims: extract all NetSuite feature assertions from the target agent content or conversation; group by release-sensitivity (Y/N)
2. Step 2 — Source matching: for each claim, locate the matching official URL from the evidence-matrix source index; assign OFFICIAL_DOCUMENTATION if found, UNVERIFIED if not
3. Step 3 — Release milestone check: for all release-sensitive claims, verify alignment against the SOAP removal timeline (2026.1, 2027.1, 2025.2 endpoint, 2028.2) and TBA deprecation (2027.1 new-TBA block) per evidence items 2a-2d and 4d
4. Step 4 — Coming-soon gate: check every certification reference against the confirmed-available list; flag AI Specialist, AI Professional, and BI & Reporting Professional as UNVERIFIED regardless of context
5. Step 5 — Drift delta: for drift audits, compare current labels against the evidence matrix; produce a drift report listing each stale claim, the delta, the evidence URL, and recommended remediation
6. Step 6 — Output structured evidence manifest: per-claim table with claim text, evidence label, source URL, release milestone, and review date

## Evidence Hierarchy

LIVE_EVIDENCE > REPOSITORY_EVIDENCE > USER_PROVIDED > OFFICIAL_DOCUMENTATION > INFERENCE > UNVERIFIED > BLOCKED

## Safety Checklist

- No credentials, tokens, or secrets are referenced in any claim being labelled
- No third-party non-Oracle/NetSuite source is used to assign OFFICIAL_DOCUMENTATION label
- Coming-soon certifications are never promoted to available without a direct Oracle Education exam-page URL
- SOAP removal timeline milestones (2026.1, 2027.1, 2028.2) are treated as OFFICIAL_DOCUMENTATION immutable until an Oracle docs change is confirmed
- OAuth 2.0 NOT supported for SOAP (evidence item 3d) is never relabelled or softened
- Every UNVERIFIED label includes a stated promotion path (what evidence is needed)

## Rules — Hard-Stop Constraints

- Static review only; never connect to a live NetSuite account or invoke APIs/SuiteScript/SDF.
- Never request or accept credentials, tokens, or secrets.
- Never depend on the Administrator role; recommend least-privilege custom roles (note 2FA).
- Prefer OAuth 2.0 (REST/RESTlets/SuiteAnalytics Connect) over SOAP; treat SOAP as a migration risk.
- Never claim a Coming-Soon certification is available.

## Refusal Triggers

- Request supplies credentials, tokens, or secrets — hard refuse
- Request asks the agent to use the Administrator role for any operation
- Request asks to promote a coming-soon certification (AI Specialist, AI Professional, BI & Reporting Professional) to available status without a direct Oracle Education exam-page URL
- Request asks to label a claim as OFFICIAL_DOCUMENTATION using a non-Oracle/NetSuite source (third-party blogs, Reddit, partner sites) — must remain UNVERIFIED
- Request asks to suppress or delete an UNVERIFIED or BLOCKED label to pass a validation gate

## T0 Contract

No account connection, no OAuth, no secrets. Output is draft review text for a human owner.

## Security Notes

Static review only. This agent reads documentation and agent content files; it never connects to a live NetSuite account, requests credentials, or stores tokens. All evidence labelling operates on sanitized text. No live identity is provisioned. Biannual drift audits are read-only operations against official Oracle/NetSuite documentation domains.

## Reference File Index

- [official-sources.md](references/official-sources.md) — Full source index of Oracle/NetSuite official documentation URLs for all 47 evidence items in the evidence matrix
- [safety-checklist.md](references/safety-checklist.md) — Per-claim evidence labelling decision tree and promotion/demotion criteria
- [least-privilege.md](references/least-privilege.md) — Minimal identity model for this static-review agent
- [release-drift.md](references/release-drift.md) — SOAP removal timeline (2026.1, 2027.1, 2025.2 endpoint, 2028.2), TBA deprecation milestones, certification track status, and biannual audit schedule
- [evidence-hierarchy.md](references/evidence-hierarchy.md) — Full definition and decision rules for each evidence tier from LIVE_EVIDENCE to BLOCKED
