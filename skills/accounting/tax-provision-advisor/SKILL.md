---
name: tax-provision-advisor
description: Multi-jurisdiction corporate income tax provision reference framework covering ASC 740 (US GAAP) and IAS 12 (IFRS). Covers current vs. deferred tax, temporary and permanent differences, deferred tax asset/liability recognition and measurement, valuation allowance (more-likely-than-not), uncertain tax positions (FIN 48 / ASC 740-10 two-step vs. IFRIC 23), OECD Pillar Two GloBE (IAS 12.4A mandatory temporary exception vs. ASC 740 no equivalent exception), enacted vs. substantively enacted tax rates, effective tax rate reconciliation, APB 23 / ASC 740-30 indefinite reinvestment assertion, intraperiod tax allocation, interim provision (estimated annual ETR method), and local GAAP variations (HGB, JGAAP/ASBJ, CAS 18, Ind AS 12). Advisory only — all outputs require verification by qualified tax counsel and external auditors.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-01"
  category: finance
  lifecycle: experimental
---

# Tax Provision Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for corporate income tax provision advisory — from the recognition and measurement of current and deferred taxes through valuation allowances, uncertain tax positions, Pillar Two interactions, and ETR reconciliation.

---

## Part 1: ASC 740 vs. IAS 12 — Core Framework Comparison

### Scope and Recognition

| Area | US GAAP (ASC 740) | IFRS (IAS 12) |
|---|---|---|
| Standard scope | All income taxes imposed by domestic or foreign jurisdictions | All income taxes (domestic and foreign), including withholding taxes on distributions to the reporting entity |
| Asset recognition threshold | All deferred tax assets recognized; reduce by **valuation allowance** if more likely than not (>50%) that some or all will not be realized (ASC 740-10-30-5) | Recognize deferred tax asset only to the extent it is **probable** (generally interpreted as >50%; aligned in practice) that sufficient future taxable profit will be available (IAS 12.24) |
| Deferred tax liability recognition | Recognize for all taxable temporary differences except the initial recognition exception (IAS 12 equivalent) and inside basis difference in investments under APB 23 / ASC 740-30 | Recognize for all taxable temporary differences except: (a) initial recognition of goodwill; (b) initial recognition exception (assets/liabilities where neither taxable profit nor accounting profit affected) |
| Rate used | **Enacted** rate at balance sheet date (ASC 740-10-25-47) — rate signed into law | **Substantively enacted** rate (IAS 12.47) — rate virtually certain to be enacted (e.g., passed by parliament awaiting royal assent) |
| Measurement | Tax basis × enacted rate; no discounting permitted | Tax basis × substantively enacted rate; no discounting permitted |

**Key divergence — Rate**: Under US GAAP, a tax rate change affects deferred taxes only when the legislation is *signed into law*. Under IFRS, the rate change affects deferred taxes once *substantively enacted* — which can be earlier (e.g., a budget announcement in the UK Parliament). This frequently results in timing differences in the provision between GAAP and IFRS preparers facing the same legislative cycle.

### Temporary vs. Permanent Differences

**Temporary difference** — difference between the tax basis of an asset or liability and its carrying amount that will reverse in future periods, giving rise to future taxable or deductible amounts.

**Permanent difference** — difference between book income and taxable income that will never reverse (e.g., non-deductible penalties, tax-exempt municipal bond interest).

| Difference Type | Deferred Tax Created? | US GAAP Citation | IFRS Citation |
|---|---|---|---|
| Taxable temporary difference (will increase taxable income when it reverses) | **Yes — deferred tax liability** | ASC 740-10-25-20 | IAS 12.15 |
| Deductible temporary difference (will decrease taxable income when it reverses) | **Yes — deferred tax asset** (subject to VA under ASC 740 / probable threshold under IAS 12) | ASC 740-10-25-5 | IAS 12.24 |
| Permanent difference | **No** | ASC 740-10-20 (definition) | IAS 12.22 (temporary difference definition excludes permanents) |

Common temporary differences:

| Item | Tax Treatment | Book Treatment | Creates |
|---|---|---|---|
| Accelerated depreciation / MACRS | Larger deduction in early years | Straight-line | **Taxable temp diff → DTL** |
| Net operating loss (NOL) carryforward | Future deduction | No book impact | **Deductible temp diff → DTA** |
| Warranty reserves | Deductible only when paid | Accrued when incurred | **Deductible temp diff → DTA** |
| Deferred revenue | Taxed when received | Recognized over time | **Deductible temp diff → DTA** |
| Goodwill amortization (US tax — §197) | 15-year straight-line deduction | Not amortized (GAAP) / amortized (IFRS 3 private) | **Taxable temp diff → DTL** |
| Unrealized gains on trading securities | Mark-to-market (some jurisdictions) or realized basis | Fair value through P&L | Varies by jurisdiction |

