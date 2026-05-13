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

> **Placeholder** — Alibaba Cloud coverage is planned for a future commit.
> The section below records the intended fallback strategy for that implementation.

### Planned decision tree

```
Request arrives
    │
    ├─ Scrape path (Tier 2) — primary for Alibaba
    │     Fetch: https://www.alibabacloud.com/pricing  (WebFetch, no auth)
    │     On success  → label result documentation-based
    │     On failure  → fall through to Tier 3 cached reference
    │
    └─ END (live API requires auth; not yet implemented for this skill)
```

**Note:** Alibaba Cloud's public pricing page is the reliable primary source for this skill.
A live billing API exists but requires authentication that cannot be solicited from users.
This section will be updated when Alibaba Cloud integration is implemented.

---

## Tencent Cloud

> **Placeholder** — Tencent Cloud coverage is planned for a future commit.
> The section below records the intended fallback strategy for that implementation.

### Planned decision tree

```
Request arrives
    │
    ├─ Scrape path (Tier 2) — primary for Tencent
    │     Fetch: https://intl.cloud.tencent.com/pricing  (WebFetch, no auth)
    │     On success  → label result documentation-based
    │     On failure  → fall through to Tier 3 cached reference
    │
    └─ END (live API requires auth; not yet implemented for this skill)
```

**Note:** Tencent Cloud's international pricing page is the reliable primary source for this
skill. A live billing API exists but requires authentication that cannot be solicited from
users. This section will be updated when Tencent Cloud integration is implemented.

---

## Fallback Failure Handling

If all available tiers fail for any provider:

1. Return a `fetch-failed` label on the affected line item.
2. State which tiers were attempted and what errors were returned (HTTP status or timeout).
3. Include an explicit uncertainty warning:
   > "Price for {provider} {resource} could not be confirmed. Omitted from total. Retry
   > or consult {pricing-page-url} directly."
4. Do not substitute a guess or a memorized price without a label.
