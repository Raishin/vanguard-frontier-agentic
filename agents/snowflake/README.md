# ❄️ Snowflake (Azure) Agents

## Overview

Two **static-review** agents for Snowflake workloads deployed on Azure — scoped to RBAC access governance and data-platform engineering. Neither agent connects to live Snowflake accounts; every verdict is an evidence-backed recommendation requiring human approval before any change reaches a production environment.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Snowflake mutation |
|---|---|---|---|
| Static-review agents | Review, design, diagnose | read-only | not allowed |

## 🗂️ Agents in this provider

| Agent | Primary use |
|---|---|
| `snowflake-rbac-access-governance-at-azure-agent` | RBAC governance review — ACCOUNTADMIN/SECURITYADMIN/SYSADMIN role separation, custom least-privilege roles, SoD enforcement, network policy review, Entra OAuth/SSO/SCIM integration posture |
| `snowflake-data-platform-engineering-at-azure-agent` | Data platform architecture review — warehouse sizing, Azure Private Link (Business Critical), storage integration to ADLS Gen2/Blob, dynamic data masking, row-access policies, object tagging, ACCESS_HISTORY auditing |

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

## 🎓 Certification anchors

These agents are grounded in the following certification domains (verify current exam availability before citing):

- **SnowPro Core Certification** (Snowflake)
- **SnowPro Advanced: Data Engineer** (Snowflake)

## 📛 Naming rationale (`-at-azure`)

All agents in this provider use the `-at-azure` suffix to make the deployment target unambiguous. Snowflake is a multi-cloud platform; these agents are scoped exclusively to Snowflake on Azure (Private Link, ADLS Gen2/Blob storage integration, Entra ID federation). Behaviour specific to AWS or GCP deployments is out of scope.

## 🛡️ Operating note

- All agents are **static-review** by default — they read configuration exports, Terraform plans, SQL role grant scripts, and network policy definitions; they do not connect to live Snowflake accounts
- Production-impacting recommendations (role revocations, network policy changes, masking policy enforcement) require explicit human approval and must follow a tested rollback path
- **Live-guard posture** is gated — if a live-guard companion is introduced in a future release, it will require account-name confirmation, principal-type audit, and approval before any mutation

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --provider snowflake --repo .
```
