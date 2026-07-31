---
name: d365-live-security-role-guard
description: Live read-only Dataverse security posture discovery — enumerate security roles, team and business-unit assignments, application users, over-privileged System Administrator assignments, and SoD-relevant privilege combinations — then propose least-privilege role design with rollback plan. Phase A read-only-runtime only; no mutation. Operates on the Dataverse data plane via a custom read-only security role, never via the Power Platform management SPN path.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-17"
  category: security
  execution_tier: read-only-runtime
  mcp_servers: []
  oauth_scopes: []
  run_as_permissions:
    required:
      - "Custom read-only Dataverse security role (Read on in-scope tables only: systemuser, role, roleprivileges, team, businessunit, systemuserroles)"
      - "Application user (SystemUser row) bound to the custom read-only role — NOT System Administrator, NOT System Customizer"
      - "Dataverse data-plane access via S2S application user (ApplicationId/AzureActiveDirectoryObjectId on SystemUser)"
    denied:
      - "System Administrator"
      - "System Customizer"
      - "Create privilege on any table"
      - "Write privilege on any table"
      - "Delete privilege on any table"
      - "Append privilege on any table"
      - "AppendTo privilege on any table"
      - "prvActOnBehalfOfAnotherUser"
      - "Power Platform management SPN path (pac admin create-service-principal — cannot be least-privileged)"
  required_egress:
    - "*.dynamics.com"
    - "login.microsoftonline.com"
  requires_credentials:
    - "DATAVERSE_CLIENT_ID"
    - "DATAVERSE_ENV_URL"
  output_attestation:
    schema: "posture-report-v1"
    signed_with: "none"
  companion_agents:
    - "d365-live-security-role-guard-agent"
---

# D365 Live Security Role Guard

## Purpose

Act as the live read-only Dataverse security posture guard. Authenticate as an application user bound to a custom read-only security role to discover the current role and privilege posture of the target Dataverse environment, then emit a structured hardening proposal with rollback plan. Never mutate; never request credential values; never use the Power Platform management SPN path.

## When to use

- Dataverse security roles must be audited for over-privileged assignments (System Administrator spread, overly broad table privileges)
- Team-based and business-unit-based role assignments need to be reviewed for SoD violations
- Application users must be enumerated to verify each is bound to a least-privilege custom role, not System Administrator
- SoD-relevant privilege combinations (e.g. Read + Export across sensitive tables) need to be identified
- A role-design proposal is needed before a formal security review or compliance audit

## Live-guard gate

This skill operates at `read-only-runtime`. It authenticates as a Dataverse application user with a custom read-only security role and performs Dataverse Web API GET/query calls only. Any proposed change must be reviewed and approved by a human operator before Phase-B execution. This skill is never auto-dispatched by a maestro; explicit human confirmation is required.

## Critical IAM constraint

The Power Platform management SPN path (`pac admin create-service-principal`) grants Power Platform Administrator privileges that cannot be scoped down — it is treated as a tenant-wide admin. This skill explicitly forbids that path. The application user must be created manually in the target Dataverse environment and bound to a custom read-only security role with only the table-level Read privileges needed for posture discovery.

## Credential posture

- App registration: use a certificate credential or managed identity — never a long-lived client secret.
- Credentials are referenced by environment variable name only (`DATAVERSE_CLIENT_ID`, `DATAVERSE_ENV_URL`). Never print, echo, or log credential values.
- The application user must be created in the target Dataverse environment and associated with the custom read-only security role before this skill runs.

## Lean operating rules

- Prefer Microsoft Learn documentation through the configured documentation MCP for Dataverse and Power Platform service behavior.
- Use sampled read-only Dataverse Web API evidence when available; label it as sampled configured-environment evidence.
- Do not execute any POST, PATCH, PUT, or DELETE Dataverse Web API call.
- If the request implies role assignment, role creation, or user modification, push back — that is Phase-B gated work.
- State what is unknown; documentation proves service behavior, not the environment's deployed state.

## Discovery targets

1. Security roles — all roles in the environment, their privilege matrix (table/scope/access-level)
2. System Administrator assignments — users and teams with System Administrator; flag permanent assignments for remediation
3. Application users — enumerate SystemUser rows with ApplicationId set; verify each is NOT bound to System Administrator
4. Team role assignments — owner teams and access teams with broad roles
5. Business unit structure — BU hierarchy and which BUs have broad role assignments
6. SoD-relevant privilege combos — Read + Export on Finance/HR/PII tables; Append-To on sensitive tables

## Response minimum

- confirmed environment URL and application user identity
- discovery summary per target above
- hardening proposals: what, why, blast-radius, affected users/teams
- rollback contract for each proposal (Phase-B)
- open questions and evidence gaps

## Official sources

- https://learn.microsoft.com/power-apps/developer/data-platform/use-multi-tenant-server-server-authentication
- https://learn.microsoft.com/power-platform/admin/database-security
- https://learn.microsoft.com/power-apps/developer/data-platform/build-web-applications-server-server-s2s-authentication
- https://learn.microsoft.com/power-platform/admin/powerplatform-api-create-service-principal
- https://learn.microsoft.com/azure/azure-sovereign-clouds/public/access-controls-dataverse-power-platform
