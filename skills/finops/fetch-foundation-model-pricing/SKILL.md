---
name: fetch-foundation-model-pricing
description: Fetch live per-token, per-image, and per-GPU-hour prices for foundation models across Anthropic, OpenAI, Google, AWS Bedrock, Azure OpenAI, OCI Generative AI, and Vertex AI. Supports single-model lookup and comparative multi-provider tables. Every price is labeled with source URL and ISO 8601 fetch timestamp. No credentials accepted.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.1"
  updated: "2026-05-13"
  category: finops
  lifecycle: experimental
---

# Fetch Foundation Model Pricing

## Purpose

Retrieve current public pricing for foundation models across the major AI/cloud providers and return structured, provenance-labeled output. Supports two modes:

- **Single-model lookup**: fetch the current price for a specific model and deployment target (e.g., Claude Sonnet 4.5 on Anthropic direct, or on Bedrock).
- **Comparative table**: build a side-by-side price comparison across two or more models or providers for the same task type (text, image, embedding, or GPU-hour).

## When to use

Use this skill when:

- The user asks "how much does model X cost per token / per image / per GPU-hour"
- The user wants to compare inference costs across two or more providers for the same model family or equivalent capability tier
- The user needs to estimate monthly AI inference spend given a volume projection
- The user wants to understand how context caching or batch pricing changes the effective cost curve
- The user wants a FOCUS-aware cost breakdown (BilledCost, EffectiveCost) for AI inference line items

## Operating rules

- **Fetch live prices first.** Use WebFetch to retrieve prices from each provider's public pricing page before relying on any internal knowledge. AI model pricing changes frequently; stale numbers mislead.
- **Label every price.** Each price value must carry exactly one provenance label:
  - `live-price` — fetched from a provider's public pricing page or API within this session; include source URL and ISO 8601 timestamp.
  - `documentation-based` — sourced from official documentation when a live fetch was not possible; note the documentation URL and its visible publication date.
  - `assumed` — derived from an analogous model or tier when no direct published price exists; state the assumption explicitly.
  - `excluded` — pricing that exists but was intentionally omitted from the output; state why.
- **Include source URL and timestamp.** For every live-price value, state the exact URL fetched and the UTC timestamp of the fetch to the minute (e.g., `2026-05-13T14:32Z`).
- **On-demand pricing only unless told otherwise.** Do not apply reserved capacity, committed use, or enterprise negotiated pricing unless the user explicitly requests it.
- **No credentials required or accepted.** All provider pricing pages are public and unauthenticated. Never ask for API keys, billing account IDs, or tenant-specific data.
- **FOCUS column mapping.** Where a cost estimate is produced, note the corresponding FOCUS v1.2 columns: `BilledCost` (what the provider charges), `EffectiveCost` (after credits or discounts), `ServiceCategory` (AI and Machine Learning), `ChargeCategory` (Usage), `SkuId` (model ID + deployment tier), `SkuPriceId` (price dimension: input-token / output-token / cached-token / image / gpu-hour).
- **Load references only when needed.**

## Pricing dimensions

| Dimension | Unit | Applies to |
|---|---|---|
| Input tokens | per 1M tokens | All text models |
| Output tokens | per 1M tokens | All text models |
| Cached input tokens | per 1M tokens | Models with context caching (Anthropic, Gemini, Bedrock) |
| Batch input tokens | per 1M tokens | Models with async batch mode (Anthropic, OpenAI, Bedrock) |
| Batch output tokens | per 1M tokens | Models with async batch mode |
| Images (input) | per image or per 1K images | Multimodal models |
| GPU-hour | per hour per GPU type | Self-hosted / dedicated endpoints (Vertex AI, Bedrock provisioned throughput, Azure PTU) |

## Providers in scope

| Provider | Deployment target | Reference |
|---|---|---|
| Anthropic | Direct API | references/providers.md |
| OpenAI | Direct API | references/providers.md |
| Google | Vertex AI | references/providers.md |
| AWS | Bedrock | references/providers.md |
| Azure | Azure OpenAI Service | references/providers.md |
| OCI | Generative AI Service | references/providers.md |

## Response minimum

Return, at minimum:

- confirmed model name(s), provider(s), and deployment target(s)
- pricing dimensions covered (input, output, cached, batch, image, GPU-hour)
- line-item price table: model | provider | dimension | unit price | provenance label | source URL | fetch timestamp
- for comparative tables: a summary row noting the cheapest and most expensive option per dimension
- key assumptions (on-demand, no reserved/committed pricing, USD unless stated)
- FOCUS column mapping for any cost estimate produced

## References

Load these only when needed:

- [Provider pricing URLs](references/providers.md) — public pricing page URLs per provider for live WebFetch.
- [Token economics](references/token-economics.md) — input/output/cached/batch pricing model, $/M token math, and context caching cost curve.
