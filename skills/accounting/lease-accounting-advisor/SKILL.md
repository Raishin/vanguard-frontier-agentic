---
name: lease-accounting-advisor
description: Multi-jurisdiction lease accounting reference framework covering ASC 842 (US GAAP) and IFRS 16, with additional coverage of UK FRS 102 (2024 periodic review amendments effective 1 Jan 2026), German HGB, JGAAP (ASBJ Statement No. 34, effective FY beginning on/after 1 Apr 2027), CAS No. 21 (China), and Ind AS 116 (India). Covers lease identification, lessee classification (ASC 842 dual model vs. IFRS 16 single finance model), right-of-use asset and lease liability measurement, discount rates (incremental borrowing rate vs. rate implicit in lease), lessor accounting (sales-type / direct-financing / operating), short-term and low-value exemptions, lease modifications and remeasurement, and sale-leaseback transactions. Advisory only — all outputs require external auditor verification for local statutory purposes.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-01"
  category: finance
  lifecycle: experimental
---

# Lease Accounting Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for lease accounting advisory — from lease identification through lessee and lessor classification, measurement, modification, and cross-GAAP comparison tables.

---

## Part 1: Lease Identification

### What Constitutes a Lease (ASC 842 / IFRS 16)

Under both ASC 842 (842-10-15-9) and IFRS 16 (paragraph 9), a contract contains a lease if it conveys the right to control the use of an identified asset for a period of time in exchange for consideration.

**Three criteria must all be met:**

| Criterion | ASC 842 Reference | IFRS 16 Reference |
|---|---|---|
| Identified asset (specific, not substitutable) | 842-10-15-3 | IFRS 16.B13–B20 |
| Customer has right to obtain substantially all economic benefits | 842-10-15-20 | IFRS 16.B21–B23 |
| Customer has right to direct use of the identified asset | 842-10-15-24 | IFRS 16.B24–B30 |

**Supplier substitution right**: If the supplier has a substantive right to substitute the asset throughout the period of use, there is no identified asset and no lease. A substitution right is substantive only if the supplier (a) has the practical ability to substitute and (b) would benefit economically from doing so. (ASC 842-10-15-10; IFRS 16.B14)

### Practical Expedient — Lease and Non-Lease Components

- **ASC 842**: Lessees may elect (by class of underlying asset) to not separate lease and non-lease components — treat as single lease component. (ASC 842-10-15-42A)
- **IFRS 16**: Same practical expedient available to lessees (IFRS 16.15). Lessors must always separate components under IFRS 15.

---

## Part 2: Lessee Classification — Dual Model (ASC 842) vs. Single Model (IFRS 16)

### Overview

| Feature | ASC 842 (US GAAP) | IFRS 16 | UK FRS 102 (post-2026) | JGAAP (pre-ASBJ No.34) | JGAAP (post-ASBJ No.34 eff. Apr 2027) | CAS 21 | Ind AS 116 |
|---|---|---|---|---|---|---|---|
| Model | **Dual** (finance / operating) | **Single** (all leases → ROU + liability, unless exempt) | Broadly IFRS 16-aligned (periodic review amendments) | Dual model (old IAS 17-like) | **Single model** (IFRS 16-aligned) | Single model (IFRS 16-aligned) | Single model (identical to IFRS 16) |
| Operating lease on balance sheet? | Yes — ROU asset + lease liability, but P&L = straight-line total expense | No operating classification; all leases on balance sheet | Yes (post-amendment) | No (off-balance-sheet for operating leases) | Yes | Yes | Yes |
| Finance lease P&L pattern | Front-loaded (depreciation + interest) | Front-loaded | Front-loaded | Front-loaded | Front-loaded | Front-loaded | Front-loaded |
| Short-term exemption | ≤ 12 months at commencement | ≤ 12 months at commencement | Available | Available | Available | Available | Available |
| Low-value exemption | Underlying asset ≤ ~$5K when new | Underlying asset ≤ ~$5K when new (indicative) | Available | N/A | Available | Available (≤ ¥40K) | Available |

### ASC 842 Finance Lease Classification Criteria (842-20-25-2)

A lessee classifies a lease as a **finance lease** if any one of the following criteria is met:

1. Transfer of ownership to lessee by or before lease end
2. Purchase option the lessee is reasonably certain to exercise
3. Lease term is for the **major part** of the remaining economic life (bright line: ≥ 75% is a common reference but not codified — judgment required)
4. Present value of lease payments plus any residual value guarantee equals or exceeds **substantially all** of the fair value (bright line: ≥ 90% is a common reference — judgment required)
5. Underlying asset is of such a **specialized nature** that it has no alternative use to the lessor at lease end

