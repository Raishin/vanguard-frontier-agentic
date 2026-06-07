---
name: equity-compensation-advisor
description: Multi-jurisdiction equity-based compensation reference framework covering stock options, RSUs, ESPPs, and performance awards under ASC 718 and IFRS 2.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Equity Compensation Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for equity-based compensation advisory — from award classification and fair value measurement through vesting and expense recognition, modification accounting, tax effects, and country-specific rules.

---

## Part 1: Award Classification — ASC 718 / IFRS 2

### Equity-Classified vs. Liability-Classified Awards

| Criterion | Equity-Classified | Liability-Classified |
|---|---|---|
| Settlement | Fixed number of shares | Cash or variable number of shares |
| Cash settlement feature | Absent | Present (e.g., SARs settled in cash) |
| Indexed to own equity | Yes — no cash alternative | No — or indexed to something other than own shares |
| Key standard | ASC 718-10-25-5 / IFRS 2.8 | ASC 718-10-25-6 / IFRS 2.34 |

**Liability-classified awards** must be remeasured at fair value each reporting date until settlement. The cumulative mark-to-market is recognised in P&L.

**Modification from equity to liability** (e.g., adding a cash settlement feature): Re-measure the award at modification date fair value; any excess over grant-date fair value is recognised immediately if the modification increases fair value, or is deferred if it does not.

### Employee vs. Non-Employee Awards

| Treatment | US GAAP | IFRS 2 |
|---|---|---|
| Employee (and similar service providers) | ASC 718-10 — measure at grant date fair value | IFRS 2.10 — measure at grant date fair value of equity instrument |
| Non-employee (post-ASU 2018-07) | ASC 718 applied to non-employees; grant date is the date on which performance commitment exists | IFRS 2.12 — measured at fair value of goods/services received; equity FV used only if goods/services FV cannot be estimated |
| Key note | ASU 2018-07 aligned non-employee treatment largely with employee treatment for US GAAP | IFRS 2 non-employee measurement has always been goods/services-first |

---

## Part 2: Stock Options — Fair Value Measurement

### Valuation Models

| Model | Use Case | Key Requirements |
|---|---|---|
| Black-Scholes | Plain-vanilla options with service conditions only; no path dependency | Expected term, volatility, risk-free rate, dividend yield |
| Binomial / Lattice | Where early exercise is expected to vary; can model suboptimal exercise; options with performance conditions | Lattice of stock prices across time steps; exercise boundary calibration |
| Monte Carlo | Market conditions (TSR-based, share price hurdles); path-dependent features | Simulates thousands of price paths; outputs expected payout under the condition |

Source: ASC 718-10-55-11; IFRS 2.B5–B6; SEC SAB Topic 14.D

### Valuation Model Inputs

| Input | ASC 718 Guidance | IFRS 2 Guidance |
|---|---|---|
| Expected term | SAB Topic 14.D: simplified method = (vesting period + contractual term) ÷ 2 for plain-vanilla options; otherwise based on historical exercise data | IFRS 2.B6: based on expected early exercise behaviour; consider vesting period and contractual life |
| Volatility | Historical volatility over a period equal to expected term; implied volatility from traded options if available; peer group volatility for newly public companies (SAB Topic 14.D.1) | IFRS 2.B7: same considerations |
| Risk-free rate | US Treasury zero-coupon rate for a term equal to expected term at grant date | Government bond yield for a term equal to expected term |
| Dividend yield | Expected dividends during expected term; adjust for declared but not yet paid dividends | Same |

---

## Part 3: RSUs, PSUs, and ESPPs

### RSUs and PSUs

**RSU grant date fair value**: Closing stock price on grant date (or volume-weighted average price per plan terms); no option pricing model required.

**Dividend equivalents**: If RSUs accrue dividends as additional shares, the dividend equivalent RSUs are additional awards; if paid in cash, subtract discounted cash flows from grant date FMV.

