# 🧱 Databricks (Azure) Agents

## Overview

Two **static-review** agents and one **mutating-runtime live-guard** agent for Azure Databricks workloads — scoped to Unity Catalog governance, lakehouse engineering, and controlled privilege grants. The static-review agents never write to live workspaces; every verdict is an evidence-backed recommendation requiring human approval before any change reaches a production environment. The Phase B live-guard agent executes a single, narrowly scoped GRANT against a live Unity Catalog instance only after written approval token, PREFLIGHT dry-run, and REVOKE rollback path are confirmed.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Databricks mutation |
|---|---|---|---|
| Static-review agents | Review, design, diagnose | read-only | not allowed |
| Live-guard (mutating-runtime) | Apply one schema-scoped GRANT to one principal; REVOKE rollback | written approval token + PREFLIGHT dry-run required | single scoped GRANT only — bulk/wildcard/admin-role grants denied |

## 🗂️ Agents in this provider

| Agent | Tier | Primary use |
|---|---|---|
| `databricks-unity-catalog-governance-at-azure-agent` | Static-review | Unity Catalog privilege review — metastore → catalog → schema → table hierarchy, schema-scoped least-privilege grants, account/workspace/metastore admin separation, run-as-service-principal enforcement |
| `databricks-lakehouse-engineering-at-azure-agent` | Static-review | Lakehouse architecture review — medallion (Bronze/Silver/Gold) design, ADLS Gen2 external locations via Access Connector managed identity, cluster policies, AKV-backed secret scopes, VNet injection/Private Link posture |
| `databricks-live-unity-catalog-grant-guard-at-azure-agent` | Live-guard (mutating-runtime) | Apply one Unity Catalog schema-scoped GRANT to one principal via SQL API; REVOKE rollback; written approval token + PREFLIGHT diff required; denies metastore/catalog-wide ALL PRIVILEGES, admin-role grants, and bulk operations |

## 🔒 Unity Catalog governance agent

The `databricks-unity-catalog-governance-at-azure-agent` reviews Unity Catalog configurations for:

- **Privilege hierarchy:** correct grant scope at metastore, catalog, schema, and table/volume level; no over-broad `ALL PRIVILEGES` on high-level objects
- **Admin separation:** account admin, workspace admin, and metastore admin roles kept distinct; no single principal holds all three
- **Least-privilege grants:** schema-scoped grants preferred over catalog-scoped; `SELECT`/`MODIFY` separated from `CREATE`/`USAGE`
- **Service principal execution:** interactive user principals not used for production job runs; service principals hold only the grants required for their job clusters
- **Data lineage and auditing:** Unity Catalog audit log coverage confirmed; column-level lineage enabled where required for data-product governance

## 🏗️ Lakehouse engineering agent

The `databricks-lakehouse-engineering-at-azure-agent` reviews lakehouse architectures for:

- **Medallion design:** Bronze → Silver → Gold separation; schema enforcement at Silver; no direct consumer access to Bronze
- **ADLS Gen2 storage access:** external locations defined via Access Connector managed identity (not storage account keys or service principal secrets); Databricks-managed vs. external table classification reviewed
- **Cluster policies:** all-purpose and job clusters governed by policy; no unrestricted `spark.executor.memory` or instance-type overrides in production
- **Secret management:** Azure Key Vault-backed secret scopes used for all credentials; no secrets in notebook code, job parameters, or init scripts
- **Network posture:** VNet injection confirmed; Private Link enabled for workspace front-end and back-end where Business Critical tier is required; no public endpoint exposure without compensating control

## 🔐 Unity Catalog grant guard agent (live-guard — mutating-runtime, Phase B)

The `databricks-live-unity-catalog-grant-guard-at-azure-agent` is a **controlled WRITE** agent that applies exactly one Unity Catalog schema-scoped GRANT to exactly one principal on a live Azure Databricks workspace. It is a Phase B mutating-runtime guard — distinct from the Phase A static-review agents above, which never touch a live workspace.

**Execution conditions — all must be met before any GRANT is issued:**
- Written approval token provided by an authorized human approver
- PREFLIGHT dry-run (`SHOW GRANTS ON SCHEMA <schema>`) executed and diff reviewed
- Target table/schema, principal (service principal or group), and privilege explicitly named
- REVOKE rollback command staged and confirmed

**What it grants:** a single `USAGE`, `SELECT`, `MODIFY`, or `CREATE` privilege on one named schema to one principal — the minimum required for the stated purpose, using least-privilege `prvWrite` equivalent on that schema only.

**Azure scope:** ADLS Gen2 external location access controlled via Access Connector managed identity; AKV-backed secret scopes; Entra-integrated service principals.

**Hard denials (agent refuses regardless of approval):**
- `ALL PRIVILEGES` on a metastore, catalog, or multi-schema target
- Account-admin, workspace-admin, or metastore-admin role grants
- Bulk or wildcard grants (more than one principal or more than one object per operation)
- Any `DELETE`, `EXECUTE`, or governance-role privilege escalation

## 🎓 Certification anchors

These agents are grounded in the following certification domains (verify current exam availability before citing):

- **DP-700** — Implementing Data Engineering Solutions Using Microsoft Fabric (Microsoft; adjacent data-engineering domain)
- **DP-750** — Implementing Data Engineering Solutions Using Azure Databricks (Microsoft; "Secure and govern Unity Catalog objects" is ~15–20% of the exam)
- **Databricks Certified Data Engineer Associate / Professional** (Databricks)

## 📛 Naming rationale (`-at-azure`)

All agents in this provider use the `-at-azure` suffix to make the deployment target unambiguous. Databricks is a multi-cloud platform; these agents are scoped exclusively to Azure Databricks (VNet injection, ADLS Gen2 storage, AKV secret scopes, Entra-integrated service principals). Behaviour specific to AWS or GCP deployments is out of scope.

## 🛡️ Operating note

- **Phase A (static-review) agents** read configuration artefacts, Terraform plans, ARM/Bicep templates, and notebook exports; they do not connect to live workspaces
- Production-impacting recommendations (privilege revocations, cluster policy enforcement, storage credential rotation) require explicit human approval and must follow a tested rollback path
- **Phase B (live-guard) agent** — `databricks-live-unity-catalog-grant-guard-at-azure-agent` — now exists and is gated: it requires a written approval token, PREFLIGHT dry-run output, principal-type audit (service principal or group only), and a staged REVOKE rollback path before any GRANT is issued to a live Unity Catalog workspace

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --provider databricks --repo .
```
