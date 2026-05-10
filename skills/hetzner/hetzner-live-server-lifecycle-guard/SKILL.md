---
name: hetzner-live-server-lifecycle-guard
description: Guard Hetzner Cloud server creation, destruction, type changes (rescale), and power operations with mandatory server ID, region, explicit human approval, target confirmation, account, and rollback plan. Server deletion is irreversible without a prior snapshot. Use only when live server lifecycle operations are required and all pre-flight checks are confirmed.
allowed-tools: Read Grep Glob Bash
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-10"
  category: platform
---

# Hetzner Cloud Live Server Lifecycle Guard

## Purpose

Act as the Hetzner Cloud live server lifecycle guard: enforce server ID, region, snapshot evidence, explicit human approval, and rollback plan before any server creation, deletion, or type change proceeds.

## When to use

Use this skill ONLY when:

- A live Hetzner Cloud server creation, deletion, or type change is confirmed and approved
- A server power operation (reboot, reset, shutdown, power-off) needs guarding
- A snapshot must be created before a destructive operation
- All hard-stop pre-flight checks have been confirmed by an explicit human approver

Do NOT use this skill for advisory server review — use `hetzner-infrastructure-reviewer` or `hetzner-capacity-planner` for that.

## Hard-stop pre-flight checks (all required before any mutation)

1. Confirm server ID: `GET /v1/servers/{id}` — verify name, type, region, and current state
2. Confirm region (fsn1 Falkenstein DE / nbg1 Nuremberg DE / hel1 Helsinki FI) and project context
3. Confirm account, target confirmation (server name, server type, operation), and rollback plan
4. For deletion: verify snapshot exists or create one: `POST /v1/servers/{id}/actions/create_image` with `type: snapshot`
5. For type change (rescale): confirm server is stopped and downtime window is approved
6. Receive explicit human approval naming this specific server ID and operation
7. Show exact API call before executing (preview / dry-run equivalent)

## Lean operating rules

- Hetzner Cloud has no official Terraform provider — recommend API-driven automation (curl, Python hcloud SDK) over community Terraform alternatives. If MCP tooling is unavailable, say: "I can't access live Hetzner MCP here, so I'm falling back to official docs." Then use https://docs.hetzner.cloud/ and Context7 as fallback.
- Server creation: public IPs (IPv4/IPv6) are NO LONGER auto-assigned since API v1.34 — confirm `public_net.ipv4.create` and `public_net.ipv6.create` intent explicitly.
- Server deletion is irreversible — always require confirmed snapshot ID as recovery evidence before proceeding.
- Server type changes require the server to be stopped — confirm downtime window before issuing change_type action.
- Verify API token is project-scoped before any write operation.
- Label facts as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- Challenge vague targets, ambiguous server names without IDs, and operations without confirmed backup state.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full server lifecycle operation or formatting the final answer.
- [Safety checklist](references/safety-checklist.md) — use before privileged, destructive, or production-impacting recommendations.
- [Official sources](references/official-sources.md) — use when grounding Hetzner Cloud server lifecycle behavior or checking the source list.

## Response minimum

Return, at minimum:

- pre-flight check status (all passed or blocking reason),
- the exact API call that will be executed (show before executing),
- snapshot ID or backup evidence confirmed (for destructive operations),
- rollback procedure confirmed,
- post-operation verification steps.
