# Pricing APIs

Public pricing endpoints for AWS, Azure, OCI, Scaleway, Gandi, Alibaba Cloud, and Tencent Cloud.
Alibaba and Tencent have no unauthenticated public API; pricing is obtained via web scrape.

---

## AWS — Price List API

**Base URL**: `https://pricing.us-east-1.amazonaws.com`

No authentication. No API key. No AWS account needed.

### Service index

```
GET https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/index.json
```

Returns a JSON map of all service codes and their per-service offer file paths.

### Per-service, per-region offer file

```
GET https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/{serviceCode}/current/{regionCode}/index.json
```

Examples:
```
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonEC2/current/us-east-1/index.json
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonRDS/current/us-east-1/index.json
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonS3/current/us-east-1/index.json
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AmazonECS/current/us-east-1/index.json
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/AWSLambda/current/us-east-1/index.json
```

⚠️ These files are very large (EC2 index is tens of MB). Use the CSV variant for scripting:
```
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/{serviceCode}/current/{regionCode}/index.csv
```

### Key response fields (products + terms)

```json
{
  "products": {
    "<sku>": {
      "sku": "...",
      "productFamily": "Compute Instance",
      "attributes": {
        "instanceType": "m5.xlarge",
        "vcpu": "4",
        "memory": "16 GiB",
        "operatingSystem": "Linux",
        "tenancy": "Shared",
        "location": "US East (N. Virginia)"
      }
    }
  },
  "terms": {
    "OnDemand": {
      "<sku>.<offerTermCode>": {
        "priceDimensions": {
          "<rateCode>": {
            "unit": "Hrs",
            "pricePerUnit": { "USD": "0.1920000000" },
            "description": "$0.192 per On Demand Linux m5.xlarge Instance Hour"
          }
        }
      }
    }
  }
}
```

### Common service codes

| Service | Code |
|---------|------|
| EC2 | `AmazonEC2` |
| RDS | `AmazonRDS` |
| S3 | `AmazonS3` |
| Lambda | `AWSLambda` |
| ECS / Fargate | `AmazonECS` |
| EKS | `AmazonEKS` |
| ElastiCache | `AmazonElastiCache` |
| CloudFront | `AmazonCloudFront` |
| DynamoDB | `AmazonDynamoDB` |
| Data Transfer | `AWSDataTransfer` |

### AWS region codes (selected)

| Region | Code |
|--------|------|
| US East (N. Virginia) | `us-east-1` |
| US West (Oregon) | `us-west-2` |
| EU (Ireland) | `eu-west-1` |
| EU (Frankfurt) | `eu-central-1` |
| AP (Singapore) | `ap-southeast-1` |
| AP (Tokyo) | `ap-northeast-1` |

---

## Azure — Retail Prices API

**Base URL**: `https://prices.azure.com/api/retail/prices`

No authentication. No API key. No Azure subscription needed.

### Basic request

```
GET https://prices.azure.com/api/retail/prices?api-version=2023-01-01-preview
```

### With OData filter

```
GET https://prices.azure.com/api/retail/prices?api-version=2023-01-01-preview&$filter={filter}
```

Filter examples:
```
armRegionName eq 'eastus' and skuName eq 'D2s v3' and priceType eq 'Consumption'
armRegionName eq 'eastus' and serviceName eq 'Virtual Machines' and contains(skuName, 'D2s')
armRegionName eq 'westeurope' and serviceName eq 'Azure Database for PostgreSQL'
armRegionName eq 'eastus' and serviceName eq 'Azure Kubernetes Service'
armRegionName eq 'eastus' and serviceName eq 'Storage' and skuName eq 'LRS Data Stored'
```

### Key response fields

```json
{
  "Items": [
    {
      "currencyCode": "USD",
      "tierMinimumUnits": 0.0,
      "retailPrice": 0.096,
      "unitPrice": 0.096,
      "armRegionName": "eastus",
      "location": "US East",
      "effectiveStartDate": "2024-06-01T00:00:00Z",
      "meterId": "...",
      "meterName": "D2s v3",
      "productId": "...",
      "skuId": "...",
      "productName": "Virtual Machines DSv3 Series",
      "skuName": "D2s v3",
      "serviceName": "Virtual Machines",
      "serviceFamily": "Compute",
      "unitOfMeasure": "1 Hour",
      "type": "Consumption",
      "isPrimaryMeterRegion": true,
      "armSkuName": "Standard_D2s_v3"
    }
  ],
  "NextPageLink": "...",
  "Count": 1
}
```

### Key filter fields

| Field | Purpose | Example |
|-------|---------|---------|
| `armRegionName` | Azure region | `eastus`, `westeurope`, `southeastasia` |
| `serviceName` | Service category | `Virtual Machines`, `Storage`, `Azure Kubernetes Service` |
| `skuName` | SKU identifier | `D2s v3`, `P10`, `LRS Data Stored` |
| `priceType` | Pricing model | `Consumption` (pay-as-you-go), `Reservation` |
| `armSkuName` | ARM SKU name (exact) | `Standard_D2s_v3` |

