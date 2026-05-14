# 💰 FinOps Skills

<p align="center">
  <!-- 🖼️ Add a FinOps logo to assets/logos/cloud/finops/ and update this path -->
  <span style="font-size:3.5em">💰</span>
</p>

> ⚠️ **ALPHA RELEASE** — All FinOps skills are currently at `lifecycle: experimental`. Use at your own risk in pre-production environments only. [Board readiness memo](../../docs/strategy/finops-maestro-board-memo.md) documents known limitations and 30-day diligence requirements.

This folder contains cross-cloud FinOps skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **7** local FinOps skills (all alpha):

| Skill | Purpose | Lifecycle | Tools |
|---|---|---|---|
| `finops-maestro` | Route FinOps tasks to the narrowest specialist or parallel team (max 4) | **experimental** | Agent Skill Read Grep Glob |
| `finops-cloud-price-advisor` | Live public pricing across AWS, Azure, OCI, Scaleway, Gandi, Alibaba, Tencent, Hetzner, Contabo, Ionos, OVHcloud | **experimental** | Read Grep Glob WebFetch |
| `fetch-foundation-model-pricing` | Live per-token, per-image, per-GPU-hour pricing across Anthropic, OpenAI, Google, Bedrock, Azure OpenAI, OCI, and Vertex | **experimental** | Read Grep Glob WebFetch |
| `kubernetes-allocation-report` | OpenCost-compatible namespace/pod/workload cost allocation mapped to FOCUS v1.2 columns | **experimental** | Read Grep Glob WebFetch |
| `rightsize-recommendation` | Pod request/limit recommendations from user-supplied p50/p95/p99 metrics; Karpenter consolidation eligibility | **experimental** | Read Grep Glob |
| `carbon-cost-pair` | Pair $ values with kgCO2e by region and service category for CSRD/SEC climate disclosure | **experimental** | Read Grep Glob WebFetch |
| `focus-spec-normalizer` | Normalize vendor bills (AWS CUR, Azure Cost Management, GCP Billing Export, OCI) into FOCUS v1.2 columns | **experimental** | Read Grep Glob |

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

### Provider coverage matrix

| Category | Providers | Skill | Status |
|---|---|---|---|
| **Foundation models** | Anthropic, OpenAI, Google, AWS Bedrock, Azure OpenAI, OCI Generative AI, Vertex | `fetch-foundation-model-pricing` | experimental |
| **Cloud compute (on-demand)** | AWS, Azure, OCI | `finops-cloud-price-advisor` | experimental |
| **Cloud compute (regional)** | Scaleway (EU), Gandi (EU), Alibaba Cloud (CN/APAC), Tencent Cloud (CN/APAC), Hetzner (EU), Contabo (EU), Ionos (EU), OVHcloud (EU) | `finops-cloud-price-advisor` | experimental |
| **Kubernetes cost** | EKS, AKS, GKE, OKE (vendor-agnostic allocation) | `kubernetes-allocation-report` + `rightsize-recommendation` | experimental |
| **Bill normalization** | AWS CUR, Azure Cost Management, GCP Billing Export, OCI Cost Analysis | `focus-spec-normalizer` | experimental |
| **Carbon tracking** | AWS, Azure, GCP, OCI | `carbon-cost-pair` | experimental |

## 🧭 Routing taxonomy

See [agents/finops/README.md](../agents/README.md) for the complete 120+ keyword taxonomy used by `finops-maestro` to classify and dispatch tasks to the right specialist skill or agent.

## ⚠️ Known limitations and disclaimers

**Alpha status**: These skills support pre-production research and estimation use cases. Production deployment requires:
1. Signed design-partner SOW with cost verification clause
2. AT-C 215 AUP report from Big 4 accounting firm documenting hard savings
3. SOC 2 Type II observation (≥150 days) on infrastructure
4. FOCUS v1.2 column-mapping audit by compliance reviewer

**Data freshness**: Cloud and foundation-model prices are live-fetched on every invocation. Pricing APIs are public and unauthenticated; no caching across sessions. If a pricing API is unavailable, the skill returns a documented-based fallback with a timestamp.

**Scope limitations**:
- Reserved instances, savings plans, committed use discounts require separate calculation (not included in base list-price estimates)
- Volume discounts (multi-year, enterprise agreements) not modeled
- Spot/preemptible pricing not included (separate query required)
- Custom pricing (e.g., negotiated cloud contracts) not discoverable

**Accuracy**: Cost estimates from these skills are **indicative only**. Use them for:
- Budget planning (±20% accuracy acceptable)
- Comparative analysis (which cloud is cheaper?)
- Anomaly detection (which workload got expensive?)

Do not use for:
- Chargeback / cost allocation without human review
- SLA-level cost forecasting
- Contractual billing disputes

**See also**: [Board readiness memo Section 8](../../docs/strategy/finops-maestro-board-memo.md#8-risk-catalog) enumerates all 21 operational risks with contractual mitigations.
