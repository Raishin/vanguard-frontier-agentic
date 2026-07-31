---
name: hedge-accounting-advisor
description: Multi-jurisdiction hedge accounting reference framework covering ASC 815 (US GAAP) and IFRS 9 hedge designation, effectiveness testing, OCI mechanics, IFRS 9 rebalancing, cost-of-hedging approach, discontinuation rules, embedded derivatives, and local GAAP treatments (German HGB §254, JGAAP ASBJ No.10, CAS 24, Ind AS 109). Includes fair value hedges, cash flow hedges, and net investment hedges with a multi-jurisdiction comparison table. Advisory only — all outputs require verification by qualified accountants and external auditors.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Hedge Accounting Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for hedge accounting advisory — from hedge type classification and instrument eligibility through effectiveness testing methodologies, OCI mechanics, rebalancing, cost-of-hedging, discontinuation rules, embedded derivatives, and local GAAP treatments.

---

## Part 1: Regulatory Framework Overview

### Primary Standards

| Standard | Jurisdiction | Hedge Accounting Scope |
|---|---|---|
| ASC 815 | US GAAP | Fair value hedges, cash flow hedges, net investment hedges; embedded derivatives (ASC 815-15) |
| IFRS 9 (Chapter 6) | IFRS jurisdictions | Fair value hedges, cash flow hedges, net investment hedges; IAS 39 macro hedge carve-out via IAS 39.81A |
| IAS 39 | Legacy IFRS (replaced by IFRS 9 for most; retained as macro hedge option) | Same three hedge types; stricter bright-line effectiveness tests (80–125%) |
| German HGB §254 | German statutory | Bewertungseinheit (valuation unit) — specific rules for hedge accounting under German commercial law |
| JGAAP ASBJ No.10 | Japan statutory | Deferral hedge accounting; allocation method; special treatment for interest rate swaps |
| CAS 24 | Chinese GAAP | Broadly converged with IFRS 9 since 2017 revision |
| Ind AS 109 | Indian GAAP | Identical to IFRS 9 Chapter 6 |

Official documentation:
- ASC 815: https://asc.fasb.org/815
- IFRS 9: https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs9.html
- IAS 39: https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias39.html
- HGB: https://www.gesetze-im-internet.de/hgb/
- ASBJ: https://www.asb.or.jp/en/accounting_standards/accounting_standards/
- Ind AS: https://www.icai.org/post/indian-accounting-standards

---

## Part 2: The Three Hedge Types

### 2.1 Fair Value Hedge

**Definition**: A hedge of the exposure to changes in the fair value of a recognized asset, recognized liability, or an unrecognized firm commitment (or a component thereof), attributable to a particular risk.

**Accounting treatment**:

| Element | ASC 815 | IFRS 9 |
|---|---|---|
| Hedging instrument | Remeasured at fair value; gain/loss in P&L | Remeasured at fair value; gain/loss in P&L |
| Hedged item | Adjusted for fair value changes attributable to hedged risk (basis adjustment) | Adjusted for fair value changes attributable to hedged risk (basis adjustment) |
| Net P&L effect | Offset of gain/loss on instrument vs. basis adjustment — only hedge ineffectiveness flows through P&L | Same; ineffectiveness only in P&L |
| Basis adjustment amortization | Amortized over remaining life when hedge is discontinued | Amortized over remaining life when hedge is discontinued |

**Key difference**: ASC 815 introduced the "last-of-layer" (now "portfolio layer") method allowing partial-term hedges of closed portfolios of prepayable financial assets (ASC 815-20-25-12A). IFRS 9 has no direct equivalent.

**Firm commitment**: A firm commitment can be a hedged item for fair value hedges under both ASC 815 (ASC 815-20-25-15) and IFRS 9 (IFRS 9.6.5.4). Under ASC 815, a firm commitment that is subsequently recognized is removed from the hedged item designation; under IFRS 9 the basis adjustment is applied when the asset/liability arising from the firm commitment is recognized.

### 2.2 Cash Flow Hedge

**Definition**: A hedge of the exposure to variability in cash flows that is attributable to a particular risk associated with a recognized asset, liability, or highly probable forecast transaction (or a component thereof).

