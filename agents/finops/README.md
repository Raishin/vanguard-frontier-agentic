# 💰 FinOps Agents

<p align="center">
  <!-- 🖼️ Add a FinOps logo to assets/logos/cloud/finops/ and update this path -->
  <span style="font-size:3.5em">💰</span>
</p>

Cross-cloud FinOps agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cost mutation |
|---|---|---|---|
| Orchestrator | Routes FinOps tasks to the narrowest specialist or parallel team | read-only | never auto-dispatches mutating agents |
| Advisory agents | Fetch live prices, estimate costs, rightsize workloads, normalize bills | read-only | not allowed by default |

## 💸 FinOps agents

| Agent | Primary use | Providers covered |
|---|---|---|
| `finops-maestro-agent` | Route FinOps tasks to the narrowest specialist or parallel team (max 4); FOCUS-aware classification; no auto-mutation | multi-cloud |
| `finops-ai-economist-agent` | AI workload economics: token economics, GPU-hour, cross-provider comparison, training-vs-inference TCO; FOCUS-mapped output | 🤖 Anthropic · 🤖 OpenAI · 🟧 Bedrock · 🟦 Azure OpenAI · 🟩 Vertex · 🟥 OCI Generative AI |
| `finops-kubernetes-rightsizer-agent` | Pod request/limit rightsizing from supplied metrics, idle scan, Karpenter consolidation eligibility, OpenCost-compatible allocation; never executes kubectl | ☸️ Kubernetes (EKS · AKS · GKE · OKE) |
| `finops-cloud-price-advisor-agent` | Fetch live on-demand prices from public pricing APIs; estimate costs for live environments or prototypes; compare AWS, Azure, and OCI pricing | 🟧 AWS · 🟦 Azure · 🟥 OCI |

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