---

## Part 2: Deferred Tax Assets and Liabilities

### Recognition and Measurement

**Step 1 — Identify temporary differences**: Compare tax basis to book carrying amount for each asset and liability.

**Step 2 — Calculate gross deferred tax**: Multiply temporary difference by the applicable enacted (US GAAP) or substantively enacted (IFRS) statutory tax rate.

**Step 3 — Assess valuation allowance (US GAAP) / probable realization (IFRS)**: See Part 3.

**Step 4 — Classify**: Under ASC 740-10-45-4 (post-ASU 2015-17), all deferred taxes are classified as **non-current**. Under IAS 12.70, deferred tax assets and liabilities are always non-current.

**Step 5 — Net**: Net deferred tax assets and liabilities by jurisdiction (same tax authority), consistent with the right of offset (ASC 740-10-45-6; IAS 12.74).

### Initial Recognition Exception

Both US GAAP and IFRS have an "initial recognition exception" preventing recognition of a deferred tax liability (or asset) for temporary differences arising at initial recognition of an asset or liability in a transaction that:
- Is not a business combination; and
- At the time of the transaction, affects neither accounting profit nor taxable profit.

US GAAP: ASC 740-10-25-51.
IFRS: IAS 12.15(b) (DTL) and IAS 12.24(c) (DTA).

### Investment in Subsidiaries — Inside Basis Differences

**US GAAP**: ASC 740-30 (formerly APB 23) — a DTL for outside basis differences in subsidiaries is not recognized if the investor asserts it is able to control the timing of the reversal and it is probable the difference will not reverse in the foreseeable future (**indefinite reinvestment assertion**).

**IFRS**: IAS 12.39 — a DTL for taxable temporary differences related to investments in subsidiaries, branches, associates, and JVs is not recognized if the parent is able to control the timing and it is probable the temporary difference will not reverse in the foreseeable future. IAS 12.44 contains the equivalent for deductible temporary differences.

**Key practical note**: If the indefinite reinvestment assertion is reversed (e.g., because a subsidiary will be sold), the previously unrecognized DTL must be recognized immediately. Under US GAAP, entities must also now consider the GILTI and FDII regimes when asserting APB 23.

---

## Part 3: Valuation Allowance (US GAAP) and Probable Realization (IFRS)

### US GAAP — Valuation Allowance (ASC 740-10-30-5)

A valuation allowance is required when it is **more likely than not** (probability >50%) that some or all of a deferred tax asset will not be realized.

**Positive evidence** (suggests realization is likely — ASC 740-10-30-17 through 30-24):
- Existing contracts or firm sales backlogs that will produce taxable income
- Excess of appreciated asset value over tax basis (indicating taxable gains available)
- Taxable income in prior carryback years
- History of ordinary income

**Negative evidence** (suggests valuation allowance needed):
- Cumulative losses in recent years (a significant piece of objective negative evidence per ASC 740-10-30-21)
- Losses expected in early carryforward years
- Unsettled circumstances that, if unfavorably resolved, would adversely affect operations in the future
- Brief carryforward or carryback period that limits use

**Four sources of taxable income** (ASC 740-10-30-18):
1. Taxable income in prior carryback years
2. Future reversals of existing taxable temporary differences
3. Tax planning strategies
4. Future taxable income exclusive of reversing temporary differences and carryforwards

All four sources must be evaluated. Scheduling of deferred tax reversals is required when reversals of taxable temporary differences are relied upon.

### IFRS — Probable Realization (IAS 12.28–12.31)

Recognize a deferred tax asset to the extent that it is probable sufficient future taxable profit will be available.

When an entity has a history of recent losses, IAS 12.35 requires convincing evidence that sufficient future taxable profit will be generated.

**Sources of taxable income (IAS 12.29)**:
- Future reversals of taxable temporary differences
- Tax planning opportunities
- Probable future taxable profits

---

## Part 4: Uncertain Tax Positions

### US GAAP — ASC 740-10 / FIN 48 Two-Step Model

**Step 1 — Recognition**: Recognize a tax benefit only if it is **more likely than not** (>50% probability) that the tax position will be sustained on examination by the taxing authority, based solely on technical merits. (ASC 740-10-25-6)

**Step 2 — Measurement**: Measure the tax benefit at the largest amount that is **more than 50% likely of being realized** upon ultimate settlement. (ASC 740-10-25-7) — this is the "cumulative probability" approach, not the single most-likely amount.

