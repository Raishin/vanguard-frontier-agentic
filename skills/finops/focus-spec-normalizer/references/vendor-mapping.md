# Vendor to FOCUS Column Mapping Tables

## AWS Cost and Usage Report (CUR) → FOCUS v1.2

AWS CUR column names follow the `<group>/<field>` naming convention (e.g., `lineItem/UnblendedCost`).

| FOCUS column | AWS CUR column(s) | Transformation | Provenance |
|---|---|---|---|
| `BilledCost` | `lineItem/BlendedCost` | Direct map. For consolidated billing accounts, blended cost distributes RI/SP savings across linked accounts. For single accounts, use `lineItem/UnblendedCost`. | mapped |
| `BilledCurrency` | `lineItem/CurrencyCode` | Direct map. | mapped |
| `BillingAccountId` | `bill/PayerAccountId` | Direct map. | mapped |
| `BillingAccountName` | Not in standard CUR | null — not exported by AWS; available in AWS Organizations API only. | null |
| `BillingPeriodStart` | `bill/BillingPeriodStartDate` | Convert to ISO 8601 UTC. | derived |
| `BillingPeriodEnd` | `bill/BillingPeriodEndDate` | Convert to ISO 8601 UTC. | derived |
| `ChargeCategory` | `lineItem/LineItemType` | Map: `Usage` → `Usage`; `Tax` → `Tax`; `Credit` → `Credit`; `RIFee` → `Purchase`; `SavingsPlanCoveredUsage` → `Usage`; `SavingsPlanRecurringFee` → `Purchase`; `Refund` → `Credit`; `EdpDiscount` → `Credit`. | derived |
| `ChargeDescription` | `lineItem/LineItemDescription` | Direct map. | mapped |
| `ChargeFrequency` | `lineItem/LineItemType` | `RIFee`/`SavingsPlanRecurringFee` → `Recurring`; `Usage` → `Usage-based`; tax/credit/fee → `One-time`. | derived |
| `ChargePeriodStart` | `lineItem/UsageStartDate` | Convert to ISO 8601 UTC. | derived |
| `ChargePeriodEnd` | `lineItem/UsageEndDate` | Convert to ISO 8601 UTC. | derived |
| `ConsumedQuantity` | `lineItem/UsageAmount` | Direct map. | mapped |
| `ConsumedUnit` | `pricing/unit` | Direct map. | mapped |
| `EffectiveCost` | `lineItem/UnblendedCost` for on-demand; `savingsPlan/SavingsPlanEffectiveCost` or `reservation/EffectiveCost` for committed. | Map on-demand rows directly; for RI/SP rows, use the amortized effective cost columns. | derived |
| `InvoiceIssuerName` | Static: `"Amazon Web Services, Inc."` | Fixed value for all AWS CUR rows. | derived |
| `ListCost` | `pricing/publicOnDemandCost` | Direct map when present; null if column not enabled in CUR config. | mapped |
| `ListUnitPrice` | `pricing/publicOnDemandRate` | Direct map when present; null if column not enabled. | mapped |
| `PricingCategory` | `lineItem/LineItemType` | `Usage` → `Standard`; `SavingsPlanCoveredUsage`/`DiscountedUsage` → `Committed`; `Spot` → `Dynamic`; other → `Other`. | derived |
| `PricingQuantity` | `lineItem/UsageAmount` | Same as ConsumedQuantity for most rows. | mapped |
| `PricingUnit` | `pricing/unit` | Direct map. | mapped |
| `ProviderName` | Static: `"Amazon Web Services"` | Fixed value. | derived |
| `PublisherName` | Static: `"Amazon Web Services"` | Fixed value. | derived |
| `Region` | `product/region` | Direct map. | mapped |
| `ServiceCategory` | `lineItem/ProductCode` | Map product code to FOCUS ServiceCategory (see note below). | derived |
| `ServiceName` | `product/ProductName` | Direct map. | mapped |
| `SkuId` | `lineItem/UsageType` | Direct map. | mapped |
| `SkuPriceId` | Not in standard CUR | null — AWS does not export a SkuPriceId equivalent in CUR. | null |
| `SubAccountId` | `lineItem/UsageAccountId` | Direct map. | mapped |
| `SubAccountName` | Not in standard CUR | null — account name not in CUR; available via Organizations API. | null |
| `AvailabilityZone` | `lineItem/AvailabilityZone` | Direct map when present. | mapped |
| `ResourceId` | `lineItem/ResourceId` | Direct map when present (requires resource IDs enabled in CUR). | mapped |
| `ResourceName` | Not in standard CUR | null — resource names not exported by AWS CUR. | null |
| `ResourceType` | `product/instanceType` or `product/productFamily` | Use `product/instanceType` for compute; `product/productFamily` otherwise. | derived |
| `Tags` | `resourceTags/<user:key>` columns | Flatten `resourceTags/user:<key>` column prefix to map of key-value pairs. | derived |

