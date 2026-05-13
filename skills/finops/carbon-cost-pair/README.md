# Carbon Cost Pair

A FinOps skill that pairs cloud spend values with kgCO2e carbon estimates by region and service category, with mandatory confidence labels and source citations for CSRD and SEC climate disclosure.

## Purpose

For every cloud spend line item, produce a paired kgCO2e estimate using the best available carbon factor for the cloud region and service category. Confidence is labeled as `vendor-published`, `third-party`, or `estimated`, and every output includes a source URL and data period acknowledgment.

## Allowed tools

`Read` `Grep` `Glob` `WebFetch`

## Usage

**Single line item:** Provide cloud provider, region, service category, and dollar value (e.g., "Pair $1,200 of AWS us-east-1 compute spend with a carbon estimate"). The skill fetches the latest available carbon factor, applies it, and returns a labeled kgCO2e output with FOCUS tag mapping.

**Multi-line comparison:** Provide multiple regions or providers (e.g., "Compare carbon intensity for the same $10,000 compute spend across AWS us-east-1, Azure eastus, and GCP us-central1"). The skill returns one estimate block per line item plus a summary comparison table.

## Trust posture

Read-only. No cloud credentials, billing account IDs, or tenant data accepted. Carbon factors are fetched from public provider sustainability pages or third-party sources. Provider data typically lags 60-90 days; the data period is stated in every output.

Defaults to Scope 2 electricity-only, market-based factors. Location-based and Scope 1+2 available on request.

See [SKILL.md](SKILL.md) for the full operating rules, confidence label decision tree, regulatory context, and output shape.
