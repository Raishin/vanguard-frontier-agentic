# Carbon Emission Methodology

## Scope 2 emissions: market-based vs location-based

Scope 2 emissions are indirect greenhouse gas emissions from the generation of purchased electricity consumed by an organization. The GHG Protocol defines two methods for Scope 2 accounting:

### Market-based method

Uses emission factors from contractual instruments that convey attributes of the electricity consumed, such as:
- Renewable Energy Certificates (RECs) or Guarantees of Origin (GOs)
- Power Purchase Agreements (PPAs) with specific generators
- Supplier-specific emission factors published by utilities

Effect: when a cloud provider purchases RECs or PPAs covering 100% of their electricity use in a region, the market-based Scope 2 emission factor for that region approaches zero. This is why hyperscaler-reported Scope 2 figures are often significantly lower than location-based figures.

Default in this skill: **market-based**, because it reflects the cloud provider's actual contractual commitments and is the value reported in vendor sustainability disclosures.

### Location-based method

Uses average emission factors for the electricity grid(s) where consumption occurs, regardless of contractual instruments. These factors are typically published by grid operators or national energy agencies (e.g., EPA eGRID for US, AIB residual mixes for Europe).

The location-based figure represents what emissions would have been if the electricity came from the average mix of sources on the local grid.

Both methods may be required for CSRD (ESRS E1) and GHG Protocol dual-reporting.

## Data lag

Provider carbon data typically lags the billing period by 60-90 days or more:

- AWS Customer Carbon Footprint Tool: approximately 3 months lag.
- Microsoft Emissions Impact Dashboard: approximately 2-3 months lag.
- Google Cloud Carbon Footprint: approximately 2 months lag.
- Third-party (Electricity Maps historical): 24-hour lag for the previous day; monthly averages available sooner.

Implication for disclosure: carbon estimates produced for a quarter's cloud spend will be based on emission factors from the same or a prior quarter. Always state the data period when citing vendor-published factors.

## How to label uncertainty

Every kgCO2e output must carry one of three confidence labels with the stated basis:

| Label | When to use | What to state |
|---|---|---|
| `vendor-published` | Provider's own sustainability API or dashboard data was retrieved | Source URL, data period (e.g., "Q3 2025"), fetch timestamp |
| `third-party` | Electricity Maps or Cloud Carbon Footprint data was used | Source URL, date of data, which grid zone was used |
| `estimated` | Regional or national grid average used as fallback | Emission factor (kgCO2eq/kWh), source (e.g., IEA 2023), year of data |

Do not present a kgCO2e figure without one of these labels. If a value cannot be computed with any of the three approaches, state that a carbon estimate is not available for this region or service category and explain why.

## Conversion from $/spend to kgCO2e

When only a dollar spend value is provided (not resource quantity), a spend-based emission factor is required. These are less precise than activity-based factors (kgCO2e per kWh or per vCPU-hour) because price varies by region, service type, and over time.

If the provider publishes a spend-normalized factor (kgCO2e/$) for the relevant service or region, use it and label as `vendor-published`.

If no spend-normalized factor is available, prefer an activity-based route:
1. Estimate the resource quantity from the spend (e.g., divide by on-demand price per vCPU-hour to get vCPU-hours).
2. Multiply vCPU-hours by the energy use per vCPU-hour (typical: 0.0025 kWh for a modern data center instance).
3. Multiply energy use by the regional emission factor (kgCO2eq/kWh).

State each step and label the result `estimated` unless the emission factor was fetched from a vendor or third-party source.

## Scope 1 emissions (note)

Scope 1 emissions (direct on-site combustion) from cloud data centers — backup generators, on-site equipment — are typically immaterial compared to Scope 2 and are not separately published by cloud providers per customer. Unless the caller specifically requests Scope 1, this skill produces Scope 2 only.

## Regulatory framing

| Framework | Requirement relevant to cloud |
|---|---|
| CSRD / ESRS E1 | Gross Scope 1, 2, and 3 by category; both market-based and location-based Scope 2; alignment with GHG Protocol |
| SEC Climate Rule | Material Scope 1 and 2; Scope 3 if material or targeted; large accelerated filers required to include |
| GHG Protocol Corporate Standard | Both market-based and location-based Scope 2 recommended; Scope 3 Category 1 (purchased goods/services) may include cloud |

Cloud emissions typically appear in:
- Scope 2 (electricity) for compute, storage, and networking in data centers
- Scope 3 Category 1 (purchased goods and services) for cloud spend treated as a supply chain input

This skill computes Scope 2 (electricity) only unless the caller specifies otherwise.
