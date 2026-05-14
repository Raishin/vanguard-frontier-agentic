# Carbon Data Sources

## Provider sustainability data (vendor-published)

### AWS — Customer Carbon Footprint Tool

| Resource | URL |
|---|---|
| Tool overview | https://aws.amazon.com/aws-cost-management/aws-customer-carbon-footprint-tool/ |
| Documentation | https://docs.aws.amazon.com/cur/latest/userguide/ccft-overview.html |
| Methodology | https://docs.aws.amazon.com/cur/latest/userguide/ccft-methodology.html |

AWS Customer Carbon Footprint Tool provides estimated carbon emissions for AWS usage in metric tons of CO2 equivalent (MTCO2e) broken down by service, region, and time period. Data is available through the AWS console and the AWS Cost Explorer API.

Important notes:
- Data typically lags approximately 3 months (90 days).
- Figures reflect AWS's own renewable energy purchases (market-based Scope 2).
- Location-based figures are also available in the methodology documentation.
- The tool does not expose a public unauthenticated API; for live fetch, use WebFetch against the public documentation and methodology pages, which publish the aggregate emission factors and regional renewable energy percentages.

For unauthenticated access to AWS carbon methodology data, fetch from:
- https://docs.aws.amazon.com/cur/latest/userguide/ccft-methodology.html
- https://sustainability.aboutamazon.com/

### Microsoft — Cloud for Sustainability Emissions API

| Resource | URL |
|---|---|
| Industry sustainability hub | https://learn.microsoft.com/en-us/industry/sustainability/ |
| Emissions Impact Dashboard | https://www.microsoft.com/en-us/sustainability/emissions-impact-dashboard |
| Microsoft Sustainability Manager docs | https://learn.microsoft.com/en-us/industry/sustainability/sustainability-manager-overview |

Microsoft publishes carbon data through the Emissions Impact Dashboard (Power BI report, not unauthenticated API) and the Microsoft Cloud for Sustainability APIs. For unauthenticated reference data, use:
- https://www.microsoft.com/en-us/sustainability/reports (annual sustainability report with regional energy data)
- https://learn.microsoft.com/en-us/azure/carbon-optimization/ (Azure carbon optimization documentation)

Data lag: typically 2-3 months behind current billing.

### Google Cloud — Carbon Footprint Export

| Resource | URL |
|---|---|
| Documentation | https://cloud.google.com/carbon-footprint/docs |
| Methodology | https://cloud.google.com/carbon-footprint/docs/methodology |
| Region carbon data | https://cloud.google.com/sustainability/region-carbon |

Google Cloud Carbon Footprint provides gross and net carbon emissions per product and region. The regional carbon data page publishes the carbon-free energy percentage per region, which can be combined with regional grid intensity to estimate net Scope 2 emissions.

For unauthenticated reference:
- https://cloud.google.com/sustainability/region-carbon — carbon-free energy percentages per region; fetch this page with WebFetch.
- https://cloud.google.com/carbon-footprint/docs/methodology — describes the market-based and location-based calculation approach.

### OCI — Sustainability Dashboard

| Resource | URL |
|---|---|
| OCI Sustainability overview | https://www.oracle.com/sustainability/ |
| Oracle sustainability report | https://www.oracle.com/corporate/citizenship/ |

OCI does not publish a per-region unauthenticated carbon API equivalent to AWS CCFT or Google Cloud Carbon Footprint. For OCI carbon estimates, fall back to the Electricity Maps regional data for the grid serving the OCI data center, or use the Cloud Carbon Footprint project's OCI emission factors.

## Third-party sources

### Electricity Maps

| Resource | URL |
|---|---|
| Global carbon intensity map | https://www.electricitymaps.com/ |
| API documentation | https://static.electricitymaps.com/api/docs/index.html |
| Open data | https://github.com/electricitymaps/electricitymaps-contrib |

Electricity Maps publishes real-time and historical grid carbon intensity (gCO2eq/kWh) for electricity grids worldwide. This is suitable for location-based Scope 2 calculations. The public API requires an API key for live data; the open data repository on GitHub contains historical zone data accessible without authentication.

For unauthenticated WebFetch, use the public zone data from the GitHub repository.

### Cloud Carbon Footprint Project

| Resource | URL |
|---|---|
| Project site | https://www.cloudcarbonfootprint.org/ |
| Emission factors repository | https://github.com/cloud-carbon-footprint/cloud-carbon-footprint |
| Coefficient data | https://github.com/cloud-carbon-footprint/cloud-carbon-footprint/blob/trunk/packages/gcp/src/domain/GcpFootprintEstimationConstants.ts |

The Cloud Carbon Footprint project publishes open-source emission coefficients for AWS, Azure, and GCP including per-region grid intensity and average embodied carbon for common instance types. These are community-maintained, not vendor-published, and should be labeled `third-party`.

## IEA and national grid averages (fallback)

If neither vendor nor third-party data is available for a region:

| Source | URL |
|---|---|
| IEA CO2 emissions from electricity data | https://www.iea.org/data-and-statistics/data-product/emissions-factors-2023 |
| EPA eGRID (US) | https://www.epa.gov/egrid |

Use the regional or national grid average in kgCO2eq/kWh and multiply by the estimated kWh consumed. Label as `estimated` and state the factor and its source year.

## Source selection priority

When multiple sources are available, use the highest-confidence source available:

1. Vendor-published (provider sustainability API or dashboard documentation for the specific region)
2. Third-party (Electricity Maps or Cloud Carbon Footprint for the specific grid region)
3. Estimated (IEA or EPA national/regional average, clearly labeled)
