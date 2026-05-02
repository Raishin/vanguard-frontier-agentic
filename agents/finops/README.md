# 💰 FinOps Agents

<p align="center">
  <!-- 🖼️ Add a FinOps logo to assets/logos/cloud/finops/ and update this path -->
  <span style="font-size:3.5em">💰</span>
</p>

Cross-cloud FinOps agent catalog for this marketplace. 😄

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cost mutation |
|---|---|---|---|
| Advisory agents | Fetch live prices, estimate costs, compare provider pricing | read-only | not allowed by default |

## 💸 FinOps agents

| Agent | Primary use | Providers covered |
|---|---|---|
| `finops-cloud-price-advisor-agent` | Fetch live on-demand prices from public pricing APIs; estimate costs for live environments or prototypes; compare AWS, Azure, and OCI pricing | 🟧 AWS · 🟦 Azure · 🟥 OCI |

## 🛡️ Operating note

- 😄 all FinOps agents stay read-only — they query public pricing APIs only
- 🔑 no billing credentials required — AWS Price List API, Azure Retail Prices API, and OCI public pricing are all unauthenticated public endpoints
- 💵 currency defaults to USD; other currencies available via Azure's native `currencyCode` parameter or public exchange rate APIs for AWS/OCI
- ⚠️ prices are on-demand list prices — reserved instance, savings plan, or committed use discounts require separate calculation
