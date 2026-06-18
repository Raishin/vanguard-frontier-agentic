# ❄️ Snowflake (Azure) Agents

## Overview

Two **static-review** agents and one **mutating-runtime live-guard** agent for Snowflake workloads deployed on Azure — scoped to RBAC access governance, data-platform engineering, and controlled privilege grants. The static-review agents never connect to live Snowflake accounts; every verdict is an evidence-backed recommendation requiring human approval before any change reaches a production environment. The Phase B live-guard agent executes a single, narrowly scoped RBAC GRANT against a live Snowflake account only after written approval token, PREFLIGHT dry-run, and REVOKE rollback path are confirmed.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Snowflake mutation |
|---|---|---|---|
| Static-review agents | Review, design, diagnose | read-only | not allowed |
| Live-guard (mutating-runtime) | Apply one RBAC GRANT (role/privilege) to one grantee; REVOKE rollback | written approval token + PREFLIGHT dry-run required | single scoped GRANT only — ACCOUNTADMIN grants, SECURITYADMIN/SYSADMIN escalation, and bulk operations denied |

## 🗂️ Agents in this provider

| Agent | Tier | Primary use |
|---|---|---|
| `snowflake-rbac-access-governance-at-azure-agent` | Static-review | RBAC governance review — ACCOUNTADMIN/SECURITYADMIN/SYSADMIN role separation, custom least-privilege roles, SoD enforcement, network policy review, Entra OAuth/SSO/SCIM integration posture |
| `snowflake-data-platform-engineering-at-azure-agent` | Static-review | Data platform architecture review — warehouse sizing, Azure Private Link (Business Critical), storage integration to ADLS Gen2/Blob, dynamic data masking, row-access policies, object tagging, ACCESS_HISTORY auditing |
| `snowflake-live-rbac-grant-guard-at-azure-agent` | Live-guard (mutating-runtime) | Apply one RBAC GRANT (role/privilege) to one grantee via SQL API; REVOKE rollback; written approval token + PREFLIGHT diff required; denies ACCOUNTADMIN grants, SECURITYADMIN/SYSADMIN escalation, and bulk operations |

## 🔒 RBAC access governance agent

The `snowflake-rbac-access-governance-at-azure-agent` reviews Snowflake access configurations for:

- **Role hierarchy:** ACCOUNTADMIN, SECURITYADMIN, and SYSADMIN kept separate; no single user holds all three; no production workloads run as ACCOUNTADMIN
- **Least-privilege custom roles:** object-level privileges granted on custom roles, not on PUBLIC or ACCOUNTADMIN; USAGE + SELECT separated from INSERT/UPDATE/DELETE/TRUNCATE
- **Segregation of duties (SoD):** role grants reviewed for incompatible privilege combinations; no single role can both create objects and approve production deployments
- **PUBLIC role restriction:** no sensitive data objects granted to PUBLIC; PUBLIC role privilege inventory reviewed
- **Network policies:** IP allowlists enforced at account and user level; MFA enforcement confirmed for privileged users
- **Identity federation:** Entra ID OAuth 2.0 / SSO / SCIM provisioning reviewed; service account OAuth client credential flows verified; no password-based authentication for production service accounts

## 🏗️ Data platform engineering agent

The `snowflake-data-platform-engineering-at-azure-agent` reviews Snowflake data platform architectures for:

- **Warehouse governance:** virtual warehouse sizing policies; auto-suspend and auto-resume configured; no always-on X-Large warehouses without cost-governance justification
- **Azure Private Link:** Private Link endpoint confirmed for Business Critical edition; public endpoint disabled where Private Link is in use; VNet service endpoint vs. Private Link distinction clarified
- **Storage integration:** external stage definitions use storage integration objects (not inline credentials); ADLS Gen2 and Azure Blob Storage integrations reviewed for least-privilege managed identity or SAS token scope
- **Dynamic data masking:** masking policies applied to columns containing PII/PHI; policy ownership separated from data ownership; conditional masking for role-based reveal reviewed
- **Row-access policies:** row-level security policies reviewed for performance impact and bypass risk; policy ownership and grant chain audited
- **Object tagging:** tag-based classification aligned to data sensitivity taxonomy; tag inheritance reviewed across databases, schemas, and tables
- **ACCESS_HISTORY:** `SNOWFLAKE.ACCOUNT_USAGE.ACCESS_HISTORY` view coverage confirmed; query history retention reviewed for audit and forensic requirements

## 🔐 RBAC grant guard agent (live-guard — mutating-runtime, Phase B)

The `snowflake-live-rbac-grant-guard-at-azure-agent` is a **controlled WRITE** agent that applies exactly one RBAC GRANT (a role-to-role grant or an object privilege to one role/user) to exactly one grantee on a live Snowflake account. It is a Phase B mutating-runtime guard — distinct from the Phase A static-review agents above, which never connect to a live account.

**Execution conditions — all must be met before any GRANT is issued:**
- Written approval token provided by an authorized human approver
- PREFLIGHT dry-run (`SHOW GRANTS TO ROLE <role>` or `SHOW GRANTS ON <object>`) executed and diff reviewed
- Target object, privilege, and grantee (role or user) explicitly named
- REVOKE rollback command staged and confirmed

**What it grants:** a single object privilege (e.g., `USAGE ON DATABASE`, `SELECT ON SCHEMA`, `OPERATE ON WAREHOUSE`) or a single role grant to one grantee — the minimum required for the stated purpose.

**Azure scope:** Azure Private Link (Business Critical edition) enforced; Entra ID OAuth 2.0 / SCIM-provisioned service account as the executing identity; no password-based authentication for the grant session.

**Hard denials (agent refuses regardless of approval):**
- Any GRANT to or from `ACCOUNTADMIN`
- Role grants that elevate a principal to `SECURITYADMIN` or `SYSADMIN`
- Bulk or wildcard grants (more than one grantee or more than one object per operation)
- `GRANT ALL PRIVILEGES` or account-scoped privilege escalation
- Any operation that bypasses Entra-federated identity (e.g., legacy password auth)

## 🎓 Certification anchors

These agents are grounded in the following certification domains (verify current exam availability before citing):

- **SnowPro Core Certification** (Snowflake)
- **SnowPro Advanced: Data Engineer** (Snowflake)

## 📛 Naming rationale (`-at-azure`)

All agents in this provider use the `-at-azure` suffix to make the deployment target unambiguous. Snowflake is a multi-cloud platform; these agents are scoped exclusively to Snowflake on Azure (Private Link, ADLS Gen2/Blob storage integration, Entra ID federation). Behaviour specific to AWS or GCP deployments is out of scope.

## 🛡️ Operating note

- **Phase A (static-review) agents** read configuration exports, Terraform plans, SQL role grant scripts, and network policy definitions; they do not connect to live Snowflake accounts
- Production-impacting recommendations (role revocations, network policy changes, masking policy enforcement) require explicit human approval and must follow a tested rollback path
- **Phase B (live-guard) agent** — `snowflake-live-rbac-grant-guard-at-azure-agent` — now exists and is gated: it requires a written approval token, PREFLIGHT dry-run output, account-name and grantee-type confirmation (Entra-federated identity only), and a staged REVOKE rollback path before any GRANT is issued to a live Snowflake account

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --provider snowflake --repo .
```
