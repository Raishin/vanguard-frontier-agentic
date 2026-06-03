---
name: variance-analysis-advisor
description: Variance decomposition framework and SEC Regulation S-K Item 303 MD&A commentary guidance for FP&A and corporate finance teams. Provides driver decomposition methodology (Volume/Price/Rate/Mix/One-Time), MD&A structural requirements with regulatory citations, restatement-risk trigger catalog, sensitivity analysis templates, and materiality threshold guidance. Advisory only — all draft commentary requires CFO certification and legal review before filing.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-01"
  category: finance
  lifecycle: experimental
---

# Variance Analysis Advisor — Reference Skill

## Purpose

Provide the complete analytical framework for variance decomposition and MD&A commentary drafting, consistent with SEC Regulation S-K Item 303 requirements and FASB ASC 270 (Interim Reporting) expectations.

## Official Documentation

| Standard | Source | Access |
|---|---|---|
| SEC Regulation S-K Item 303 (MD&A) — eCFR | https://www.ecfr.gov/current/title-17/chapter-II/part-229/subpart-229.300/section-229.303 | **Fully public** (eCFR) |
| SEC Regulation S-K Item 303 — Cornell LII | https://www.law.cornell.edu/cfr/text/17/229.303 | **Fully public** |
| SEC Final Rule 33-10890 (2020 MD&A Amendments) | https://www.sec.gov/files/rules/final/2020/33-10890.pdf | **Fully public PDF** |
| SEC 2020 Interpretive Guidance on MD&A | https://www.sec.gov/files/rules/interp/2020/33-10751.pdf | **Fully public PDF** |
| SEC 2003 MD&A Guidance | https://www.sec.gov/rules-regulations/2003/12/commission-guidance-regarding-managements-discussion-analysis-financial-condition-results-operations | **Fully public** |
| FASB ASC 270 (Interim Reporting) | https://asc.fasb.org/270 | Free account required |
| FASB ASC 280 (Segment Reporting) | https://asc.fasb.org/280 | Free account required |
| FASB ASC 205-20 (Discontinued Operations) | https://asc.fasb.org/205-20 | Free account required |
| SEC Non-GAAP Financial Measures Guidance | https://www.sec.gov/divisions/corpfin/guidance/nongaapinterp.htm | Public |

---

## SEC Regulation S-K Item 303 — MD&A Requirements

### Annual (10-K) Results of Operations Requirements (S-K 303(b)(2))

**Required disclosures:**
1. Material changes in net sales/revenues between periods — explain underlying causes, not just state the number
2. Material changes in cost of revenues — distinguish volume, price, and mix effects
3. Material changes in gross margin — quantify and explain
4. Material changes in each significant operating expense line
5. Material changes in income from operations
6. Known trends, demands, commitments, events, or uncertainties expected to materially affect results (S-K 303(b)(1)) — this is forward-looking and requires careful legal review

**Key SEC guidance (Release 33-8350):**
- "Material" = would a reasonable investor consider the information important in making an investment decision (qualitative + quantitative)
- Quantitative threshold generally used in practice: ≥5% change in a line item, or absolute dollar threshold based on company size
- Comparative period requirement: typically year-over-year (most recent two fiscal years)
- Causes must be described with specificity — "other income increased primarily due to gain on sale of building" not "other income increased"

### Interim (10-Q) Requirements (S-K 303(b)(1), ASC 270)

**Required disclosures:**
1. Material changes vs. the corresponding period of the prior year (year-to-date and quarterly)
2. Material changes vs. the most recent annual period (when seasonal or where there are significant changes)
3. ASC 270-10-45-14: Disclose information about unusual or infrequently occurring items
4. ASC 270-10-45-4: Each interim period stands on its own — do not defer recognition to a later "expected" quarter

**Key SEC guidance:** Interim MD&A may be less detailed than annual but must cover material changes. The SEC staff frequently comments on interim MD&A that simply repeats boilerplate from prior periods without explaining period-specific drivers.

---

## Driver Decomposition Framework

### Standard Four-Factor Decomposition

For every material revenue or cost-of-revenue variance, decompose into:

| Factor | Definition | How to Compute |
|---|---|---|
| **Volume effect** | Change attributable to selling more/fewer units | (Current units − Prior units) × Prior price |
| **Price/Rate effect** | Change attributable to price changes | Current units × (Current price − Prior price) |
| **Mix effect** | Change attributable to shift in product/segment composition | Total change − Volume effect − Price effect − FX effect |
| **One-time items** | Non-recurring items: restructuring, asset sales, legal settlements | Identify and quantify each; tag as one-time |

