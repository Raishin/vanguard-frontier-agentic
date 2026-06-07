---
name: fixed-assets-advisor
description: Multi-jurisdiction fixed assets, depreciation, and impairment reference framework covering PP&E, intangibles, right-of-use assets, and goodwill under US GAAP and IFRS.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Fixed Assets & Impairment Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for fixed assets, depreciation, and impairment advisory — from PP&E initial recognition through useful life reviews, revaluation, impairment testing (with critical reversibility divergence), goodwill, intangibles, R&D capitalisation, and the interaction with tax depreciation.

---

## Part 1: PP&E — Recognition and Initial Measurement

### Recognition Criteria

**US GAAP (ASC 360-10-05)**:
An item of PP&E is recognized when it is probable that future economic benefits will flow to the entity and the cost can be measured reliably. No explicit recognition probability threshold is stated; capitalisation vs. expense is a matter of policy based on materiality thresholds.

**IFRS (IAS 16.7)**:
An item of PP&E is recognized as an asset when: (a) it is probable that future economic benefits will flow to the entity; and (b) the cost can be measured reliably. Both conditions must be met.

**German HGB (§246, §253 HGB)**:
All assets must be recognized (Aktivierungspflicht — mandatory capitalisation for assets that meet the definition). No probability assessment is applied separately; legal ownership or economic substance determines whether an asset is recognized.

**JGAAP (ASBJ Statement No. 11)**:
Broadly consistent with IFRS criteria for recognition; physical assets acquired must be capitalized at cost.

### Cost Model (All Jurisdictions)

Initial measurement at cost:

```
Cost = Purchase price + Import duties + Non-refundable purchase taxes
     + Directly attributable costs to bring the asset to working condition
     − Trade discounts and rebates
```

Directly attributable costs include: site preparation, delivery and installation, professional fees (architects, engineers), and estimated dismantlement/restoration costs (decommissioning provision — IAS 37 / ASC 410-20).

**What is NOT included in cost:**
- General and administrative overhead (unless directly attributable).
- Start-up and pre-opening costs (IAS 16.19; ASC 360-10 — expense as incurred).
- Initial operating losses.
- Staff training to operate the asset.

### IFRS Revaluation Model (IAS 16.29–16.42) — No US GAAP Equivalent

Under IFRS, after initial recognition an entity may choose the **revaluation model** as its accounting policy for an entire class of PP&E:

- The asset is carried at **fair value** at the revaluation date less subsequent accumulated depreciation and impairment.
- Revaluations must be carried out with sufficient regularity that the carrying amount does not differ materially from fair value at the balance sheet date.
- **Revaluation surplus** (increase): recognized in **OCI** (other comprehensive income) and accumulated in equity as "revaluation surplus" (IAS 16.39). Exception: if the increase reverses a previous revaluation decrease recognized in P&L, the increase is recognized in P&L to the extent of the prior decrease.
- **Revaluation decrease**: recognized in **P&L** (IAS 16.40). Exception: if a revaluation surplus exists for the same asset, the decrease is debited against OCI first.
- Treatment of depreciation on revaluation surplus: an entity may transfer the incremental depreciation (i.e., depreciation based on revalued amount less depreciation based on original cost) from revaluation surplus to retained earnings each period — or transfer the full surplus on derecognition. Both are permitted under IAS 16.41.

**CRITICAL**: The revaluation model has **no equivalent under US GAAP**. Under ASC 360, PP&E is measured at historical cost less accumulated depreciation. Revaluation upward is never permitted under US GAAP.

### Componentisation

**IFRS (IAS 16.43) — REQUIRED**:
Each part of an item of PP&E with a cost that is significant in relation to the total cost of the item must be depreciated separately. This is componentisation (also called "component accounting"). A significant component that has a different useful life from the rest of the asset must be separated.

**Example**: An aircraft — airframe (25-year useful life), engines (12-year useful life), interior (7-year useful life) — must be depreciated as separate components under IFRS.

**US GAAP (ASC 360) — OPTIONAL**:
Componentisation is not required under US GAAP. Entities may depreciate assets as a whole or componentise voluntarily. This is a significant practical divergence for entities dual-reporting.

