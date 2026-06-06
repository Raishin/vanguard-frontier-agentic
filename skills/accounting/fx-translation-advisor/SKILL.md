---
name: fx-translation-advisor
description: Multi-jurisdiction reference framework for foreign currency translation and remeasurement covering functional currency determination, ASC 830 / IAS 21 method selection, CTA in OCI, highly inflationary economy treatment, net investment hedge interactions, and multi-GAAP comparison across US GAAP, IFRS, German HGB, JGAAP, CAS 19, and Ind AS 21.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# FX Translation Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for foreign currency translation and remeasurement advisory — from functional currency determination through method selection, CTA routing, highly inflationary economy treatment, net investment hedge interactions, and jurisdictional FX control overlays.

---

## Part 1: Functional Currency Determination

### ASC 830-10-45 (US GAAP) and IAS 21.9–21.14 (IFRS)

The **functional currency** is the currency of the primary economic environment in which an entity operates. Determination uses a hierarchy of indicators:

**Primary indicators (IAS 21.9 / ASC 830-10-45-2):**

| Indicator | Question |
|---|---|
| Sales price currency | In which currency are sales prices denominated and settled? |
| Sales market currency | Which currency dominates the competitive environment in which the entity sells? |
| Labor and material cost currency | In which currency are the entity's principal costs (labor, materials) denominated and settled? |

**Secondary indicators (IAS 21.10 / ASC 830-10-45-4):**

| Indicator | Question |
|---|---|
| Financing currency | In which currency are debt instruments denominated and settled? |
| Cash flow currency | In which currency are receipts from operating activities usually retained? |

**Group entity indicators (IAS 21.11):**
- Is the foreign operation an extension of the parent's activities (parent's currency indicators dominate)?
- Or does it operate with significant autonomy (local currency indicators dominate)?

**Conflict resolution:**
- When primary and secondary indicators conflict, judgment is required.
- IAS 21.12 provides that management uses judgment to determine the currency that most faithfully represents the economic effects of transactions, events, and conditions.
- ASC 830-10-45-6: when indicators conflict, management considers the totality of the facts.

**Functional currency once determined:**
- IAS 21.13: once determined, the functional currency does not change unless there is a change in the underlying transactions, events, and conditions.
- ASC 830-10-45-7: change in functional currency is treated prospectively from the date of the change.

---

## Part 2: Translation Method vs. Remeasurement Method

### The Two-Step Framework

**Step 1 — Is the entity's record-keeping currency the same as its functional currency?**
- No → Apply **remeasurement** (temporal method) to convert records into functional currency first.
- Yes → Proceed to Step 2.

**Step 2 — Is the functional currency the same as the parent's presentation currency?**
- No → Apply **translation** (current rate method) to convert functional currency statements into presentation currency.
- Yes → No translation required.

---

### Remeasurement (Temporal Method)

**When it applies**: foreign currency records ≠ functional currency (entity keeps books in a currency that is not its functional currency).

**Rate assignment — ASC 830-10-45-17 / IAS 21.23–21.24:**

| Item Category | Rate |
|---|---|
| Monetary assets (cash, receivables, payables, debt) | **Closing rate** (balance sheet date) |
| Non-monetary assets carried at historical cost (inventory at cost, PP&E, goodwill, intangibles) | **Historical rate** (rate at date of transaction) |
| Non-monetary assets carried at fair value (investment properties at FV, equity securities at FV) | **Rate at date of fair value measurement** |
| Income statement items related to non-monetary items (COGS, depreciation, amortization) | **Historical rate** (matching the underlying asset) |
| Other income statement items (revenue, operating expenses not linked to non-monetary items) | **Average rate** for the period (or transaction date rate if more appropriate) |

**P&L impact**: remeasurement gains and losses go to **P&L** (not OCI).
- ASC 830-20-35-1: transaction gains and losses on monetary items → P&L.
- IAS 21.28: exchange differences on monetary items → P&L in the period.

---

### Translation (Current Rate Method)

