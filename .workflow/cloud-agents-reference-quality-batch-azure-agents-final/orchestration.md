# Azure agents reference-quality final singleton orchestration

Date: 2026-06-05
Provider: Azure
Asset type: agents
Batch size: 1

## Target

1. agents/azure/azure-waf-security-review-agent

## Why singleton

The sorted Azure agent set had one remaining unprocessed Azure agent after exact five-item batches. Processing it as a narrowed final singleton prevents the exact-five rule from deadlocking completion of Azure agent alignment.

## Evidence discipline

- Used Microsoft Learn documentation through the user configured documentation MCP for documented Azure service behavior.
- Treated documentation as documentation-based evidence only; it does not prove deployed tenant, subscription, RBAC, quotas, resources, costs, incidents, compliance, or production readiness.
- No AWS assets were intentionally changed.
- No committed docs should mention private tool labels, workstation aliases, connection handles, or environment-specific identifiers.

## AgentCore-style alignment

- Matched the AgentCore reference-pack pattern: component operations reference, official sources, safety checklist, workflow/output, and documentation evidence discipline.
- Removed stale framing that claimed eight WAF Security principles; current Microsoft Learn evidence frames Security around Zero Trust, CIA, and five Security pillar learning areas.