### AWS ServiceCategory mapping

| AWS ProductCode | FOCUS ServiceCategory |
|---|---|
| `AmazonEC2` | Compute |
| `AmazonEKS` | Containers |
| `AmazonECS` | Containers |
| `AmazonS3` | Storage |
| `AmazonRDS` | Databases |
| `AmazonDynamoDB` | Databases |
| `AmazonCloudFront` | Networking |
| `AmazonVPC` | Networking |
| `AWSLambda` | Compute |
| `AmazonBedrock` | AI and Machine Learning |
| `AmazonSageMaker` | AI and Machine Learning |
| `AmazonRedshift` | Analytics |
| `AmazonAthena` | Analytics |
| `AWSGlue` | Analytics |
| `AmazonSNS` | Integration |
| `AmazonSQS` | Integration |
| Other | Other |

---

## Azure Cost Management Export → FOCUS v1.2

Azure Cost Management export columns vary by export type (ActualCost vs AmortizedCost). The table below assumes the standard ActualCost export schema.

| FOCUS column | Azure Cost Management column(s) | Transformation | Provenance |
|---|---|---|---|
| `BilledCost` | `CostInBillingCurrency` | Direct map. | mapped |
| `BilledCurrency` | `BillingCurrencyCode` | Direct map. | mapped |
| `BillingAccountId` | `BillingAccountId` | Direct map. | mapped |
| `BillingAccountName` | `BillingAccountName` | Direct map. | mapped |
| `BillingPeriodStart` | `BillingPeriodStartDate` | Convert to ISO 8601 UTC. | derived |
| `BillingPeriodEnd` | `BillingPeriodEndDate` | Convert to ISO 8601 UTC. | derived |
| `ChargeCategory` | `ChargeType` | Map: `Usage` → `Usage`; `Purchase` → `Purchase`; `Tax` → `Tax`; `Refund` → `Credit`; `Adjustment` → `Adjustment`. | derived |
| `ChargeDescription` | `MeterName` + `MeterCategory` | Concatenate: `<MeterCategory> - <MeterName>`. | derived |
| `ChargeFrequency` | `ChargeType` | `Usage` → `Usage-based`; `Purchase` → `Recurring` or `One-time` depending on Frequency field. | derived |
| `ChargePeriodStart` | `Date` | Parse as start of day UTC. | derived |
| `ChargePeriodEnd` | `Date` | Parse as end of day UTC (start + 24h). | derived |
| `ConsumedQuantity` | `Quantity` | Direct map. | mapped |
| `ConsumedUnit` | `UnitOfMeasure` | Direct map. | mapped |
| `EffectiveCost` | `CostInBillingCurrency` for ActualCost export; `AmortizedCost` for AmortizedCost export. | For RI/savings plan amortization, use the AmortizedCost export. | mapped/derived |
| `InvoiceIssuerName` | Static: `"Microsoft"` | Fixed value. | derived |
| `ListCost` | `PayGPrice` × `Quantity` when both available | Multiply if PayGPrice column is present. | derived |
| `ListUnitPrice` | `PayGPrice` | Direct map when present. | mapped |
| `PricingCategory` | `PricingModel` | Map: `OnDemand` → `Standard`; `Spot` → `Dynamic`; `Reservation` → `Committed`; `SavingsPlan` → `Committed`. | derived |
| `PricingQuantity` | `Quantity` | Direct map. | mapped |
| `PricingUnit` | `UnitOfMeasure` | Direct map. | mapped |
| `ProviderName` | Static: `"Microsoft Azure"` | Fixed value. | derived |
| `PublisherName` | `PublisherName` | Direct map when present; fall back to `"Microsoft"`. | mapped |
| `Region` | `ResourceLocation` | Direct map. | mapped |
| `ServiceCategory` | `ServiceFamily` | Map ServiceFamily to FOCUS ServiceCategory (see note below). | derived |
| `ServiceName` | `ConsumedService` | Direct map. | mapped |
| `SkuId` | `MeterId` | Direct map. | mapped |
| `SkuPriceId` | `PartNumber` | Direct map when present. | mapped |
| `SubAccountId` | `SubscriptionId` | Direct map. | mapped |
| `SubAccountName` | `SubscriptionName` | Direct map. | mapped |
| `AvailabilityZone` | Not in standard export | null — Azure does not export AZ in Cost Management export by default. | null |
| `ResourceId` | `ResourceId` | Direct map. | mapped |
| `ResourceName` | `ResourceName` | Direct map. | mapped |
| `ResourceType` | `ResourceType` | Direct map. | mapped |
| `Tags` | `Tags` | Parse JSON tag string into key-value map. | derived |

