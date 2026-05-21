# FOCUS v1.2 Column Definitions

Source: https://focus.finops.org/the-current-release/

All columns below are part of the FOCUS v1.2 specification. Required columns must be present in every conformant billing row. Conditional columns are required when the described condition is true. Recommended columns are optional but strongly encouraged.

## Required columns

| Column | Type | Description | Example |
|---|---|---|---|
| `BilledCost` | Decimal | Amount charged to the billing account for the charge after any discounts, credits, and tax. | `42.50` |
| `BilledCurrency` | String | Currency of BilledCost. ISO 4217 alphabetic code. | `"USD"` |
| `BillingAccountId` | String | Identifier assigned to a billing account by the provider. | `"123456789012"` |
| `BillingAccountName` | String | Display name of the billing account. | `"My AWS Account"` |
| `BillingPeriodEnd` | DateTime (ISO 8601) | Exclusive end date/time of the billing period for the charge. | `"2026-05-01T00:00:00Z"` |
| `BillingPeriodStart` | DateTime (ISO 8601) | Inclusive start date/time of the billing period for the charge. | `"2026-04-01T00:00:00Z"` |
| `ChargeCategory` | String (enum) | High-level classification of the charge type. One of: `Usage`, `Purchase`, `Tax`, `Credit`, `Adjustment`. | `"Usage"` |
| `ChargeDescription` | String | Description of the charge from the provider's billing system. | `"$0.0464 per On Demand Linux m5.large Instance Hour"` |
| `ChargeFrequency` | String (enum) | How often the charge occurs. One of: `Usage-based`, `Recurring`, `One-time`. | `"Usage-based"` |
| `ChargePeriodEnd` | DateTime (ISO 8601) | Exclusive end date/time for the period covered by this charge row. | `"2026-04-02T00:00:00Z"` |
| `ChargePeriodStart` | DateTime (ISO 8601) | Inclusive start date/time for the period covered by this charge row. | `"2026-04-01T00:00:00Z"` |
| `ConsumedQuantity` | Decimal | Amount of the consumed resource. | `730.0` |
| `ConsumedUnit` | String | Unit of ConsumedQuantity. | `"Hrs"` |
| `EffectiveCost` | Decimal | Amortized cost of the charge after all applicable discounts, credits, and amortized upfront fees. | `38.00` |
| `InvoiceIssuerName` | String | Name of the entity that issued the invoice. Often same as ProviderName. | `"Amazon Web Services, Inc."` |
| `ListCost` | Decimal | Charge based on the list unit price without any discounts applied. | `46.00` |
| `ListUnitPrice` | Decimal | Published unit price for the charge without any discounts. | `0.0464` |
| `PricingCategory` | String (enum) | Describes the pricing model of the charge. One of: `Standard`, `Dynamic`, `Committed`, `Other`. | `"Standard"` |
| `PricingQuantity` | Decimal | Amount of the charge using the pricing unit. | `730.0` |
| `PricingUnit` | String | Unit of PricingQuantity. | `"Hrs"` |
| `ProviderName` | String | Name of the cloud provider. | `"Amazon Web Services"` |
| `PublisherName` | String | Name of the entity that published the service. Often same as ProviderName. | `"Amazon Web Services"` |
| `Region` | String | Provider-specific identifier for the geographic area. | `"us-east-1"` |
| `ServiceCategory` | String (enum) | High-level classification of the service type. One of: `AI and Machine Learning`, `Analytics`, `Business Applications`, `Compute`, `Containers`, `Databases`, `Developer Tools`, `Identity`, `Integration`, `IoT`, `Management and Governance`, `Media`, `Migration`, `Mixed`, `Mobile`, `Networking`, `Other`, `Security`, `Storage`, `Web`. | `"Compute"` |
| `ServiceName` | String | Display name of the service. | `"Amazon EC2"` |
| `SkuId` | String | Provider-specific identifier for the SKU. | `"RunInstances:0014"` |
| `SubAccountId` | String | Identifier for the sub-account (AWS account, Azure subscription, GCP project, OCI compartment). | `"987654321098"` |
| `SubAccountName` | String | Display name of the sub-account. | `"production-workloads"` |

## Conditional columns

| Column | Type | Required when | Description |
|---|---|---|---|
| `AvailabilityZone` | String | Resource is zonal | Provider zone identifier within a region. |
| `CommitmentDiscountCategory` | String (enum) | PricingCategory = Committed | Category of the commitment discount: `Spend` or `Usage`. |
| `CommitmentDiscountId` | String | PricingCategory = Committed | Identifier of the commitment discount instrument. |
| `CommitmentDiscountName` | String | PricingCategory = Committed | Display name of the commitment discount instrument. |
| `CommitmentDiscountType` | String | PricingCategory = Committed | Commitment type label (e.g., `Reserved Instance`, `Savings Plan`, `Committed Use Discount`). |
| `ContractedCost` | Decimal | Contracted price differs from list | Cost at the contracted unit price before discounts. |
| `ContractedUnitPrice` | Decimal | Contracted price differs from list | Contracted unit price. |
| `ResourceId` | String | Resource is identifiable | Provider resource identifier. |
| `ResourceName` | String | Resource has a display name | Display name of the resource. |
| `ResourceType` | String | Resource type is classifiable | Provider-specific resource type classification. |
| `SkuPriceId` | String | Provider publishes SKU price IDs | Provider-specific identifier for the unit price of this charge. |
| `Tags` | Map (String → String) | Tags exist on the resource | Key-value pairs of resource tags. |

## Recommended columns

| Column | Type | Description |
|---|---|---|
| `BillingExchangeRate` | Decimal | Exchange rate applied when converting from pricing currency to billing currency. |
| `BillingExchangeRateDate` | DateTime (ISO 8601) | Date/time the exchange rate was set. |
| `InvoiceId` | String | Identifier of the invoice or statement for this charge. |

## Notes on null handling

FOCUS requires that required columns be present in every conformant row. When a vendor billing format does not supply data for a required FOCUS column, set the value to `null` and document the gap. A null in a required column indicates a normalization gap, not a schema violation, as long as it is disclosed.

For `EffectiveCost`, `ListCost`, and `ContractedCost`: many vendors do not provide all three in their standard export formats. AWS CUR provides `lineItem/UnblendedCost` (≈ ListCost for on-demand, ≈ EffectiveCost for RI/SP amortized), but not a separate ListCost column. Document which cost columns map to which FOCUS columns and note any amortization assumptions.
