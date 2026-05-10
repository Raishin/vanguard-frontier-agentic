---
name: hetzner-live-firewall-rule-guard
description: Guard Hetzner Cloud Firewall rule mutations and server attachment changes with mandatory pre-mutation snapshot of current rules, blast-radius review, explicit human approval, target confirmation, account, region, and rollback plan. Use only when live Firewall rule changes are required and all pre-flight checks are confirmed.
allowed-tools: Read Grep Glob Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: security
---

# Hetzner Cloud Live Firewall Rule Guard

## Purpose

Act as the Hetzner Cloud live Firewall rule guard: enforce pre-mutation snapshot, blast-radius review, and rollback plan before any Firewall rule mutation or attachment change proceeds.

## When to use

Use this skill ONLY when:

- A live Hetzner Cloud Firewall rule mutation (add, update, delete inbound or outbound rule) is confirmed and approved
- A Firewall attachment to servers or Label groups needs to change
- A Firewall needs to be created or deleted
- All hard-stop pre-flight checks have been confirmed by an explicit human approver

Do NOT use this skill for advisory Firewall review — use `hetzner-infrastructure-reviewer` for that.

## Hard-stop pre-flight checks (all required before any mutation)

1. Snapshot current Firewall rules: `GET /v1/firewalls/{id}` — store as rollback evidence
2. Review blast-radius: `GET /v1/firewalls/{id}` resources field — list all attached servers and Label groups
3. Confirm target Firewall ID and project context
4. Confirm account, region (fsn1 / nbg1 / hel1), and rollback plan (rule revert procedure or Firewall detach)
5. Receive explicit human approval naming this specific Firewall and rule change

## Lean operating rules

- Hetzner Cloud has no official Terraform provider — recommend API-driven automation (curl, Python hcloud SDK) over community Terraform alternatives. If MCP tooling is unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.cloud/ and Context7 as fallback.
- Verify API token is project-scoped before any write operation.
- Public IPs are opt-in since API v1.34 — verify exposure before and after Firewall changes.
- Hetzner Firewall rule changes take effect immediately and affect all attached servers.
- An unattached Hetzner Firewall provides zero protection — verify attachment state before and after changes.
- Label facts as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- Challenge broad 0.0.0.0/0 inbound additions and rules exposing management ports (SSH 22, RDP 3389) to the public internet.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full Firewall mutation or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding Hetzner Cloud Firewall behavior or checking the source list.

## Response minimum

Return, at minimum:

- pre-flight check status (all passed or blocking reason),
- the exact API call that will be executed (show before executing),
- blast-radius summary (servers and Label groups affected),
- rollback procedure confirmed,
- post-change verification steps.
