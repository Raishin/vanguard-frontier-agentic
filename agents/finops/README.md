# 💰 FinOps Agents

<p align="center">
  <!-- 🖼️ Add a FinOps logo to assets/logos/cloud/finops/ and update this path -->
  <span style="font-size:3.5em">💰</span>
</p>

> ⚠️ **ALPHA RELEASE** — All FinOps agents are currently at `lifecycle: experimental`. Use at your own risk in pre-production environments only. [Board readiness memo](../../docs/strategy/finops-maestro-board-memo.md) documents known limitations and 30-day diligence requirements.

Cross-cloud FinOps agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cost mutation |
|---|---|---|---|
| Orchestrator | Routes FinOps tasks to the narrowest specialist or parallel team | read-only | never auto-dispatches mutating agents |
| Advisory agents | Fetch live prices, estimate costs, rightsize workloads, normalize bills | read-only | not allowed by default |

## 💸 FinOps agents

| Agent | Primary use | Providers covered | Lifecycle |
|---|---|---|---|
| `finops-maestro-agent` | Route FinOps tasks to the narrowest specialist or parallel team (max 4); FOCUS-aware classification; no auto-mutation | multi-cloud | **experimental** |
| `finops-ai-economist-agent` | AI workload economics: token economics, GPU-hour, cross-provider comparison, training-vs-inference TCO; FOCUS-mapped output | 🤖 Anthropic · 🤖 OpenAI · 🟧 Bedrock · 🟦 Azure OpenAI · 🟩 Vertex · 🟥 OCI Generative AI | **experimental** |
| `finops-kubernetes-rightsizer-agent` | Pod request/limit rightsizing from supplied metrics, idle scan, Karpenter consolidation eligibility, OpenCost-compatible allocation; never executes kubectl | ☸️ Kubernetes (EKS · AKS · GKE · OKE) | **experimental** |
| `finops-cloud-price-advisor-agent` | Fetch live on-demand prices from public pricing APIs; estimate costs for live environments or prototypes; compare AWS, Azure, and OCI pricing | 🟧 AWS · 🟦 Azure · 🟥 OCI | **experimental** |

## 🧭 Routing Taxonomy

The `finops-maestro-agent` classifies FinOps tasks using keyword matching across three domains:

### AI Economist Keywords (24 keywords)
`token`, `tokens`, `inference`, `foundation`, `model`, `LLM`, `GPT`, `Claude`, `Gemini`, `Bedrock`, `OpenAI`, `Anthropic`, `Vertex`, `GPU`, `A100`, `H100`, `MI300X`, `Trainium`, `TPU`, `training cost`, `fine-tune`, `prompt cache`, `batch`, `context window`

**Routes to**: `finops-ai-economist-agent` (single mode)

### Kubernetes Rightsizer Keywords (25 keywords)
`Kubernetes`, `K8s`, `pod`, `deployment`, `statefulset`, `namespace`, `node`, `node pool`, `rightsizing`, `rightsize`, `request`, `limit`, `p95`, `p99`, `Karpenter`, `consolidation`, `VPA`, `HPA`, `idle`, `OpenCost`, `allocation`, `PVC`, `PV`, `LoadBalancer`, `cluster`

**Routes to**: `finops-kubernetes-rightsizer-agent` (single mode)

### Cloud Price Advisor Keywords (71 keywords)
**Tier 1 (cloud platforms)**: `AWS pricing`, `Azure pricing`, `OCI pricing`, `EC2`, `VM`, `instance price`, `list price`, `Price List`, `Retail Prices`, `monthly cost`, `prototype cost`, `estimate`, `currency`, `EUR`, `GBP`, `JPY`, `data transfer`, `egress price`

**Tier 2 (European / regional)**: `scaleway pricing`, `scaleway cost`, `scaleway eu pricing`, `scaleway fr-par`, `scaleway nl-ams`, `gandi pricing`, `gandi vps cost`, `gandi domain pricing`, `eu-fr pricing`, `eu-nl pricing`

