---
name: finops-cloud-price-advisor
description: Fetch live public prices and build cost estimates for AWS, Azure, OCI, Scaleway, Gandi, Alibaba Cloud, and Tencent Cloud using each provider's public pricing API or official documentation. Supports live-environment cost analysis and prototype cost planning. Currency defaults to USD; EUR and CNY supported natively.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.2.0"
  updated: "2026-05-13"
  category: finops
---

# FinOps Cloud Price Advisor

## Purpose

Act as a live cloud pricing advisor. Fetch current on-demand prices from each cloud provider's public pricing API (or official documentation where no API is available) and produce cost estimates for real or planned workloads across seven cloud providers.

Two modes:
- **Live environment**: enumerate running resources, fetch current prices, return a line-item cost estimate.
- **Prototype**: accept a planned architecture spec, fetch prices for the described resource types, return a pre-provisioning cost estimate.

## When to use

Use this skill when:

- the user asks "how much does X cost on AWS / Azure / OCI / Scaleway / Gandi / Alibaba / Tencent"
- the user wants a monthly or annual cost estimate for a specific resource type or architecture
- the user wants to compare equivalent resource costs across two or more clouds (including EU and Asia-Pacific providers)
- the user needs a pre-provisioning cost estimate before deploying a prototype
- the user wants to understand the live spend baseline of an existing inventory
- the user requests cost estimates in a specific currency (USD, EUR, CNY, GBP, JPY, etc.)
- the user asks about EU-based cloud pricing (Scaleway in France/Netherlands, Gandi)
- the user asks about Asia-Pacific cloud pricing (Alibaba Cloud in mainland China or APAC, Tencent Cloud)

## Lean operating rules

- **Fetch live prices first.** Use WebFetch to call public pricing APIs where available. Do not rely on memory for prices — cloud pricing changes; stale numbers mislead.
- **Label every price with its source.** State the API timestamp or documentation date, not just the price. Use the provenance label appropriate to the source (see below).
- **Provenance labels are mandatory.** Every numeric price must carry one label:
  - `live-price` — fetched from a public API in this session (include URL + ISO 8601 timestamp)
  - `documentation-based` — from official pricing documentation (include URL + last-verified date)
  - `assumed` — from an analogous SKU or default ratio (state the assumption explicitly)
  - `excluded` — intentionally omitted from the estimate (state why)
- **Default currency is USD.** Switch to another currency only when explicitly requested. EUR and CNY are supported natively for Scaleway/Gandi and Alibaba/Tencent respectively. Load currency-handling reference for conversion approach.
- **Distinguish modes.** Label each output as `live-environment estimate` or `prototype estimate`.
- **On-demand pricing only unless told otherwise.** Do not apply reserved instance, savings plan, committed use discount, or spot/preemptible pricing unless the user asks.
- **Do not hallucinate prices.** If the API call fails or returns no match, use the documented fallback chain from provider-fallbacks reference. Label fallback estimates as `documentation-based`.
- **Region matters.** Confirm the target region before fetching; pricing varies materially by region.
- **No credentials required for most providers.** AWS, Azure, OCI, and Scaleway have public unauthenticated pricing APIs. Gandi requires a user-provided API key (never stored). Alibaba and Tencent use scrape-based fallback.
- Load references only when needed.

## References

Load these only when needed:

- [Pricing APIs](references/pricing-apis.md) — public endpoint URLs, query parameters, and response field mapping for all seven providers.
- [Estimation workflow](references/estimation-workflow.md) — step-by-step workflow for live-environment and prototype estimates with multi-cloud comparison table.
- [Currency handling](references/currency-handling.md) — USD default behaviour, EUR conversion, and CNY conversion with timestamp requirements.
- [Official sources](references/official-sources.md) — authoritative pricing documentation links for each provider.
- [Provider fallbacks](references/provider-fallbacks.md) — per-provider fallback decision trees (live API → scrape → cached docs) and Gandi user-provided key handling.

## Response minimum

Return, at minimum:

- confirmed cloud(s), region(s), and resource type(s)
- pricing API source and timestamp (or fallback label if live fetch failed)
- line-item table: resource | SKU / tier | quantity | unit price (USD) | monthly cost
- total estimated monthly cost and annualized equivalent
- key assumptions (on-demand, OS/license, data transfer excluded unless specified)
- open unknowns that would change the estimate materially
