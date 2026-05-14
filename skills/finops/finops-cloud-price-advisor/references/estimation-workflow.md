# Estimation Workflow

Two distinct modes: **live-environment** (current inventory → cost) and **prototype** (planned spec → cost).

---

## Mode 1 — Live Environment Cost Estimate

Use when: the user wants to know what an existing deployed environment costs based on actual running resources.

### Step 1 — Confirm scope

Before fetching anything:
- Cloud provider(s): AWS / Azure / OCI (or multi-cloud)
- Region(s): confirm exact region; pricing varies
- Resource scope: specific service, entire account/subscription/tenancy, or filtered subset
- Time window: monthly (default) or annual
- Currency: USD (default) or specified

### Step 2 — Inventory

Preferred: user provides a resource list (instance IDs, sizes, quantities).

Fallback: ask user to share a sanitized inventory export:
- AWS: `aws ec2 describe-instances --query 'Reservations[*].Instances[*].[InstanceId,InstanceType,State.Name]'`
- Azure: `az resource list --query '[].{id:id,type:type,location:location}'`
- OCI: `oci compute instance list --compartment-id <ocid> --all --query 'data[*].{id:id,"shape":shape,"state":"lifecycle-state"}'`

Do not ask for raw API responses with secrets, billing account IDs, or customer-specific metadata.

### Step 3 — Fetch live prices

For each unique (resource-type, region) pair in the inventory:
1. Load `references/pricing-apis.md`
2. Call the appropriate public pricing API via WebFetch
3. Record: unit price, unit of measure, effective date, currency (USD)

### Step 4 — Calculate

For each line item:
```
monthly_cost = unit_price_per_hour × hours_per_month × quantity
hours_per_month = 730  (standard FinOps convention: 365 days × 24 hours / 12)
```

For storage (priced per GB-month):
```
monthly_cost = price_per_gb_month × total_gb × quantity
```

For data transfer (priced per GB):
```
monthly_cost = price_per_gb × estimated_monthly_gb_transferred
```

### Step 5 — Output

Return the line-item table and totals. See SKILL.md "Response minimum" for format.

---

## Mode 2 — Prototype Cost Estimate

Use when: the user wants to estimate cost for a planned architecture before provisioning.

### Step 1 — Collect the spec

Ask the user to describe the planned architecture. Minimum needed:
- Resource types (e.g., EC2 m5.xlarge, Azure Standard_D4s_v3, OCI VM.Standard.E4.Flex)
- Quantities (instance count, GB of storage, expected GB data transfer/month)
- Region(s)
- Operating system / license model (Linux / Windows; affects compute pricing)
- Expected usage hours per month (default: 730 = always-on)

For prototype mode, it is acceptable to use reasonable defaults when the user has not specified every detail. Clearly label every assumed value.

### Step 2 — Decompose into billable components

Typical components for a web application prototype:

| Component | Billable units |
|-----------|---------------|
| Compute (VM / container) | CPU-hours |
| OS / license (Windows, RHEL) | Additional per-hour |
| Managed database | vCPU-hours + storage GB-month + backup storage |
| Object storage | GB stored/month + GET/PUT requests + egress GB |
| Load balancer | Per hour + LCU or data-processed GB |
| Container orchestration | Cluster management fee + node hours |
| Serverless functions | Invocations + GB-seconds |
| Egress / data transfer | GB out to internet |
| Monitoring / logging | GB ingested + GB stored |

Only include components that are in the user's spec. Do not inflate with unused services.

### Step 3 — Fetch live prices

Same as Mode 1 Step 3.

### Step 4 — Calculate

Same as Mode 1 Step 4.

Label every assumed value in the output with `[assumed]`.

### Step 5 — Sensitivity summary

For prototype estimates, include a brief sensitivity section:
- What is the biggest cost driver?
- What assumption has the most uncertainty?
- What would change the estimate by >20% if revised?

