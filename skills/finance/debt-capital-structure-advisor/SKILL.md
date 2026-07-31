---
name: debt-capital-structure-advisor
description: Multi-jurisdiction reference framework for debt and capital structure advisory — optimal capital structure theory (M&M, trade-off, pecking order), leverage and credit metrics, debt instruments, covenant analysis, refinancing, WACC optimization, Basel III/IV capital requirements, liability management, rating agency methodologies, and ESG-linked financing (SLBs/SLLs, green bonds). Advisory only — never executes transactions, accesses banking systems, or writes to any system of record.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-03"
  category: finance
  lifecycle: experimental
---

# Debt & Capital Structure Advisor Skill

> Read-only reference framework. All conclusions are advisory and educational. Capital structure decisions, credit agreements, and financing transactions require qualified legal counsel, investment bankers, and credit professionals. This skill does not constitute investment advice, a fairness opinion, or legal advice, and does not form a financial-advisor or investment-advisor relationship.

---

## Part 1 — Optimal Capital Structure Theory

### 1.1 Modigliani–Miller Theorems

**Proposition I (no taxes, no frictions):** Under perfect capital markets (no taxes, no transaction costs, no information asymmetry), firm value is independent of capital structure. V_L = V_U.

**Proposition II (no taxes):** Cost of equity rises linearly with leverage to offset the benefit of cheaper debt. WACC is constant.

**Proposition I (with corporate taxes):** Firm value increases with leverage due to the interest tax shield. V_L = V_U + PV(Tax Shield). Tax shield = τ × D (perpetuity at risk-free rate for permanent debt).

**Proposition II (with taxes):** r_E = r_U + (r_U − r_D) × (1 − τ) × (D/E). The after-tax cost of debt reduces the leverage penalty on equity cost.

**Key limitations:** M&M ignores distress costs, agency costs, information asymmetry, and market access constraints — motivating the trade-off and pecking order theories.

### 1.2 Trade-Off Theory

Optimal capital structure balances the tax benefit of debt against expected costs of financial distress:

**V_L = V_U + PV(Tax Shield) − PV(Financial Distress Costs) − PV(Agency Costs)**

| Cost Type | Description | Driver |
|---|---|---|
| **Direct distress costs** | Legal fees, restructuring advisor fees, court costs | ~3–5% of pre-distress firm value for large firms |
| **Indirect distress costs** | Lost customers, vendor pricing, management distraction, foregone investment | Estimated 10–23% of firm value in empirical studies |
| **Agency cost of debt** | Asset substitution (risk shifting); under-investment problem (Myers 1977); claim dilution | Mitigated by covenants |
| **Agency cost of equity** | Free cash flow problem (Jensen 1986) — excess FCF may be misallocated | Mitigated by leverage and dividend policy |

**Prediction:** Firms with high tangible assets (collateral), stable cash flows, and high profitability use more debt. Growth firms (high intangibles, investment options) use less.

### 1.3 Pecking Order Theory (Myers & Majluf 1984)

Firms prefer internal financing first, then debt, then equity — due to information asymmetry costs:

1. **Retained earnings** — no adverse selection signal
2. **Debt** — lower information asymmetry than equity; signals management believes firm is not overvalued
3. **Equity** — "lemons" signal; equity issuance interpreted as overvaluation

**Implication:** No optimal debt ratio target; financing is driven by investment needs and internal cash generation. Profitable firms accumulate financial slack and use less debt.

**Evidence:** Empirically strong in the short run; trade-off theory better explains cross-sectional leverage differences across industries.

### 1.4 Market Timing Theory (Baker & Wurgler 2002)

Managers issue equity when market-to-book is high (perceived overvaluation) and repurchase when low. Capital structure is a cumulative outcome of past market timing decisions rather than convergence toward a target.

### 1.5 Dynamic Trade-Off — Mean Reversion

Empirical evidence (Leary & Roberts 2005; Flannery & Rangan 2006): firms adjust toward target leverage at ~25–30% per year. Adjustment is slower when transaction costs are high. Mean reversion speed varies by credit quality and market conditions.

---

## Part 2 — Leverage and Credit Metrics

### 2.1 Core Credit Metrics

