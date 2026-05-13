# Provider Fallbacks

Decision tree for each provider: when to use a live API versus cached documentation pricing.
Used in conjunction with [./pricing-apis.md](./pricing-apis.md) and
[./official-sources.md](./official-sources.md).

---

## Fallback Principle

Every provider follows the same three-tier priority:

```
1. Live API   — real-time prices; highest accuracy; label: live-price
2. Scrape     — fetch official pricing page via WebFetch; label: documentation-based
3. Cached docs — static pricing from this reference file; label: documentation-based (stale)
```

Use the highest tier available given the request context. Always attach a provenance label
and, for live prices, the response timestamp.

---

## Security Rules (All Providers)

These rules apply without exception across every provider and every fallback tier:

- **Never prompt users for credentials.** If a key is needed and not provided, drop to the
  next fallback tier silently.
- **If the user explicitly includes a key in their request**, use it once for the live API
  call, then discard it. Log the following message and nothing else about the key:
  > "User-provided API key received; using live pricing. Key will not be stored."
- **Never log or echo the key value itself.** Do not include it in intermediate results,
  debug output, or citations.
- **Never store, cache, or carry a key across turns.** Each request is a fresh context;
  any key from a prior turn must not be assumed to be present.
- **Label all outputs** with the correct provenance tier. A `documentation-based` label is
  not a failure — it is honest and expected when no key is available.

---

## Gandi

### Decision tree

```
Request arrives
    │
    ├─ Does the request contain an explicit user-provided Gandi API key?
    │       │
    │       ├─ YES → Live API path (Tier 1)
    │       │         Log: "User-provided API key received; using live pricing. Key will not be stored."
    │       │         Call: GET https://api.gandi.net/v5/price-list
    │       │               Authorization: Apikey <user-provided-key>
    │       │         On success  → label result live-price; include response timestamp
    │       │         On failure  → log HTTP status; fall through to Tier 2
    │       │         After fetch → discard key; do not retain across turns
    │       │
    │       └─ NO  → Documentation path (Tier 2)
    │                 Fetch: https://www.gandi.net/domain/pricing  (WebFetch, no auth)
    │                 On success  → label result documentation-based
    │                 On failure  → use Tier 3 cached reference below
    │
    └─ END
```

### Tier 1 — Live API

| Attribute | Value |
|-----------|-------|
| Endpoint | `https://api.gandi.net/v5/price-list` |
| Auth header | `Authorization: Apikey <user-provided-key>` |
| Rate limit | 100 requests/second |
| Response currency | EUR and USD (both present) |
| Provenance label | `live-price` |
| Post-fetch action | Discard key; never carry across turns |

### Tier 2 — Official Pricing Page (WebFetch, no auth)

| Attribute | Value |
|-----------|-------|
| URL | `https://www.gandi.net/domain/pricing` |
| Auth required | No |
| Provenance label | `documentation-based` |
| Frequency note | Fetch at request time; do not rely on cached page content |

### Tier 3 — Cached Reference (static fallback of last resort)

Use only when both Tier 1 and Tier 2 fetches fail.

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Gandi | — |
| Instance type | VPS Start 2 | Smallest standard VPS tier |
| vCPU | 1 | — |
| RAM | 2 GiB | — |
| Storage | 20 GiB SSD | Included in instance price |
| Region | eu (EU default) | — |
| Monthly estimate | ~€2.99/month | `documentation-based` (stale; verify before use) |
| USD note | Convert using live EUR/USD rate | See official-sources.md — Exchange Rate Sources |

> Always note in the output that this figure is a static cached reference and may not
> reflect the current price. Direct the user to https://www.gandi.net/domain/pricing to
> verify.

---

## Scaleway

### Decision tree