### Azure ServiceFamily to FOCUS ServiceCategory

| Azure ServiceFamily | FOCUS ServiceCategory |
|---|---|
| Compute | Compute |
| Containers | Containers |
| Storage | Storage |
| Databases | Databases |
| Networking | Networking |
| AI + Machine Learning | AI and Machine Learning |
| Analytics | Analytics |
| Security | Security |
| Identity | Identity |
| Integration | Integration |
| Developer Tools | Developer Tools |
| Management and Governance | Management and Governance |
| Other | Other |

---

## GCP Billing Export (BigQuery) → FOCUS v1.2

GCP Billing Export uses nested/repeated fields in BigQuery. The table below uses dot notation for nested fields.

| FOCUS column | GCP Billing Export field(s) | Transformation | Provenance |
|---|---|---|---|
| `BilledCost` | `cost` + sum of applicable `credits[].amount` | Add credit amounts (negative values) to cost. | derived |
| `BilledCurrency` | `currency` | Direct map. | mapped |
| `BillingAccountId` | `billing_account_id` | Direct map. | mapped |
| `BillingAccountName` | Not in standard export | null — account name not included. | null |
| `BillingPeriodStart` | `invoice.month` | Parse YYYYMM as first day of month UTC. | derived |
| `BillingPeriodEnd` | `invoice.month` | Parse YYYYMM as first day of next month UTC. | derived |
| `ChargeCategory` | `cost_type` | Map: `regular` → `Usage`; `tax` → `Tax`; `adjustment` → `Adjustment`; `rounding_error` → `Adjustment`. | derived |
| `ChargeDescription` | `sku.description` | Direct map. | mapped |
| `ChargeFrequency` | `cost_type` | `regular` → `Usage-based`; `tax` → `One-time`. | derived |
| `ChargePeriodStart` | `usage_start_time` | Convert to ISO 8601 UTC. | derived |
| `ChargePeriodEnd` | `usage_end_time` | Convert to ISO 8601 UTC. | derived |
| `ConsumedQuantity` | `usage.amount` | Direct map. | mapped |
| `ConsumedUnit` | `usage.unit` | Direct map. | mapped |
| `EffectiveCost` | `cost` | GCP exports net cost after credits applied; same as BilledCost in most cases. | mapped |
| `InvoiceIssuerName` | Static: `"Google"` | Fixed value. | derived |
| `ListCost` | `cost_at_list` (if present in resource-level export) | Direct map when available; null otherwise. | mapped |
| `ListUnitPrice` | Not in standard export | null — not available in standard BigQuery export. | null |
| `PricingCategory` | `credits[].type` | If credits include `COMMITTED_USAGE_DISCOUNT`: `Committed`; else `Standard`. | derived |
| `PricingQuantity` | `usage.amount_in_pricing_units` | Direct map. | mapped |
| `PricingUnit` | `usage.pricing_unit` | Direct map. | mapped |
| `ProviderName` | Static: `"Google Cloud"` | Fixed value. | derived |
| `PublisherName` | Static: `"Google Cloud"` | Fixed value. | derived |
| `Region` | `location.region` | Direct map. | mapped |
| `ServiceCategory` | `service.description` | Map service name to FOCUS ServiceCategory (see note below). | derived |
| `ServiceName` | `service.description` | Direct map. | mapped |
| `SkuId` | `sku.id` | Direct map. | mapped |
| `SkuPriceId` | Not in standard export | null — not available. | null |
| `SubAccountId` | `project.id` | Direct map. | mapped |
| `SubAccountName` | `project.name` | Direct map. | mapped |
| `AvailabilityZone` | `location.zone` | Direct map. | mapped |
| `ResourceId` | `resource.name` | Direct map when present. | mapped |
| `ResourceName` | `resource.global_name` | Direct map when present. | mapped |
| `ResourceType` | Not in standard export | null — not available in standard export. | null |
| `Tags` | `labels` (repeated key-value) | Flatten repeated label records into key-value map. | derived |