Example:
```
Biggest driver: RDS db.r6g.xlarge instance ($0.48/hr × 730 hr = $350/month, 42% of total)
Highest uncertainty: data egress volume — assumed 100 GB/month; at 1 TB/month cost doubles
```

---

## Multi-Cloud Comparison

When comparing AWS vs Azure vs OCI vs Scaleway (or any subset) for the same workload:

1. Map resource types to equivalents across clouds:

| Workload | AWS | Azure | OCI | Scaleway | Gandi | Alibaba Cloud | Tencent Cloud |
|---------|-----|-------|-----|---------|-------|---------------|---------------|
| 4 vCPU / 16 GB VM | m5.xlarge | Standard_D4s_v3 | VM.Standard.E4.Flex (4 OCPU, 16 GB) | GP1-M (4 vCPU, 16 GB) | VPS Business (4 vCPU, 16 GB) | ecs.g7.xlarge (4 vCPU, 16 GiB) | Standard S5.LARGE16 (4 vCPU, 16 GiB) |
| Managed PostgreSQL | RDS PostgreSQL db.t4g.medium | Azure Database for PostgreSQL Flexible | OCI MySQL Database / ADB-S | Scaleway RDB PostgreSQL | Not available (use external managed DB) | RDS for PostgreSQL (Basic / High-Availability) | TencentDB for PostgreSQL |
| Object storage | S3 Standard | Azure Blob Storage LRS | OCI Object Storage | Scaleway OSS | Gandi Object Storage | OSS Standard | COS Standard |
| Container cluster | EKS | AKS | OKE | Scaleway Kapsule | Not available (use self-managed) | ACK (Alibaba Container Service for Kubernetes) | TKE (Tencent Kubernetes Engine) |

2. Fetch prices for each cloud in the user's preferred region(s).
3. Present side-by-side comparison table, USD, monthly.
4. Note: OCI often prices differently for Flex shapes (OCPU + memory separately); adjust comparison accordingly.
5. Note: Scaleway pricing is EUR-native; apply a live EUR/USD exchange rate (see
   [./official-sources.md](./official-sources.md) — Exchange Rate Sources) and display the
   EUR price alongside the converted USD amount. Label the conversion date and rate used.
6. Note: Gandi pricing is available in both EUR and USD via the API. If the user has not
   provided an API key, use the official pricing page and label the estimate as
   `documentation-based`. See [./provider-fallbacks.md](./provider-fallbacks.md) for the
   decision tree.
7. Note: Alibaba Cloud pricing is scrape-based (no public unauthenticated API). All prices
   must be labeled `documentation-based`. For mainland (`cn-*`) regions, prices are in CNY;
   apply a CNY-to-USD conversion with a live rate and timestamp (see
   [./currency-handling.md](./currency-handling.md) — CNY section). International regions
   are priced in USD. See [./provider-fallbacks.md](./provider-fallbacks.md) for the full
   scrape fallback chain.
8. Note: Tencent Cloud pricing is scrape-based (no public unauthenticated API). JavaScript
   rendering may be required on the primary pricing page. All prices must be labeled
   `documentation-based`. For mainland (`ap-beijing`, `ap-shanghai`, `ap-guangzhou`) regions,
   prices are in CNY; apply a CNY-to-USD conversion with a live rate and timestamp (see
   [./currency-handling.md](./currency-handling.md) — CNY section). International regions
   are priced in USD. See [./provider-fallbacks.md](./provider-fallbacks.md) for the full
   scrape fallback chain.

### Scaleway reference instance for comparison

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Scaleway | — |
| Instance type | PRO2-XS | Smallest production-grade Scaleway instance |
| vCPU | 2 | — |
| RAM | 8 GiB | — |
| Root storage | 20 GiB SSD (local) | Included in instance price |
| Region | fr-par (Paris, France) | eu-fr |
| Monthly estimate | ~€10–14/month | `documentation-based` (official pricing page) |
| API reference | `instances_b_ssd_x86_64_pro2_xs` | beta billing API SKU name |
| USD note | Convert using live EUR/USD rate | See official-sources.md |

