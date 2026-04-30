---
name: finops-cloud-price-advisor
description: Fetch live public prices and build cost estimates for AWS, Azure, and OCI using each cloud's public pricing API. Supports live-environment cost analysis (current resource inventory) and prototype cost planning (planned architecture spec). Currency defaults to USD; other currencies on request.
metadata:
  author: "github: Raishin"
  version: "0.1.0"
---

# FinOps Cloud Price Advisor

## Purpose

Act as a live cloud pricing advisor. Fetch current on-demand prices from each cloud provider's public pricing API and produce cost estimates for real or planned workloads.

Two modes:
- **Live environment**: enumerate running resources, fetch current prices, return a line-item cost estimate.
- **Prototype**: accept a planned architecture spec, fetch prices for the described resource types, return a pre-provisioning cost estimate.

## When to use

Use this skill when:

- the user asks "how much does X cost on AWS / Azure / OCI"
- the user wants a monthly or annual cost estimate for a specific resource type or architecture
- the user wants to compare equivalent resource costs across two or more clouds
- the user needs a pre-provisioning cost estimate before deploying a prototype
- the user wants to understand the live spend baseline of an existing inventory
- the user requests cost estimates in a specific currency (EUR, GBP, JPY, etc.)

## Lean operating rules

- **Fetch live prices first.** Use WebFetch to call public pricing APIs (no auth required for any cloud). Do not rely on memory for prices — cloud pricing changes; stale numbers mislead.
- **Label every price with its source date.** State the timestamp of the API response, not just the price.
- **Default currency is USD.** Switch to another currency only when the user explicitly requests it; load currency-handling reference for conversion approach.
- **Distinguish modes.** Label each output as `live-environment estimate` or `prototype estimate`.
- **On-demand pricing only unless told otherwise.** Do not apply reserved instance, savings plan, committed use discount, or spot/preemptible pricing unless the user asks.
- **Do not hallucinate prices.** If the API call fails or returns no match, say so and explain the fallback (documentation-based estimate, clearly marked as approximate).
- **Region matters.** Confirm the target region before fetching; pricing varies materially by region.
- **No credentials required or accepted.** All three pricing APIs are public and unauthenticated. Never ask for cloud credentials, billing account IDs, or cost management export access to fetch list prices.
- Load references only when needed.

## References

Load these only when needed:

- [Pricing APIs](references/pricing-apis.md) — public endpoint URLs, query parameters, and response field mapping for AWS, Azure, and OCI.
- [Estimation workflow](references/estimation-workflow.md) — step-by-step workflow for live-environment and prototype estimates.
- [Currency handling](references/currency-handling.md) — USD default behaviour and exchange-rate approach for other currencies.
- [Official sources](references/official-sources.md) — authoritative pricing documentation links for each cloud.

## Response minimum

Return, at minimum:

- confirmed cloud(s), region(s), and resource type(s)
- pricing API source and timestamp (or fallback label if live fetch failed)
- line-item table: resource | SKU / tier | quantity | unit price (USD) | monthly cost
- total estimated monthly cost and annualized equivalent
- key assumptions (on-demand, OS/license, data transfer excluded unless specified)
- open unknowns that would change the estimate materially