**Disclosure requirements**:
- Unrecognized tax benefit (UTB) roll-forward reconciliation (ASC 740-10-50-15A)
- Amount of UTBs that, if recognized, would affect the effective tax rate
- Interest and penalties policy
- Reasonably possible changes in UTBs within 12 months (including potential settlement ranges)

### IFRS — IFRIC 23

**Recognition threshold**: IFRIC 23.13 — recognize when it is probable (>50%) that the tax authority will accept the tax treatment used or planned.

**Measurement** (IFRIC 23.15): Use either:
- **Most likely amount** — if outcome is binary or has a narrow range; or
- **Expected value** — if outcome has a range of possible amounts.

Use whichever better predicts the resolution. Unlike ASC 740-10, IFRIC 23 does not prescribe the cumulative probability method.

**Key divergence**: ASC 740-10 uses the *largest amount with >50% cumulative probability* (effectively, the most conservative recognized amount from the available range). IFRIC 23 uses most likely or expected value depending on which better predicts the outcome. In practice, IFRIC 23 often results in recognition of a larger benefit than ASC 740-10 for the same fact pattern.

| Area | ASC 740-10 (US GAAP) | IFRIC 23 (IFRS) |
|---|---|---|
| Recognition threshold | More likely than not (>50%) based on technical merits alone | Probable that tax authority will accept the treatment |
| Measurement method | Largest amount with >50% cumulative probability | Most likely amount or expected value |
| Assumption about detection | Assumes tax authority examines with full knowledge | Must assess whether detection is probable first (IFRIC 23.9) |
| Interest/penalties | Policy choice: in income tax expense or separate financial statement line | Policy choice; follow IAS 12 or IAS 37 |

---

## Part 5: OECD Pillar Two GloBE — Income Tax Provision Interaction

### Overview of Pillar Two

The OECD/G20 Inclusive Framework Pillar Two (Global Anti-Base Erosion — GloBE) rules impose a **global minimum tax of 15%** on large multinational groups (consolidated revenue ≥ EUR 750 million).

Key mechanisms:
- **Qualified Domestic Minimum Top-up Tax (QDMTT)**: top-up tax in the low-tax jurisdiction itself
- **Income Inclusion Rule (IIR)**: top-up tax at parent entity level
- **Undertaxed Profits Rule (UTPR)**: backstop mechanism if IIR not in place

Source: OECD GloBE Model Rules — https://www.oecd.org/tax/beps/global-anti-base-erosion-model-rules-pillar-two.htm

### IAS 12.4A — Mandatory Temporary Exception (IFRS)

The IASB introduced a **mandatory temporary exception** in IAS 12 (effective immediately upon publication, May 2023) under which an entity:
- **Does not recognize** deferred tax assets or liabilities arising from Pillar Two legislation; and
- **Does not disclose** information required by IAS 12.81(c) about such deferred taxes.

The entity **does recognize** current tax liabilities (or assets) related to Pillar Two taxes as they arise — it is only deferred taxes on Pillar Two that are exempted (IAS 12.4B).

**Disclosure**: Entities must disclose the use of the exception and, from periods when Pillar Two legislation is enacted or substantively enacted in a jurisdiction where the entity operates, qualitative and quantitative disclosures about its Pillar Two exposure (IAS 12.88A–12.88B).

Source: IAS 12 (amended May 2023) — https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias12.html

### ASC 740 — No Equivalent Exception (US GAAP)

ASC 740 contains **no equivalent mandatory temporary exception**. Under US GAAP:
- Pillar Two top-up taxes are treated as income taxes within the scope of ASC 740
- Deferred tax effects of Pillar Two legislation must be recognized in the period the legislation is enacted
- US entities must evaluate the impact of enacted foreign Pillar Two legislation on their existing deferred tax balances

**Practical impact**: A US GAAP preparer with subsidiaries in jurisdictions that have enacted a QDMTT must evaluate whether that QDMTT is an income tax within the scope of ASC 740 and compute any resulting deferred tax effects. An IFRS preparer with identical subsidiaries would use the IAS 12.4A exception and record only current tax.

### Pillar Two — Effective Tax Rate Impact

Pillar Two top-up taxes will affect the ETR reconciliation:
- IFRS preparers: current tax line only (no deferred); must disclose separately (IAS 12.88A)
- US GAAP preparers: current and deferred tax; included within normal ASC 740 framework

---

## Part 6: Enacted vs. Substantively Enacted Tax Rates

