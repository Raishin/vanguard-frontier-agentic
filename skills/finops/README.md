# 💰 FinOps Skills

<p align="center">
  <!-- 🖼️ Add a FinOps logo to assets/logos/cloud/finops/ and update this path -->
  <span style="font-size:3.5em">💰</span>
</p>

This folder contains cross-cloud FinOps skills curated for this marketplace.

## Local marketplace portfolio

This folder contains **1** local FinOps skill:

- `finops-cloud-price-advisor`

## Portfolio posture

Cross-cloud FinOps skills for live price lookup, cost estimation, provider comparison, and budget governance.

These skills are intentionally conservative:

- fetch prices from public unauthenticated APIs only — no billing credentials required
- always distinguish on-demand list price from effective price (reserved instances, savings plans, committed use discounts not included by default)
- prefer live API lookups over cached or memory-based price estimates — cloud prices change frequently
- when comparing providers, normalize compute specs (vCPU, RAM, storage type) before comparing price
- flag GPU and accelerated compute costs explicitly — they dominate bills and are often overlooked

Providers covered: 🟧 AWS Price List API · 🟦 Azure Retail Prices API · 🟥 OCI public pricing API

Run `npm run validate` after changing cataloged FinOps skills.