### Borrowing Cost Capitalisation

**US GAAP (ASC 835-20)**:
Interest must be capitalised on qualifying assets (assets that require a substantial period of time to get ready for intended use or sale). Capitalisation period: from when expenditures are being made, activities necessary to prepare the asset are in progress, and interest costs are being incurred.

**IFRS (IAS 23.8)**:
Borrowing costs directly attributable to the acquisition, construction, or production of a qualifying asset must be capitalised. A qualifying asset is one that necessarily takes a substantial period of time to get ready for its intended use or sale.

**German HGB (§255 Abs. 3 HGB)**:
Capitalisation of borrowing costs is **permitted but not required** (Wahlrecht — option). This contrasts with IAS 23 where capitalisation is mandatory.

### Subsequent Expenditure

**Betterment (capital expenditure)**: Expenditure that increases the future economic benefits (extends useful life, increases capacity, improves quality) — capitalize.

**Maintenance and repair**: Expenditure that merely maintains the existing service potential — expense as incurred.

**Component replacement (IFRS IAS 16.13)**: When a major component is replaced, the new component is capitalised and the carrying amount of the replaced component is derecognized (even if it was not separately identified).

---

## Part 2: Depreciation

### Depreciation Methods

| Method | Description | When Appropriate |
|---|---|---|
| **Straight-line** | Equal charge each period: (Cost − Residual Value) / Useful Life | Consistent benefit pattern; most common for buildings, furniture, software |
| **Declining balance** | Fixed rate applied to carrying amount: e.g., 200% DDB or 150% DB | Accelerated benefit pattern; technology assets that lose value quickly |
| **Units of production** | Charge per unit produced: (Cost − RV) / Estimated Total Units × Units Produced | Assets where wear is driven by usage, not time: mining equipment, aircraft engines |
| **Sum-of-years-digits** | Accelerated; decreasing fraction each year | Less common; similar economics to declining balance |

The method chosen must reflect the pattern in which the asset's future economic benefits are expected to be consumed (IAS 16.62; ASC 360-10-35-4).

### Useful Life and Residual Value Reviews

**IFRS (IAS 16.51)**:
The useful life and residual value of each component must be reviewed **at least at each annual reporting date**. Any change is a **change in accounting estimate** (IAS 8.36) — prospective effect on depreciation only (no restatement of prior periods).

**US GAAP (ASC 250-10-45)**:
Changes in useful life or residual value are changes in accounting estimate — prospective adjustment to depreciation charge. No annual review requirement is explicitly mandated, but changes are required when facts and circumstances indicate a revision is warranted.

**German HGB (§253 Abs. 3 HGB)**:
Useful lives follow AfA (Absetzung für Abnutzung) tables published by the Federal Ministry of Finance (BMF). These are not binding but widely used as the standard reference for both HGB and tax depreciation. The 2023 AfA table updates are the current reference.

**Special HGB rule — GWG (Geringwertige Wirtschaftsgüter)**:
Assets with a net acquisition cost ≤ €800 (excluding VAT) may be fully expensed in the year of acquisition — §6 Abs. 2 EStG (German Income Tax Act — applicable to HGB tax accounts). Under pooling method (§6 Abs. 2a EStG), assets between €250 and €800 may be pooled and depreciated over 5 years.

**JGAAP**:
Special depreciation allowances (tokubetsu shōkyaku — 特別償却) allow accelerated depreciation for qualifying investments (specific industries, R&D equipment, energy efficiency). These are tax-driven but frequently aligned with book treatment for unlisted Japanese entities.

### Depreciation Commencement

Both IFRS (IAS 16.55) and US GAAP (ASC 360-10-35-4) require depreciation to commence when the asset is **available for use** (i.e., in the location and condition necessary for it to operate in the manner intended by management) — not when it is first placed into use.

---

## Part 3: Impairment of PP&E

### Impairment Indicators (Both Standards)

Assess at each reporting date whether there is any **indication** that an asset may be impaired (IAS 36.9 / ASC 360-10-35-21):

**External indicators:**
- Significant decline in market value beyond normal use/passage of time.
- Significant adverse changes in the technological, market, economic, or legal environment.
- Market interest rate increases reducing the discount rate used to measure value in use.
- Market capitalisation below net asset value (IAS 36.12 — specific IFRS indicator).