**Accounting treatment**:

| Element | ASC 815 | IFRS 9 |
|---|---|---|
| Effective portion of gain/loss on instrument | Recognized in OCI (accumulated in AOCI) | Recognized in OCI (accumulated in hedging reserve — "cash flow hedge reserve") |
| Ineffective portion | Immediately to P&L | Immediately to P&L |
| Reclassification from OCI to P&L | When hedged transaction affects P&L (or when it results in recognition of a non-financial asset/liability — basis adjustment option) | Same; basis adjustment applies when hedged item is a non-financial asset or liability (IFRS 9.6.5.11) |
| Forecast transaction no longer expected | Reclassify all deferred AOCI to P&L immediately (ASC 815-30-40-5) | Reclassify immediately to P&L (IFRS 9.6.5.12(b)) |

**Highly probable threshold**: ASC 815 uses "probable" (same as "more likely than not" — >50% but in practice treated as high probability). IFRS 9 uses "highly probable" (consistently interpreted as substantially higher than probable, often approximately 90%+ by practitioners).

**Basis adjustment (non-financial items)**: Both standards allow (and IFRS 9 requires for non-financial items) a basis adjustment — the deferred OCI amount is included in the initial carrying amount of the acquired asset or assumed liability rather than being reclassified to P&L.

### 2.3 Net Investment Hedge

**Definition**: A hedge of the foreign currency exposure of a net investment in a foreign operation (as defined in IAS 21 / ASC 830-30).

**Accounting treatment**:

| Element | ASC 815 | IFRS 9 / IAS 21 |
|---|---|---|
| Effective portion of gain/loss | Recorded in OCI (CTA — cumulative translation adjustment) | Recorded in OCI (translation reserve) |
| Ineffective portion | P&L | P&L |
| Reclassification to P&L | Upon disposal or partial disposal of the foreign operation | Upon disposal or partial disposal of the foreign operation |
| Hedging instrument eligibility | Derivative or non-derivative monetary instrument | Derivative or non-derivative monetary instrument (IFRS 9.6.5.13) |

**Key ASC 815 rule**: The hedging instrument must be denominated in the functional currency of the foreign operation being hedged, or the domestic currency of the reporting entity. ASC 815-20-25-67 and 25-68.

**IFRS 9 rule**: The hedging instrument can be held by any entity within the consolidated group, not only the parent. This provides greater flexibility versus ASC 815. (IFRS 9.6.5.13 and related IFRS 9.BC6.355–BC6.362 basis for conclusions.)

---

## Part 3: Hedging Instrument and Hedged Item Eligibility

### 3.1 Hedging Instrument Eligibility

| Instrument Type | ASC 815 | IFRS 9 |
|---|---|---|
| Derivative measured at fair value through P&L | Eligible (general rule) | Eligible (general rule) |
| Non-derivative financial asset/liability | Eligible **only** for hedges of foreign currency risk | Eligible **only** for hedges of foreign currency risk (IFRS 9.6.2.2) |
| Written option (as hedging instrument) | Eligible **only** if net sold option position (i.e., premium received ≥ premium paid) — ASC 815-20-25-94 | Eligible only if purchased option; written option generally not eligible (IFRS 9.B6.2.4) unless it offsets a purchased option (net) |
| Proportion of a derivative | Eligible (percentage of notional) — ASC 815-20-25-78 | Eligible (IFRS 9.6.2.4) |
| Combination of instruments | Eligible — ASC 815-20-25-75 | Eligible (IFRS 9.6.2.5) |
| Intragroup derivatives | Not eligible in consolidated statements | Not eligible in consolidated statements (IFRS 9.6.2.3) |

### 3.2 Hedged Item Eligibility

