---
name: treasury-liquidity-advisor
description: Multi-jurisdiction reference framework for corporate treasury operations, cash and liquidity management, FX and currency risk, hedge accounting qualification (ASC 815 / IFRS 9), FX translation (ASC 830 / IAS 21), Basel III LCR/NSFR, Dodd-Frank and EMIR derivatives reporting, and country-specific cash repatriation and capital control regimes (China SAFE, India FEMA, Brazil IOF, Argentina BCRA). Advisory only — never executes transactions or writes to any system of record.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-01"
  category: finance
  lifecycle: experimental
---

# Treasury & Liquidity Advisor Skill

> Read-only reference framework. All conclusions are advisory. Treasury regulations, capital controls, and
> withholding tax rates change frequently. Verify current requirements with local legal counsel and qualified
> tax advisors before executing any treasury strategy.

---

## Part 1 — Cash Pooling Structures and Jurisdiction Restrictions

### 1.1 Pooling Structure Types

| Structure | Mechanism | Key Benefit | Primary Risk |
|---|---|---|---|
| **Physical zero-balance pooling** | Actual cash swept to header account; intercompany loans created | Optimal interest offset; full cash concentration | Thin capitalization; withholding tax on intercompany interest; documentation burden |
| **Notional pooling** | Balances netted for interest calculation; no actual movement | Simplicity; no IC loan documentation for sweeps | Banks may require full netting rights; blocked in some jurisdictions |
| **Cross-border intercompany lending** | Formal intercompany loans between entities | Flexible; usable where pooling is unavailable | Transfer pricing; OECD TP Guidelines Chapter X; registration requirements |

### 1.2 Country-Specific Restrictions Matrix

| Country | Physical Pooling | Notional Pooling | Cross-Border Loans | Key Regulator | Key Restriction |
|---|---|---|---|---|---|
| **China (CNY)** | Domestic: allowed. Cross-border: restricted by SAFE | Cross-border: not commonly available | Permitted; capped at 2× registered capital (external debt quota); must register with SAFE; rate capped near LPR | SAFE, PBOC | SAFE Circular 19 (2019) governs cross-border pools |
| **India (INR)** | Domestic: allowed within group. Cross-border: **not permitted** | Cross-border: not permitted | Require RBI approval; subject to ECB (External Commercial Borrowing) guidelines; withholding on interest | RBI | FEMA (Foreign Exchange Management Act 1999); RBI Master Direction on ECB |
| **Brazil (BRL)** | Domestic: functional. Cross-border: historically restricted | Cross-border: operationally difficult | BACEN regulates; IOF (Financial Operations Tax, 0.38% on most FX flows) applies; profit remittance: 15% withholding (25% for tax-haven recipients) | BACEN, Receita Federal | Foreign capital must be registered with BACEN |
| **Argentina (ARS)** | *Substantially liberalized April 14, 2025:* cepo cambiario capital controls lifted; companies can repatriate profits | Now largely open post-April 2025 | Materially liberalized; watch BCRA guidance; pre-April 2025 transactions still under prior rules | BCRA | Volatile — verify with local counsel before acting |
| **Eurozone (EUR)** | Fully open; SEPA harmonizes EUR flows | Widely available | No restrictions within EU/EEA; arm's-length pricing required | ECB, local NCAs | Transfer pricing per OECD TP Guidelines Chapter X |
| **United States (USD)** | Fully open | Widely available | No restrictions; IRC §385 related-party debt documentation required | Federal Reserve, IRS | Thin cap analysis; debt vs. equity characterization |
| **United Kingdom (GBP)** | Fully open | Available | No restrictions; thin-cap rules apply (UK CTA 2010 Part 7A) | HMRC | Transfer pricing; interest limitation rules (Finance Act 2017) |
| **Japan (JPY)** | Domestic: functional. Cross-border: available | Available | No restrictions | FSA, BOJ | JGAAP vs. IFRS 9 hedge accounting divergence |
| **Australia (AUD)** | Fully open | Available | No restrictions; thin-cap via Div. 820 ITAA 1997 | ATO | Transfer pricing |

### 1.3 Transfer Pricing on Cash Pools

OECD Transfer Pricing Guidelines Chapter X (2020) specifically addresses financial transactions including cash pooling. Key requirements:
- Header company must earn a fair market spread for the pooling service.
- Arm's-length rate documentation required for each intercompany loan created by physical sweep.
- Short-term nature does not exempt from TP documentation requirements.
- Country-by-Country Reporting (CbCR) — OECD BEPS Action 13 — requires disclosure of intercompany financing flows for groups with consolidated revenue ≥ €750M.

