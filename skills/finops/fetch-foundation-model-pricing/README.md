# Fetch Foundation Model Pricing

A FinOps skill that retrieves live public pricing for foundation models across major AI and cloud providers, returning structured tables with mandatory provenance labels and source timestamps.

## Purpose

Fetch current per-token, per-image, and per-GPU-hour prices from Anthropic, OpenAI, Google (Vertex AI), AWS Bedrock, Azure OpenAI Service, and OCI Generative AI. Supports single-model lookups and side-by-side comparative tables.

## Allowed tools

`Read` `Grep` `Glob` `WebFetch`

## Usage

**Single-model lookup:** Provide a model name and deployment target (e.g., "What does Claude Sonnet 4.5 cost per million input tokens on Bedrock?"). The skill fetches the live price, labels it with source URL and ISO 8601 timestamp, and returns a single-row price table.

**Comparative table:** Provide two or more models or providers and a task type (e.g., "Compare GPT-4o, Claude Sonnet, and Gemini Pro on input and output token cost"). The skill fetches each price independently and builds a labeled multi-row comparison with cheapest/most-expensive summary rows.

## Trust posture

Read-only. No cloud credentials, billing account IDs, or tenant data accepted. All pricing pages are public and unauthenticated. Every price value carries a provenance label (`live-price`, `documentation-based`, `assumed`, or `excluded`) with source URL and fetch timestamp.

FOCUS v1.2 column mapping is included for any cost estimate produced (BilledCost, EffectiveCost, ServiceCategory, ChargeCategory, SkuId, SkuPriceId).

See [SKILL.md](SKILL.md) for the full operating protocol, pricing dimensions, and response shape.