### US GAAP — Enacted Rate (ASC 740-10-25-47)

"The enacted tax law is the basis for computing deferred tax." Under US GAAP, only *enacted* law (signed by the relevant executive authority) is the basis — proposed or announced changes have no effect until signed.

**Example**: A US federal rate change passes both chambers of Congress on December 28 but is not signed by the President until January 3. Under ASC 740, the December 31 deferred tax balance uses the *old* rate. The change is recorded in the period that includes January 3.

### IFRS — Substantively Enacted Rate (IAS 12.47)

Deferred taxes are measured using tax rates that "have been enacted or substantively enacted by the end of the reporting period." Substantively enacted means the rate change is essentially a formality — e.g., the bill has passed its final substantive legislative hurdle and awaits only royal assent or a ceremonial signing.

**Example (UK)**: A UK corporation tax rate change passes the third reading in the House of Commons and receives royal assent one week later. Under IAS 12, the rate is substantively enacted once it passes the final legislative stage, which may be before royal assent — depending on UK parliamentary practice. In practice, UK entities often treat royal assent as the substantively enacted date.

### Rate-at-Close Checklist

- [ ] Identify all jurisdictions where deferred taxes exist
- [ ] Verify the enacted (US GAAP) or substantively enacted (IFRS) rate for each jurisdiction as of the balance sheet date
- [ ] Document the specific legislative action (signing date or substantive enactment date)
- [ ] Assess whether any rate changes announced after the balance sheet date qualify as non-adjusting events requiring disclosure (IAS 10.21 / ASC 855-10-50)
- [ ] Recompute deferred taxes at the correct rate and record the income statement effect of any rate change

---

## Part 7: Effective Tax Rate Reconciliation

### Required Reconciliation

**US GAAP**: ASC 740-10-50-12 — public companies must disclose a reconciliation of the reported amount of income tax expense to the amount computed by multiplying pretax income by the applicable statutory rate.

**IFRS**: IAS 12.81(c) — entities must disclose a numerical reconciliation between (i) tax expense and (ii) the product of accounting profit and the applicable tax rate, or between (iii) average effective tax rate and the applicable tax rate.

### Common ETR Reconciliation Line Items

| Line Item | Direction | Explanation |
|---|---|---|
| Income at statutory rate | Base | Pretax income × domestic statutory rate |
| State and local taxes | Usually positive | Net of federal benefit |
| Foreign rate differential | Positive or negative | Tax at foreign statutory rate vs. domestic rate |
| Non-deductible expenses | Positive | Penalties, entertainment, certain M&A costs |
| Tax-exempt income | Negative | Municipal bond interest, qualifying dividends |
| Research and development credits | Negative | §41 R&D credit (US); similar credits elsewhere |
| Return-to-provision adjustments | Positive or negative | Differences between prior-year provision and filed return |
| Valuation allowance movement | Positive or negative | Change in valuation allowance on DTAs |
| Uncertain tax positions | Positive or negative | Net change in UTB reserve |
| Pillar Two top-up taxes | Positive (increase) | Additional current tax for QDMTT / IIR |
| Stock-based compensation | Positive or negative | Excess tax benefit / shortfall on option exercises (ASC 718) |
| GILTI (Global Intangible Low-Tax Income) | Positive | Inclusion under §951A (US MNEs) |
| FDII (Foreign-Derived Intangible Income) | Negative | Deduction under §250 |

### Interim Provision — Estimated Annual ETR Method

**US GAAP (ASC 740-270)**: The estimated annual ETR is applied to ordinary income/loss for the year-to-date period. Discrete items (items that cannot be estimated on an annual basis, such as return-to-provision adjustments) are recorded in the quarter in which they occur.

**IFRS (IAS 34.30(c))**: Apply the estimated annual ETR to pre-tax income at the interim date. Discrete items are recorded in the period in which they arise.

**Common errors in interim provision**:
- Using the jurisdiction-level statutory rate as the estimated ETR rather than computing the entity-level estimated annual ETR
- Failing to exclude jurisdictions with loss positions from the consolidated ETR when they provide no benefit
- Including Pillar Two as a discrete item when it is part of the ordinary annual rate

---

## Part 8: Intraperiod Tax Allocation

**US GAAP (ASC 740-20)**: Income tax expense is allocated among:
- Continuing operations
- Discontinued operations
- Other comprehensive income (OCI)
- Additional paid-in capital (APIC — limited items, e.g., certain employee benefit plans and stock compensation)

The general principle is that items giving rise to tax effects are measured and presented in the same financial statement location as the pretax item. The "backwards tracing" requirement (ASC 740-20-45-7) means that the tax effect of OCI items is recorded in OCI, not in the income tax line in the income statement.