| Metric | Formula | Typical Thresholds |
|---|---|---|
| **Total Leverage** | Total Debt / EBITDA | Investment grade: <2.5×; BB: 2.5–4.5×; B: 4.5–7×; CCC: >7× |
| **Net Leverage** | (Total Debt − Cash) / EBITDA | Often used in HY covenant definitions |
| **Senior Secured Leverage** | Senior Secured Debt / EBITDA | Relevant for 1st-lien covenant tests |
| **Interest Coverage (EBITDA)** | EBITDA / Interest Expense | IG: >8×; BB: 3–6×; B: 1.5–3×; CCC: <1.5× |
| **Interest Coverage (EBIT)** | EBIT / Interest Expense | More conservative; EBIT excludes D&A |
| **DSCR** | (EBITDA − Capex − Tax − Working Capital) / Debt Service | Project finance: typically ≥1.20× minimum; 1.40× target |
| **Fixed Charge Coverage** | (EBITDA − Capex) / (Interest + Scheduled Principal) | Maintenance covenant common in bank credit agreements |
| **Debt / Total Capital** | Total Debt / (Total Debt + Equity) | IG threshold often <40–50% |
| **FFO / Debt** | Funds from Operations / Total Debt | S&P credit metric; FFO = EBITDA − Interest − Tax |

### 2.2 EBITDA Adjustments and Addbacks

Credit agreements and rating agency analyses typically allow EBITDA adjustments. Key addbacks (and risks):

| Addback Type | Rationale | Analytical Risk |
|---|---|---|
| **Non-recurring charges** | Restructuring, M&A costs, one-time items | Recurrence risk; "recurring non-recurring" charges inflate adjusted EBITDA |
| **Run-rate synergies** | Pro-forma for completed acquisitions | Realization uncertainty; time horizon |
| **Stock-based compensation** | Non-cash charge | Dilution impact; cash settlement risk |
| **Lease expense (pre-IFRS 16)** | EBITDAR analysis when significant operating leases | Ignores lease liabilities under ASC 842 / IFRS 16 |
| **LTM vs. NTM EBITDA** | Trailing vs. forward-looking | Forward EBITDA introduces forecast risk |