If none of the above: **operating lease**.

### IFRS 16 Single-Model Lessee Treatment

All leases (other than exempt short-term and low-value) are treated as **finance-type** on balance sheet:
- Recognize a **right-of-use (ROU) asset** and **lease liability** at commencement date
- Depreciate ROU asset (typically straight-line over shorter of lease term or useful life)
- Unwind lease liability using effective interest method
- There is no separate "operating lease" balance sheet classification for lessees

---

## Part 3: Lease Measurement

### Initial Measurement

#### Lease Liability (both ASC 842 and IFRS 16)

Present value of lease payments not yet paid, discounted using:
- **Rate implicit in the lease** — if readily determinable; otherwise
- **Lessee's incremental borrowing rate (IBR)** — the rate a lessee would have to pay to borrow, over a similar term, with a similar security, in a similar economic environment, funds necessary to obtain an asset of similar value

**Lease payments included in the liability:**
- Fixed payments (less any lease incentives receivable)
- Variable lease payments that depend on an index or rate (initially measured using the index or rate at commencement date)
- Exercise price of a purchase option the lessee is reasonably certain to exercise
- Lease payments in an optional renewal period if reasonably certain to exercise
- Penalties for early termination if lease term reflects lessee exercising termination option

**Excluded from lease liability:**
- Variable payments based on performance or usage (expensed as incurred)
- Short-term and low-value lease payments (if exemption elected)

Source: ASC 842-20-30-5; IFRS 16.26–16.28

#### Right-of-Use (ROU) Asset

**Initial measurement** = Lease liability + lease payments made at or before commencement + initial direct costs + estimate of dismantlement/restoration costs (ASC 420 / IAS 37) – lease incentives received

Source: ASC 842-20-30-1; IFRS 16.24

### Subsequent Measurement

#### Lease Liability

- Increase carrying amount to reflect accretion of interest (effective interest method)
- Reduce carrying amount to reflect lease payments made
- Remeasure (see Part 5) when lease is modified or when certain events trigger reassessment

#### ROU Asset

| Classification | Depreciation Method | Period |
|---|---|---|
| Finance lease (ASC 842) | Straight-line (typical) unless another systematic method better represents the pattern of consumption | Shorter of lease term or useful life (if transfer of ownership or purchase option reasonably certain: useful life) |
| Operating lease (ASC 842) | Not separately depreciated — total lease cost allocated on straight-line basis; ROU asset is plugged | Over lease term |
| IFRS 16 (all leases) | Straight-line or another systematic method | Shorter of lease term or useful life (same ownership/purchase option rule as above) |

---

## Part 4: Discount Rate — Incremental Borrowing Rate (IBR) Determination

### IBR Factors (ASC 842-20-30-3 / IFRS 16.26)

The IBR must reflect:
1. **Credit quality** of the lessee entity
2. **Lease term** (duration of the obligation)
3. **Collateral** — security that would be provided in a borrowing (the leased asset is the natural proxy)
4. **Currency** of the lease payments
5. **Economic environment** (country/jurisdiction)

**Practical approaches commonly used:**
- Start from a risk-free rate (government bond), then add credit spread for lessee credit quality and a security adjustment
- Use quoted rates from the lessee's existing secured borrowings of similar maturity as a starting point
- Use market data from comparable secured borrowing facilities

**ASC 842 lessor-rate exception (private companies)**: A lessee that is not a public business entity may elect to use the risk-free rate as a practical expedient (elected by class of underlying asset). (ASC 842-20-30-3)

**IFRS 16 rate implicit in lease — when readily determinable**: The rate that causes the present value of lease payments plus the unguaranteed residual value to equal the fair value of the underlying asset plus initial direct costs of the lessor. (IFRS 16.A — Defined terms)

---

## Part 5: Lease Modifications and Remeasurement

### Modification vs. Remeasurement Trigger

| Event | ASC 842 Treatment | IFRS 16 Treatment |
|---|---|---|
| Change in scope or consideration not in original contract | **Modification** — evaluate as separate new lease or modification of existing | **Modification** — same approach |
| Index/rate change (CPI, LIBOR, etc.) | Remeasure at the next adjustment date using new index/rate | Remeasure at the next adjustment date |
| Reassessment of lease term (renewal/termination option) | Reassess if significant event or change in circumstances | Reassess when a significant event or change occurs |
| Purchase option reassessment | Reassess same events | Same |

### Modification Accounting (ASC 842-20-35-3; IFRS 16.44–16.46)

**Modification treated as a separate new lease if** both:
1. Modification grants the lessee an additional right of use not included in the original lease
2. Lease payments increase commensurate with standalone price

