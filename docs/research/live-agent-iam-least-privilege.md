# Live-Agent IAM Least-Privilege — M365 & D365

> Research grounding for **phased live agents** on the Microsoft board. Decision (locked):
> **Phase A = `read-only-runtime` live-guard agents now** (least-privilege READ, discover +
> propose, mutation behind explicit human approval); **Phase B = `mutating-runtime`** in a
> later separately-hardened wave. This report fixes the IAM contract so the agents are
> Fortune-50-defensible.
>
> **Date:** 2026-06-17 · Evidence: E3 official Microsoft docs · Confidence H/M/L.

---

## 1. The non-negotiable design rule

An agent that holds **standing write credentials** to a production M365 tenant or D365
environment is the exact anti-pattern this board's `entra`/`purview`/`sod` agents exist to
catch. Therefore:

- **Phase A agents are `read-only-runtime`**: they authenticate with **least-privilege READ
  scopes only**, discover posture, and **emit a proposed change + rollback plan** — they do
  **not** mutate.
- **Mutation is a separate, gated act** (Phase B `mutating-runtime`): human approval,
  blast-radius, rollback, signed attestation, and the maestro **live-guard gate** (never
  auto-dispatched).

The repo schema already encodes this: `execution_tier`, `oauth_scopes`,
`run_as_permissions.{required,denied}`, `required_egress`, `requires_credentials`,
`output_attestation`, `eval_fixtures` (skill frontmatter `liveAgentFields`).

---

## 2. M365 (Microsoft Graph) — least-privilege IAM