```
Request arrives
    │
    ├─ Does the request contain an explicit user-provided Scaleway IAM API key?
    │       │
    │       ├─ YES → Live API path (Tier 1 — beta)
    │       │         Log: "User-provided API key received; using live pricing. Key will not be stored."
    │       │         Call: GET https://api.scaleway.com/billing/v2beta1/products
    │       │               X-Auth-Token: <user-provided-key>
    │       │         On success  → label result live-price; include response timestamp
    │       │                       Note: endpoint is beta; stability is low-medium
    │       │         On 404/error → log status; fall through to Tier 2
    │       │         After fetch → discard key; do not retain across turns
    │       │
    │       └─ NO  → Documentation path (Tier 2)
    │                 Fetch: https://www.scaleway.com/en/pricing/  (WebFetch, no auth)
    │                 On success  → label result documentation-based
    │                 On failure  → use Tier 3 cached reference
    │
    └─ END
```

### Tier 1 — Beta Billing API

| Attribute | Value |
|-----------|-------|
| Endpoint | `https://api.scaleway.com/billing/v2beta1/products` |
| Auth header | `X-Auth-Token: <user-provided-key>` |
| Stability | Beta (low-medium); may return 404 or undocumented errors |
| Rate limit | ~60 requests/minute (per-route limits undocumented) |
| Response currency | EUR only |
| Provenance label | `live-price` |
| USD conversion | Required; use live EUR/USD rate from official-sources.md |
| Post-fetch action | Discard key; never carry across turns |

### Tier 2 — Official Pricing Page (WebFetch, no auth)

| Attribute | Value |
|-----------|-------|
| URL | `https://www.scaleway.com/en/pricing/` |
| Auth required | No |
| Provenance label | `documentation-based` |
| Currency | EUR; convert to USD using live rate |

### Tier 3 — Cached Reference (static fallback of last resort)

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Scaleway | — |
| Instance type | PRO2-XS | Smallest production-grade instance |
| vCPU | 2 | — |
| RAM | 8 GiB | — |
| Storage | 20 GiB SSD (local) | Included in instance price |
| Region | fr-par (Paris, France) | — |
| Monthly estimate | ~€10–14/month | `documentation-based` (stale; verify before use) |
| USD note | Convert using live EUR/USD rate | See official-sources.md — Exchange Rate Sources |

---

## Alibaba Cloud

No public unauthenticated pricing API exists for Alibaba Cloud. The fallback chain starts
at Tier 2 (scrape) since Tier 1 (live authenticated API) is not usable for this skill.

### Decision tree

```
Request arrives
    │
    ├─ Tier 2a — Primary scrape
    │     Fetch: https://www.alibabacloud.com/cloud-computing/pricing  (WebFetch, no auth)
    │     HTML parser required; page contains product cards and pricing zones per region
    │     On success  → parse product pricing cards → label result documentation-based
    │     On failure (HTML structure changed, timeout, 403, 429)
    │             → fall through to Tier 2b
    │
    ├─ Tier 2b — Cost calculator fallback
    │     Fetch: https://www.alibabacloud.com/price-calculator  (WebFetch, no auth)
    │     On success  → extract visible pricing data → label result documentation-based
    │                   Include note: "Primary pricing page unavailable; estimate from calculator"
    │     On failure  → fall through to Tier 3 cached reference
    │
    ├─ Tier 3 — Cached reference (static fallback of last resort)
    │     Use cached reference data below
    │     Label: documentation-based (stale; pricing may be outdated)
    │     Include note: "All live sources unavailable; price may be stale — verify at
    │                    https://www.alibabacloud.com/cloud-computing/pricing"
    │
    └─ END
```

> **CNY note**: For mainland (`cn-*`) regions, all prices from any tier are in CNY.
> Apply CNY-to-USD conversion using a live rate with timestamp before reporting in USD.
> See [./currency-handling.md](./currency-handling.md) — CNY section for the full
> conversion procedure and mandatory timestamp fields.

### Tier 2a — Primary Pricing Page (WebFetch, no auth)