**Advisory:** Rating agencies (S&P, Moody's, Fitch) use their own adjusted EBITDA methodologies that may differ materially from credit agreement definitions. Verify the applicable definition in each context.

### 2.3 IFRS 16 / ASC 842 Impact on Credit Metrics

Under IFRS 16 and ASC 842, most operating leases are capitalized onto the balance sheet as right-of-use (ROU) assets and lease liabilities. Impact on credit metrics:

| Metric | Pre-IFRS 16 / ASC 842 | Post-IFRS 16 / ASC 842 |
|---|---|---|
| **Debt** | Excludes operating lease obligations | Includes lease liabilities → higher reported debt |
| **EBITDA** | Includes full operating lease expense | Excludes lease expense (replaced by D&A + interest) → higher EBITDA |
| **Net Leverage** | Lower reported (off-balance-sheet) | Higher numerator (lease liabilities) but higher denominator (EBITDA) |
| **Interest Coverage** | Lease expense above EBIT line | Interest on lease liability in interest expense; EBITDA excludes full lease cost |

**Comparability:** S&P and Moody's capitalize operating leases in their adjusted metrics regardless of accounting standard — use 8× rent multiple for long-term leases. Many credit agreements exclude IFRS 16 / ASC 842 lease liabilities from "financial indebtedness" definitions.

---

## Part 3 — Debt Instruments

### 3.1 Instrument Taxonomy

| Instrument | Seniority | Security | Typical Pricing | Typical Use |
|---|---|---|---|---|
| **Revolving Credit Facility (RCF)** | Senior secured | 1st lien | SOFR/EURIBOR + 150–300 bps | Working capital; liquidity backstop |
| **Term Loan A (TLA)** | Senior secured | 1st lien | SOFR + 150–300 bps; amortizing | Bank market; investment grade and leveraged |
| **Term Loan B (TLB)** | Senior secured | 1st lien | SOFR + 250–500 bps; 1% annual amortization | Institutional/CLO market; leveraged buyouts |
| **Senior Secured Notes (SSN)** | Senior secured | 1st or 2nd lien | Fixed coupon; 5–10 year | Bond market; LBO financing |
| **Senior Unsecured Notes** | Senior unsecured | None | Fixed coupon; 5–10 year | Investment grade bond market |
| **High Yield (HY) Bonds** | Senior unsecured / subordinated | None | Fixed coupon 200–600 bps above UST | Sub-IG corporate bond market |
| **Convertible Notes** | Senior unsecured | None | Below-market coupon; conversion premium 20–40% | Equity-linked; low cash interest cost |
| **Mezzanine / PIK** | Junior subordinated | None or 2nd lien | Cash + PIK; 10–15% all-in | LBO; bridge financing |
| **Unitranche** | Single-class | 1st lien | Blended rate 400–700 bps over SOFR | Mid-market; simplified structure |
| **Second Lien** | Junior secured | 2nd lien | SOFR + 500–800 bps | Leveraged; bridge to HY bonds |

### 3.2 Bank vs. Bond Market Comparison

| Feature | Syndicated Bank Loan | High Yield Bond |
|---|---|---|
| **Disclosure** | Private; credit agreement negotiated | Public SEC-registered (S-11) or 144A with registration rights |
| **Voting/amendment** | Required lender consent (majority or supermajority) | Indenture trustee; bondholder consent via consent solicitation |
| **Repayment** | Prepayable at par (subject to call protection in TLB) | Non-call period (NC2 to NC5); make-whole call; call schedule |
| **Maintenance covenants** | Common (leverage, DSCR, coverage) | Incurrence-only in HY bonds |
| **Flexibility** | More amendable; relationship-based | Less flexible post-issuance |
| **Rate** | Floating (SOFR/EURIBOR-linked) | Fixed |
| **Tenor** | 3–7 years (bank); 5–7 years (TLB) | 5–10 years |
| **Minimum size** | No minimum; smaller deals feasible | Typically ≥$200M for HY |

### 3.3 Benchmark Rate Transition

LIBOR ceased publication for USD (June 30, 2023) and all remaining currencies. Replacement benchmarks:

| Currency | Replacement | Notes |
|---|---|---|
| **USD** | SOFR (Secured Overnight Financing Rate) | CME Term SOFR widely used for term loan pricing |
| **EUR** | €STR (Euro Short-Term Rate) / EURIBOR retained | EURIBOR not discontinued; €STR for derivative fallbacks |
| **GBP** | SONIA (Sterling Overnight Index Average) | SONIA compounded in arrears |
| **JPY** | TONA (Tokyo Overnight Average Rate) | |
| **CHF** | SARON (Swiss Average Rate Overnight) | |

**Credit Spread Adjustment (CSA):** 11.448 bps (1-month SOFR), 26.161 bps (3-month), 42.826 bps (6-month) — ARRC/ISDA recommended spreads for hardwired fallbacks.

---

## Part 4 — Covenant Analysis

### 4.1 Maintenance vs. Incurrence Covenants

| Type | Definition | Market | Test Frequency |
|---|---|---|---|
| **Maintenance covenant** | Must satisfy ratio at each test date regardless of action | Bank credit agreements | Quarterly (or semi-annually) |
| **Incurrence covenant** | Must satisfy ratio only when taking a specified action (e.g., incurring additional debt, making acquisitions, paying dividends) | High yield bond indentures | Event-triggered |

**Investment grade vs. leveraged:** Investment grade revolvers typically have one maintenance covenant (leverage or interest coverage) or are covenant-lite. Leveraged credit agreements may be covenant-lite (incurrence only) for TLBs; TLAs typically retain maintenance covenants.

### 4.2 Common Covenant Types

**Financial maintenance covenants:**

| Covenant | Typical Definition | Purpose |
|---|---|---|
| **Maximum Total Net Leverage** | Net Debt / Consolidated EBITDA ≤ X.Xx | Core lever for debt capacity monitoring |
| **Maximum Senior Secured Leverage** | Senior Secured Debt / EBITDA ≤ X.Xx | First-lien specific; protect senior lenders |
| **Minimum Interest Coverage** | EBITDA / Cash Interest ≥ X.Xx | Cash interest serviceability |
| **Minimum Fixed Charge Coverage** | (EBITDA − Capex − Cash Taxes) / Fixed Charges ≥ 1.00–1.10× | Cash flow after maintenance capex |
| **Minimum DSCR** | CFADS / Debt Service ≥ 1.20× | Project finance; infrastructure |
| **Maximum Capex** | Annual capital expenditure ≤ $X | Preserve cash for debt service |

**Negative covenants (incurrence):**

- **Debt basket:** Permitted additional indebtedness (fixed + ratio-based)
- **Lien basket:** Permitted additional security interests (gratis liens, specified liens)
- **Restricted payments basket:** Dividends, share buybacks, restricted junior payments — often subject to ratio test + builder basket
- **Investment basket:** Permitted acquisitions and investments
- **Asset sale basket:** Permitted disposals; excess proceeds reinvestment requirement
- **Affiliate transactions:** Arms-length requirement; fairness opinion thresholds

### 4.3 DSCR Analysis Framework

**Debt Service Coverage Ratio (Project Finance):**

DSCR = Cash Flow Available for Debt Service (CFADS) / Debt Service

**CFADS = Revenue − Operating Costs − Tax − Changes in Working Capital − Maintenance Capex**

| DSCR Threshold | Interpretation |
|---|---|
| **≥ 1.40×** | Comfortable; typical project finance target case |
| **1.20–1.40×** | Minimum maintenance covenant range |
| **1.00–1.20×** | Thin coverage; covenant waiver territory |
| **< 1.00×** | Debt service shortfall; default event |

### 4.4 Restricted Payments Basket Analysis

Restricted payments (dividends, share buybacks, junior debt payments) are governed by:

1. **Condition precedent:** No event of default (or incurrence default) outstanding
2. **Pro forma leverage test:** After giving effect to the payment, leverage ≤ specified ratio
3. **Builder basket:** Cumulative capacity = 50% of Consolidated Net Income since closing date + other addbacks (equity proceeds, asset sale proceeds, investment returns)
4. **Fixed baskets:** Small fixed-dollar baskets for management fees, ordinary course dividends

**Advisory:** Restricted payments covenant analysis requires review of the actual credit agreement and indenture definition of "Consolidated Net Income" and "Restricted Subsidiary." Definitions vary substantially.

### 4.5 Covenant Headroom Analysis

**Headroom = (Actual Metric − Covenant Threshold) / Covenant Threshold × 100%**

Advisors typically model EBITDA stress scenarios (10–25% downside) to assess covenant breach risk. Lenders may require waivers or amendments when headroom falls below ~10–15%.

---

## Part 5 — Credit Ratings and Rating Agency Methodologies

### 5.1 Rating Scale Comparison

| Category | S&P | Moody's | Fitch | Description |
|---|---|---|---|---|
| **Investment Grade** | | | | |
| Highest quality | AAA | Aaa | AAA | Extremely strong capacity |
| Very high quality | AA+/AA/AA− | Aa1/Aa2/Aa3 | AA+/AA/AA− | Very strong |
| High quality | A+/A/A− | A1/A2/A3 | A+/A/A− | Strong; somewhat susceptible to adverse conditions |
| Good quality | BBB+/BBB/BBB− | Baa1/Baa2/Baa3 | BBB+/BBB/BBB− | Adequate; more susceptible |
| **Speculative Grade** | | | | |
| Speculative | BB+/BB/BB− | Ba1/Ba2/Ba3 | BB+/BB/BB− | Less vulnerable; speculative characteristics |
| Highly speculative | B+/B/B− | B1/B2/B3 | B+/B/B− | More vulnerable; adverse conditions impair |
| Substantial risk | CCC+ to C | Caa1 to Ca | CCC to C | Vulnerable; currently impaired |
| Default | D | C | D | In default |

**Investment grade / sub-investment grade boundary:** BBB−/Baa3 — crossing this triggers forced selling by IG mandates; significant spread widening.

### 5.2 S&P Corporate Rating Methodology

S&P's issuer credit rating framework (Corporate Methodology, updated 2021):

1. **Business Risk Profile:** Competitive position (market share, diversification, profitability) + Industry risk
2. **Financial Risk Profile:** Anchor ratio = FFO/Debt. Key thresholds:
   - Minimal: FFO/Debt > 60% (AAA–AA)
   - Modest: 45–60% (A)
   - Intermediate: 30–45% (BBB)
   - Significant: 20–30% (BB)
   - Aggressive: 12–20% (B)
   - Highly leveraged: <12% (CCC)
3. **Comparable ratings analysis:** Notch up/down based on peers
4. **Modifiers:** Diversification, capital structure, financial policy, liquidity, management/governance, group influence

**S&P FFO:** Net income + D&A + interest expense − interest income (operating basis). Excludes one-time items.

### 5.3 Moody's Corporate Rating Methodology

Moody's uses a scorecard approach varying by industry (Moody's Rating Methodology for Global Manufacturing Companies, 2021, and sector-specific variants):