---

## Part 2 — Liquidity Position Analysis (Basel III LCR/NSFR Framework)

### 2.1 Applicability

Basel III LCR and NSFR apply directly to **banks and SIFIs (systemically important financial institutions)**. Non-bank corporates are NOT directly subject. Corporate treasury teams use the framework for:
- Understanding banking counterparty behavior under stress
- Structuring revolving credit facility terms (banks price facilities per LCR treatment of committed lines)
- Benchmarking internal liquidity metrics

### 2.2 LCR — Liquidity Coverage Ratio (BCBS 238)

**Formula:** LCR = Stock of HQLA / Total net cash outflows over 30-day stress period ≥ 100%

**HQLA tiers:**
- Level 1: Coins and banknotes, central bank reserves, sovereign/central bank assets (0% haircut)
- Level 2A: Government/PSE securities (15% haircut; capped at 40% of total HQLA)
- Level 2B: RMBS, corporate bonds rated AA or higher, equity (25–50% haircut; capped at 15% of total HQLA)

**BIS Basel III Monitoring Exercise (Dec 31, 2024):**
- Group 1 banks (Tier 1 capital > €3B): weighted average LCR = **134.8%** (3 banks below 100% minimum)
- Group 2 banks: weighted average LCR = **181.2%**

**Official source:** [bis.org/publ/bcbs238.pdf](https://www.bis.org/publ/bcbs238.pdf)

### 2.3 NSFR — Net Stable Funding Ratio (BCBS 295 / BCBS 324)

**Formula:** NSFR = Available Stable Funding (ASF) / Required Stable Funding (RSF) ≥ 100%

**Time horizon:** 1-year funding profile (compared to LCR's 30-day stress)

**BIS Basel III Monitoring Exercise (Dec 31, 2024):**
- Group 1 banks: weighted average NSFR = **123.7%** (all banks above 100%)

**Official source:** [bis.org/bcbs/publ/d295.htm](https://www.bis.org/bcbs/publ/d295.htm) and [bis.org/bcbs/publ/d324.pdf](https://www.bis.org/bcbs/publ/d324.pdf)

### 2.4 Corporate Treasury Liquidity Metrics (Non-Bank)

| Metric | Formula | What it Measures |
|---|---|---|
| **Current ratio** | Current assets / Current liabilities | Short-term liquidity; >1× preferred |
| **Quick ratio** | (Cash + Receivables) / Current liabilities | Liquidity excluding inventory |
| **Cash conversion cycle** | DIO + DSO − DPO | Days to convert inventory to cash |
| **Revolving credit headroom** | Available facility − Outstanding draws | Liquidity buffer |
| **Days liquidity on hand** | Cash + Available revolving / Daily cash burn | Runway metric |

---

## Part 3 — Hedge Accounting Qualification

### 3.1 Hedge Types

| Hedge Type | Hedged Item | Accounting Treatment |
|---|---|---|
| **Fair value hedge** | Firm commitment or asset/liability carried at amortized cost | Changes in fair value of both hedging instrument AND hedged item recognized in P&L; basis adjustment |
| **Cash flow hedge** | Highly probable forecast transaction or variable-rate exposure | Effective portion → OCI (accumulated); reclassified to P&L when hedged item affects earnings |
| **Net investment hedge** | Net assets of a foreign operation | Effective portion → OCI (CTA); reclassified to P&L on disposal of the foreign operation |

### 3.2 ASC 815 vs. IFRS 9 — Hedge Accounting Qualification Comparison

| Feature | **ASC 815 (US GAAP)** | **IFRS 9 (IFRS)** |
|---|---|---|
| **Framework** | Rules-based; prescriptive criteria | Principles-based; economic relationship test |
| **Effectiveness testing — prospective** | Must demonstrate hedge will be highly effective; qualitative critical-terms-match permitted after ASU 2017-12 | "Economic relationship" test — qualitative; no 80–125% numerical threshold |
| **Effectiveness testing — retrospective** | 80–125% quantitative range (or qualitative after ASU 2017-12) | **No retrospective test required** |
| **Shortcut method** | Available for qualifying plain-vanilla interest rate swaps matching all terms | **Not available** |
| **Eligible hedged items** | Specific items; benchmark interest rate component hedgeable (ASU 2017-12) | Broader: risk components of non-financial items allowed; layer component hedging (ASU 2022-01 equivalent) |
| **Basis adjustment (fair value hedge)** | Included in carrying amount; amortized on de-designation | Included in carrying amount |
| **Macro / portfolio hedging** | **Not permitted** under ASC 815 (FASB project ongoing) | IAS 39 portfolio fair value hedge carve-out retained in IFRS 9; IASB macro hedging project (DP/2014/1) still incomplete |
| **Non-derivative hedging instruments** | Not permitted for most hedge types | Permitted for FX risk hedges |
| **Documentation** | Contemporaneous at hedge inception | Contemporaneous at hedge inception |

**Key ASU updates:**
- ASU 2017-12: Simplified hedge accounting; eliminated retrospective quantitative test option; added last-of-layer method
- ASU 2022-01: Portfolio layer method for fair value hedges of interest rate risk
- ASU 2025-09: Post-LIBOR benchmark rate transition issues

### 3.3 Qualification Checklist (Both Standards)

1. **Identify:** hedging instrument (must be a derivative, or non-derivative FI for IFRS 9 FX hedges)
2. **Identify:** hedged item (eligible under applicable standard)
3. **Specify:** hedge type (fair value / cash flow / net investment)
4. **Document (contemporaneous):** hedging relationship, risk management objective, hedge ratio
5. **Demonstrate (prospective):** economic relationship / effectiveness (qualitative or quantitative)
6. **Assess (ongoing):** rebalancing requirements (IFRS 9) or de-designation if criteria no longer met (ASC 815)

### 3.4 Local GAAP Hedge Accounting Variations

| Jurisdiction | Standard | Key Difference from IFRS 9 |
|---|---|---|
| **Germany** | HGB §254 (Bewertungseinheit) | Linked valuation — stricter than IFRS 9; no fair value recognition |
| **Japan** | ASBJ Accounting Standard No. 10 | Separate effectiveness testing (similar to 80–125% range); own documentation |
| **India** | Ind AS 109 = IFRS 9 (consolidated); AS 30/31/32 (statutory) | Statutory accounts may differ from Ind AS consolidated |
| **China** | CAS 24 | Follows IFRS 9 hedge accounting model closely |
| **Brazil** | CPC 48 | = IFRS 9 equivalent |

**Three-layer documentation implication:** Fortune-50 centralized treasury must track hedge relationships separately for: (a) consolidated IFRS/US GAAP, (b) each subsidiary's local statutory GAAP, and (c) local tax treatment.

---

## Part 4 — FX Exposure Analysis

### 4.1 Functional Currency Determination

| Criterion | ASC 830 | IAS 21 |
|---|---|---|
| **Primary approach** | Indicators-based; primary and secondary indicators | Hierarchical: primary indicators (sales prices, costs) first; secondary if inconclusive |
| **Sales price indicator** | Currency in which sales prices are denominated | Currency that mainly influences sales prices |
| **Cost indicator** | Currency of the country whose costs determine selling prices | Currency that mainly influences labor and materials costs |
| **Change in functional currency** | Prospective from date of change | Prospective from date of change |

### 4.2 Remeasurement vs. Translation

| Method | When Applied | Rate Used | P&L vs. OCI |
|---|---|---|---|
| **Temporal (Remeasurement)** | Functional currency = reporting currency (foreign entity is integral operation) | Monetary items: closing rate; non-monetary at historical cost: historical rate; non-monetary at fair value: rate at fair value date | FX gains/losses → **P&L** |
| **Current Rate (Translation)** | Functional currency ≠ reporting currency (foreign operation is a separate entity) | Assets/liabilities: closing rate; income/expenses: transaction date rate or average rate | FX differences → **OCI (CTA)** |

### 4.3 Common FX Translation Errors

| Error | Standard Violated | Consequence |
|---|---|---|
| Monetary items translated at historical rate | ASC 830-10 / IAS 21 §23 | Understated/overstated balance sheet; P&L distortion |
| CTA not recycled on disposal of foreign entity | ASC 830-30-40 / IAS 21 §48 | Common material audit finding; understated disposal gain/loss |
| IC payables/receivables not revalued at period-end spot rate | ASC 830-20-35 / IAS 21 §23 | IC mismatch; consolidation error |
| Incorrect OCI vs. P&L classification | ASC 830-20 / IAS 21 §28 | Earnings manipulation risk |
| Inconsistent rate application across entities (e.g., different closing times) | ASC 830-30-45 / IAS 21 §26 | Consolidation difference |

### 4.4 Hyperinflationary Economies

| Standard | Approach | Jurisdictions (illustrative) |
|---|---|---|
| **ASC 830** | Remeasure as if functional = parent's reporting currency; gains/losses → P&L | Argentina (historically), Venezuela, Zimbabwe |
| **IAS 29 / IAS 21** | Restate local functional currency statements using IAS 29 purchasing power adjustments, then translate | Argentina (IAS 29 applies since 2018), Lebanon, South Sudan |

---

## Part 5 — Cash Repatriation and Withholding Tax

### 5.1 Repatriation Channels

1. **Dividend** — most common; withholding tax applies in most jurisdictions; requires retained earnings
2. **Royalties / IP licensing** — taxed as business income; transfer pricing scrutiny
3. **Service fees / management charges** — transfer pricing; thin rules on deductibility
4. **Intercompany loan repayment** — no withholding on principal; interest withholding varies
5. **Capital reduction** — regulatory approval required in many jurisdictions

### 5.2 Withholding Tax Matrix (illustrative; treaty rates vary)

| Country | Dividend WHT (domestic) | Reduced by tax treaty (example) | Notes |
|---|---|---|---|
| **China** | 10% | 5% (if ≥ 25% shareholding under many treaties, e.g., HK-China) | SAFE registration of equity investment required before repatriation |
| **India** | 20% | 10–15% (US-India treaty); 10% (Mauritius route subject to GAAR) | Repatriation straightforward; TP on IC services scrutinized |
| **Brazil** | 0% (dividends exempt at entity level since 1996) | N/A | Interest on Net Equity (JCP) — deductible for payor, taxed at 15% for recipient |
| **Japan** | 20.42% | 5% (US-Japan treaty for ≥ 10% shareholding) | Full conversion available; no capital controls |
| **Germany** | 26.375% | 5% (EU Parent-Subsidiary Directive — 0% for EU subsidiaries; 5% for US) | EU PSD: 0% WHT for EU-resident parent |
| **Australia** | 30% | 5% (US-Australia treaty for ≥ 10% shareholding) | Dividend imputation credit system reduces economic double tax |
| **UK** | 0% (no WHT on dividends under UK domestic law since 2016) | N/A | No withholding; PE risks for UK-based treasury centers |
| **United States** | 30% (non-treaty) | 5% (most treaties for corporate parent ≥ 10% ownership) | PTEP (previously taxed earnings and profits) — post-TCJA repatriation mechanism |

**Advisory note:** Treaty rates depend on meeting conditions (ownership threshold, beneficial ownership, LOB clauses). Verify current treaty text and domestic anti-avoidance rules with qualified tax counsel before structuring repatriation.

### 5.3 China SAFE — Specific Requirements

- **Equity investment registration:** Foreign equity must be registered with SAFE before profits can be repatriated.
- **External debt quota:** Cross-border loans subject to macro-prudential regulation; total cross-border financing capped at 2× registered capital (for non-financial enterprises).
- **Dividend timeline:** Typically 3–6 months from audit completion to cash receipt in offshore account.
- **Capital account transactions:** Most require SAFE approval or filing; current account (trade) transactions: banks can handle without SAFE pre-approval if documentation is in order.
- **Circular 19 (2019):** Governs cross-border capital pool; domestic participating entities must meet solvency and compliance requirements.

**Source:** SAFE — [safe.gov.cn/en/](https://www.safe.gov.cn/en/) — English summaries available

### 5.4 India FEMA / RBI — Specific Requirements

- **FEMA 1999:** Governs all FX transactions for Indian residents.
- **Liberalised Remittance Scheme (LRS):** Individuals up to USD 250,000/year; corporates use separate ODI (Overseas Direct Investment) / FDI routes.
- **ECB (External Commercial Borrowings):** RBI master direction sets eligible borrowers, lenders, minimum maturity, all-in-cost ceiling (benchmarked to SOFR/LIBOR successor + spread).
- **Royalties / service fees:** Automatic route available for royalty up to 8% of sales / 25% of net foreign exchange earnings; above = government approval.
- **Dividend repatriation:** Automatic; no prior RBI approval required after audit is complete.

**Source:** RBI Master Direction on ECB: [rbi.org.in](https://www.rbi.org.in/) — fully public

---

## Part 6 — Derivatives Reporting Obligations

### 6.1 Dodd-Frank (US) — CFTC

- **Scope:** Swaps (IRS, CDS, FX swaps/forwards designated as swaps) by US persons and non-US persons with US nexus.
- **End-user exception:** Corporates qualifying as non-financial end-users can claim exception from mandatory clearing if hedging commercial risk; must notify CFTC (Form EE or equivalent platform).
- **Reporting:** All swaps must be reported to a CFTC-registered Swap Data Repository (SDR) — either dealer-side reporting or dual-sided.
- **CFTC registration:** Dealers and major swap participants must register; end-users generally do not.

**Official source:** [cftc.gov/LawRegulation/DoddFrankAct/](https://www.cftc.gov/LawRegulation/DoddFrankAct/index.htm) — fully public

### 6.2 EMIR (EU) — ESMA

- **Scope:** OTC derivatives contracts concluded by EU counterparties (financial and non-financial counterparties above clearing threshold).
- **Clearing obligation:** NFC+ (non-financial counterparty above threshold) — must clear through a CCP for IRS and CDS in certain categories.
- **Reporting obligation:** Both counterparties (or one delegating to the other) must report to a registered Trade Repository.
- **EMIR Refit (2019) / EMIR 3.0 (2024):** Revised thresholds and reporting formats; ISO 20022 XML format for trade reports from April 2024.
- **UK EMIR (post-Brexit):** UK retained EMIR obligations under UK law via onshoring; now supervised by FCA. Substantively similar to EU EMIR.

**Official source:** [esma.europa.eu/data-reporting/emir-reporting](https://www.esma.europa.eu/data-reporting/emir-reporting) — fully public

### 6.3 ISO 20022 — Treasury Impact

SWIFT completed mandatory migration from MT to ISO 20022 (MX messages) for cross-border interbank payments on **November 22, 2025**. Corporate treasury implications:
- Banks mandated; corporates strongly encouraged by end-2026 (structured country codes, town names required).
- Richer data in ISO 20022 (pacs.008, camt.053) enables automated reconciliation.
- TMS systems must support ISO 20022 processing.

---

## Part 7 — Official Documentation URLs

| Standard / Regulation | URL | Access |
|---|---|---|
| **IFRS 9** (Financial Instruments — hedge accounting §6.4–6.8) | [ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/](https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/) | Free with registration |
| **IAS 21** (Effects of Changes in FX Rates) | [ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/](https://www.ifrs.org/issued-standards/list-of-standards/ias-21-the-effects-of-changes-in-foreign-exchange-rates/) | Free with registration |
| **IAS 39** (legacy hedge accounting — still used for portfolio fair value hedges) | [ifrs.org/issued-standards/list-of-standards/ias-39-financial-instruments-recognition-and-measurement/](https://www.ifrs.org/issued-standards/list-of-standards/ias-39-financial-instruments-recognition-and-measurement/) | Free with registration |
| **ASC 815** (Derivatives and Hedging) | [asc.fasb.org](https://asc.fasb.org/) → search "815" | Free with registration |
| **ASC 830** (Foreign Currency Matters) | [asc.fasb.org](https://asc.fasb.org/) → search "830" | Free with registration |
| **FASB ASU 2017-12** (Hedge Accounting Improvements) | [storage.fasb.org/ASU%202017-12.pdf](https://storage.fasb.org/ASU%202017-12.pdf) | Fully public |
| **Basel III LCR** (BCBS 238) | [bis.org/publ/bcbs238.pdf](https://www.bis.org/publ/bcbs238.pdf) | Fully public |
| **Basel III NSFR** (BCBS 295) | [bis.org/bcbs/publ/d295.htm](https://www.bis.org/bcbs/publ/d295.htm) | Fully public |
| **CFTC Dodd-Frank End-User Exception** | [cftc.gov/LawRegulation/DoddFrankAct/](https://www.cftc.gov/LawRegulation/DoddFrankAct/index.htm) | Fully public |
| **EU EMIR** (Regulation 648/2012) | [eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32012R0648](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32012R0648) | Fully public |
| **ESMA EMIR Reporting** | [esma.europa.eu/data-reporting/emir-reporting](https://www.esma.europa.eu/data-reporting/emir-reporting) | Fully public |
| **SAFE (China) — English** | [safe.gov.cn/en/](https://www.safe.gov.cn/en/) | Fully public |
| **RBI — ECB Master Direction** | [rbi.org.in](https://www.rbi.org.in/) | Fully public |
| **Fed SR 10-6** (Liquidity Risk Management) | [federalreserve.gov/supervisionreg/srletters/sr1006.pdf](https://www.federalreserve.gov/supervisionreg/srletters/sr1006.pdf) | Fully public |

---

## Mandatory Advisory Note

This analysis is advisory and based solely on the facts described. Treasury regulations, capital controls, and withholding tax rates change frequently and vary by entity type. Verify current requirements with local legal counsel and qualified tax advisors before executing any treasury strategy. Do not use this analysis as the basis for actual financial transactions. This skill does not form a financial-advisor or investment-advisor relationship.