---

## OCI Billing → FOCUS v1.2

OCI provides billing data through the Cost and Usage Reports (CSV export) and the Usage API.

| FOCUS column | OCI column(s) | Transformation | Provenance |
|---|---|---|---|
| `BilledCost` | `cost/myCost` | Direct map. | mapped |
| `BilledCurrency` | `currency/currencyCode` | Direct map. | mapped |
| `BillingAccountId` | `product/tenancyId` | Direct map (tenancy OCID). | mapped |
| `BillingAccountName` | Not in standard export | null — tenancy name not in CSV export. | null |
| `BillingPeriodStart` | `lineItem/intervalUsageStart` | Convert to ISO 8601 UTC (first day of month for monthly export). | derived |
| `BillingPeriodEnd` | `lineItem/intervalUsageEnd` | Convert to ISO 8601 UTC. | derived |
| `ChargeCategory` | `lineItem/lineItemType` | Map: `USAGE` → `Usage`; `CREDIT` → `Credit`; `TAX` → `Tax`. | derived |
| `ChargeDescription` | `product/resourceName` + `product/serviceName` | Concatenate service and resource name. | derived |
| `ChargeFrequency` | `lineItem/lineItemType` | `USAGE` → `Usage-based`; `CREDIT` → `One-time`. | derived |
| `ChargePeriodStart` | `lineItem/intervalUsageStart` | Convert to ISO 8601 UTC. | derived |
| `ChargePeriodEnd` | `lineItem/intervalUsageEnd` | Convert to ISO 8601 UTC. | derived |
| `ConsumedQuantity` | `usage/consumedQuantity` | Direct map. | mapped |
| `ConsumedUnit` | `usage/consumedQuantityUnits` | Direct map. | mapped |
| `EffectiveCost` | `cost/myCost` | Same as BilledCost for OCI standard export. | mapped |
| `InvoiceIssuerName` | Static: `"Oracle"` | Fixed value. | derived |
| `ListCost` | `cost/unitPrice` × `usage/consumedQuantity` | Compute from unit price and quantity when both present. | derived |
| `ListUnitPrice` | `cost/unitPrice` | Direct map. | mapped |
| `PricingCategory` | Not in standard export | Default to `Standard` unless commitment discount indicators present. | assumed |
| `PricingQuantity` | `usage/billedQuantity` | Direct map when present. | mapped |
| `PricingUnit` | `usage/billedQuantityOverageUnits` or `usage/consumedQuantityUnits` | Use billed quantity unit when present; fall back to consumed unit. | derived |
| `ProviderName` | Static: `"Oracle Cloud Infrastructure"` | Fixed value. | derived |
| `PublisherName` | Static: `"Oracle"` | Fixed value. | derived |
| `Region` | `product/region` | Direct map. | mapped |
| `ServiceCategory` | `product/service` | Map OCI service name to FOCUS ServiceCategory (see note below). | derived |
| `ServiceName` | `product/serviceName` | Direct map. | mapped |
| `SkuId` | `product/skuPartNumber` | Direct map. | mapped |
| `SkuPriceId` | Not in standard export | null — not available. | null |
| `SubAccountId` | `product/compartmentId` | Direct map (compartment OCID). | mapped |
| `SubAccountName` | `product/compartmentName` | Direct map. | mapped |
| `AvailabilityZone` | `product/availabilityDomain` | Direct map. | mapped |
| `ResourceId` | `product/resourceId` | Direct map when present. | mapped |
| `ResourceName` | `product/resourceName` | Direct map. | mapped |
| `ResourceType` | Not in standard export | null — not in OCI cost export. | null |
| `Tags` | `tags/<key>` columns | Flatten tag columns (prefix `tags/`) into key-value map. | derived |

### OCI service to FOCUS ServiceCategory

| OCI product/service | FOCUS ServiceCategory |
|---|---|
| Compute | Compute |
| Container Engine for Kubernetes | Containers |
| Object Storage | Storage |
| Block Storage | Storage |
| Database | Databases |
| MySQL | Databases |
| NoSQL | Databases |
| Networking | Networking |
| Load Balancing | Networking |
| Generative AI | AI and Machine Learning |
| Data Science | AI and Machine Learning |
| Data Flow | Analytics |
| Other | Other |