| Hedged Item Type | ASC 815 | IFRS 9 |
|---|---|---|
| Recognized financial asset/liability | Eligible | Eligible |
| Firm commitment | Eligible (fair value hedge only, unless FX risk) | Eligible (IFRS 9.6.3.2) |
| Forecast transaction | Eligible (cash flow hedge) | Eligible (IFRS 9.6.3.3) |
| Net investment in foreign operation | Eligible | Eligible |
| Risk component of non-financial item | Eligible only for FX and commodity price risk components | Eligible if separately identifiable and reliably measurable (IFRS 9.6.3.7(a)) — broader than ASC 815 |
| Aggregated exposure (derivative + hedged item combined) | Not eligible | Eligible under IFRS 9.6.3.4 — IFRS 9 unique feature |
| Layer component of a portfolio | "Portfolio layer" method (ASC 815-20-25-12A) | Not directly equivalent; IFRS 9 has a "layer of a nominal amount" concept (IFRS 9.B6.3.19) |
| Net position | Generally not eligible | Eligible under specific conditions (IFRS 9.6.6) for groups of items |

---

## Part 4: Effectiveness Testing

### 4.1 ASC 815 — Effectiveness Requirements

**Quantitative threshold**: The hedge must demonstrate that actual results will be within the 80–125% range (the "offset ratio") — ASC 815-20-25-80.

**Methods permitted under ASC 815**:

| Method | Description | Applicability |
|---|---|---|
| Dollar-offset (cumulative or period-by-period) | Compare cumulative fair value changes of instrument vs. hedged item | Most hedges |
| Regression analysis | Statistical method demonstrating high correlation | Interest rate, FX, commodity |
| Hypothetical derivative method | Compare actual derivative to a "perfect" hypothetical derivative | Common for interest rate swaps |
| Long-haul method (interest rate swaps) | Full quantitative assessment each period | Interest rate swaps |
| Short-cut method (interest rate swaps) | Assume perfect effectiveness when specific criteria met (ASC 815-20-25-100 to 25-116) | Interest rate swaps on floating-rate debt meeting strict criteria |
| Critical-terms-match | Assume perfect effectiveness for FX hedges on firm commitments (ASC 815-20-25-129) | FX forward contracts matching terms of firm commitment |

**Short-cut method criteria** (ASC 815-20-25-102): The notional matches the principal of the hedged debt, the swap's floating rate matches the hedged item's index, the swap repricing dates match the debt, no floor/cap asymmetry exists, no prepayment risk on the debt, and the fair value of the swap at inception is zero.

**Prospective vs. retrospective testing**: Both are required under legacy ASC 815. Post-2017 ASU 2017-12 amendments streamlined this — retrospective quantitative testing is no longer required if the entity uses a quantitative prospective test.

### 4.2 IFRS 9 — Effectiveness Requirements

**No bright-line threshold**: IFRS 9 replaced the 80–125% test with three qualitative and quantitative criteria (IFRS 9.6.4.1):

1. **Economic relationship**: There is an economic relationship between the hedging instrument and the hedged item — the hedge ratio makes economic sense.
2. **Credit risk does not dominate**: The effect of credit risk does not dominate the value changes resulting from the economic relationship.
3. **Hedge ratio**: The hedge ratio of the hedging relationship is the same as that resulting from the quantity of the hedging instrument actually used and the quantity of the hedged item actually being hedged (IFRS 9.B6.4.9).

**Effectiveness assessment methods** (IFRS 9 does not prescribe specific methods; commonly used):
- Dollar offset
- Regression analysis
- Hypothetical derivative

**Rebalancing**: When the hedge ratio changes due to changes in the relationship (but the risk management objective remains unchanged), IFRS 9 requires rebalancing rather than discontinuation (IFRS 9.6.5.5 and B6.5.7–B6.5.21). See Part 6 below.

### 4.3 Comparison Table — Effectiveness Testing

| Criterion | ASC 815 | IFRS 9 |
|---|---|---|
| Quantitative threshold | 80–125% offset ratio | None (principles-based economic relationship test) |
| Prospective test required | Yes | Yes |
| Retrospective test required | No (post-ASU 2017-12) | No |
| Method prescribed | No (multiple permitted) | No (entity chooses; must be documented) |
| Rebalancing required when ratio drifts | No — discontinue or redesignate | Yes — rebalance (discontinuation only if risk objective changes) |
| Credit risk dominance test | Not explicitly stated as a criterion | Explicit criterion (IFRS 9.6.4.1(b)) |