- **Grid factors:** Scale, business profile metrics, profitability, leverage/coverage, financial policy
- **Key ratios:** Debt/EBITDA; (EBITDA − Capex) / Debt; EBITA / Interest; FCF/Debt; Retained Cash Flow/Net Debt
- **Qualitative overlays:** Governance; ESG factors; liquidity; event risk
- **Loss Given Default:** Determines notch difference between issuer rating and issue rating based on security/seniority

### 5.4 Fitch Corporate Rating Methodology

Fitch uses Net Debt/EBITDA and EBITDA/Interest as primary leverage and coverage metrics. Fitch applies sector-specific overlays (Fitch Corporate Rating Criteria, updated 2023). Key features:
- **Outlook/Watch:** Positive/Stable/Negative/Rating Watch Positive or Negative
- **IDR vs. issue rating:** IDR (Issuer Default Rating) is the senior unsecured benchmark; instrument notching per recovery analysis
- **ESG Relevance Scores:** 1–5 scale integrated into rating rationale since 2019

### 5.5 Rating Agency ESG Integration

All three agencies have integrated ESG factors into their rating criteria:

| Agency | Framework | Disclosure |
|---|---|---|
| **S&P** | ESG Evaluation (separate product); ESG credit indicators embedded in rating reports (E-1 to E-5 per factor) | Rating reports; ESG evaluation reports |
| **Moody's** | ESG scores (1–5 per factor); credit impact scores (CIS-1 to CIS-5) | Moody's ESG Dashboard |
| **Fitch** | ESG Relevance Scores (1–5 per factor); 4–5 = credit-relevant | ESG Relevance Scores section in all rating actions |