| Claim | Evidence | Conf |
|---|---|---|
| Graph has **delegated** (act as signed-in user) vs **application/app-only** (no user, daemon) permissions; background agents use **application permissions + tenant admin consent**. | E3 — [permissions overview](https://learn.microsoft.com/graph/permissions-overview), [app-only access primer](https://learn.microsoft.com/entra/identity-platform/app-only-access-primer) | H |
| **Least privilege is explicit**: request the narrowest scope — e.g. `User.ReadBasic.All` instead of `User.Read.All`; separate `*.Read` vs `*.ReadWrite` exist *so* apps follow Zero Trust least privilege. | E3 — [app-only primer](https://learn.microsoft.com/entra/identity-platform/app-only-access-primer#authorizing-an-app-to-make-application-only-calls), [protected API example](https://learn.microsoft.com/security/zero-trust/develop/protected-api-example) | H |
| App-only permissions are **static** (declared on the app registration), require **admin consent**, and granting Graph app roles needs **Privileged Role Administrator**. | E3 — [consent-types](https://learn.microsoft.com/entra/identity-platform/consent-types-developer), [permissions-consent-overview](https://learn.microsoft.com/entra/identity-platform/permissions-consent-overview) | H |
| Read-only discovery scopes for a live-guard exist as dedicated read app-roles, e.g. `Directory.Read.All`, `Policy.Read.All`, `Policy.Read.ConditionalAccess`, `User.Read.All`, `Group.Read.All`, `AuditLog.Read.All`, `Reports.Read.All`, `SecurityEvents.Read.All`. | E3 — [permissions reference](https://learn.microsoft.com/graph/permissions-reference) | H |
| The app should itself be governed: **Conditional Access for workload identities**, credential = certificate or **managed identity** (not a long-lived client secret), and the app's secret rotation is an operational burden to minimize. | E3 — [Conditional Access target resources](https://learn.microsoft.com/entra/identity/conditional-access/concept-conditional-access-cloud-apps), [app registration least privilege](https://learn.microsoft.com/power-platform/guidance/alm-accelerator/app-registration-strategy) | M |

**Live-guard M365 agent IAM contract (Phase A):**
- `execution_tier: read-only-runtime`
- `oauth_scopes` (application, admin-consented, READ-only): `Directory.Read.All`, `Policy.Read.All`, `AuditLog.Read.All` (scope per agent domain; identity-guard adds `Policy.Read.ConditionalAccess`, `RoleManagement.Read.Directory`).
- `run_as_permissions.denied`: every `*.ReadWrite*` / `*.Write` / `Directory.ReadWrite.All` / `RoleManagement.ReadWrite.Directory`.
- `requires_credentials`: env-var names only (e.g. `GRAPH_CLIENT_ID`, `GRAPH_TENANT_ID`); **never** secrets in repo/chat; prefer certificate/managed identity.
- `required_egress`: `graph.microsoft.com`, `login.microsoftonline.com`.
- `output_attestation`: signed verdict; `mcp_servers`: read-only Graph MCP if configured.

---

## 3. D365 / Dataverse — least-privilege IAM (the sharp edge)

| Claim | Evidence | Conf |
|---|---|---|
| Dataverse **data-plane** access for an app is via an **application user** (SystemUser row linked by `ApplicationId`/`AzureActiveDirectoryObjectId`) that **must be bound to a security role** defining least-privilege row/field/table access. Create a **custom least-privilege (read-only) security role** — **not System Administrator**. | E3 — [multi-tenant S2S auth: create application user + security role](https://learn.microsoft.com/power-apps/developer/data-platform/use-multi-tenant-server-server-authentication), [role-based security](https://learn.microsoft.com/power-platform/admin/database-security) | H |
| Dataverse RBAC is fine-grained (environment/role/table/row/column, field-level security); predefined roles follow "minimum required access"; **minimize System Administrator assignments**. | E3 — [access controls for Dataverse](https://learn.microsoft.com/azure/azure-sovereign-clouds/public/access-controls-dataverse-power-platform), [database-security](https://learn.microsoft.com/power-platform/admin/database-security) | H |
| **⚠️ Management-plane caveat:** a service principal registered via `pac admin create-service-principal` is treated **like a user with the Power Platform Administrator role — granular roles can't be assigned to limit it**. So admin/management APIs are inherently coarse-privileged. | E3 — [create service principal (PAC CLI) — limitations](https://learn.microsoft.com/power-platform/admin/powerplatform-api-create-service-principal#limitations-of-service-principals) | H |

**Design consequence (ruthless):** a D365 live-guard agent must work on the **Dataverse data plane** with a **custom read-only security role** (least privilege achievable), and must **avoid the Power-Platform-management SPN path** for posture review because it cannot be least-privileged. If management-plane data is unavoidable, that is a **Phase-B, human-operated, gated** action — never a standing agent credential.

**Live-guard D365 agent IAM contract (Phase A):**
- `execution_tier: read-only-runtime`
- `run_as_permissions.required`: a named **custom read-only Dataverse security role** (Read on the in-scope tables only; no Create/Write/Delete/Append; no System Administrator/System Customizer).
- `run_as_permissions.denied`: `System Administrator`, `System Customizer`, Write/Create/Delete/Append-To, `prvActOnBehalfOfAnotherUser`.
- `requires_credentials`: `DATAVERSE_CLIENT_ID`, `DATAVERSE_ENV_URL` (env-var names only).
- `required_egress`: `*.dynamics.com`, `login.microsoftonline.com`.

---

## 4. Mandatory per-agent artifacts (matches repo live-guard agents)

Each live agent dir ships, in addition to `AGENT.md` + `metadata.json` + 7 harness adapters:
- **`PERMISSIONS.md`** — exact least-privilege scope/role set (required + denied), credential posture (cert/managed identity), egress allow-list.
- **`PREFLIGHT.md`** — checks before any run (scope verification, environment confirmation, read-only assertion).
- **`ROLLBACK.md`** — for Phase-B mutations: rollback steps, owner, trigger, time-box. For Phase A: explicitly "no mutation; nothing to roll back."
- Maestro registration as a **live-guard** (in `tests/fixtures/microsoft-maestro-routing/taxonomy.json` `live_guards`), so the maestro **never auto-dispatches** it (gate mode + explicit confirmation).

## 5. Verification debt

- Confirm the exact current Graph app-role identifiers per agent domain against [permissions-reference](https://learn.microsoft.com/graph/permissions-reference) at build time.
- Confirm Dataverse custom-role privilege names against the security-roles reference before stamping `run_as_permissions`.
- All scopes/roles are **declarations for review**; this repo ships specs, not live credentials.

## Sources

- https://learn.microsoft.com/graph/permissions-overview
- https://learn.microsoft.com/entra/identity-platform/app-only-access-primer
- https://learn.microsoft.com/security/zero-trust/develop/protected-api-example
- https://learn.microsoft.com/entra/identity-platform/permissions-consent-overview
- https://learn.microsoft.com/graph/permissions-reference
- https://learn.microsoft.com/power-apps/developer/data-platform/use-multi-tenant-server-server-authentication
- https://learn.microsoft.com/power-platform/admin/database-security
- https://learn.microsoft.com/power-platform/admin/powerplatform-api-create-service-principal
- https://learn.microsoft.com/azure/azure-sovereign-clouds/public/access-controls-dataverse-power-platform