---

## Part 5: OCI Mechanics by Hedge Type

### 5.1 Fair Value Hedge — OCI Mechanics

Fair value hedges do **not** use OCI for the effective portion. Both the hedging instrument gain/loss and the hedged item basis adjustment flow through P&L. The only OCI impact arises from:
- **Cost of hedging** (options time value, forward points) if the entity elects the cost-of-hedging approach under IFRS 9 (see Part 7).
- **Equity method investees**: If the hedged item is equity-method investment, OCI treatment may apply in specific circumstances.

### 5.2 Cash Flow Hedge — OCI Mechanics

**Effective portion — recognized in OCI**:

The lower of (a) the cumulative gain/loss on the hedging instrument since inception of the hedge, and (b) the cumulative change in fair value (present value of cash flows) of the hedged item since inception.

**Ineffective portion — recognized immediately in P&L**:

Any excess of the gain/loss on the hedging instrument over the movement in the hedged item (over-hedging) is P&L immediately.

**Reclassification from OCI (AOCI under ASC 815 / cash flow hedge reserve under IFRS 9)**:

- When the hedged forecast transaction affects P&L (e.g., variable interest expense recognized, hedged sale recognized as revenue) → reclassify from OCI to same P&L line
- When hedged transaction results in recognition of a non-financial asset/liability → either reclassify to P&L when asset/liability affects P&L, OR adjust the basis of the asset/liability by the deferred OCI amount (basis adjustment method)
- If forecast transaction is no longer expected to occur → reclassify entire deferred OCI balance immediately to P&L

**Line item presentation**: Under both ASC 815 and IFRS 9, the reclassified OCI must be presented in the same line item as the hedged item (e.g., revenue, cost of goods sold, interest expense). This is a key disclosure and presentation requirement.

### 5.3 Net Investment Hedge — OCI Mechanics

**Effective portion → CTA / Translation Reserve (OCI)**:
- Recorded in OCI alongside the foreign operation's translation adjustment
- Remains in OCI until disposal or partial disposal of the foreign operation

**Recycling on disposal**:
- ASC 815 (ASC 830-30-40-1): Full CTA is reclassified to P&L on disposal (including CTA on the hedging instrument)
- IFRS 9 / IAS 21: Translation reserve reclassified to P&L on disposal of foreign operation

**Partial disposal**:
- ASC 815 (ASC 830-30-40-1A): Proportionate CTA reclassified to P&L on partial disposal
- IFRS 9 / IAS 21.48C: Proportionate translation reserve reclassified on partial disposal only when it results in loss of control

---

## Part 6: IFRS 9 Rebalancing (No ASC 815 Equivalent)

### 6.1 What Triggers Rebalancing

Rebalancing is required under IFRS 9 when the hedge ratio needs to be adjusted because:
- The ratio between the quantity of hedging instrument and the quantity of hedged item has changed due to changes in the underlying relationship (e.g., the commodity price behavior changes)
- The risk management objective is **unchanged** — only the ratio needs adjustment

**IFRS 9.B6.5.7**: Rebalancing does not apply when the entity adjusts the hedge ratio for risk management reasons unrelated to the economics (e.g., gaming the hedge ratio to avoid recognizing ineffectiveness). The standard explicitly prohibits rebalancing that would adjust the ratio away from the one actually used.

### 6.2 How Rebalancing Works

When rebalancing by **increasing the hedging instrument**:
1. Continue accounting for the unchanged portion as before
2. Treat the additional quantity as a new hedge, recognized at current fair value (no prior AOCI reclassification)
3. The hedge ratio becomes the new combined ratio

When rebalancing by **decreasing the hedging instrument**:
1. Continue accounting for the remaining unchanged portion as before
2. The removed portion of the hedging instrument is accounted for separately (fair value through P&L from that point)
3. The associated AOCI balance for the discontinued portion is reclassified based on whether the hedged transaction is still expected to occur

When rebalancing by **decreasing the hedged item**:
1. The hedge ratio decreases in the hedged item direction
2. The portion no longer part of the hedge is subject to discontinuation rules for that portion only

