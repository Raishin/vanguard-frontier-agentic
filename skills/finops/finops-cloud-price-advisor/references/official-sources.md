# Official Sources

Authoritative pricing documentation for each cloud provider. Use these as ground truth when live API results are ambiguous or unavailable.

---

## AWS

| Resource | URL |
|----------|-----|
| Price List API overview | `https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/price-changes.html` |
| Price List API reference | `https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/using-ppslong.html` |
| EC2 pricing page | `https://aws.amazon.com/ec2/pricing/on-demand/` |
| RDS pricing page | `https://aws.amazon.com/rds/pricing/` |
| S3 pricing page | `https://aws.amazon.com/s3/pricing/` |
| Lambda pricing page | `https://aws.amazon.com/lambda/pricing/` |
| EKS pricing page | `https://aws.amazon.com/eks/pricing/` |
| Fargate pricing page | `https://aws.amazon.com/fargate/pricing/` |
| Data Transfer pricing | `https://aws.amazon.com/ec2/pricing/on-demand/#Data_Transfer` |
| Price List service index | `https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/index.json` |
| AWS Pricing Calculator | `https://calculator.aws/pricing/2/home` |

## Azure

| Resource | URL |
|----------|-----|
| Retail Prices API docs | `https://learn.microsoft.com/en-us/rest/api/cost-management/retail-prices/azure-retail-prices` |
| VM pricing page | `https://azure.microsoft.com/en-us/pricing/details/virtual-machines/linux/` |
| AKS pricing page | `https://azure.microsoft.com/en-us/pricing/details/kubernetes-service/` |
| Azure SQL pricing | `https://azure.microsoft.com/en-us/pricing/details/azure-sql-database/single/` |
| Azure PostgreSQL Flexible | `https://azure.microsoft.com/en-us/pricing/details/postgresql/flexible-server/` |
| Blob Storage pricing | `https://azure.microsoft.com/en-us/pricing/details/storage/blobs/` |
| Bandwidth pricing | `https://azure.microsoft.com/en-us/pricing/details/bandwidth/` |
| Azure Functions pricing | `https://azure.microsoft.com/en-us/pricing/details/functions/` |
| Azure Pricing Calculator | `https://azure.microsoft.com/en-us/pricing/calculator/` |
| Cost Management overview | `https://learn.microsoft.com/en-us/azure/cost-management-billing/cost-management-billing-overview` |

## OCI

| Resource | URL |
|----------|-----|
| OCI Price List (HTML) | `https://www.oracle.com/cloud/price-list.html` |
| OCI Cost Analysis overview | `https://docs.oracle.com/en-us/iaas/Content/Billing/Concepts/costanalysisoverview.htm` |
| OCI Compute shapes | `https://docs.oracle.com/en-us/iaas/Content/Compute/References/computeshapes.htm` |
| OCI Autonomous Database pricing | `https://www.oracle.com/autonomous-database/pricing/` |
| OCI Object Storage pricing | `https://www.oracle.com/cloud/storage/object-storage/pricing/` |
| OCI Networking / egress pricing | `https://www.oracle.com/cloud/networking/pricing/` |
| OCI Cloud Estimator (calculator) | `https://cloudestimator.oracle.com` |
| OCI Cost and Usage API | `https://docs.oracle.com/en-us/iaas/Content/Billing/Tasks/costanalysis_topic-create_report.htm` |

## Scaleway

| Resource | URL | Status | Currency |
|----------|-----|--------|---------|
| Official pricing page | `https://www.scaleway.com/en/pricing/` | Production, public, no auth | EUR |
| Billing API reference | `https://developer.scaleway.com/en/products/billing/api/` | Beta (auth required) | EUR |
| Developer documentation | `https://www.scaleway.com/en/developers/api/` | Production | N/A |
| Changelog | `https://www.scaleway.com/en/changelog/` | Production | N/A |
| API key management | `https://console.scaleway.com/iam/api-keys` | Production | N/A |
| Scaleway Pricing Calculator | `https://www.scaleway.com/en/cost-calculator/` | Production | EUR |

> Scaleway pricing is EUR-native. No USD pricing is available via the API. Use a live
> exchange rate source (see Exchange Rate Sources below) to convert to USD or other
> currencies. The `billing/v2beta1` API endpoint requires a Scaleway IAM token; the
> official pricing page is the reliable unauthenticated fallback. Verify beta API status
> at https://www.scaleway.com/en/changelog/ before any integration.

## Gandi