**Carbon transition risk:** High-emitting sectors (energy, utilities, materials, autos) face explicit negative rating pressure from carbon transition pathways. Paris Agreement alignment analysis increasingly incorporated.

---

## Part 6 — Refinancing Analysis and Maturity Wall Management

### 6.1 Refinancing Decision Framework

**Motivations for refinancing:**
- Maturity extension (avoid refinancing risk)
- Interest rate reduction (lower all-in cost)
- Covenant reset (improve flexibility)
- Leverage reduction / mix shift (bank to bond)
- ESG-linked conversion (transition to SLL/SLB)

**Key economics:**
- **Call premium / make-whole cost:** For HY bonds in non-call period; make-whole = NPV of remaining coupon + principal at T+50 bps (or specified spread); can be substantial in low-rate environments
- **Prepayment premium on TLBs:** Typically 101 soft call in first 6 months post-issuance; otherwise par
- **Transaction costs:** Underwriting fees (1–3% of issuance size); legal fees; commitment fees

**Break-even analysis:** PV(interest savings) vs. PV(refinancing costs). Typical break-even 12–24 months for opportunistic refinancings.

### 6.2 Maturity Wall Management

**Definition:** Concentration of debt maturities in a short time window creating refinancing cliff risk.

**Maturity profile analysis:**
1. Map each debt instrument to maturity date
2. Identify RCF maturity (typically shortest — often 3–5 years)
3. Identify springing maturity triggers (RCF often springs if HY bonds not refinanced by specified date)
4. Stress-test access to capital markets under adverse conditions (rating downgrade, market dislocation)

**Management strategies:**
- **Staggered maturities:** Spread maturities across 3–7 year window
- **Proactive refinancing:** Refinance 2–3 years before maturity (avoid "refinancing cliff" in final 12 months)
- **RCF renewal:** Maintain RCF with 12+ months headroom on maturity
- **Amend-and-extend:** Bank market mechanism to extend maturities via lender vote

**Rule of thumb:** Avoid more than 30–35% of total debt maturing in any 12-month window.

### 6.3 Liability Management Transactions

| Transaction | Mechanism | Purpose |
|---|---|---|
| **Tender offer** | Issuer purchases notes for cash at premium to par | Reduce outstanding principal; accelerate maturity reduction |
| **Open market repurchase** | Purchase at market price up to indenture basket | Opportunistic; typically restricted to specific baskets |
| **Exchange offer** | Holders exchange existing notes for new notes (different tenor/coupon) | Maturity extension; distressed exchange (may constitute default under rating agency criteria) |
| **Consent solicitation** | Request holder consent to amend indenture terms (covenants, call protection) | Loosen restrictions; not a restructuring |
| **Par call / special mandatory redemption** | Contractual redemption at par | Asset sale proceeds; equity issuance triggers |

**Distressed exchange:** S&P and Moody's treat exchanges at below par as a "selective default" (SD) or "D" event. Resolves upon close of exchange offer.

---

## Part 7 — WACC Optimization and Capital Structure Efficiency

### 7.1 WACC Formula

**WACC = (E/V) × r_E + (D/V) × r_D × (1 − τ)**

Where:
- E = Market value of equity; D = Market value of debt; V = E + D
- r_E = Cost of equity (CAPM: r_f + β × ERP)
- r_D = Pre-tax cost of debt (YTM on outstanding debt)
- τ = Marginal corporate tax rate

### 7.2 Cost of Equity — CAPM Parameters

