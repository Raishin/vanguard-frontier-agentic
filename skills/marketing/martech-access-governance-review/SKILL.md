---
name: martech-access-governance-review
description: Use this skill when reviewing access governance across a marketing technology stack — OAuth grants and connected apps, API keys and tokens, CRM and marketing-automation role assignments, and integration scopes. Trigger when a user provides an OAuth connected-app inventory, an integration scope list, a CRM/MAP role matrix, an API-key inventory, or asks whether their martech integrations are over-permissioned, whether stale connectors still hold live tokens, or how to apply least privilege to their marketing stack.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: security
  lifecycle: experimental
---

# Martech Access Governance Review

## Purpose
This skill reviews identity and access governance across a marketing technology stack — the CRM, marketing automation platform, CDP, analytics, and the long tail of connected SaaS apps. Marketing operations accumulate OAuth grants, API keys, and seat permissions faster than any other business function, and rarely deprovision them. The result is a stack where third-party connectors hold full-CRM scopes, a single shared admin key authenticates a dozen tools, and a contractor's connected app still has a live refresh token a year after the engagement ended. This is one of the most exploited SaaS breach paths: the marketing stack holds the entire customer database and is governed loosely. The review catches over-broad OAuth scopes, shared and non-rotating credentials, stale grants, missing token expiry, and absent ownership before they become an incident.

## Lean operating rules
- Treat any third-party connected app granted a full-read or read-write scope over the entire CRM contact/lead database when its function needs a narrow scope as HIGH — over-broad scope is the blast radius if that vendor is breached.
- Treat a single API key or service account shared across multiple tools or integrations as HIGH — it cannot be rotated or revoked without an outage, and a leak compromises every consumer.
- Treat any long-lived API key or OAuth grant with no rotation schedule and no expiry as HIGH — a leaked non-expiring credential is valid until someone notices.
- Treat a connected app, integration, or token tied to a departed employee, ended vendor engagement, or decommissioned tool that still holds a live grant as HIGH — stale credentials are unattended attack surface.
- Treat a CRM or marketing-automation integration credentialed with an admin or owner role when an API-only or limited integration role exists as HIGH — privilege beyond function violates least privilege.
- Treat a connected app or API key with no named human or team owner as HIGH — unowned credentials are never reviewed and never revoked.
- Flag marketing user seats holding bulk-export or full-database-export permission beyond the few who need it as MEDIUM — bulk export is the exfiltration path.
- Flag the absence of a recurring access-review cadence for connected apps and integration credentials as MEDIUM.
- Flag OAuth grants that include offline-access / refresh-token scope where only short-lived interactive access is needed as MEDIUM.
- Flag credentials transmitted or stored in plaintext (in tag managers, spreadsheets, shared docs, or automation tools) as HIGH.
- Do not recommend revoking a grant without naming the integration it powers and the marketing workflow that breaks.
- Label every finding with evidence basis: inventory provided, role matrix provided, documentation-based, or inference from missing config.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- OAuth scope blast-radius findings (connected apps over-scoped vs function)
- Shared / non-rotating credential findings
- Stale grant findings (departed users, ended vendors, dead tools)
- Integration role assessment (admin used where limited role exists)
- Ownership and access-review cadence gaps
- Bulk-export permission distribution assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