**Internal indicators:**
- Evidence of obsolescence or physical damage.
- Significant adverse changes in use (idle, discontinued operations, restructuring).
- Internal evidence that economic performance is worse than expected.

### US GAAP Two-Step Test (ASC 360-10-35)

**Step 1 — Recoverability test (ASC 360-10-35-17)**:
Compare the asset's carrying amount to the **sum of undiscounted future cash flows** expected from the asset (including eventual disposal). If carrying amount ≤ undiscounted cash flows → **no impairment; stop here**.

**Step 2 — Measurement of impairment loss (ASC 360-10-35-17)**:
If carrying amount > undiscounted cash flows → impairment exists. Measure impairment loss as: carrying amount − **fair value** of the asset.

```
Impairment loss (US GAAP) = Carrying amount − Fair value
```

Fair value is determined per ASC 820 (fair value measurement hierarchy — Level 1, 2, 3).

**CRITICAL — US GAAP: Impairment losses on PP&E are NOT reversible (ASC 360-10-35-21)**. Once written down, the new carrying amount becomes the new cost basis. If the asset's fair value subsequently recovers, no write-up is permitted under US GAAP.

### IFRS Single-Step Test (IAS 36)

**No recoverability pre-screen**: Under IAS 36, if an indicator exists, compare carrying amount directly to **recoverable amount**.

**Recoverable amount (IAS 36.18)**:
```
Recoverable amount = Higher of:
  (a) Fair Value Less Costs of Disposal (FVLCTD / FVLCTS)
  (b) Value in Use (VIU) — discounted present value of future cash flows
```

**Impairment loss (IFRS)**:
```
Impairment loss = Carrying amount − Recoverable amount
(if carrying amount > recoverable amount)
```

**CRITICAL — IFRS: Impairment losses on PP&E and intangibles ARE reversible (IAS 36.117)**. If, in a subsequent period, indicators suggest the impairment loss may no longer exist or may have decreased, recalculate the recoverable amount. The reversal is recognized in P&L up to the carrying amount that would have existed had no impairment been recognized (net of depreciation). Exception: **goodwill impairment is NEVER reversible under IFRS (IAS 36.124)**.

### Value in Use (VIU) — IFRS Specific

VIU calculation (IAS 36.31–36.57):
- **Cash flows**: pre-tax cash flow projections based on reasonable, supportable assumptions; typically 5-year management forecast + terminal value.
- **Discount rate**: pre-tax rate reflecting current market assessments of the time value of money and the risks specific to the asset.
- **Terminal value**: extrapolation using a steady-state or declining growth rate not exceeding long-term average growth rate for the market.
- **What to exclude from VIU**: cash flows from future restructurings not yet committed; cash flows from improving/enhancing asset performance; financing cash flows; income tax effects (pre-tax basis).

---

## Part 4: Goodwill

### Initial Recognition

**US GAAP (ASC 805 — Business Combinations)**:
Goodwill = Total consideration transferred + Fair value of NCI (non-controlling interest) + Fair value of previously held equity interest − Fair value of identifiable net assets acquired.

US GAAP requires **full goodwill** (NCI measured at fair value, not at proportionate share of identifiable net assets).

**IFRS (IFRS 3 — Business Combinations)**:
IFRS allows a **choice** at each acquisition between:
- **Full goodwill method**: NCI at fair value (same result as US GAAP).
- **Partial goodwill method (proportionate NCI)**: NCI at proportionate share of identifiable net assets → goodwill recognized only for the parent's share.

**CRITICAL divergence**: The choice between full and partial goodwill under IFRS is made on a transaction-by-transaction basis. US GAAP allows only full goodwill.

### Subsequent Measurement — No Amortisation

Both US GAAP (ASC 350-20) and IFRS (IAS 36.80) prohibit the amortisation of goodwill. Instead, goodwill is tested for impairment at least annually.

**Private company exception (US GAAP — ASC 350-20-35-63)**:
A private company or not-for-profit organization may elect to amortize goodwill on a straight-line basis over a useful life not to exceed 10 years (ASU 2014-02).