| Parameter | Description | Estimation Notes |
|---|---|---|
| **r_f** | Risk-free rate | 10-year government bond yield; use currency-consistent benchmark |
| **β (levered)** | Equity beta; measure of systematic risk | Historical regression (60M monthly returns); 5-year weekly; consensus sources (Bloomberg, Damodaran) |
| **β (unlevered)** | Asset beta; strip financial leverage | β_U = β_L / [1 + (1−τ) × D/E] |
| **ERP** | Equity risk premium | Damodaran implied ERP; survey-based (Fernandez); historical 5–6% for US; country risk premium added for EM |
| **Size premium** | Small-cap additional return | CRSP/Duff & Phelps deciles; typically 0–3% depending on market cap |
| **Country risk premium** | Sovereign default spread × volatility adjustment | Damodaran country CRP methodology |

### 7.3 Debt Tax Shield vs. Distress Cost Trade-Off

| Factor | Effect on Optimal Leverage |
|---|---|
| **Higher marginal tax rate** | Increases value of debt tax shield → higher optimal leverage |
| **Higher non-debt tax shields** (depreciation, NOLs) | Reduce incremental value of interest deduction → lower optimal leverage |
| **Higher asset tangibility** | Better collateral → lower distress costs → higher optimal leverage |
| **Higher asset specificity / intangibles** | Lower recovery in distress → higher distress costs → lower optimal leverage |
| **Higher EBITDA volatility** | Greater probability of distress → lower optimal leverage |
| **Higher growth options** | Under-investment problem more severe; distress destroys option value → lower optimal leverage |

### 7.4 Capital Structure by Industry (Illustrative)

| Sector | Typical Net Leverage | Rationale |
|---|---|---|
| **Utilities / Regulated** | 4–6× net debt/EBITDA | Stable, contracted cash flows; regulatory asset base supports high leverage |
| **Infrastructure / Toll Roads** | 6–10× project-level | Long-dated concession; predictable traffic/revenue |
| **Technology (mature)** | 0–1× | Asset-light; high growth optionality; large cash balances |
| **Pharmaceuticals** | 1–3× | IP-driven cash flows; patent cliff risk limits leverage |
| **Consumer Staples** | 1–3× | Stable cash flows; M&A-driven spikes |
| **Retail** | 1–3× (excl. leases) | Operating leverage; cyclicality |
| **Leveraged Buyouts (LBO)** | 5–7× at entry | Acquisition-driven; deleveraging through FCF |
| **Real Estate (REIT)** | 35–45% LTV | NAV-based; interest coverage key; regulatory REIT rules |

---

## Part 8 — Basel III/IV Capital Requirements for Financial Institutions

### 8.1 Basel III Framework Overview

**Regulatory capital tiers (BCBS Basel III: A global regulatory framework for more resilient banks, June 2011; revised December 2017 — "Basel IV"):**

| Tier | Instruments | Key Criteria |
|---|---|---|
| **Common Equity Tier 1 (CET1)** | Common shares, retained earnings, AOCI (subject to filters) | Fully loss-absorbing; no maturity; discretionary distributions; no embedded derivatives |
| **Additional Tier 1 (AT1)** | Perpetual contingent convertible bonds (CoCos); other perpetual instruments with loss absorption | PONV (Point of Non-Viability) trigger; write-down or conversion mechanism; Basel III §55 criteria |
| **Tier 2** | Subordinated debt ≥5-year maturity; eligible loan loss provisions; AOCI from AFS | Subordinated; step-downs disallowed; max 2% RWA of Tier 2 eligible provisions |

**Minimum capital ratios (Basel III, fully phased-in):**

| Ratio | Minimum | Conservation Buffer | G-SIB Surcharge | Countercyclical Buffer |
|---|---|---|---|---|
| **CET1 / RWA** | 4.5% | +2.5% = 7.0% | 1.0–3.5% | 0–2.5% |
| **Tier 1 / RWA** | 6.0% | +2.5% = 8.5% | Included above | — |
| **Total Capital / RWA** | 8.0% | +2.5% = 10.5% | Included above | — |
| **Leverage Ratio** | 3.0% (Tier 1 / Exposure) | G-SIBs: +0.5–1.0× surcharge | — | — |

**Basel IV (finalized December 2017; implementation January 2025, full phase-in January 2028):**
- Output floor: RWA calculated using internal models must be ≥ 72.5% of standardized approach RWA
- Revised credit risk standardized approach (SA-CR)
- Fundamental Review of the Trading Book (FRTB) — market risk capital
- SA-CCR for counterparty credit risk (replaced CEM)
- Operational risk standardized approach (BA-CVA)