| Resource | URL | Status | Currency | Key management |
|----------|-----|--------|---------|----------------|
| Official pricing page | `https://www.gandi.net/domain/pricing` | Production, public, no auth | EUR, USD | N/A — public page |
| Price List API | `https://api.gandi.net/v5/price-list` | Production, auth required | EUR, USD | User-provided API key (never stored by agent) |
| API documentation | `https://api.gandi.net/docs/` | Production | N/A | N/A |
| LiveDNS documentation | `https://doc.livedns.gandi.net/` | Production | N/A | N/A |
| API key management | `https://account.gandi.net/en/users/api-keys` | Production | N/A | User manages their own keys |

> Gandi pricing is available in both EUR and USD via the API response. The Price List API
> requires a user-provided API key (`Authorization: Apikey <key>`). The agent never prompts
> for, stores, or logs API keys. If no key is supplied in the request, fall back to the
> official pricing page and label the estimate as `documentation-based`.
> See [./provider-fallbacks.md](./provider-fallbacks.md) for the full decision tree.

## Alibaba Cloud

| Resource | URL | Status | Currency |
|----------|-----|--------|---------|
| Official pricing page | `https://www.alibabacloud.com/cloud-computing/pricing` | Production, scrape-based (HTML parser required) | CNY (mainland), USD (International endpoint) |
| Cost calculator | `https://www.alibabacloud.com/price-calculator` | Production, scrape-based (HTML parser required) | CNY (mainland), USD (International endpoint) |
| ECS product page | `https://www.alibabacloud.com/product/ecs` | Production | N/A |
| RDS product page | `https://www.alibabacloud.com/product/apsaradb-for-rds` | Production | N/A |
| OSS product page | `https://www.alibabacloud.com/product/oss` | Production | N/A |

> Alibaba Cloud has no public unauthenticated pricing API. Pricing is obtained by scraping
> the official pricing page via WebFetch. An HTML parser is required; the page does not
> expose a JSON feed. All prices must be labeled `documentation-based`. For mainland
> (`cn-*`) regions, prices are in CNY; apply a CNY-to-USD conversion with a live rate and
> timestamp (see CNY section below). International (`ap-*`, `us-*`, `eu-*`) regions are
> priced in USD. See [./provider-fallbacks.md](./provider-fallbacks.md) for the full
> scrape fallback chain.

## Tencent Cloud

| Resource | URL | Status | Currency |
|----------|-----|--------|---------|
| CVM pricing page | `https://cloud.tencent.com/product/cvm/pricing` | Production, scrape-based (JavaScript rendering may be required) | CNY (mainland), USD (International endpoint) |
| Cost calculator | `https://cloud.tencent.com/price` | Production, scrape-based (JavaScript rendering may be required) | CNY (mainland), USD (International endpoint) |
| TencentDB product page | `https://cloud.tencent.com/product/cdb` | Production | N/A |
| COS product page | `https://cloud.tencent.com/product/cos` | Production | N/A |
| CLB product page | `https://cloud.tencent.com/product/clb` | Production | N/A |

> Tencent Cloud has no public unauthenticated pricing API. Pricing is obtained by scraping
> the official CVM pricing page via WebFetch. JavaScript rendering may be required to
> resolve dynamically loaded price tables. All prices must be labeled `documentation-based`.
> For mainland (`ap-beijing`, `ap-shanghai`, `ap-guangzhou`, and other mainland) regions,
> prices are in CNY; apply a CNY-to-USD conversion with a live rate and timestamp (see CNY
> section below). International regions are priced in USD. See
> [./provider-fallbacks.md](./provider-fallbacks.md) for the full scrape fallback chain.

## Exchange Rate Sources

| Source | URL | Auth | Notes |
|--------|-----|------|-------|
| ExchangeRate-API (preferred) | `https://open.er-api.com/v6/latest/USD` | None | Major currencies; updated daily |
| ECB daily reference rates (fallback) | `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml` | None | EUR-denominated; reliable but limited currency set |
| ExchangeRate-API CNY endpoint | `https://v6.exchangerate-api.com/v6/latest/CNY` | None | CNY-denominated; use for CNY→USD conversion; updated daily |
| PBoC daily rate (CNY cached fallback) | `https://www.pbc.gov.cn/` | None | People's Bank of China published daily rate; use when ExchangeRate-API is unavailable |

Do not use sources that require API keys (e.g., openexchangerates.org). The agent must not accept or store API keys.

---

## Grounding Rule

When a live API fetch returns a price that differs significantly from the official pricing page, prefer the live API result (it is more current) but note the discrepancy. If the API result appears clearly wrong (e.g., $0.00 or orders-of-magnitude off), fall back to the official pricing page and label the estimate as `documentation-based`.
