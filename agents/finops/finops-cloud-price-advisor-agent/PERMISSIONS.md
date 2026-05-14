# Permissions: FinOps Cloud Price Advisor

## Read-only posture

The FinOps Cloud Price Advisor fetches data from **public, unauthenticated** pricing APIs only. It does not read from, write to, or mutate any cloud environment.

No cloud credentials of any kind are required or accepted.

---

## AWS

No IAM permissions required. The AWS Price List API is public:

```
https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/...
```

If the user also wants to enumerate their actual running resources (live-environment mode), they need a **read-only** IAM identity with at minimum:

```json
{
  "Effect": "Allow",
  "Action": [
    "ec2:DescribeInstances",
    "ec2:DescribeVolumes",
    "rds:DescribeDBInstances",
    "s3:ListAllMyBuckets",
    "s3:GetBucketLocation",
    "ecs:ListClusters",
    "ecs:ListServices",
    "eks:ListClusters",
    "lambda:ListFunctions",
    "cloudwatch:GetMetricStatistics"
  ],
  "Resource": "*"
}
```

This agent does **not** need or use billing API access (`ce:GetCostAndUsage`, `ce:GetCostForecast`) — it builds estimates from public list prices, not from actual billing data.

---

## Azure

No Azure RBAC permissions required. The Azure Retail Prices API is public:

```
https://prices.azure.com/api/retail/prices
```

If the user also wants to enumerate their actual running resources (live-environment mode), a read-only Azure role is sufficient:

```json
{
  "Name": "FinOps Price Advisor Reader",
  "IsCustom": true,
  "Actions": [
    "Microsoft.Compute/virtualMachines/read",
    "Microsoft.Compute/disks/read",
    "Microsoft.DBforPostgreSQL/flexibleServers/read",
    "Microsoft.Sql/servers/databases/read",
    "Microsoft.Storage/storageAccounts/read",
    "Microsoft.ContainerService/managedClusters/read",
    "Microsoft.Web/sites/read"
  ],
  "NotActions": [],
  "AssignableScopes": [
    "/subscriptions/<SUBSCRIPTION_ID>"
  ]
}
```

No Cost Management or Billing Reader role is needed — this agent uses public list prices only.

---

## OCI

No OCI IAM permissions required. The OCI public pricing API is unauthenticated:

```
https://apexapps.oracle.com/pls/apex/cloudestimator/r/api/prices
```

If the user also wants to enumerate their actual running resources (live-environment mode), the following OCI policy is sufficient (read-only, compartment-scoped):

```
Allow group FinOpsAdvisorReadOnly to inspect instances in compartment <compartment-name>
Allow group FinOpsAdvisorReadOnly to inspect volumes in compartment <compartment-name>
Allow group FinOpsAdvisorReadOnly to inspect autonomous-databases in compartment <compartment-name>
Allow group FinOpsAdvisorReadOnly to inspect object-family in compartment <compartment-name>
Allow group FinOpsAdvisorReadOnly to inspect clusters in compartment <compartment-name>
```

No cost-analysis or billing policy is needed for public price lookups.

---

## Scaleway

Pricing access is partially authenticated. The Billing API v2 (beta) requires an IAM token. If the user does not provide one, the agent falls back to the public pricing page (no auth required).

- **API (authenticated path):** Scaleway Billing API v2 (beta) — requires user-supplied IAM token.
- **Fallback (unauthenticated path):** <https://www.scaleway.com/en/pricing/>
- **Provenance label when falling back:** `documentation-based`
- **Agent MUST NOT:** store IAM tokens, log keys, or request credentials from the user unprompted.

---

## Gandi

No public unauthenticated pricing API is available. An API key may optionally be supplied by the user for live price lookups.

- **API (authenticated path):** Gandi Price List API — requires user-supplied API key.
- **Fallback (unauthenticated path):** <https://www.gandi.net/en/domain/price/list>
- **Provenance label when key not provided:** `documentation-based`
- **Agent MAY:** Use the key for a single pricing request in the current session; discard immediately after.
- **Agent MUST NOT:** store, log, cache, or transmit the key; ask for the key unprompted (the user must volunteer it); use the key for any purpose other than Gandi pricing.
- **Example permitted acknowledgement:** "Using your Gandi API key to fetch live pricing. The key will not be stored."

---

## Alibaba Cloud

No public unauthenticated pricing API is available. Pricing is fetched by scraping official pricing pages.

- **Scrape targets:**
  - International (USD): <https://www.alibabacloud.com/cloud-computing/pricing>
  - China (CNY): <https://www.aliyun.com/price>
- **Provenance label:** always `documentation-based`
- **Agent MUST NOT:** accept or request an Alibaba Cloud AccessKey ID/Secret; accept RAM role credentials.

---

## Tencent Cloud

No public unauthenticated pricing API is available. Pricing is fetched by scraping official pricing pages.

- **Scrape target:** <https://cloud.tencent.com/product/cvm/pricing> (International, USD)
- **Provenance label:** always `documentation-based`
- **Agent MUST NOT:** accept or request a Tencent Cloud SecretId/SecretKey; accept CAM policy credentials.

---

## Exchange Rate API

No authentication required:

```
https://open.er-api.com/v6/latest/USD
```

Falls back to ECB XML (also public, no auth):

```
https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml
```