### Azure region codes (selected)

| Region | `armRegionName` |
|--------|----------------|
| East US | `eastus` |
| West US 2 | `westus2` |
| West Europe | `westeurope` |
| North Europe | `northeurope` |
| Southeast Asia | `southeastasia` |
| Japan East | `japaneast` |

---

## OCI — Public Pricing API

**Base URL**: `https://apexapps.oracle.com/pls/apex/cloudestimator/r/api`

No authentication required for public list prices.

### All prices endpoint

```
GET https://apexapps.oracle.com/pls/apex/cloudestimator/r/api/prices
```

Returns a JSON array with all OCI service SKUs and their list prices.

### Key response fields

```json
{
  "items": [
    {
      "partNumber": "B88317",
      "displayName": "VM.Standard.E4.Flex - OCPU",
      "currencyCodeLocalizations": [
        {
          "currencyCode": "USD",
          "prices": [
            {
              "model": "PAY_AS_YOU_GO",
              "value": "0.025",
              "unit": "OCPU Per Hour"
            }
          ]
        }
      ]
    }
  ]
}
```

### Alternative: Oracle Cloud Pricing page JSON

```
GET https://www.oracle.com/a/ocom/docs/cloud/oci-price-list.json
```

This is the machine-readable version of the Oracle Cloud Price List. Structure may vary by release.

### Oracle pricing page (human-readable)

```
https://www.oracle.com/cloud/price-list.html
```

### OCI shape pricing pattern

OCI Flex VMs charge separately per OCPU and per GB of memory:
- Compute: OCPU-hour rate × number of OCPUs
- Memory: GB-hour rate × number of GB RAM
- Standard shapes (non-Flex): flat hourly rate per shape

### OCI regions for pricing context

OCI pricing is generally region-independent for compute (same price globally), but data egress and some services do vary. Always confirm whether the target workload has significant egress.

---

## Scaleway — Billing API (beta) / Pricing Page

**Stable fallback**: `https://www.scaleway.com/en/pricing/`

The Scaleway billing catalog API was in active beta as of mid-2025. Until it reaches GA,
treat the official pricing page as the authoritative source for documentation-based estimates.
The beta endpoint is documented below for completeness.

> **Note**: Scaleway pricing is EUR-native. USD conversion must be handled separately using
> a live exchange rate source (see [./official-sources.md](./official-sources.md) — Exchange
> Rate Sources). Always display the EUR price first, then the converted amount with the
> conversion date and rate used.

### Beta billing catalog endpoint

```
GET https://api.scaleway.com/billing/v2beta1/products
X-Auth-Token: <IAM-API-key>
```

Requires a valid Scaleway IAM API key with at minimum `billing:read` permission scope.

### Key response fields (beta)

```json
{
  "products": [
    {
      "name": "instances_b_ssd_x86_64_pro2_xs",
      "display_name": "Production PRO2-XS",
      "region": "fr-par",
      "price": {
        "currency_code": "EUR",
        "units": 0,
        "nanos": 4300000
      },
      "unit_of_measure": "hour"
    }
  ]
}
```

> **Stability warning**: The `billing/v2beta1` endpoint shape may change before GA.
> Always check https://www.scaleway.com/en/changelog/ for updates before integrating.

### Supported resource types (beta coverage)

| Category | Examples |
|----------|---------|
| Compute (Instances) | PRO2, DEV1, GP1 series |
| Block Storage (SBS) | BSSD volumes |
| Object Storage | Scaleway OSS |
| Managed Database (RDB) | PostgreSQL, MySQL managed instances |
| Kubernetes (Kapsule) | Node pool billing |
| Serverless Functions | Invocation + GB-second billing |

### Authentication

| Attribute | Value |
|-----------|-------|
| Method | `X-Auth-Token` header |
| Credential | Scaleway IAM API key (`SCALEWAY_API_KEY`) |
| Minimum scope | `billing:read` |
| Key creation | https://console.scaleway.com/iam/api-keys |

### Rate limits

| Attribute | Value |
|-----------|-------|
| Global limit | ~60 requests/minute (per-route limits not separately documented) |
| Recommended strategy | Single catalog fetch per session; cache results |

### Scaleway region codes

| Region | Code |
|--------|------|
| Paris (France) | `fr-par` |
| Amsterdam (Netherlands) | `nl-ams` |
| Warsaw (Poland) | `pl-waw` |

---

## Gandi — Price List API

**Base URL**: `https://api.gandi.net/v5`