| Attribute | Value |
|-----------|-------|
| URL | `https://www.alibabacloud.com/cloud-computing/pricing` |
| Auth required | No |
| Parser required | Yes — HTML; no JSON feed |
| Provenance label | `documentation-based` |
| Currency | CNY (mainland `cn-*` regions); USD (international regions) |
| Staleness risk | Medium — page structure may change without notice |

### Tier 2b — Cost Calculator (WebFetch, no auth)

| Attribute | Value |
|-----------|-------|
| URL | `https://www.alibabacloud.com/price-calculator` |
| Auth required | No |
| Provenance label | `documentation-based` |
| Additional note | Include: "Primary pricing page unavailable; estimate derived from calculator page" |

### Tier 3 — Cached Reference (static fallback of last resort)

Use only when both Tier 2a and Tier 2b fetches fail.

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Alibaba Cloud | — |
| Instance type | ecs.t6-c1m1.small | Entry-level burstable instance |
| vCPU | 1 | — |
| RAM | 1 GiB | — |
| Storage | 20 GiB cloud disk | Billed separately |
| Region | cn-shanghai (Mainland China) | CNY region |
| Monthly estimate (CNY) | ~¥130 CNY/month | `documentation-based` (stale; verify before use) |
| Monthly estimate (USD) | ~$18 USD/month | Conversion requires live CNY/USD rate with timestamp |
| USD note | Convert using live CNY/USD rate | See currency-handling.md — CNY section |

> Always note in the output that this figure is a static cached reference and may not
> reflect the current price. Direct the user to
> https://www.alibabacloud.com/cloud-computing/pricing to verify.

---

## Tencent Cloud

No public unauthenticated pricing API exists for Tencent Cloud. The fallback chain starts
at Tier 2 (scrape) since Tier 1 (live authenticated API) is not usable for this skill.
JavaScript rendering may be required on the primary pricing page.

### Decision tree

```
Request arrives
    │
    ├─ Tier 2a — Primary scrape
    │     Fetch: https://cloud.tencent.com/product/cvm/pricing  (WebFetch, no auth)
    │     Note: JavaScript rendering may be required; dynamically loaded price tables
    │     On success  → parse CVM price tables → label result documentation-based
    │     On failure (JS rendering unavailable, HTML structure changed, timeout, 403, 429)
    │             → fall through to Tier 2b
    │
    ├─ Tier 2b — Cost calculator fallback
    │     Fetch: https://cloud.tencent.com/price  (WebFetch, no auth)
    │     Note: JavaScript rendering may also be required here
    │     On success  → extract visible pricing data → label result documentation-based
    │                   Include note: "Primary CVM pricing page unavailable; estimate from calculator"
    │     On failure  → fall through to Tier 3 cached reference
    │
    ├─ Tier 3 — Cached reference (static fallback of last resort)
    │     Use cached reference data below
    │     Label: documentation-based (stale; pricing may be outdated)
    │     Include note: "All live sources unavailable; price may be stale — verify at
    │                    https://cloud.tencent.com/product/cvm/pricing"
    │
    └─ END
```

> **CNY note**: For mainland (`ap-beijing`, `ap-shanghai`, `ap-guangzhou`, and other mainland)
> regions, all prices from any tier are in CNY. Apply CNY-to-USD conversion using a live rate
> with timestamp before reporting in USD. See
> [./currency-handling.md](./currency-handling.md) — CNY section for the full conversion
> procedure and mandatory timestamp fields.

### Tier 2a — Primary CVM Pricing Page (WebFetch, no auth)

| Attribute | Value |
|-----------|-------|
| URL | `https://cloud.tencent.com/product/cvm/pricing` |
| Auth required | No |
| JS rendering | May be required to resolve dynamically loaded price tables |
| Provenance label | `documentation-based` |
| Currency | CNY (mainland regions); USD (international regions) |
| Staleness risk | Medium — page structure may change; JS rendering adds fragility |

### Tier 2b — Cost Calculator (WebFetch, no auth)

