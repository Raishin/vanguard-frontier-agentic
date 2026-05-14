# Currency Handling

## Default: USD

All cloud pricing APIs return prices in USD by default. Unless the user explicitly requests a different currency, return all estimates in USD.

State the currency clearly in the output header:
```
Currency: USD (on-demand list price, no discounts applied)
```

---

## Other Currencies — User Request

When the user asks for a non-USD estimate:

1. Fetch the USD price from the cloud pricing API.
2. Convert using an exchange rate from one of these public sources (WebFetch):
   - **Preferred** — ExchangeRate-API free endpoint: `https://open.er-api.com/v6/latest/USD` (no auth, returns JSON with major currencies).
   - Fallback — European Central Bank daily reference rates: `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml` (no auth, EUR-denominated).

> **Do not use** Open Exchange Rates (`openexchangerates.org`) for this skill. It requires
> an `app_id` API key. This agent must not accept or store API keys. The two public
> sources above are sufficient for approximation.

### Preferred approach (ExchangeRate-API, no auth)

```
GET https://open.er-api.com/v6/latest/USD
```

Response:
```json
{
  "base_code": "USD",
  "time_last_update_utc": "2026-04-30 00:02:01",
  "rates": {
    "EUR": 0.9245,
    "GBP": 0.7931,
    "JPY": 144.52,
    "AUD": 1.5521,
    "CAD": 1.3802,
    "SGD": 1.3357,
    "HKD": 7.7823,
    "BRL": 5.6741,
    "INR": 83.47
  }
}
```

Usage: `amount_in_target_currency = usd_price × rates[TARGET_CURRENCY_CODE]`

### Labelling converted amounts

Always show both the USD source and the converted amount:

```
Monthly cost: $234.50 USD → €216.84 EUR (ECB rate 2026-04-30: 1 USD = 0.9245 EUR)
```

Never present a converted price without disclosing the exchange rate and its date.

---

## CNY (Chinese Yuan Renminbi)

### When CNY applies

Alibaba Cloud and Tencent Cloud price their mainland China regions in CNY:

| Provider | CNY regions |
|----------|------------|
| Alibaba Cloud | `cn-beijing`, `cn-shanghai`, `cn-zhangjiakou`, `cn-hangzhou`, `cn-shenzhen` |
| Tencent Cloud | `ap-beijing`, `ap-shanghai`, `ap-guangzhou`, `ap-chengdu`, `ap-nanjing` |

International regions for both providers (e.g., `ap-southeast-1`, `ap-singapore`) are priced
in USD and do not require CNY conversion.

### CNY→USD conversion

Use the following formula:

```
usd_value = cny_value / exchange_rate
```

Where `exchange_rate` is the CNY-per-USD rate (e.g., 7.24 means ¥7.24 = $1.00).

### Live conversion rate sources

| Source | URL | Auth | Priority |
|--------|-----|------|---------|
| ExchangeRate-API CNY endpoint (preferred) | `https://v6.exchangerate-api.com/v6/latest/CNY` | None | Primary |
| ECB daily feed | `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml` | None | Secondary (EUR base; derive CNY via USD cross-rate) |
| PBoC published daily rate (cached fallback) | `https://www.pbc.gov.cn/` | None | Tertiary — use only when primary and secondary are unavailable |

> Do not use sources that require API keys. The agent must not accept or store API keys.

### Mandatory timestamp requirement

Every CNY→USD conversion output must include all three of the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `conversion_rate` | float | The CNY-per-USD rate applied (e.g., `7.24`) |
| `source_url` | string | The URL of the rate service that was used |
| `timestamp` | ISO 8601 | When the rate was fetched (e.g., `2026-05-13T08:00:00Z`) |

If the rate is from a cached or stale source (more than 24 hours old), label it explicitly:
`assumed: 24h stale` and include the staleness note alongside the converted amount.

### Example output label

```
Monthly cost: ¥130 CNY → $17.94 USD
[documentation-based + live-rate: 7.25 CNY/USD @ 2026-05-13T08:00:00Z via https://v6.exchangerate-api.com/v6/latest/CNY]
```

If using a stale cached rate (tertiary fallback):

```
Monthly cost: ¥130 CNY → $17.94 USD
[documentation-based + assumed: 24h stale rate 7.25 CNY/USD — verify at https://www.pbc.gov.cn/]
```

### Labelling CNY amounts

Always show both the CNY source price and the converted USD amount:

```
Monthly cost: ¥600 CNY → $82.76 USD (PBoC/ExchangeRate-API rate 2026-05-13: 1 USD = 7.25 CNY)
```

Never present a converted CNY price without disclosing the exchange rate and its fetch timestamp.

---

## Azure Retail Prices API — Native Currency Support

The Azure Retail Prices API accepts a `currencyCode` query parameter and returns prices in that currency natively:

```
GET https://prices.azure.com/api/retail/prices?api-version=2023-01-01-preview
    &currencyCode=EUR
    &$filter=armRegionName eq 'westeurope' and serviceName eq 'Virtual Machines'
```

Supported currency codes: EUR, GBP, JPY, AUD, CAD, SGD, HKD, BRL, INR, CHF, SEK, DKK, NOK, KRW, MXN, ZAR, and others. Check the API response; unsupported codes return HTTP 400.

When using the Azure API for non-USD estimates, prefer the native `currencyCode` parameter over post-fetch conversion. Note the effective date from the response.

---

## AWS and OCI — USD Only from API

AWS Price List API and OCI pricing API return USD only. For non-USD on these clouds, use the post-fetch conversion approach above.

---

## Rounding

- Show unit prices to 4 decimal places (e.g., $0.0960/hr).
- Show monthly totals to 2 decimal places (e.g., $70.08/month).
- Annual totals: multiply monthly by 12, round to 2 decimal places.

---

## Disclaimer Template

Include in every non-USD estimate:

> Exchange rate applied: {RATE} (source: {SOURCE}, {DATE}). Cloud list prices are in USD; converted amounts are approximate. Actual billing currency and exchange rate depend on your cloud provider agreement and may differ.