**IFRS — no amortisation exception for any entity type** (full impairment test required annually regardless of entity size).

### Goodwill Impairment Test

**US GAAP (ASC 350-20-35)**:

Step 0 (Qualitative assessment): Assess whether it is more likely than not (>50%) that the fair value of a reporting unit is less than its carrying amount. If not, no quantitative test required.

Quantitative test: Compare fair value of the reporting unit to its carrying amount (including goodwill). If carrying amount > fair value → goodwill impairment = excess of carrying amount over fair value, capped at carrying amount of goodwill for that reporting unit.

**Level of test**: Reporting unit (an operating segment or one level below — ASC 350-20-20).

**IFRS (IAS 36.80–36.99)**:
Goodwill is tested at the level of the **cash-generating unit (CGU)** to which it is allocated. A CGU is the smallest identifiable group of assets that generates largely independent cash inflows.

Impairment test: Compare carrying amount of the CGU (including allocated goodwill) to its recoverable amount (higher of FVLCTD and VIU). If carrying amount > recoverable amount → impairment loss allocated first to goodwill, then pro rata to other assets.

**Goodwill impairment is NEVER reversible under either US GAAP (ASC 350-20-35-16) or IFRS (IAS 36.124).**

---

## Part 5: Intangible Assets

### Recognition Criteria (ASC 350 / IAS 38)

An intangible asset is recognized when:
1. **Identifiable**: separable from the entity (capable of being sold, transferred, licensed separately) OR arises from contractual or other legal rights.
2. **Controlled**: the entity has the power to obtain the future economic benefits and can restrict others' access.
3. **Future economic benefits**: expected to flow to the entity.

### R&D: Critical US GAAP vs. IFRS Divergence

**US GAAP (ASC 730)**:
All research and development costs are **expensed as incurred**. No distinction between research phase and development phase. Exception: software development costs (ASC 350-40 for internal-use software; ASC 985-20 for software to be sold):
- Costs incurred after technological feasibility is established (for software to be sold) may be capitalized.
- Costs incurred in the application development stage (for internal-use software) are capitalized.

**IFRS (IAS 38.54–38.67)**:
- **Research phase** (IAS 38.54): all expenditure expensed as incurred (consistent with US GAAP).
- **Development phase** (IAS 38.57): expenditure must be capitalized when ALL six criteria are met:
  1. Technical feasibility of completing the asset.
  2. Intention to complete the asset and use or sell it.
  3. Ability to use or sell the asset.
  4. Probable future economic benefits exist.
  5. Adequate resources to complete development.
  6. Ability to reliably measure the expenditure attributable to the asset.

**CRITICAL**: This is one of the largest divergences between US GAAP and IFRS in practice. Under IFRS, pharmaceutical development costs, software development costs, and new product development costs meeting the IAS 38.57 criteria must be capitalized and then amortized. Under US GAAP, the same costs are expensed immediately.

**German HGB (§248 Abs. 2 HGB)**:
Internally generated intangibles (including development costs) may be **optionally** capitalized (Aktivierungswahlrecht — activated since BilMoG 2009 reform). If capitalized, a dividend restriction (Ausschüttungssperre) applies — the amount cannot be distributed as dividends. **Research costs remain expensed**.

**JGAAP**:
Development costs may be deferred under certain conditions (ASBJ guidance). Broadly follows a similar framework to IFRS but less prescriptive; practice varies.

### Finite vs. Indefinite-Lived Intangibles

| Category | Treatment |
|---|---|
| **Finite-lived** | Amortized over useful life (straight-line unless another pattern is more appropriate); tested for impairment when indicators exist |
| **Indefinite-lived** | NOT amortized; tested for impairment at least annually (and when indicators exist) — same as goodwill treatment |

Under IFRS (IAS 38.109), the classification as indefinite-lived must be reviewed annually.

---

## Part 6: Multi-Jurisdiction Summary Table