### 6.3 ASC 815 — No Rebalancing Concept

ASC 815 does not have a rebalancing concept. If the hedge ratio drifts from the documented ratio, the entity must:
- Discontinue the hedge relationship, and
- Redesignate a new hedge relationship (or redesignate with new terms)

This is a significant practical difference from IFRS 9 in multi-period hedging programs.

---

## Part 7: Cost-of-Hedging Approach (IFRS 9 Only)

### 7.1 Scope — IFRS 9.6.5.15–6.5.16

The cost-of-hedging approach applies to two specific components:
1. **Time value of options** (IFRS 9.6.5.15): When only the intrinsic value of an option is designated as the hedging instrument, the change in time value is deferred in OCI (the "costs of hedging" reserve)
2. **Forward element of forward contracts** (IFRS 9.6.5.16): When only the spot element of a forward contract is designated, the change in forward points (forward element) is deferred in OCI

### 7.2 Aligned vs. Transaction-Related Hedges

| Hedge Type | Time Value / Forward Points Treatment |
|---|---|
| **Transaction-related** hedge (hedging a forecast transaction or firm commitment that results in recognition of a non-financial item) | Deferred in OCI → included in the initial cost/carrying amount of the non-financial item (basis adjustment) when it is recognized |
| **Time-period-related** hedge (hedging a financial item, or hedging on a time-period basis such as a commodity price risk over rolling periods) | Deferred in OCI → amortized to P&L over the period of the hedge on a rational basis (straight-line or other systematic method) |

### 7.3 ASC 815 — No Direct Equivalent

ASC 815 does not have the cost-of-hedging concept. Under ASC 815:
- If only the intrinsic value of an option is designated, the excluded component (time value) is recognized in P&L immediately (ASC 815-20-25-83)
- **Alternatively** (post-ASU 2017-12): The excluded component can be recognized in OCI and reclassified to P&L on a systematic basis (the "amortization approach" under ASC 815-20-25-83A–83C). This is similar in effect to IFRS 9's cost-of-hedging but is structured differently.

**Key difference**: IFRS 9 requires the entity to separately identify whether the hedge is transaction-related or time-period-related and applies different reclassification mechanics accordingly. ASC 815's amortization approach is simpler — always amortize to P&L.

---

## Part 8: Discontinuation Rules

### 8.1 Mandatory Discontinuation (Both Standards)

Both ASC 815 and IFRS 9 require discontinuation when:
- The hedging instrument expires, is sold, terminated, or exercised (and a replacement or rollover is not part of the documented hedging strategy)
- The hedge no longer meets the effectiveness criteria

### 8.2 Voluntary Discontinuation

| Standard | Voluntary Discontinuation |
|---|---|
| ASC 815 | **Permitted at any time** — the entity may voluntarily de-designate a qualifying hedge relationship (ASC 815-20-40-1) |
| IFRS 9 | **Prohibited** if the hedge still qualifies — IFRS 9.6.5.6 explicitly prohibits voluntary discontinuation of a qualifying hedge. An entity **may** choose not to apply hedge accounting from inception, but once designated and qualifying, it cannot be voluntarily discontinued |

**Practical implication**: Under IFRS 9, if an entity wishes to stop applying hedge accounting, it must either:
(a) Change its risk management objective (which is the trigger for mandatory discontinuation), or
(b) Restructure the hedge so it no longer qualifies

This is a fundamental difference from ASC 815 and creates strategic differences in hedge program design.

### 8.3 Accounting on Discontinuation — Cash Flow Hedges

| Scenario | ASC 815 | IFRS 9 |
|---|---|---|
| Hedged transaction still expected to occur | Deferred AOCI remains and is reclassified when transaction occurs | Deferred OCI remains and reclassified when transaction occurs (IFRS 9.6.5.12(a)) |
| Hedged transaction no longer expected | Deferred AOCI reclassified immediately to P&L | Deferred OCI reclassified immediately to P&L (IFRS 9.6.5.12(b)) |

### 8.4 Accounting on Discontinuation — Fair Value Hedges