**When it applies**: functional currency ≠ presentation currency (entity translates its functional-currency statements into the parent's presentation currency).

**Rate assignment — ASC 830-30-45-3 / IAS 21.39–21.42:**

| Item Category | Rate |
|---|---|
| Assets (all) | **Closing rate** (balance sheet date) |
| Liabilities (all) | **Closing rate** (balance sheet date) |
| Income statement items (revenue, expenses, gains, losses) | **Average rate** for the period (or transaction date rate if exchange rates fluctuate significantly) |
| Equity components (share capital, additional paid-in capital, retained earnings at time of investment) | **Historical rate** (rate at date of investment or transaction) |
| Dividends declared | **Rate at declaration date** |

**OCI routing**: the net translation difference is the **Cumulative Translation Adjustment (CTA)**:
- ASC 830-30-45-12: CTA → accumulated in OCI (AOCI); not reclassified to P&L until disposal of the foreign operation.
- IAS 21.39: translation differences → OCI (Translation Reserve); recycled to P&L on disposal (IAS 21.48).

---

## Part 3: CTA Accumulation, Disposal, and Recycling

### Accumulation in OCI

CTA accumulates in OCI for each period that:
- The foreign operation exists with a functional currency different from the parent's presentation currency.
- Exchange rates differ between the opening and closing balance sheet dates.

### Disposal — CTA Recycling

**IFRS (IAS 21.48):**
- On disposal of a foreign operation, the cumulative CTA related to that operation is **reclassified from OCI to P&L** (recycled) as a reclassification adjustment.
- IAS 21.48C: on partial disposal of a subsidiary that includes a foreign operation, the entity reclassifies a proportionate share of the CTA to the non-controlling interest.

**US GAAP (ASC 830-30-40-1):**
- On sale or liquidation of a foreign operation, the **pro-rata portion of the CTA** related to that operation is **released from AOCI and recognized in P&L**.
- Key difference from IFRS: under US GAAP, partial disposal of a subsidiary does not trigger partial CTA reclassification unless the subsidiary is deconsolidated. ASC 830-30-40 requires full deconsolidation or sale to trigger release.

### Long-Term Monetary Intragroup Items (IAS 21.32)

**IAS 21.32**: exchange differences on monetary items that form part of a net investment in a foreign operation (i.e., long-term intragroup loans with no scheduled repayment) → OCI (not P&L) at the consolidated level.
- At the individual entity level, these may be recognized in P&L.
- The OCI election applies only in consolidated financial statements.

**US GAAP equivalent (ASC 830-20-35-3)**: long-term intercompany transactions that are of an investment nature → CTA in OCI at the consolidated level (same result, different articulation).

---

## Part 4: Highly Inflationary Economies

### ASC 830-10-45-11 (US GAAP)

**Trigger**: cumulative three-year inflation rate > 100%.

**Consequence**: the foreign entity's functional currency is **replaced by the parent's reporting currency** (USD for a US parent) for accounting purposes. The entity must then **remeasure** using the temporal method with USD as the functional currency.

**Practical implication**: non-monetary items (PP&E, inventory) retain their historical USD cost basis; no translation adjustment in OCI — all differences go to P&L.

**Affected jurisdictions (as of 2025–2026):**
- **Argentina**: designated highly inflationary for US GAAP purposes; also IAS 29 hyperinflationary.
- **Turkey**: designated highly inflationary for US GAAP purposes; IAS 29 applied from 1 January 2022.
- Monitor: Ethiopia, Haiti, Iran, Lebanon, South Sudan, Sudan, Venezuela, Yemen, Zimbabwe — verify current status against published indices.

### IAS 29 (IFRS) — Hyperinflationary Economies

**Trigger**: IAS 29.3 indicators include cumulative inflation rate over three years approaching or exceeding 100%, but judgment is required across qualitative indicators including:
- General population preferring to keep wealth in non-monetary assets or relatively stable currencies.
- Monetary amounts stated in terms of a relatively stable foreign currency.
- Interest, wages, and prices linked to a price index.
- Three-year cumulative inflation approaching or exceeding 100%.

**Consequence**: financial statements (including comparatives) are **restated to the measuring unit current at the balance sheet date** using a general price index. Non-monetary items are adjusted by applying the change in the general price index from the date of acquisition.

**IAS 29.8**: all items in the statement of financial position not already expressed in terms of the measuring unit current at the balance sheet date are restated by applying a general price index.

**Key difference from ASC 830**: IAS 29 restates the financial statements (purchasing power restatement); ASC 830-10-45-11 changes the functional currency designation and applies temporal method remeasurement. These are different mechanical approaches.

---

## Part 5: Net Investment Hedge Accounting

### ASC 830-20 / IAS 21.32 and Interaction with ASC 815 / IFRS 9

**Net investment in a foreign operation** = the reporting entity's interest in the net assets of a foreign operation.

**Hedge of net investment:**

| Framework | Standard | Hedge gain/loss routing |
|---|---|---|
| US GAAP | ASC 830-20-35-3 + ASC 815-35-1 | Effective portion → OCI (CTA); reclassified to P&L on disposal of the investment |
| IFRS | IAS 21.32 + IFRS 9.6.5.13 | Effective portion → OCI (Translation Reserve); recycled to P&L on disposal |

**Eligible hedging instruments:**
- Foreign currency-denominated debt instruments (non-derivative hedging instruments).
- Foreign currency forward contracts or cross-currency swaps (derivative hedging instruments).

**Designation requirements:**
- The hedged item is the net assets of the foreign operation, not individual assets/liabilities.
- Hedge must be formally documented and assessed for effectiveness.

---

## Part 6: Multi-GAAP Comparison Table

| Area | US GAAP (ASC 830) | IFRS (IAS 21) | German HGB | JGAAP (ASBJ No. 22) | CAS 19 (China) | Ind AS 21 |
|---|---|---|---|---|---|---|
| Translation method | Current rate for all A/L; average for P&L; historical for equity | Same as US GAAP | Closing rate for A/L; historical for equity; average or closing for P&L (simplified) | Broadly similar to IFRS; closing rate for A/L; average for P&L | Similar to IAS 21; closing for A/L; average for P&L | Converged with IAS 21; minor carve-outs |
| CTA in OCI | Yes — AOCI; no P&L recycling until disposal | Yes — Translation Reserve; **recycled to P&L on disposal** | No explicit OCI concept; translation differences generally recognized in equity reserve (§ 308a HGB) | Translation differences → OCI; recycled to P&L on disposal (similar to IFRS) | Translation differences → OCI; recycled on disposal (similar to IAS 21) | Same as IAS 21 — recycled on disposal |
| Remeasurement (temporal) gain/loss | P&L | P&L (IAS 21.28) | P&L | P&L | P&L | P&L |
| Highly inflationary | Change functional currency to USD (ASC 830-10-45-11) | IAS 29 restatement to current purchasing power | No specific guidance; general prudence principle applies | No specific guidance | Broadly follows IAS 29 concept | Follows IAS 29 |
| Long-term intragroup monetary items | OCI at consolidated level (ASC 830-20-35-3) | OCI at consolidated level (IAS 21.32); P&L at entity level | P&L | P&L (no OCI election available) | Similar to IAS 21 | Same as IAS 21 |
| CTA recycling on partial disposal of subsidiary | Only on full deconsolidation (ASC 830-30-40) | Proportionate share reclassified (IAS 21.48C) | N/A | Similar to IFRS | Similar to IAS 21 | Same as IAS 21 |

---

## Part 7: Jurisdictional FX Control Overlays

### China — SAFE (State Administration of Foreign Exchange)

**Relevant to FX translation accounting:**
- FX conversion and **repatriation of profits** from Chinese subsidiaries require SAFE approval; timing of approval affects which exchange rate is used in translation.
- **Intercompany loans** (cross-border): SAFE registration required; loan principal and interest repatriation subject to SAFE approval and registration — affects the characterization of the loan (short-term trade payable vs. long-term investment-nature loan under IAS 21.32).
- **Capital account transactions**: foreign direct investment receipts and equity capital repatriation require SAFE approval; timing impacts the historical rate used for equity layer.
- Source: SAFE Regulations on Foreign Exchange Administration — https://www.safe.gov.cn/en/

### India — FEMA (Foreign Exchange Management Act) and RBI

**Relevant to FX translation accounting:**
- **Capital account transactions** (equity investment, external commercial borrowings — ECB): require RBI approval or registration under FEMA 1999.
- **ECB regulations**: Indian entities borrowing from foreign parents must comply with RBI ECB Master Direction; the drawn-down currency affects functional currency indicators.
- **Repatriation of dividends**: permissible subject to compliance with Companies Act and FEMA; timing affects average rate vs. declaration-date rate application.
- **Compounding**: FEMA violations may result in compounding proceedings — advisors should flag FEMA compliance when advising on FX-denominated intragroup structures.
- Source: RBI Master Directions on External Commercial Borrowings — https://www.rbi.org.in/Scripts/BS_ViewMasDirections.aspx

### Brazil — IOF and SPED

**IOF (Imposto sobre Operações Financeiras — Tax on Financial Transactions):**
- IOF applies to FX conversion transactions (IOF/Câmbio); current rate varies by transaction type (capital inflows, loans, equity).
- IOF on cross-border loans affects the effective interest cost, which may affect functional currency indicators (financing cost currency).
- Source: Receita Federal — https://www.gov.br/receitafederal/en/subjects/taxes/iof

**SPED (Sistema Público de Escrituração Digital):**
- Brazilian entities must maintain accounting records in BRL and file SPED (ECD — Electronic Accounting Bookkeeping) in BRL regardless of group reporting currency.
- If a Brazilian subsidiary's functional currency is USD (e.g., due to USD-denominated operations), the entity still maintains statutory BRL books and remeasures to USD for group consolidation — a two-step process.
- Source: Receita Federal SPED — https://www.gov.br/receitafederal/pt-br/assuntos/orientacao-tributaria/declaracoes-e-demonstrativos/sped-sistema-publico-de-escrituracao-digital

---

## Part 8: Common FX Translation Errors by Category

| Error Category | Standard Violated | Detection Method |
|---|---|---|
| Using average rate for balance sheet monetary items | ASC 830-30-45-3 / IAS 21.39 | Compare rate applied to closing rate at balance sheet date |
| Using closing rate for non-monetary items under temporal method | ASC 830-10-45-17 / IAS 21.23 | Verify historical rate tracking for PP&E, inventory, goodwill |
| Not recycling CTA to P&L on disposal (IFRS) | IAS 21.48 | Disposal checklist: confirm CTA balance released at disposal date |
| Incorrectly recycling CTA under US GAAP (partial disposal) | ASC 830-30-40 | Confirm deconsolidation event before releasing AOCI |
| Applying translation instead of remeasurement for a USD-functional entity in a non-USD currency environment | ASC 830-10-45 / IAS 21.17 | Functional currency determination workpaper review |
| Not re-designating functional currency for highly inflationary economy | ASC 830-10-45-11 / IAS 29.3 | Inflation index monitoring procedure |
| Long-term intragroup loan treated as transactional (P&L) rather than investment-nature (OCI) | IAS 21.32 / ASC 830-20-35-3 | Loan characterization checklist: repayment expectation, documentation |
| Income statement at closing rather than average rate | ASC 830-30-45-3 / IAS 21.40 | Rate application template check |

---

## Part 9: Official Documentation — Publicly Accessible URLs

| Standard | Resource | URL | Access |
|---|---|---|---|
| ASC 830 (FX Translation) | FASB ASC | https://asc.fasb.org/830 | Registration required for full text |
| IAS 21 (FX Translation) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias21.html | Fully public |
| IAS 29 (Hyperinflationary Economies) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias29.html | Fully public |
| German HGB | Federal Ministry of Justice | https://www.gesetze-im-internet.de/hgb/ | Fully public (German) |
| JGAAP ASBJ Statement No. 22 | ASBJ English translations | https://www.asb.or.jp/en/accounting_standards/accounting_standards/ | Fully public |
| CAS 19 (China) | MOF China | https://www.mof.gov.cn/ | Limited English; Deloitte/PwC translations recommended |
| Ind AS 21 | ICAI | https://www.icai.org/post/indian-accounting-standards | Fully public |
| China SAFE | SAFE English portal | https://www.safe.gov.cn/en/ | Fully public |
| India RBI ECB Master Direction | RBI | https://www.rbi.org.in/Scripts/BS_ViewMasDirections.aspx | Fully public |
| Brazil IOF | Receita Federal | https://www.gov.br/receitafederal/en/subjects/taxes/iof | Fully public |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described above. FX translation and remeasurement determinations depend on facts and circumstances that may not be fully captured in the described scenario. Local statutory reporting requirements vary by jurisdiction and change frequently. Capital control and jurisdictional overlay analysis (SAFE, FEMA, IOF) is informational only and does not constitute legal or regulatory advice. This analysis does not constitute authoritative accounting guidance, a compliance opinion, or a legal opinion in any jurisdiction. Verify with qualified accounting, legal, and treasury advisors before relying on this analysis for compliance or reporting purposes.