| Topic | US GAAP | IFRS | German HGB | JGAAP | China CAS | Ind AS |
|---|---|---|---|---|---|---|
| Revaluation model | Not permitted (cost only) | Permitted (IAS 16.29) — OCI | Not permitted (cost only; strict lower of cost or market — §253) | Not permitted | Not permitted (CAS 4 — cost model) | Permitted (Ind AS 16 = IAS 16) |
| Componentisation | Optional | Required (IAS 16.43) | No explicit requirement; practice varies | No explicit requirement | Required under CAS 4 | Required (Ind AS 16.43) |
| Borrowing cost capitalisation | Required (ASC 835-20) | Required (IAS 23.8) | Optional (§255 Abs. 3 HGB) | Required for qualifying assets | Required (CAS 17) | Required (Ind AS 23) |
| Impairment test approach | Two-step: undiscounted recoverability + fair value (ASC 360-10-35) | Single-step: recoverable amount = higher of FVLCTD and VIU (IAS 36) | Strict lower of cost or market (§253 Abs. 3) — write-down mandatory if market value < book | Similar to IFRS in principle | IAS 36 equivalent (CAS 8) | Ind AS 36 = IAS 36 |
| Impairment reversal — PP&E | **NOT permitted** (ASC 360-10-35-21) | **Permitted** (IAS 36.117) | Not permitted for permanent impairment; reversal permitted if temporary (§253 Abs. 5) | Reversal permitted if conditions reverse | Reversal prohibited (CAS 8.17) | Permitted (Ind AS 36.117) |
| Goodwill impairment reversal | **NOT permitted** (ASC 350-20-35-16) | **NOT permitted** (IAS 36.124) | Not permitted | Not permitted | Not permitted | Not permitted (Ind AS 36.124) |
| R&D capitalisation | Expense all (ASC 730) — software exceptions | Research: expense; Development: capitalise when 6 criteria met (IAS 38.57) | Development: optional capitalisation (§248 Abs. 2 HGB) | Development deferral allowed | Research: expense; Development: IAS 38-equivalent criteria (CAS 6) | Development: capitalise (Ind AS 38.57 = IAS 38) |
| Goodwill NCI method | Full goodwill only | Choice: full or partial goodwill (IFRS 3) | Proportionate / partial goodwill typical | Proportionate | Full consolidation; proportionate goodwill typical | Choice: full or partial (Ind AS 103 = IFRS 3) |
| GWG immediate expensing | Section 179 / bonus depreciation (tax; no book equivalent) | No equivalent | ≤€800 (§6 Abs. 2 EStG) | Special depreciation allowances | Accelerated depreciation for qualifying assets | No equivalent |

---

## Part 7: Tax Depreciation Interaction

### Book vs. Tax Basis Differences — Deferred Tax

Differences between carrying amount of PP&E for financial reporting purposes and its tax base generate deferred tax assets or liabilities:

```
Temporary difference = Carrying amount − Tax base

Taxable temporary difference → Deferred Tax Liability (DTL)
  (book value > tax base: asset depreciated faster for tax → tax base < book)

Deductible temporary difference → Deferred Tax Asset (DTA)
  (tax value > book: tax base > book; rare for PP&E but can arise for impaired assets)
```

**US GAAP (ASC 740)**: Deferred tax calculated using **enacted** tax rates at the balance sheet date.
**IFRS (IAS 12)**: Deferred tax calculated using **substantively enacted** rates.

### US Tax Depreciation

**Section 179 (26 U.S.C. §179)**: Allows immediate expensing of qualifying business property in the year of purchase up to an annual limit ($1,220,000 for 2024; indexed for inflation; phase-out begins when property placed in service exceeds $3,050,000 for 2024).

**Bonus depreciation (26 U.S.C. §168(k))**: Additional first-year depreciation allowance for qualifying property. Phase-down schedule:
- 2023: 80%
- 2024: 60%
- 2025: 40%
- 2026: 20%
- 2027 and after: 0% (unless legislatively extended)

Both Section 179 and bonus depreciation create timing differences vs. GAAP book depreciation → deferred tax liability on the balance sheet.

### UK Capital Allowances

UK tax depreciation is based on **capital allowances** rather than accounting depreciation:
- **Annual Investment Allowance (AIA)**: 100% first-year deduction on qualifying plant and machinery, up to £1,000,000 per year.
- **Writing Down Allowance (WDA)**: 18% (main rate pool) or 6% (special rate pool — integral features, long-life assets) per year on a reducing balance basis.
- **Full Expensing (April 2023 onwards)**: 100% first-year allowance for qualifying main rate plant and machinery.

