# Pricing APIs

Public, unauthenticated pricing endpoints for AWS, Azure, and OCI.

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

## Pricing API Comparison

| Feature | AWS | Azure | OCI |
|---------|-----|-------|-----|
| Auth required | No | No | No |
| Filter by region | Yes (URL path) | Yes (OData) | N/A (global) |
| Filter by SKU | Via JSON parse | OData `skuName` | JSON parse |
| Unit of measure | Per hour | Per hour | Per hour / OCPU |
| Currency in response | USD only | USD (and others via `currencyCode`) | USD |
| Real-time | Yes | Yes | Yes |
| Notes | Large files; prefer region-scoped | Best developer experience; OData is powerful | Flat list; Flex shapes split OCPU + memory |

---

## WebFetch Usage Notes

When calling these endpoints via WebFetch:
- AWS EC2 `index.json` for a single region is very large. Fetch the CSV variant or use the JSON and filter in-context.
- Azure API returns paginated results; follow `NextPageLink` if present.
- OCI API returns a single large array; filter by `displayName` substring or `partNumber` after fetch.
- If a fetch fails (network timeout, 403, 429), label the result as `fetch-failed` and fall back to documentation-based estimate with explicit uncertainty warning.