The basis adjustment on the hedged item (accumulated fair value changes) remains on the balance sheet and is amortized over the remaining life of the hedged item. Under ASC 815 (ASC 815-25-40-1) and IFRS 9 (IFRS 9.6.5.10), this amortization uses the effective interest method.

---

## Part 9: Embedded Derivatives

### 9.1 ASC 815-15 — Embedded Derivatives

**Bifurcation requirement**: An embedded derivative must be bifurcated from the host contract and accounted for separately at fair value through P&L if all three criteria are met (ASC 815-15-25-1):
1. The economic characteristics and risks of the embedded derivative are not clearly and closely related to the host contract
2. The hybrid instrument is not already measured at fair value with changes in P&L
3. A separate instrument with the same terms would be a derivative

**Common examples requiring bifurcation**:
- Equity-indexed debt (returns tied to equity index — not clearly and closely related to a debt host)
- Credit-linked notes where the credit risk underlying is different from the issuer's credit risk
- Foreign currency-denominated debt held by an entity for which the currency is not functional or widely used (unless the currency is the functional currency of either party, or widely used in commerce for the specific type of transaction)

**Exception for FX**: ASC 815-15-15-9 provides a broad exception for contracts that require payment in a currency that is the functional currency of either party OR a currency commonly used in contracts of the same type.

### 9.2 IFRS 9.4.3 — Embedded Derivatives

**General rule**: Under IFRS 9, the embedded derivative concept primarily applies to **financial liabilities** and non-financial host contracts. For **financial assets**, IFRS 9 applies the SPPI test (solely payments of principal and interest) to the hybrid instrument as a whole — if the hybrid fails SPPI, the entire instrument is measured at fair value through P&L (IFRS 9.4.1.2A). No bifurcation for financial asset hybrids.

**For financial liabilities and non-financial hosts** (IFRS 9.4.3.3): Bifurcation required if:
1. Economic characteristics of the embedded derivative are not closely related to the host
2. A separate instrument with the same terms would be a derivative
3. The hybrid is not measured entirely at FVTPL

**Key difference**: IFRS 9 eliminated bifurcation for financial asset hybrids — the entire instrument is measured at FVTPL if the hybrid fails SPPI. ASC 815 retains bifurcation for financial asset hybrids.

---

## Part 10: Local GAAP Treatments

### 10.1 German HGB — §254 Bewertungseinheit (Valuation Unit)

**Basis**: German Commercial Code (Handelsgesetzbuch), §254 HGB, introduced by BilMoG (Bilanzrechtsmodernisierungsgesetz) in 2009.

**Concept**: A Bewertungseinheit allows the entity to offset fair value changes of hedging instruments and hedged items within a designated "valuation unit," preventing recognition of unrealized losses on the hedging instrument alone (consistent with the HGB imparity principle).

**Conditions** (§254 HGB):
- Offsetting relationship must exist between the financial risks of the hedged item and hedging instrument
- Hedge must be documented and demonstrate effectiveness (similar in concept to IFRS 9 but no specific effectiveness test prescribed)
- The Bewertungseinheit must be dissolved when the hedging relationship terminates

**Hedge types permitted**: §254 HGB applies to fair value hedges of existing positions and cash flow hedges of forecast transactions. Net investment hedges are addressed in §308a HGB in the context of consolidated financial statements.

**Contrast with IFRS 9**: Under HGB, the base principle (Niederstwertprinzip / imparity) would require recognizing an unrealized loss on an out-of-the-money derivative immediately. The Bewertungseinheit suspends this requirement for the hedged portion. Unlike IFRS 9, there is no OCI mechanism in HGB — the offsetting effect is achieved by simply not recognizing the valuation movements in the income statement to the extent they offset.

**Disclosure**: §285 Nr. 23 HGB requires disclosure of the type, scope, and purpose of Bewertungseinheiten in the notes.

Source: https://www.gesetze-im-internet.de/hgb/

### 10.2 JGAAP — ASBJ Statement No. 10 (Deferral Hedge)