### Germany — AfA Tables

The German BMF (Bundesministerium der Finanzen) publishes AfA (Absetzung für Abnutzung) depreciation tables defining standard useful lives for categories of assets. These are used for both tax (EStG §7) and HGB accounts for unlisted entities.

**Key AfA rates (2023 table examples):**
- Office furniture: 13 years
- PCs and servers: 3 years (revised 2021 — previously 3 years, confirmed)
- Industrial machinery: varies by industry (typically 8–15 years)
- Buildings (commercial): 33 years (3% straight-line) or 2%/2.5% for older buildings

**Declining balance option (§7 Abs. 2 EStG)**: Temporarily reintroduced for assets acquired 2020–2022 at 2.5× the straight-line rate, max 25% per year. Check current legislative status for more recent acquisitions.

### Japan — Special Depreciation Allowances

JGAAP allows accelerated depreciation (tokubetsu shōkyaku — 特別償却) under the Special Taxation Measures Law (Sōzei Tokubetsu Sochihō) for qualifying investments:
- Specific industries and qualifying productive assets.
- Typically 30–50% additional depreciation in the first year.
- Creates book-tax timing differences generating deferred tax under JGAAP.

---

## Part 8: Official Documentation — Publicly Accessible URLs

| Standard / Resource | URL | Access |
|---|---|---|
| ASC 360 (PP&E) | https://asc.fasb.org/360 | Public (free registration) |
| ASC 350 (Intangibles / Goodwill) | https://asc.fasb.org/350 | Public (free registration) |
| ASC 730 (R&D) | https://asc.fasb.org/730 | Public (free registration) |
| ASC 835-20 (Capitalisation of Interest) | https://asc.fasb.org/835 | Public (free registration) |
| ASC 805 (Business Combinations) | https://asc.fasb.org/805 | Public (free registration) |
| IAS 16 (PP&E) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias16.html | Public |
| IAS 36 (Impairment) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias36.html | Public |
| IAS 38 (Intangibles) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias38.html | Public |
| IAS 23 (Borrowing Costs) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias23.html | Public |
| IFRS 3 (Business Combinations) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs3.html | Public |
| German HGB §253 (Bewertung) | https://www.gesetze-im-internet.de/hgb/__253.html | Public (German) |
| German HGB §255 (Anschaffungs-/Herstellungskosten) | https://www.gesetze-im-internet.de/hgb/__255.html | Public (German) |
| German EStG §7 (AfA) | https://www.gesetze-im-internet.de/estg/__7.html | Public (German) |
| BMF AfA Tables (2023) | https://www.bundesfinanzministerium.de/Content/DE/Downloads/BMF_Schreiben/Steuerarten/Einkommensteuer/2023-12-22-afa-tabelle-allgemein.pdf | Public (German) |
| ASBJ English Accounting Standards | https://www.asb.or.jp/en/accounting_standards/accounting_standards/ | Public |
| China CAS (MOF summary) | https://www.mof.gov.cn/zhengwuxinxi/caijingshuju/201612/t20161227_2572491.htm | Limited English |
| Ind AS 16/36/38 (ICAI) | https://www.icai.org/post/indian-accounting-standards | Public |
| IRC §179 (Section 179 Expensing) | https://www.law.cornell.edu/uscode/text/26/179 | Public |
| UK Capital Allowances (HMRC) | https://www.gov.uk/capital-allowances | Public |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described above. Impairment analyses require formal assessment by qualified independent valuers, and conclusions must be reviewed by external auditors before being reflected in financial statements. Local statutory reporting requirements, tax depreciation rules, and GAAP differences vary by jurisdiction and change frequently. This analysis does not constitute authoritative accounting guidance, a compliance opinion, a valuation opinion, or a tax opinion in any jurisdiction. Verify all conclusions with qualified local auditors, valuers, tax advisors, and legal counsel before relying on this analysis for compliance, financial reporting, or transaction purposes. This analysis does not form an accountant-client relationship.
