# 🧱 Databricks (Azure) Agents

## Overview

Two **static-review** agents for Azure Databricks workloads — scoped to Unity Catalog governance and lakehouse engineering. Neither agent writes to live workspaces; every verdict is an evidence-backed recommendation requiring human approval before any change reaches a production environment.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live Databricks mutation |
|---|---|---|---|
| Static-review agents | Review, design, diagnose | read-only | not allowed |

## 🗂️ Agents in this provider

| Agent | Primary use |
|---|---|
| `databricks-unity-catalog-governance-at-azure-agent` | Unity Catalog privilege review — metastore → catalog → schema → table hierarchy, schema-scoped least-privilege grants, account/workspace/metastore admin separation, run-as-service-principal enforcement |
| `databricks-lakehouse-engineering-at-azure-agent` | Lakehouse architecture review — medallion (Bronze/Silver/Gold) design, ADLS Gen2 external locations via Access Connector managed identity, cluster policies, AKV-backed secret scopes, VNet injection/Private Link posture |

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

## 🎓 Certification anchors

These agents are grounded in the following certification domains (verify current exam availability before citing):

- **DP-700** — Implementing Data Engineering Solutions Using Microsoft Fabric (Microsoft; adjacent data-engineering domain)
- **DP-750** — Implementing Data Engineering Solutions Using Azure Databricks (Microsoft; "Secure and govern Unity Catalog objects" is ~15–20% of the exam)
- **Databricks Certified Data Engineer Associate / Professional** (Databricks)

## 📛 Naming rationale (`-at-azure`)

All agents in this provider use the `-at-azure` suffix to make the deployment target unambiguous. Databricks is a multi-cloud platform; these agents are scoped exclusively to Azure Databricks (VNet injection, ADLS Gen2 storage, AKV secret scopes, Entra-integrated service principals). Behaviour specific to AWS or GCP deployments is out of scope.

## 🛡️ Operating note

- All agents are **static-review** by default — they read configuration artefacts, Terraform plans, ARM/Bicep templates, and notebook exports; they do not connect to live workspaces
- Production-impacting recommendations (privilege revocations, cluster policy enforcement, storage credential rotation) require explicit human approval and must follow a tested rollback path
- **Live-guard posture** is gated — if a live-guard companion is introduced in a future release, it will require subscription confirmation, principal-type audit, and approval before any mutation

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --provider databricks --repo .
```