> **Authentication required.** Gandi pricing is not available unauthenticated.
> The agent never stores or logs API keys. User must supply the key explicitly
> in the request. See [../references/provider-fallbacks.md](./provider-fallbacks.md)
> for the full decision tree.

### Price list endpoint

```
GET https://api.gandi.net/v5/price-list
Authorization: Apikey <user-provided-key>
```

**Critical:** Replace `<user-provided-key>` with the key the user explicitly
provided in the current request. Never prompt for credentials and never store
or log any key value.

### Key response fields

```json
[
  {
    "product": {
      "type": "instance",
      "name": "web-server-start2"
    },
    "unit_price": [
      {
        "currency": "EUR",
        "duration": "monthly",
        "price": "2.99"
      },
      {
        "currency": "USD",
        "duration": "monthly",
        "price": "3.27"
      }
    ],
    "description": "VPS Start 2 — 1 vCPU / 2 GB RAM / 20 GB SSD"
  }
]
```

### Authentication

| Attribute | Value |
|-----------|-------|
| Method | `Authorization: Apikey <key>` header |
| Credential | User-provided API key (never stored by agent) |
| Key creation | https://account.gandi.net/en/users/api-keys |
| Fallback if no key | Use official pricing page (label: `documentation-based`) |

### Rate limits

| Attribute | Value |
|-----------|-------|
| Global limit | 100 requests/second |
| Recommended strategy | Single fetch per session; cache results in-context |

### Supported resource types

| Category | Examples |
|----------|---------|
| VPS (Simple Hosting / Cloud) | Start 2, Pro, Business tiers |
| Domain names | TLD-specific pricing (varies per extension) |
| DNS (LiveDNS) | Included with domain; no separate charge |
| Email | Gandi Mail per-mailbox pricing |
| SSL Certificates | DV and EV certificate pricing |
| Object Storage / CDN | Pay-per-GB storage and transfer |

### Gandi currency note

Gandi prices are available in **EUR** and **USD** via the API response.
Always display the EUR price first when both are present. If only one currency
is returned, convert using a live exchange rate (see
[./official-sources.md](./official-sources.md) — Exchange Rate Sources).

---

## Alibaba Cloud — Scrape-Based Pricing

**No public unauthenticated pricing API exists for Alibaba Cloud.**

Pricing is obtained by scraping the official pricing page. All estimates derived
from this source must be labeled `documentation-based`. See
[./provider-fallbacks.md](./provider-fallbacks.md) for the full scrape fallback chain.

### Primary pricing source

```
https://www.alibabacloud.com/cloud-computing/pricing
```

HTML page containing product cards and pricing zones per region.
An HTML parser is required; the page does not expose a JSON feed.

### Cost calculator (secondary source)

```
https://www.alibabacloud.com/price-calculator
```

Use as a fallback when the primary pricing page cannot be parsed.

### Authentication

None required. Public page, no API key.

### Rate limits

No explicit rate limit published. Treat as a standard web scrape:
- Do not send more than one request per session for pricing data.
- Do not retry aggressively on failure; fall back to cached data instead.

### Regions supported

| Region type | Region codes |
|-------------|-------------|
| Mainland China (CNY) | `cn-beijing`, `cn-shanghai`, `cn-zhangjiakou`, `cn-hangzhou`, `cn-shenzhen` |
| Asia-Pacific (USD) | `ap-southeast-1` (Singapore), `ap-northeast-1` (Tokyo), `ap-southeast-5` (Jakarta) |
| Other International (USD) | `us-west-1` (Silicon Valley), `eu-central-1` (Frankfurt) |

> **Currency note**: Mainland `cn-*` regions are priced in CNY. International `ap-*` and
> other non-mainland regions are priced in USD. Apply CNY-to-USD conversion for mainland
> estimates; see [./currency-handling.md](./currency-handling.md) — CNY section.

### Supported products

| Product | Description |
|---------|-------------|
| ECS | Elastic Compute Service (virtual machines) |
| RDS | Relational Database Service (managed database) |
| OSS | Object Storage Service |
| CDN | Content Delivery Network |
| SLB | Server Load Balancer |

---

## Tencent Cloud — Scrape-Based Pricing

**No public unauthenticated pricing API exists for Tencent Cloud.**

Pricing is obtained by scraping the official pricing page. JavaScript rendering
may be required for some product pages. All estimates must be labeled
`documentation-based`. See [./provider-fallbacks.md](./provider-fallbacks.md)
for the full scrape fallback chain.

### Primary pricing source

```
https://cloud.tencent.com/product/cvm/pricing
```

CVM (Cloud Virtual Machine) pricing page. JavaScript rendering may be required
to resolve dynamically loaded price tables.

### Cost calculator (secondary source)

```
https://cloud.tencent.com/price
```

Use as a fallback when the primary pricing page cannot be parsed.

### Authentication

None required. Public page, no API key.