**IFRS (IAS 12.52–12.65)**: Same principle — tax relating to OCI items is recognized in OCI; tax relating to equity transactions is recognized in equity.

---

## Part 9: Local GAAP Variations

### German HGB — Latente Steuern (§ 274 HGB)

- HGB adopted a deferred tax model (effective 2010 Bilanzrechtsmodernisierungsgesetz / BilMoG).
- Under § 274 HGB, deferred taxes arise from differences between the *HGB commercial balance sheet* and the *tax balance sheet* (Steuerbilanz).
- **Option to recognize DTA**: Under HGB, recognition of net deferred tax assets (where the aggregate is an asset position) is **optional** for certain entities. Large corporations typically recognize; smaller entities may elect not to.
- **Rate used**: Current statutory tax rate (combined corporate income tax + solidarity surcharge + trade tax, typically ~30% for German corporates).
- **No valuation allowance concept**: Instead, deferred tax assets are recognized only if they are expected to be utilized.
- Source: HGB § 274 — https://www.gesetze-im-internet.de/hgb/__274.html

### JGAAP — ASBJ Statement No. 28 (Revised 2018)

- Japanese GAAP follows an asset-liability method substantially aligned with IAS 12 in concept.
- ASBJ Statement No. 28 (Accounting Standard for Tax Effect Accounting) governs.
- Deferred taxes are measured at the enacted tax rate.
- Valuation allowances required (called "evaluation adjustment") where realization is not expected.
- **Pillar Two**: Japan enacted QDMTT effective January 2024. ASBJ issued guidance aligning closely with IAS 12.4A temporary exception.
- Source: ASBJ — https://www.asb.or.jp/en/accounting_standards/accounting_standards/

### CAS 18 — China Accounting Standards for Business Enterprises No. 18

- CAS 18 is substantially converged with IAS 12 but with PRC-specific differences.
- **Enacted rate**: CAS 18 uses the applicable tax rate "that is expected to apply to the period when the asset is realized or the liability is settled" — aligned with enacted rate requirement.
- PRC enterprise income tax rate is generally 25%; high-tech enterprises qualify for a 15% preferential rate.
- Deferred tax on temporary differences related to investments in subsidiaries: same exemption as IAS 12.39 when repatriation is controlled.
- Source: MOF China — https://www.mof.gov.cn/

### Ind AS 12 — India (Converged with IAS 12)

- Ind AS 12 is virtually identical to IAS 12 with minor differences.
- **Rate**: Substantively enacted rate consistent with IAS 12.47.
- India corporate tax rate: Companies Ordinance 2019 reduced the base rate to 22% (plus surcharge and cess, effective ~25.17% for domestic companies); new manufacturing companies: 15% base.
- **Schedule III disclosures**: Ind AS preparers must present detailed deferred tax disclosures in Schedule III (Balance Sheet note) with movement table.
- Source: ICAI Ind AS standards — https://www.icai.org/post/indian-accounting-standards

---

## Part 10: Official Documentation — Publicly Accessible URLs

| Standard | Resource | URL | Access |
|---|---|---|---|
| ASC 740 (Income Taxes) | FASB ASC | https://asc.fasb.org/740 | **Publicly accessible** |
| IAS 12 (Income Taxes) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias12.html | **Fully public** |
| IFRIC 23 (UTP under IAS 12) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifric23.html | **Fully public** |
| OECD Pillar Two GloBE Model Rules | OECD | https://www.oecd.org/tax/beps/global-anti-base-erosion-model-rules-pillar-two.htm | **Fully public** |
| OECD Pillar Two — main page | OECD | https://www.oecd.org/en/topics/pillar-two.html | **Fully public** |
| HGB § 274 (Latente Steuern) | German Federal Law (gesetze-im-internet) | https://www.gesetze-im-internet.de/hgb/__274.html | **Fully public (German)** |
| ASBJ Statement No. 28 (JGAAP) | ASBJ English translations | https://www.asb.or.jp/en/accounting_standards/accounting_standards/ | **Fully public** |
| Ind AS 12 | ICAI | https://www.icai.org/post/indian-accounting-standards | **Fully public** |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described above. Tax provision requirements vary by jurisdiction, entity type, and filing status and are subject to legislative change. This analysis does not constitute authoritative accounting guidance, a tax opinion, or legal advice in any jurisdiction. Verify all tax provision positions, uncertain tax positions, and Pillar Two exposures with qualified tax counsel and external auditors before relying on this analysis for financial reporting purposes.
