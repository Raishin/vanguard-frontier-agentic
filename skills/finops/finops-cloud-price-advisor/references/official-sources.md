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

## Exchange Rate Sources

| Source | URL |
|--------|-----|
| ExchangeRate-API (no auth) | `https://open.er-api.com/v6/latest/USD` |
| ECB daily reference rates | `https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml` |

---

## Grounding Rule

When a live API fetch returns a price that differs significantly from the official pricing page, prefer the live API result (it is more current) but note the discrepancy. If the API result appears clearly wrong (e.g., $0.00 or orders-of-magnitude off), fall back to the official pricing page and label the estimate as `documentation-based`.