**Official source:** [bis.org/bcbs/publ/d424.htm](https://www.bis.org/bcbs/publ/d424.htm) (December 2017 — Basel IV final rule)

### 8.2 AT1 CoCo Bond Analysis

**Contingent Convertible (CoCo) bonds:** AT1 capital instruments that absorb losses at Point of Non-Viability (PONV) or at a specified CET1 trigger:

| Feature | Description |
|---|---|
| **Trigger types** | Mechanical (CET1 < 5.125% or 7%); Regulatory discretion (PONV) |
| **Loss absorption** | Write-down to zero; or conversion to equity |
| **Distribution** | Coupon is fully discretionary; regulator can restrict |
| **Maturity** | Perpetual; callable only with regulatory approval |
| **Tax treatment** | Coupon may be tax-deductible (varies by jurisdiction) |

**Credit Suisse AT1 write-down (March 2023):** Swiss FINMA wrote down ~CHF 16bn AT1 CoCos to zero while equity received consideration in UBS acquisition — departed from conventional creditor hierarchy. Triggered regulatory review across jurisdictions.

### 8.3 MREL / TLAC Requirements

**TLAC (Total Loss-Absorbing Capacity):** FSB standard for G-SIBs — minimum 18% of RWA or 6.75% of Leverage Ratio Exposure from January 2022.

**MREL (Minimum Requirement for Own Funds and Eligible Liabilities):** EU Bank Recovery and Resolution Directive (BRRD) requirement for EU banks — entity-specific; typically 25–32% of RWA for large EU banks.

**Eligible MREL/TLAC instruments:**
- CET1, AT1, Tier 2
- Senior non-preferred (SNP) debt (structural / contractual subordination)
- Senior preferred excluded for MREL/TLAC bail-in eligibility at most European banks

---

## Part 9 — ESG-Linked Financing

### 9.1 Green Bonds — ICMA Green Bond Principles (GBP)

**ICMA Green Bond Principles (2021, updated 2022):** Voluntary process guidelines with four core components:

1. **Use of Proceeds:** Defined green project categories (renewable energy, energy efficiency, clean transportation, sustainable water, green buildings, climate adaptation, biodiversity, circular economy)
2. **Process for Project Evaluation and Selection:** Issuer's environmental sustainability objectives; eligibility criteria; classification process
3. **Management of Proceeds:** Ring-fenced or tracked in sub-portfolio; formal internal process
4. **Reporting:** Annual use-of-proceeds report; impact reporting (quantitative where feasible: MWh generated, CO₂e avoided, hectares preserved)

**Second-party opinion (SPO):** Market standard — external verifier confirms framework alignment with GBP. Leading SPO providers: Sustainalytics, ISS ESG, CICERO, DNV, Bureau Veritas.

**Post-issuance verification:** Annual report; assurance from external auditor or verifier recommended.

**EU Green Bond Standard (EU GBS):** Regulation (EU) 2023/2631 — mandatory alignment with EU Taxonomy for Use of Proceeds; enhanced disclosure; mandatory external review. Effective December 21, 2024.

### 9.2 Sustainability-Linked Bonds (SLBs) — ICMA SLB Principles

**ICMA Sustainability-Linked Bond Principles (2020, updated 2023):** Performance-based bond where financial/structural characteristics vary based on KPI achievement.

**Five core components:**
1. **Selection of KPIs:** Material, core, measurable, externally verifiable, and benchmarkable
2. **Calibration of SPTs (Sustainability Performance Targets):** Ambitious relative to issuer baseline; science-based preferred; pre-issuance baseline documentation required
3. **Bond Characteristics:** Coupon step-up (typically +12.5 to 25 bps) or premium redemption if SPT not met
4. **Reporting:** Annual KPI performance report; comparison to SPT
5. **Verification:** External verification of SPT performance at each observation date

**Key risk:** "Sustainability washing" — overly easy SPTs that do not represent meaningful ambition. Investors and regulators scrutinizing SPT credibility.

### 9.3 Sustainability-Linked Loans (SLLs) — LMA/APLMA/LSTA Principles

**LMA/APLMA/LSTA Sustainability-Linked Loan Principles (2021, updated 2023):**

| Component | Requirements |
|---|---|
| **KPIs** | Core to borrower's business; measurable; independently verifiable; comparable |
| **SPTs** | Ambitious; consistent with borrower's sustainability strategy; pre-agreed |
| **Margin adjustment** | Typically ±2.5–10 bps per KPI; can step up or down based on performance |
| **Reporting** | Annual; borrower confirms KPI performance; made available to facility participants |
| **Review** | External review / verification at least annually |

**Common KPIs:** GHG emissions intensity (Scope 1+2, sometimes Scope 3); renewable energy %; waste recycling rate; water intensity; workplace safety rate; diversity metrics; supplier ESG ratings.

**Greenwashing risk:** LMA principles are voluntary. Borrowers must ensure SPT credibility — regulators (FCA, SEC) increasing scrutiny on sustainability-linked instruments.

### 9.4 Social Bonds and Sustainability Bonds

**Social Bond Principles (ICMA, 2021):** Same four-pillar structure as GBP; proceeds directed to social projects (affordable housing, food security, healthcare, employment generation, financial inclusion).

**Sustainability Bond Guidelines (ICMA, 2021):** Combined green + social use of proceeds.

**Transition Bonds:** Finance transition of high-emitting sectors toward lower-carbon pathways; not yet a formal ICMA standard but referenced in Climate Transition Finance Handbook (2020).

---

## Part 10 — Official Documentation URLs

| Standard / Regulation | URL | Access |
|---|---|---|
| **Basel III — Global Regulatory Framework** | [bis.org/publ/bcbs189.pdf](https://www.bis.org/publ/bcbs189.pdf) | Fully public |
| **Basel IV — December 2017 Final Rule** | [bis.org/bcbs/publ/d424.htm](https://www.bis.org/bcbs/publ/d424.htm) | Fully public |
| **ICMA Green Bond Principles (2021)** | [icmagroup.org/sustainable-finance/the-principles-guidelines-and-handbooks/green-bond-principles-gbp/](https://www.icmagroup.org/sustainable-finance/the-principles-guidelines-and-handbooks/green-bond-principles-gbp/) | Fully public |
| **ICMA SLB Principles (2020)** | [icmagroup.org/sustainable-finance/the-principles-guidelines-and-handbooks/sustainability-linked-bond-principles-slbp/](https://www.icmagroup.org/sustainable-finance/the-principles-guidelines-and-handbooks/sustainability-linked-bond-principles-slbp/) | Fully public |
| **LMA SLL Principles (2021)** | [lma.eu.com/application/files/8416/1481/2920/SLL_Principles_Feb_2021_V05.pdf](https://www.lma.eu.com/application/files/8416/1481/2920/SLL_Principles_Feb_2021_V05.pdf) | Fully public |
| **EU Green Bond Standard (2023/2631)** | [eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32023R2631](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32023R2631) | Fully public |
| **IFRS 9 — Financial Instruments** | [ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/](https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/) | Free with registration |
| **IFRS 16 — Leases** | [ifrs.org/issued-standards/list-of-standards/ifrs-16-leases/](https://www.ifrs.org/issued-standards/list-of-standards/ifrs-16-leases/) | Free with registration |
| **ASC 842 — Leases** | [asc.fasb.org](https://asc.fasb.org/) → search "842" | Free with registration |
| **S&P Corporate Rating Methodology** | [spglobal.com/ratings/en/research/articles/191113-corporate-methodology-9741010](https://www.spglobal.com/ratings/en/research/articles/191113-corporate-methodology-9741010) | Free with registration |
| **Moody's Rating Methodology** | [moodys.com/researchandratings/methodology](https://www.moodys.com/researchandratings/methodology) | Free with registration |
| **Fitch Corporate Rating Criteria** | [fitchratings.com/research/corporate-finance/corporate-rating-criteria-04-02-2025](https://www.fitchratings.com/research/corporate-finance/corporate-rating-criteria-04-02-2025) | Free |
| **BRRD (Bank Recovery and Resolution)** | [eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32014L0059](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32014L0059) | Fully public |
| **FSB TLAC Term Sheet** | [fsb.org/wp-content/uploads/TLAC-Principles-and-Term-Sheet-for-publication-final.pdf](https://www.fsb.org/wp-content/uploads/TLAC-Principles-and-Term-Sheet-for-publication-final.pdf) | Fully public |
| **ICMA Climate Transition Finance Handbook** | [icmagroup.org/sustainable-finance/the-principles-guidelines-and-handbooks/climate-transition-finance-handbook/](https://www.icmagroup.org/sustainable-finance/the-principles-guidelines-and-handbooks/climate-transition-finance-handbook/) | Fully public |

---

## Mandatory Advisory Note

This analysis is advisory and educational, based solely on the facts and scenario described. Debt and capital structure analysis involves complex legal, financial, and regulatory considerations that depend on specific facts and circumstances. This skill does not constitute investment advice, a fairness opinion, a solvency opinion, legal advice, or credit analysis for execution purposes. Conclusions labeled `advisory` should be verified with qualified investment bankers, legal counsel, and credit professionals before any financing decision is made. Rating agency methodologies and credit market conditions change frequently. Do not use this analysis as the basis for actual financing transactions, credit decisions, or securities trading.