**Otherwise**: Modify the existing lease — remeasure the lease liability using a revised discount rate at the modification effective date; adjust the ROU asset.

**If modification decreases scope**: Recognize a gain or loss for the partial or full lease termination.

---

## Part 6: Lessor Accounting

### Lessor Classification (ASC 842 / IFRS 16)

Lessors continue to apply a **dual classification model** under both US GAAP and IFRS.

| Lease Type | ASC 842 Standard | IFRS 16 Standard |
|---|---|---|
| Sales-type lease | ASC 842-30-25-1 | IFRS 16.67 (finance lease) |
| Direct financing lease | ASC 842-30-25-3 | Not a separate category under IFRS 16 |
| Operating lease | ASC 842-20-25-1 (lessee); ASC 842-30-25-5 (lessor) | IFRS 16.88 |

#### ASC 842 Lessor Classification Criteria

A lessor classifies as **sales-type** if any one of five criteria is met (same five criteria as lessee finance lease test — transfer of ownership, purchase option, major part of economic life, substantially all fair value, specialized nature). (ASC 842-10-25-2)

**Direct financing**: If none of the five criteria are met but (a) the present value of the sum of lease payments and any residual value guaranteed by the lessee or any other third party equals or exceeds substantially all the fair value, and (b) it is probable that the lessor will collect the lease payments plus any residual value. (ASC 842-10-25-3)

**Operating**: All other leases.

#### Income Recognition — Lessor

| Type | Revenue Pattern |
|---|---|
| Sales-type lease | Selling profit/loss at commencement + interest income over the lease term (effective interest) |
| Direct financing lease | Interest income only (no selling profit at commencement — manufacturer/dealer profit deferred) |
| Operating lease | Straight-line rental income over the lease term |

---

## Part 7: Sale-Leaseback Transactions

### ASC 842-40 / IFRS 16.98–16.103

**Step 1: Is the transfer a sale?**
Apply ASC 606 / IFRS 15 revenue recognition criteria to determine whether the transfer of the asset constitutes a sale.

**If transfer is a sale (sale-leaseback):**
- Seller-lessee: derecognize the asset; recognize a **right-of-use asset** equal to the proportion of the previous carrying amount relating to the right retained; recognize only the gain/loss attributable to the rights transferred to the buyer-lessor
- Buyer-lessor: account for the purchase of an asset and a lease originated to the seller-lessee

**If transfer is NOT a sale (failed sale-leaseback = financing arrangement):**
- Seller-lessee: continue to recognize the asset; recognize a financial liability equal to the proceeds received (ASC 842-40-25-5; IFRS 16.103)
- Buyer-lessor: do not recognize the underlying asset; recognize a financial asset

**Key practical issue**: If leaseback payments are above or below market, an adjustment may be required to the gain/loss recognized on the sale. (IFRS 16.100; ASC 842-40-30-1)

---

## Part 8: Short-Term and Low-Value Exemptions

### Short-Term Lease Exemption (ASC 842 / IFRS 16)

- Applies to leases with a lease term of **12 months or less** at commencement date (including renewal options that are reasonably certain to be exercised)
- Election made by **class of underlying asset** (ASC 842) or lease-by-lease (IFRS 16.B3)
- If elected: recognize lease payments as expense on a straight-line basis over the lease term — no ROU asset or lease liability recognized

Source: ASC 842-20-25-1; IFRS 16.5(a), 16.B3

### Low-Value Asset Exemption (IFRS 16 only)

- No equivalent in ASC 842
- IFRS 16 BC100 indicates the threshold was calibrated to assets with a fair value when new of approximately **USD 5,000 or less** (indicative — not a bright line in the standard)
- Common examples: laptops, personal computers, small items of office furniture
- Election made **lease-by-lease** — no class-level election required
- If elected: expense payments straight-line — no ROU asset or lease liability

Source: IFRS 16.5(b), 16.B3–B8

**CAS 21 (China)**: Recognizes a low-value threshold aligned with IFRS 16, calibrated at approximately **CNY 40,000** when new.

---

## Part 9: Multi-Jurisdiction Effective Dates and Status

| Standard | Jurisdiction | Effective Date | Status |
|---|---|---|---|
| ASC 842 | United States (US GAAP) | Already effective (public entities FY 2019; private entities FY 2022) | Effective — full adoption required |
| IFRS 16 | IFRS jurisdictions (EU, Australia, etc.) | Already effective (1 Jan 2019 for most) | Effective — full adoption required |
| UK FRS 102 (amended — lease accounting aligned toward IFRS 16) | United Kingdom | Accounting periods beginning on or after **1 January 2026** | Forthcoming — early adoption permitted |
| ASBJ Statement No. 34 (new lease standard) | Japan (JGAAP listed entities) | Fiscal years beginning on or after **1 April 2027** | Forthcoming — early adoption permitted from FY beginning 1 Apr 2026 |
| CAS No. 21 (revised 2019) | China | Effective **1 January 2019** for listed entities in China; **1 January 2021** for other enterprises | Effective |
| Ind AS 116 | India | Effective **1 April 2019** | Effective |