**Tier 3 (APAC)**: `alibaba cloud pricing`, `alibaba cloud cost`, `aliyun pricing`, `alicloud pricing`, `alibaba ecs pricing`, `tencent cloud pricing`, `tencent cloud cost`, `tencent cvm pricing`, `tencentdb pricing`, `cn-beijing pricing`, `cn-shanghai pricing`, `ap-southeast pricing`, `ap-northeast pricing`, `cny pricing`, `renminbi pricing`, `rmb cloud cost`

**Tier 4 (major clouds)**: `google cloud pricing`, `gcp pricing`, `gcp cost`, `google compute engine pricing`, `gke cost`, `huawei cloud pricing`, `huaweicloud pricing`, `huawei cloud cost`, `ecs huawei pricing`, `huawei obs cost`

**Tier 5 (hosting providers)**: `contabo pricing`, `contabo vps cost`, `contabo cloud cost`, `contabo server pricing`, `hetzner pricing`, `hetzner cloud cost`, `hetzner vps pricing`, `hetzner dedicated cost`, `ionos pricing`, `ionos cloud cost`, `ionos vps pricing`, `ionos cloud server cost`, `ovhcloud pricing`, `ovh cloud cost`, `ovhcloud public cloud pricing`, `ovhcloud vps cost`

**Routes to**: `finops-cloud-price-advisor-agent` (single mode)

### Multi-Domain Dispatch Examples

**Two-domain example** (Kubernetes + AI):
```
User: "Rightsize our GPU pods running inference and estimate model cost."
Route: finops-kubernetes-rightsizer-agent, finops-ai-economist-agent
Mode: parallel(2)
```

**Three-domain example** (AI + Kubernetes + Cloud pricing):
```
User: "Review AI spend, find overprovisioned pods, and benchmark pricing vs GCP."
Route: finops-ai-economist-agent, finops-kubernetes-rightsizer-agent, finops-cloud-price-advisor-agent
Mode: parallel(3)
```

Hard ceiling: 4 specialists maximum. Any request requiring >4 agents is refused with recommendation to split into multiple queries.

## 🛡️ Operating note

- 😄 all FinOps agents stay read-only — they query public pricing APIs only
- 🔑 no billing credentials, kubeconfig, bearer tokens, API keys, or tenant data are required or accepted — refusal is unconditional
- 💵 currency defaults to USD; other currencies available via Azure's native `currencyCode` parameter or public exchange rate APIs for AWS/OCI
- ⚠️ prices are on-demand list prices — reserved instance, savings plan, or committed use discounts require separate calculation
- 🧭 the maestro never auto-dispatches mutating specialists — any mutation request requires an explicit human approval gate and a handoff packet
- 🏷️ every numeric value is labeled `live-price` / `live-evidence` / `documentation-based` / `assumed` / `excluded`
- 📐 FOCUS v1.2 column mapping is emitted where applicable (BilledCost, EffectiveCost, ServiceCategory, ChargeCategory, SkuPriceId)
- 🌱 carbon-cost pairing available via the `carbon-cost-pair` skill for CSRD/SEC climate disclosure alignment

## 📍 Provider scope

**Current:** AWS, Azure, OCI (via `finops-cloud-price-advisor-agent`); all foundation-model providers (via `finops-ai-economist-agent`); vendor-agnostic Kubernetes (via `finops-kubernetes-rightsizer-agent`).

**Future:** EU-region pricing (Scaleway, Gandi), APAC cloud providers (Alibaba, Tencent), and additional billing normalizers can extend the portfolio.

## ⚠️ Known limitations (Cycle 10d board assessment)

- **AI Economist**: Foundation-model pricing is live-fetched; deprecation of older model versions not detected (e.g., Claude 2 sunset flagged at API level, not in pricing page)
- **Kubernetes Rightsizer**: Assumes uniform cluster availability; does not model zone-specific failures or multi-AZ cost trade-offs
- **Price Advisor**: Reserved instance and savings plan pricing requires separate calculation (not included in base advisory)
- **Maestro routing**: Keyword taxonomy may fail on novel phrasing; fallback is human clarification request

See [board memo Section 8](../../docs/strategy/finops-maestro-board-memo.md#8-risk-catalog) for 21 enumerated risks and mitigations.