**TSR-based PSUs** (Total Shareholder Return): Market condition → Monte Carlo simulation required. Market conditions are never "improbable" for expense recognition purposes. If the service condition is met, the full target-level expense is recognised regardless of whether the TSR threshold is achieved (ASC 718-10-25-20 / IFRS 2.21).

**Performance condition reassessment**: At each reporting date, update the probability assessment for performance conditions. Apply cumulative catch-up adjustments when probability changes. Compare to a market condition which is fixed at grant date.

**Vesting tranche accounting**:
- Straight-line: Recognise total award expense ratably over the requisite service period.
- Graded/accelerated: Recognise each tranche separately over its own vesting period (accelerated method). Required under IFRS 2.15A for awards with graded vesting; US GAAP offers a policy election.

### ESPPs (Employee Stock Purchase Plans)

| Criterion | Compensatory (ASC 718) | Non-Compensatory |
|---|---|---|
| Discount from fair value | > 5% | ≤ 5% |
| Lookback feature | Any lookback > 12 months | No lookback (or ≤ beginning-of-offering-period price with ≤ 12-month offering) |
| Section 423 qualified | Not determinative for accounting | Plan must be Section 423-qualified AND meet all non-compensatory criteria |
| Key standard | ASC 718-50-25-1 | ASC 718-50-25-2 |

For compensatory ESPPs: Measure fair value at the beginning of the offering period; use Black-Scholes with expected term = offering period length.

---

## Part 4: Forfeitures

| Treatment | ASC 718 (post-ASU 2016-09) | IFRS 2 |
|---|---|---|
| Policy choice | Elect to estimate forfeitures at grant date OR recognise expense only for vested awards (actual forfeiture method) | No choice: always estimate expected forfeitures; adjust estimate at each reporting date |
| Change in estimate | Recognised prospectively as a change in estimate | Recognised prospectively |
| Key standard | ASC 718-10-30-3 (post-ASU 2016-09 policy election) | IFRS 2.19–2.21 |

**Common error**: Applying the actual forfeiture method under US GAAP without making a formal accounting policy election, or forgetting that IFRS 2 requires forfeiture estimation.

---

## Part 5: Modification Accounting

### ASC 718 (ASC 718-20-55)

A modification is any change in terms or conditions of a share-based award. On modification date:
1. Measure grant-date fair value of the original award (if it had been unmodified).
2. Measure modification-date fair value of the modified award.
3. Any **incremental fair value** (excess of modified FV over original FV at modification date) is additional compensation cost.
4. Improbable-to-probable change: if the original award was improbable of vesting and the modification makes it probable, recognise modification-date fair value as if it were a new grant.

### IFRS 2 Modification Types (IFRS 2.27–29)

| Type | Definition | Accounting |
|---|---|---|
| Type I | Modification that increases total fair value (beneficial modification) | Recognise incremental fair value over remaining vesting period |
| Type II | Modification that increases number of equity instruments | Recognise incremental fair value of additional instruments |
| Type III | Modification that does not increase fair value or number of instruments (detrimental modification) | Continue recognising original grant-date FV; ignore modification |

**Key IFRS 2 rule**: A detrimental modification (Type III) does not reduce total compensation cost below the original grant-date fair value. The company is "locked in" to the original expense.

---

## Part 6: Tax Effects

### US GAAP — Post-ASU 2016-09 (ASC 718-740)

| Item | Treatment |
|---|---|
| Deferred tax asset | Recognised on book compensation expense at the corporate tax rate |
| Excess tax benefit (windfall) | Recognised in P&L (income tax benefit) when the tax deduction exceeds cumulative book expense |
| Tax shortfall | Recognised in P&L (income tax expense) when book expense exceeds tax deduction |
| Section 162(m) | $1M deduction limit for covered employees (CEO, CFO, and the next 3 highest-paid); ISO and performance-based exemptions largely eliminated for post-2017 grants under TCJA |
| ISO vs. NSO | ISO: no ordinary income on exercise (AMT applies); capital gain on qualifying disposition; no corporate deduction on qualifying disposition. NSO: ordinary income on exercise = spread; W-2 reporting; corporate deduction equal to spread |

