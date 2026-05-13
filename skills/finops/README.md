# 💰 FinOps Skills

<p align="center">
  <!-- 🖼️ Add a FinOps logo to assets/logos/cloud/finops/ and update this path -->
  <span style="font-size:3.5em">💰</span>
</p>

This folder contains cross-cloud FinOps skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **7** local FinOps skills:

| Skill | Purpose | Tools |
|---|---|---|
| `finops-maestro` | Route FinOps tasks to the narrowest specialist or parallel team (max 4) | Agent Skill Read Grep Glob |
| `finops-cloud-price-advisor` | Live public pricing across AWS, Azure, and OCI for live-environment or prototype cost estimation | Read Grep Glob WebFetch |
| `fetch-foundation-model-pricing` | Live per-token, per-image, per-GPU-hour pricing across Anthropic, OpenAI, Google, Bedrock, Azure OpenAI, OCI, and Vertex | Read Grep Glob WebFetch |
| `kubernetes-allocation-report` | OpenCost-compatible namespace/pod/workload cost allocation mapped to FOCUS columns | Read Grep Glob WebFetch |
| `rightsize-recommendation` | Pod request/limit recommendations from user-supplied p50/p95/p99 metrics; Karpenter consolidation eligibility | Read Grep Glob |
| `carbon-cost-pair` | Pair $ values with kgCO2e by region and service category for CSRD/SEC climate disclosure | Read Grep Glob WebFetch |
| `focus-spec-normalizer` | Normalize vendor bills (AWS CUR, Azure Cost Management, GCP Billing Export, OCI) into FOCUS v1.2 columns | Read Grep Glob |

## Portfolio posture

Cross-cloud, harness-portable FinOps skills covering AI workload economics, Kubernetes rightsizing, cross-cloud price advisory, FOCUS-spec normalization, and carbon-cost pairing.

These skills are intentionally conservative:

- fetch prices from public unauthenticated APIs only — no billing credentials, kubeconfig, bearer tokens, or tenant data accepted
- always distinguish on-demand list price from effective price (reserved instances, savings plans, committed use discounts not included by default)
- prefer live API lookups over cached or memory-based price estimates — foundation-model and cloud prices change frequently
- when comparing providers, normalize compute specs (vCPU, RAM, GPU SKU, storage type) before comparing price
- flag GPU and accelerated compute costs explicitly — they dominate AI workload bills
- label every numeric output: `live-price` / `live-evidence` / `documentation-based` / `assumed` / `excluded`
- emit FOCUS v1.2 column mappings where applicable — vendor-column adapters are explicit, not assumed

Providers covered: 🟧 AWS · 🟦 Azure · 🟥 OCI (pricing); 🤖 Anthropic · 🤖 OpenAI · 🟩 Vertex · 🟧 Bedrock · 🟦 Azure OpenAI · 🟥 OCI Generative AI (foundation models); ☸️ Kubernetes (vendor-agnostic); 🟩 GCP (bill normalization via focus-spec-normalizer).

**Pricing API scope:** AWS, Azure, OCI. EU and APAC cloud provider pricing support is a future enhancement.