> **Provenance label**: `documentation-based` — price derived from the official Scaleway
> pricing page (https://www.scaleway.com/en/pricing/). The beta billing API requires auth;
> if a live fetch succeeds, upgrade the label to `live-price` and include the timestamp.

### Gandi reference instance for comparison

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Gandi | — |
| Instance type | VPS Start 2 | Smallest standard Gandi VPS tier |
| vCPU | 1 | — |
| RAM | 2 GiB | — |
| Storage | 20 GiB SSD | Included in instance price |
| Region | eu (EU default) | — |
| Monthly estimate | ~€2.99/month | `documentation-based` (official pricing page) |
| USD note | EUR price shown; convert using live EUR/USD rate | See official-sources.md |

> **Provenance label**: `documentation-based` — price derived from the official Gandi
> pricing page (https://www.gandi.net/domain/pricing). If the user supplies an API key
> in the request, call `https://api.gandi.net/v5/price-list` with
> `Authorization: Apikey <key>` and upgrade the label to `live-price`. See
> [./provider-fallbacks.md](./provider-fallbacks.md) for the full decision tree.

### Alibaba Cloud ECS reference instance for comparison

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Alibaba Cloud | — |
| Instance type | ecs.t6-c1m1.small | Entry-level burstable instance |
| vCPU | 1 | — |
| RAM | 1 GiB | — |
| Storage | 20 GiB cloud disk | Not included; billed separately |
| Region | cn-shanghai (Shanghai, Mainland China) | CNY region |
| Monthly estimate (CNY) | ~¥130 CNY/month | `documentation-based` |
| Monthly estimate (USD) | ~$18 USD/month | `documentation-based` + live-rate conversion required |
| Currency note | CNY price shown; convert using live CNY/USD rate with timestamp | See currency-handling.md — CNY section |

> **Provenance label**: `documentation-based` — price derived from the official Alibaba Cloud
> pricing page (https://www.alibabacloud.com/cloud-computing/pricing). No live API is
> available without authentication. CNY-to-USD conversion must use a live rate with timestamp;
> see [./currency-handling.md](./currency-handling.md) — CNY section. Scrape-based; price may
> be stale if the page structure has changed. See
> [./provider-fallbacks.md](./provider-fallbacks.md) for the full fallback chain.

### Tencent Cloud CVM reference instance for comparison

| Field | Value | Provenance |
|-------|-------|-----------|
| Provider | Tencent Cloud | — |
| Instance type | Standard S5.LARGE8 | Standard compute instance |
| vCPU | 2 | — |
| RAM | 8 GiB | — |
| Storage | 50 GiB cloud disk | Not included; billed separately |
| Region | ap-beijing (Beijing, Mainland China) | CNY region |
| Monthly estimate (CNY) | ~¥600 CNY/month | `documentation-based` |
| Monthly estimate (USD) | ~$83 USD/month | `documentation-based` + live-rate conversion required |
| Currency note | CNY price shown; convert using live CNY/USD rate with timestamp | See currency-handling.md — CNY section |

> **Provenance label**: `documentation-based` — price derived from the official Tencent Cloud
> CVM pricing page (https://cloud.tencent.com/product/cvm/pricing). No live API is available
> without authentication. JavaScript rendering may be required to resolve dynamically loaded
> price tables. CNY-to-USD conversion must use a live rate with timestamp; see
> [./currency-handling.md](./currency-handling.md) — CNY section. See
> [./provider-fallbacks.md](./provider-fallbacks.md) for the full fallback chain.

---

## Estimate Quality Labels

Always label estimates with one of:
- `live-price`: price fetched from API in this session with timestamp
- `documentation-based`: price from official pricing page docs (may be weeks old)
- `assumed`: value not provided by user and not fetched; based on typical pattern
- `excluded`: component not included in estimate; state why