| Attribute | Value |
|-----------|-------|
| URL | `https://cloud.tencent.com/price` |
| Auth required | No |
| JS rendering | May be required |
| Provenance label | `documentation-based` |
| Additional note | Include: "Primary CVM pricing page unavailable; estimate derived from calculator page" |

### Tier 3 — Cached Reference (static fallback of last resort)

Use only when both Tier 2a and Tier 2b fetches fail.

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Tencent Cloud | — |
| Instance type | Standard S5.LARGE8 | Standard compute instance |
| vCPU | 2 | — |
| RAM | 8 GiB | — |
| Storage | 50 GiB cloud disk | Billed separately |
| Region | ap-beijing (Beijing, Mainland China) | CNY region |
| Monthly estimate (CNY) | ~¥600 CNY/month | `documentation-based` (stale; verify before use) |
| Monthly estimate (USD) | ~$83 USD/month | Conversion requires live CNY/USD rate with timestamp |
| USD note | Convert using live CNY/USD rate | See currency-handling.md — CNY section |

> Always note in the output that this figure is a static cached reference and may not
> reflect the current price. Direct the user to
> https://cloud.tencent.com/product/cvm/pricing to verify.

---

## CNY→USD Conversion Fallback

Used whenever Alibaba Cloud (mainland `cn-*` regions) or Tencent Cloud (mainland regions)
prices are expressed in CNY and must be converted to USD for reporting.

### Decision tree

```
CNY price obtained (any tier)
    │
    ├─ Tier 1 — ExchangeRate-API (preferred, no auth)
    │     Fetch: https://v6.exchangerate-api.com/v6/latest/CNY
    │     On success  → use CNY-per-USD rate from response
    │                   record: conversion_rate, source_url, timestamp (ISO 8601)
    │     On failure  → fall through to Tier 2
    │
    ├─ Tier 2 — ECB daily feed (EUR base; cross-rate via USD)
    │     Fetch: https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml
    │     Derive CNY/USD cross-rate: CNY_per_USD = (ECB CNY_per_EUR) / (ECB USD_per_EUR)
    │     On success  → use derived rate
    │                   record: conversion_rate, source_url, timestamp (ISO 8601)
    │     On failure  → fall through to Tier 3
    │
    ├─ Tier 3 — Cached rate (stale fallback of last resort)
    │     Use the most recently known CNY/USD rate from this reference file
    │     Label: assumed: 24h stale
    │     Include note: "Exchange rate could not be refreshed; rate may be stale — verify
    │                    at https://www.pbc.gov.cn/ before relying on this conversion"
    │
    └─ END
```

### Mandatory output fields for every CNY→USD conversion

| Field | Type | Example |
|-------|------|---------|
| `conversion_rate` | float (CNY per USD) | `7.25` |
| `source_url` | string | `https://v6.exchangerate-api.com/v6/latest/CNY` |
| `timestamp` | ISO 8601 | `2026-05-13T08:00:00Z` |

If the rate is from Tier 3 (stale), also include:
- `staleness_label`: `assumed: 24h stale`
- `verify_url`: `https://www.pbc.gov.cn/`

### Example labels

Tier 1 or Tier 2 (live rate):
```
[documentation-based + live-rate: 7.25 CNY/USD @ 2026-05-13T08:00:00Z via https://v6.exchangerate-api.com/v6/latest/CNY]
```

Tier 3 (stale cached rate):
```
[documentation-based + assumed: 24h stale rate 7.25 CNY/USD — verify at https://www.pbc.gov.cn/]
```

---

## Fallback Failure Handling

If all available tiers fail for any provider:

1. Return a `fetch-failed` label on the affected line item.
2. State which tiers were attempted and what errors were returned (HTTP status or timeout).
3. Include an explicit uncertainty warning:
   > "Price for {provider} {resource} could not be confirmed. Omitted from total. Retry
   > or consult {pricing-page-url} directly."
4. Do not substitute a guess or a memorized price without a label.