**Basis**: Accounting Standard for Financial Instruments, ASBJ Statement No. 10 (originally issued 1999, revised multiple times; current version effective from fiscal years beginning on or after April 1, 2022 for most provisions).

**Three permitted methods**:

| Method | Description | Applicability |
|---|---|---|
| **Deferral hedge** (繰延ヘッジ) | Gains/losses on hedging instrument deferred in net assets (OCI equivalent) until recognized | General method for qualifying hedges |
| **Fair value hedge** (時価ヘッジ) | Fair value changes of both hedging instrument and hedged item recognized in P&L | Only for financial assets/liabilities measured at fair value |
| **Allocation method** (振当処理) | Special treatment for interest rate swaps and currency swaps meeting strict criteria; net amounts recognized | Interest rate swaps; currency swaps on foreign currency debt |

**Allocation method (振当処理)** for interest rate swaps — requires:
- Notional matches the principal
- Swap tenor matches the hedged debt tenor
- Fixed/floating match
- No net settlement provisions inconsistent with the hedged item

When the allocation method is applied, the interest rate swap is not remeasured at fair value; instead, net interest settlements are recognized as an adjustment to interest expense. Similar in economic effect to the ASC 815 short-cut method.

**Effectiveness testing**: JGAAP requires hedge effectiveness documentation. The standard broadly requires that hedging relationships be effective — no specific 80–125% bright-line applies, but the concept of offsetting is required.

Source: https://www.asb.or.jp/en/accounting_standards/accounting_standards/

### 10.3 CAS 24 — Chinese Accounting Standards for Enterprises No. 24 (Hedging)

**Basis**: Ministry of Finance CAS No. 24, revised in 2017 to converge with IFRS 9 Chapter 6.

**Substantive alignment with IFRS 9**: CAS 24 (2017 revision) closely follows IFRS 9.6 with the same three hedge types, same qualifying criteria (economic relationship, no credit risk dominance, appropriate hedge ratio), and similar OCI mechanics.

**Key differences from IFRS 9**:
- Macro hedge / portfolio hedge: CAS 24 does not adopt IAS 39 macro hedge carve-out
- Time value of options / forward points: CAS 24 incorporates cost-of-hedging concepts broadly consistent with IFRS 9.6.5.15–16
- Regulatory environment: Application requires compliance with MOF guidance; listed A-share companies follow CSRC disclosure requirements

Source: http://www.mof.gov.cn/

### 10.4 Ind AS 109 — Indian Accounting Standards 109

**Basis**: Ministry of Corporate Affairs, Ind AS 109, effective for annual periods beginning on or after April 1, 2018 for listed companies and certain other entities.

**Identical to IFRS 9**: Ind AS 109 is word-for-word identical to IFRS 9 for hedge accounting (Chapter 6). All IFRS 9 guidance on hedge types, eligibility, effectiveness testing, rebalancing, cost of hedging, and discontinuation applies without modification.

**Notable Ind AS context**:
- Reserve Bank of India (RBI) regulations govern which instruments can be used as hedges by regulated entities (banks, NBFCs) — these regulatory constraints exist outside Ind AS 109
- SEBI regulations may impose additional disclosure requirements for listed entities
- Ind AS 109 carve-out: India adopted IFRS 9 with a carve-out for the classification and measurement of certain financial assets (not for hedge accounting)

Source: https://www.icai.org/post/indian-accounting-standards

---

## Part 11: Multi-Jurisdiction Comparison Table