### Rate limits

No explicit rate limit published. Treat as a standard web scrape:
- Do not send more than one request per session for pricing data.
- Do not retry aggressively on failure; fall back to cached data instead.

### Regions supported

| Region type | Region codes |
|-------------|-------------|
| Mainland China (CNY) | `ap-beijing`, `ap-shanghai`, `ap-guangzhou`, `ap-chengdu`, `ap-nanjing` |
| Asia-Pacific International (USD) | `ap-singapore`, `ap-tokyo`, `ap-seoul`, `ap-bangkok`, `ap-mumbai` |
| Other International (USD) | `na-ashburn` (East US), `eu-frankfurt` (Germany) |

> **Currency note**: Mainland `ap-beijing`, `ap-shanghai`, `ap-guangzhou` (and other
> mainland regions) are priced in CNY. International regions are priced in USD. Apply
> CNY-to-USD conversion for mainland estimates; see
> [./currency-handling.md](./currency-handling.md) — CNY section.

### Supported products

| Product | Description |
|---------|-------------|
| CVM | Cloud Virtual Machine (compute instances) |
| TencentDB | Managed relational database (MySQL, PostgreSQL, etc.) |
| COS | Cloud Object Storage |
| CLB | Cloud Load Balancer |
| TKE | Tencent Kubernetes Engine |

---

## Pricing API Comparison

| Feature | AWS | Azure | OCI | Scaleway | Gandi | Alibaba | Tencent |
|---------|-----|-------|-----|---------|-------|---------|---------|
| Auth required | No | No | No | Yes (IAM token) | Yes (user-provided key) | No (scrape) | No (scrape) |
| Filter by region | Yes (URL path) | Yes (OData) | N/A (global) | Yes (response field) | N/A (global list) | N/A (parse page) | N/A (parse page) |
| Filter by SKU | Via JSON parse | OData `skuName` | JSON parse | JSON parse | JSON parse | HTML parse | HTML parse |
| Unit of measure | Per hour | Per hour | Per hour / OCPU | Per hour | Per month (primary) | Per hour / month | Per hour / month |
| Currency in response | USD only | USD (+ native via param) | USD | EUR only | EUR and USD | CNY (mainland), USD (intl) | CNY (mainland), USD (intl) |
| Real-time | Yes | Yes | Yes | Beta (stability low-medium) | Yes (auth required) | No (scrape, may be stale) | No (scrape, may be stale) |
| Notes | Large files; prefer region-scoped | Best developer experience; OData is powerful | Flat list; Flex shapes split OCPU + memory | Beta endpoint; use pricing page as fallback; EUR conversion required | User must supply API key; fallback to docs page if no key provided | Scrape-based; CNY conversion required for mainland regions; label `documentation-based` | Scrape-based; JS rendering may be needed; CNY conversion required for mainland regions; label `documentation-based` |

---

## WebFetch Usage Notes

When calling these endpoints via WebFetch:
- AWS EC2 `index.json` for a single region is very large. Fetch the CSV variant or use the JSON and filter in-context.
- Azure API returns paginated results; follow `NextPageLink` if present.
- OCI API returns a single large array; filter by `displayName` substring or `partNumber` after fetch.
- Scaleway billing API (`/billing/v2beta1/products`) requires an `X-Auth-Token` header. If no token is available, fall back to the official pricing page and label the estimate as `documentation-based`. The beta endpoint may return `404` or an undocumented error shape before GA.
- Gandi price list API (`/v5/price-list`) requires `Authorization: Apikey <key>`. If the user has not provided a key in the current request, do not prompt — fall back to the official pricing page (https://www.gandi.net/domain/pricing) and label the estimate as `documentation-based`. If a user-provided key is present, log: "User-provided API key received; using live pricing. Key will not be stored." then discard the key after the fetch.
- Alibaba Cloud pricing page (`https://www.alibabacloud.com/cloud-computing/pricing`) is scrape-based; no JSON API exists. Parse HTML product cards. If the page structure has changed or the fetch fails, fall back to the price calculator page, then to a cached documentation-based estimate. Label all Alibaba prices as `documentation-based`. For mainland (`cn-*`) regions, always apply a CNY-to-USD conversion with a live rate and timestamp.
- Tencent Cloud pricing page (`https://cloud.tencent.com/product/cvm/pricing`) is scrape-based; JavaScript rendering may be required. If the primary page fails, fall back to `https://cloud.tencent.com/price`, then to a cached documentation-based estimate. Label all Tencent prices as `documentation-based`. For mainland (`ap-beijing`, `ap-shanghai`, `ap-guangzhou`) regions, always apply a CNY-to-USD conversion with a live rate and timestamp.
- If a fetch fails (network timeout, 403, 429), label the result as `fetch-failed` and fall back to documentation-based estimate with explicit uncertainty warning.