Source: ASC 718-740-35; IRC §162(m); IRC §422; IRC §83

### IFRS 2 Tax (IAS 12 + IFRS 2.58)

- Current tax deduction based on intrinsic value of the award at the tax measurement date (typically exercise for options; vesting for RSUs) — not grant-date fair value.
- If intrinsic value > cumulative book expense: excess recognised directly in equity.
- Deferred tax asset: recognised on book expense to the extent it is probable that a future tax deduction will be available.

### Multi-Jurisdiction Tax Rules

| Jurisdiction | Key Rules |
|---|---|
| Germany | § 19a EStG (2021 reform): Deferred taxation on employee share schemes — tax on share value deferred until earliest of sale, leaving employer, or 12 years post-grant. Applies to qualifying startups and SMEs. Ordinary employment income (§ 19 EStG) applies outside § 19a scope. |
| Japan | 税制適格 (Qualified tax-favored) stock options: No tax on exercise; capital gains tax on sale. Must meet J-SOX criteria (exercise price ≥ grant price; exercise ≤ ¥12M/year; option holder is director/employee; exercise period 2–10 years post-grant). 非適格 (Non-qualified): income tax on exercise spread as employment income. |
| China | Offshore equity for domestic employees requires SAFE (State Administration of Foreign Exchange) registration (SAFE Circular 7 / Circular 37). Tax: Individual income tax on equity income treated as wages/salaries; collected by employer at exercise. |
| India | Perquisite tax on exercise: spread between FMV on exercise date and exercise price treated as salary perquisite; TDS applies at exercise. Capital gains tax on subsequent sale (LTCG/STCG). SEBI ESOP (Employee Stock Option Plan) Regulations 2021 govern listed company plan requirements. |
| UK / EU | UK: HMRC-approved schemes (EMI, CSOP, SAYE, SIP) offer tax advantages; unapproved options subject to income tax on exercise. IFRS 2 applies for IFRS reporters. |

---

## Part 7: Official Documentation — Publicly Accessible URLs

| Standard / Resource | URL | Access |
|---|---|---|
| ASC 718 (FASB) | https://asc.fasb.org/718 | Public (registration may be required for full text) |
| IFRS 2 (IASB) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs2.html | Fully public |
| SEC SAB Topic 14 | https://www.sec.gov/interps/account/sab14.htm | Fully public |
| IRS Topic 427 — Stock Options | https://www.irs.gov/taxtopics/tc427 | Fully public |
| IRC §422 (ISOs) | https://www.law.cornell.edu/uscode/text/26/422 | Fully public |
| IRC §162(m) | https://www.law.cornell.edu/uscode/text/26/162 | Fully public |
| Germany § 19a EStG | https://www.gesetze-im-internet.de/estg/__19a.html | Fully public (German) |
| Japan FSA / FIEA (stock option rules) | https://www.fsa.go.jp/en/laws_regulations/ | Fully public |
| China SAFE Circular 7 / 37 | http://www.safe.gov.cn/ | Partially public (Chinese) |
| India SEBI ESOP Regulations 2021 | https://www.sebi.gov.in/legal/regulations/dec-2021/sebi-share-based-employee-benefits-and-sweat-equity-regulations-2021_54104.html | Fully public |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the award profile and facts described above. Equity compensation accounting involves complex interactions between accounting standards, tax law, and securities regulations that vary by jurisdiction and change frequently. This analysis does not constitute legal, tax, or securities advice. Verify all conclusions with qualified external auditors, tax advisors, and legal counsel before relying on this analysis for any compliance or transactional purpose.