| Feature | ASC 815 (US GAAP) | IFRS 9 | IAS 39 | German HGB §254 | JGAAP ASBJ No.10 | CAS 24 | Ind AS 109 |
|---|---|---|---|---|---|---|---|
| Hedge types | FV, CF, NI | FV, CF, NI | FV, CF, NI | Valuation unit (FV and CF) | Deferral hedge, FV hedge, Allocation | FV, CF, NI | FV, CF, NI (identical to IFRS 9) |
| Effectiveness test | 80–125% (quantitative) | Principles-based (economic relationship) | 80–125% (quantitative) | No specific threshold; economic offset required | Economic offset; no bright-line | Same as IFRS 9 | Same as IFRS 9 |
| Voluntary discontinuation | Permitted at any time | Prohibited if qualifying | Permitted (like ASC 815 in this respect) | Dissolves when hedge terminates; no voluntary exit concept | Generally follows designation — no explicit prohibition | Prohibited (follows IFRS 9) | Prohibited (identical to IFRS 9) |
| Rebalancing | Not available; redesignate required | Required when hedge ratio changes (but risk objective unchanged) | Not available | Not applicable | Not applicable | Same as IFRS 9 | Same as IFRS 9 |
| Cost of hedging (time value / forward points) | Amortization approach for excluded components (post-ASU 2017-12) | Explicit cost-of-hedging reserve (IFRS 9.6.5.15–16); transaction-related vs. time-period distinction | Not available (excluded components to P&L) | Not applicable as a separate concept | Not applicable | Substantially consistent with IFRS 9 | Identical to IFRS 9 |
| Aggregated exposures as hedged item | Not permitted | Permitted (IFRS 9.6.3.4) | Not permitted | Not addressed | Not addressed | Permitted (follows IFRS 9) | Permitted (identical to IFRS 9) |
| Net position hedging | Not permitted (except narrow exceptions) | Permitted under specific conditions (IFRS 9.6.6) | Not permitted | Not addressed | Not addressed | Same as IFRS 9 | Same as IFRS 9 |
| Portfolio layer / macro hedge | Portfolio layer method for closed portfolios (ASC 815-20-25-12A) | IAS 39 macro hedge carve-out available (IFRS 9 prohibits fair value hedges of open portfolios — entities may elect IAS 39 for macro hedge) | IAS 39.81A permits macro fair value hedges of interest rate risk in open portfolios | Aggregated positions possible within §254 | Not specifically addressed | No macro hedge carve-out | No macro hedge carve-out (identical to IFRS 9) |
| Embedded derivatives — financial assets | Bifurcation required (ASC 815-15) | SPPI test on whole instrument; no bifurcation | Bifurcation required | Imparity principle applies to entire instrument | Similar bifurcation concept | Follows IFRS 9 for financial assets | Identical to IFRS 9 |
| OCI mechanism | AOCI for CF and NI hedges | Cash flow hedge reserve / hedging reserve for CF; translation reserve for NI | AOCI for CF and NI | No OCI (offset via suspended recognition) | Net assets section (OCI equivalent) for deferral hedge | OCI for CF and NI | Identical to IFRS 9 |

---

## Part 12: Designation, Documentation, and Formal Requirements

### 12.1 Documentation Requirements

Both ASC 815 and IFRS 9 require formal hedge documentation **at inception** (not retrospectively). Key elements:

| Element | ASC 815 Reference | IFRS 9 Reference |
|---|---|---|
| Risk management objective and strategy | ASC 815-20-25-1(c) | IFRS 9.6.4.1(b) |
| Nature of the risk being hedged | ASC 815-20-25-1(d) | IFRS 9.6.4.1(b) |
| Identity of hedging instrument | ASC 815-20-25-1(a) | IFRS 9.6.4.1 |
| Identity of hedged item | ASC 815-20-25-1(a) | IFRS 9.6.4.1 |
| Effectiveness assessment method | ASC 815-20-25-1(e) | IFRS 9.6.4.1(c) |
| Hedge ratio | Not explicitly required (implied) | IFRS 9.B6.4.9 — explicitly required |
| Rebalancing policy | Not applicable | IFRS 9.B6.5.7 |

### 12.2 Retrospective Designation

Neither ASC 815 nor IFRS 9 permits retrospective designation of hedge accounting. Documentation must exist on the date of designation.

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described. Hedge accounting requirements vary by jurisdiction, entity type, and the specific terms of each hedging relationship. This analysis does not constitute authoritative accounting guidance, a compliance opinion, a legal opinion, or formal hedge designation documentation in any jurisdiction. Formal hedge documentation, effectiveness assessments, and auditor acceptance require qualified accountants and external auditors. Local GAAP conclusions (HGB, JGAAP, CAS, Ind AS) should be verified with local statutory auditors. This analysis does not form an accountant-client relationship.