**Total check:** Volume + Price + Mix + One-time = Total $ variance ± rounding

### Extended Decomposition for Operating Expenses

| Factor | Definition |
|---|---|
| **Headcount-driven** | Change in FTE count × average cost per FTE |
| **Rate-driven** | FTE count × change in average cost per FTE (compensation, benefits) |
| **Program-driven** | Discretionary spend (marketing programs, R&D projects, capex timing) |
| **One-time** | Non-recurring charges: impairments, restructuring, severance |

### FX Translation Effect (Multinationals)

Disclose separately when the company reports in a currency other than functional currencies of subsidiaries:
- Constant currency revenue = prior-period revenue × (current-period FX rate / prior-period FX rate)
- FX effect = actual change − constant currency change
- SEC staff routinely comments on non-GAAP constant currency metrics that are not reconciled per SEC Non-GAAP guidance

---

## MD&A Commentary Structure

### Recommended Section Structure (Results of Operations)

```
1. Overview (1-2 sentences): "Results for [period] reflected [top driver], partially offset by [counter-driver]."
2. Revenue:
   a. Total revenue: $X vs. $Y prior period (+/-$Z, +/-W%)
   b. By segment or product line (if material)
   c. Driver explanation: volume, pricing, mix, one-time
   d. Geographic breakdown (if material)
3. Cost of revenues / Gross margin:
   a. Total cost: $X vs. $Y (+/-$Z)
   b. Gross margin: X% vs. Y% (±Z bps)
   c. Driver explanation: material cost drivers
4. Operating expenses (by line):
   a. S&M / R&D / G&A: each with $ change, % change, driver explanation
5. Income from operations:
   a. GAAP operating income / loss: $ change
6. Non-GAAP reconciliation (if company presents non-GAAP): per SEC Non-GAAP guidance
7. Liquidity and Capital Resources (separate section, not covered here)
```

### Paragraph Tagging

Tag each paragraph before finalizing:

| Tag | Meaning |
|---|---|
| `required-disclosure` | Mandated by S-K 303; must be in the filing |
| `material-trend` | Material change meeting quantitative/qualitative threshold |
| `management-chosen` | Discretionary color; legal must review for forward-looking statement compliance |
| `safe-harbor` | Forward-looking statements that require safe-harbor language under PSLRA |

---

## Materiality Thresholds

**Quantitative guidance (practice standard):**
- Revenue variance ≥ 5% of prior period revenue → material; requires explanation
- Operating expense variance ≥ 5% of prior period operating expense AND > $X materiality (company-size dependent) → material
- Any line item that moves gross margin ≥ 50 basis points → material

**Qualitative override:** A <5% variance is still material if it represents a reversal of a trend, involves a new business line, or signals a future uncertainty.

**Single-driver concentration risk:** If one driver accounts for >80% of a total variance explanation, flag for auditor review — this pattern appears in many restatement cases (channel stuffing, bill-and-hold).

---

## Restatement-Risk Trigger Catalog

Flag these patterns as risk indicators requiring internal audit or external auditor review:

| Trigger | Pattern | Associated Standard |
|---|---|---|
| Revenue recognition cutoff | Spike in deferred revenue or unearned at period-end; large reversals in subsequent period | ASC 606-10-25-23 |
| Channel stuffing | Revenue increase concentrated in last two weeks of period with high return rates | ASC 606-10-25-1(e) collectability |
| Bill-and-hold | Physical goods revenue recognized without customer possession or control | ASC 606-10-55-83 through 55-84 |
| Improper capitalization | Unusual decrease in operating expense correlated with unusual increase in capex | ASC 340-40, ASC 730 |
| One-time items misclassified | Operating income improved significantly but pre-tax income did not → items below the line? | S-K 303 materiality |
| FX timing games | Revenue spike correlated with period-end FX rate movements | ASC 830 functional currency |
| Segment recast | Business unit results improve markedly in a quarter when segment definitions changed | ASC 280-10 |

---

## Mandatory Advisory Note

Every output from this agent must end with:

> **Advisory**: This draft is advisory and based solely on the data and descriptions provided. It does not constitute authoritative financial guidance, legal advice, or a professional opinion. Final disclosure language requires CFO certification, Disclosure Committee review, and approval from legal counsel and external auditors before filing with the SEC or any regulatory body. Forward-looking statements require specific safe-harbor language under the Private Securities Litigation Reform Act (PSLRA).
