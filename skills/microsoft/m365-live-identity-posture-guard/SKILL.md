---
name: m365-live-identity-posture-guard
description: Live read-only Microsoft Entra identity and Conditional Access posture discovery — enumerate CA policies, MFA coverage gaps, privileged role assignments and PIM configuration, risky sign-ins, and stale guest accounts — then propose least-privilege hardening steps with blast-radius assessment and rollback plan. Phase A read-only-runtime only; no mutation.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: security
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes:
    - "Directory.Read.All"
    - "Policy.Read.All"
    - "Policy.Read.ConditionalAccess"
    - "RoleManagement.Read.Directory"
    - "AuditLog.Read.All"
  run_as_permissions:
    required:
      - "Directory.Read.All (application permission, admin-consented)"
      - "Policy.Read.All (application permission, admin-consented)"
      - "Policy.Read.ConditionalAccess (application permission, admin-consented)"
      - "RoleManagement.Read.Directory (application permission, admin-consented)"
      - "AuditLog.Read.All (application permission, admin-consented)"
    denied:
      - "Directory.ReadWrite.All"
      - "Policy.ReadWrite.ConditionalAccess"
      - "RoleManagement.ReadWrite.Directory"
      - "User.ReadWrite.All"
      - "Group.ReadWrite.All"
      - "Application.ReadWrite.All"
      - "Any *.Write or *.ReadWrite.* permission"
  required_egress:
    - "graph.microsoft.com"
    - "login.microsoftonline.com"
  requires_credentials:
    - "GRAPH_CLIENT_ID"
    - "GRAPH_TENANT_ID"
  output_attestation:
    schema: "posture-report-v1"
    signed_with: "none"
  companion_agents:
    - "m365-live-identity-posture-guard-agent"
---

# M365 Live Identity Posture Guard

## Purpose

Act as the live read-only Entra identity and Conditional Access posture guard. Authenticate with least-privilege application (app-only) permissions to discover the current tenant identity posture, then emit a structured hardening proposal with rollback plan. Never mutate; never request credentials values.

## When to use

- Entra Conditional Access policy coverage must be audited for gaps (legacy auth, MFA exemptions, excluded users)
- MFA registration and coverage needs to be measured across the tenant or a specific population
- Privileged role assignments (Global Administrator, Privileged Role Administrator, etc.) and PIM configuration must be reviewed
- Risky sign-in or risky user signals from Identity Protection need to be surfaced for triage
- Stale external/guest accounts need to be identified for lifecycle review

## Live-guard gate

This skill operates at `read-only-runtime`. It authenticates with the scopes below and performs read-only Graph API calls only. Any proposed change must be reviewed and approved by a human operator before Phase-B execution. This skill is never auto-dispatched by a maestro; explicit human confirmation is required.

## Credential posture

- App registration: use a certificate credential or managed identity — never a long-lived client secret.
- Credentials are referenced by environment variable name only (`GRAPH_CLIENT_ID`, `GRAPH_TENANT_ID`). Never print, echo, or log credential values.
- The app must be registered in the target tenant with the scopes above admin-consented by a Privileged Role Administrator.

## Lean operating rules

- Prefer Microsoft Learn documentation through the configured documentation MCP for Graph API and Entra service behavior.
- Use sampled read-only Graph evidence when available; label it as sampled configured-environment evidence.
- Do not execute any write, patch, post, or delete Graph call.
- If the request implies policy modification, role assignment, or user-state change, push back — that is Phase-B gated work.
- State what is unknown; documentation proves service behavior, not the tenant's deployed state.
- Load references only when needed; do not dump reference text into the response.

## Discovery targets

1. Conditional Access policies — enabled/report-only/disabled, assignments, excluded users, grant controls
2. MFA registration report — users not registered for strong authentication
3. Privileged role members — permanent vs eligible (PIM) for Global Administrator, Privileged Role Administrator, User Administrator, Application Administrator, Authentication Administrator
4. Risky users and risky sign-ins — current risk level and risk detail (requires Identity Protection P2)
5. Guest accounts — external member/guest users with last-sign-in date for staleness assessment

## Response minimum

- confirmed tenant ID and app identity (from token claims, not user input)
- discovery summary per target above
- hardening proposals: what, why, blast-radius, dependencies
- rollback contract for each proposal (Phase-B)
- open questions and evidence gaps

## Official sources

- https://learn.microsoft.com/graph/permissions-reference
- https://learn.microsoft.com/entra/identity-platform/app-only-access-primer
- https://learn.microsoft.com/entra/identity/conditional-access/concept-conditional-access-policies
- https://learn.microsoft.com/graph/api/resources/conditionalaccesspolicy
- https://learn.microsoft.com/entra/id-protection/concept-identity-protection-risks