### JGAAP Transition Note (ASBJ Statement No. 34)

Under the current JGAAP (pre-Statement No. 34), most operating leases remain off-balance-sheet. Statement No. 34 introduces a single on-balance-sheet model broadly aligned with IFRS 16. Entities with significant off-balance-sheet operating leases under old JGAAP will recognize material new ROU assets and lease liabilities on first adoption.

### UK FRS 102 Transition Note

The Financial Reporting Council's 2024 periodic review of FRS 102 substantially aligns Section 20 (Leases) with IFRS 16. UK companies applying FRS 102 will need to transition from the legacy IAS 17-style dual model to a single on-balance-sheet model for lessees effective for periods beginning on or after 1 January 2026.

---

## Part 10: GAAP Comparison — Quick Reference

| Area | ASC 842 | IFRS 16 | UK FRS 102 (post-2026) | German HGB | JGAAP (pre-Apr 2027) | JGAAP (post-Apr 2027) | CAS 21 | Ind AS 116 |
|---|---|---|---|---|---|---|---|---|
| Lessee model | Dual | Single | Single (IFRS 16-aligned) | Operating = off-B/S; finance = on-B/S | Dual (old IAS 17) | Single | Single | Single |
| Short-term exemption | Yes (≤ 12 mo, by asset class) | Yes (≤ 12 mo) | Yes | N/A | N/A | Yes | Yes | Yes |
| Low-value exemption | No | Yes (~$5K) | Yes | N/A | N/A | Yes | Yes (~CNY 40K) | Yes |
| Lessor model | Sales-type / direct financing / operating | Finance / operating | Finance / operating | HGB rules | Old IAS 17 | IFRS 16-aligned | IFRS 16-aligned | IFRS 16-aligned |
| Discount rate | Rate implicit / IBR; risk-free rate option for non-PBE | Rate implicit / IBR | Rate implicit / IBR | N/A (no measurement framework for operating) | Rate implicit / IBR | Rate implicit / IBR | Rate implicit / IBR | Rate implicit / IBR |
| Sale-leaseback | ASC 842-40 / ASC 606 sale test | IFRS 16.98 / IFRS 15 sale test | IFRS 16-aligned | HGB realization principle | Legacy IAS 17 approach | IFRS 16-aligned | IFRS 16-aligned | IFRS 16-aligned |
| Official standard | https://asc.fasb.org/842 | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs16.html | FRC FRS 102 | HGB §285, §314 | ASBJ Statement No. 13 | ASBJ Statement No. 34 | MOF CAS 21 | ICAI Ind AS 116 |

---

## Part 11: Official Documentation — Publicly Accessible URLs

| Standard | Resource | URL | Access |
|---|---|---|---|
| ASC 842 | FASB Accounting Standards Codification | https://asc.fasb.org/842 | **Fully public** (summary; full text requires FASB login) |
| IFRS 16 | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs16.html | **Fully public** |
| UK FRS 102 | FRC | https://www.frc.org.uk/library/standards-codes-policy/accounting/uk-and-ireland-accounting-standards/standards-in-issue/frs-102-the-financial-reporting-standard-applicable-in-the-uk-and-republic-of-ireland/ | **Fully public** |
| German HGB | Federal Ministry of Justice | https://www.gesetze-im-internet.de/hgb/ | **Fully public (German)** |
| JGAAP — ASBJ Statement No. 34 | ASBJ (English summaries) | https://www.asb.or.jp/en/accounting_standards/accounting_standards/ | **Fully public** |
| CAS 21 | ICAI / MOF guidance | https://www.mof.gov.cn/zhengwuxinxi/caijingshuju/201612/t20161227_2572491.htm | Limited English; Deloitte/PwC translations recommended |
| Ind AS 116 | ICAI | https://www.icai.org/post/indian-accounting-standards | **Fully public** |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described above. Lease accounting conclusions depend heavily on contract-specific facts, jurisdiction, and entity type. Standards in several jurisdictions (UK FRS 102, JGAAP) are subject to forthcoming amendments — verify effective dates and transition rules. This analysis does not constitute authoritative accounting guidance, a compliance opinion, or a legal opinion in any jurisdiction. Verify lease accounting treatments with qualified local auditors and legal counsel before relying on this analysis for compliance purposes. No accountant-client relationship is formed.
