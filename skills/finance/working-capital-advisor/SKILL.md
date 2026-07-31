---
name: working-capital-advisor
description: Multi-jurisdiction reference framework for working capital management — cash conversion cycle (CCC) optimization, accounts receivable management (collections, credit policy, aging analysis, factoring, invoice discounting, AR securitization, ASC 860 / IFRS 9 SPPI derecognition), accounts payable optimization (payment term extension, dynamic discounting, supply chain finance / reverse factoring, IAS 7.44A / ASU 2022-04 classification), inventory management (EOQ, JIT, safety stock, ABC analysis, IAS 2 vs. ASC 330), 13-week rolling cash forecasting (IAS 7 direct method), and working capital financing (ABL, receivables financing, SCF platforms, trade finance). Advisory only — never writes to ERP, AR/AP systems, or any system of record.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-03"
  category: finance
  lifecycle: experimental
---

# Working Capital Advisor Skill

> Read-only reference framework. All conclusions are advisory. Working capital financing structures, receivables derecognition eligibility, and supply chain finance classification involve fact-specific legal and accounting judgments. Verify with qualified external auditors, legal counsel, and relevant financial advisors before implementing any working capital program or financing structure.

---

## Part 1 — Cash Conversion Cycle (CCC) Framework

### 1.1 CCC Formula and Components

**CCC = DIO + DSO − DPO**

| Component | Formula | What it Measures | Improvement Direction |
|---|---|---|---|
| **DSO** (Days Sales Outstanding) | (Accounts Receivable / Revenue) × Days | How quickly customers pay | Reduce — accelerate collections |
| **DIO** (Days Inventory Outstanding) | (Inventory / COGS) × Days | How long inventory is held | Reduce — lean inventory management |
| **DPO** (Days Payable Outstanding) | (Accounts Payable / COGS) × Days | How long the company takes to pay suppliers | Increase — extend payment terms |
| **CCC** | DIO + DSO − DPO | Net days cash is tied up in operations | Reduce — ideally negative for high-DPO models |

**Negative CCC** (e.g., Amazon, major retailers): Customers pay before suppliers are paid — net working capital is a source of cash, not a use.

### 1.2 Industry CCC Benchmarks (illustrative; verify with current sources)

| Industry | Typical DSO (days) | Typical DIO (days) | Typical DPO (days) | Typical CCC (days) |
|---|---|---|---|---|
| **Manufacturing (general)** | 45–65 | 50–80 | 35–55 | 50–90 |
| **Retail (general)** | 5–20 | 30–60 | 30–60 | 0–30 |
| **Technology (hardware)** | 40–60 | 40–70 | 40–70 | 30–60 |
| **Healthcare (hospital)** | 40–60 | 20–40 | 30–50 | 20–50 |
| **Construction** | 60–90 | 30–50 | 40–70 | 30–80 |
| **Food & Beverage** | 25–45 | 20–40 | 30–60 | −10 to +40 |
| **Professional Services** | 45–75 | N/A | 20–40 | 20–50 |

**Source guidance:** Verify current benchmarks against REL Hackett Group Working Capital Survey, Dun & Bradstreet industry statistics, or Bloomberg industry data for the specific sector and geography.

### 1.3 CCC Improvement Levers by Component

| Lever | Component | Mechanism | Risk / Consideration |
|---|---|---|---|
| Tighten credit policy | DSO | Reduce default risk; shorten payment terms for high-risk customers | Revenue impact if terms are non-competitive |
| Accelerate invoicing | DSO | Invoice immediately upon delivery / milestone | Requires billing process discipline |
| Collections automation | DSO | Automated dunning, e-invoicing, payment portals | Implementation cost |
| Factoring / invoice discounting | DSO | Monetize receivables before customer payment | Cost of financing; derecognition eligibility |
| Reverse factoring / SCF | DPO | Pay suppliers early via third-party funder; extend own DPO | Classification risk (IAS 7.44A); supplier relationship |
| Payment term extension | DPO | Negotiate longer standard terms with suppliers | Supplier relationship; pricing impact |
| Dynamic discounting | Both | Early payment for discount; optimizes both buyer and supplier CCC | Requires treasury capacity |
| EOQ / JIT implementation | DIO | Reduce inventory to economic minimum | Supply chain disruption risk |
| ABC inventory analysis | DIO | Prioritize high-value (A) items for tight control | Requires data classification |
| Safety stock optimization | DIO | Right-size buffer stock per item | Service level risk if too low |

---

## Part 2 — Accounts Receivable Management

### 2.1 Credit Policy Framework

A sound credit policy should address:

1. **Customer credit assessment** — credit scoring criteria, financial statement analysis, trade reference checks, credit bureau data (Dun & Bradstreet, Experian Commercial).
2. **Credit limit setting** — tiered limits by customer risk rating; annual review for significant customers.
3. **Payment terms** — standard terms by customer segment (e.g., Net 30, Net 60, 2/10 Net 30 for early-pay discount).
4. **Collections escalation** — automated reminders (7 days before due, day of, 7 days after, 30 days after); manual follow-up; collection agency threshold.
5. **Write-off policy** — aging threshold for provision (IAS 37 / ASC 450) and write-off (IAS 39 / IFRS 9 ECL model for financial statement purposes).
6. **Dispute resolution process** — root cause analysis; escalation matrix; billing accuracy targets.

### 2.2 AR Aging Analysis Framework

**Never use named debtor data or customer-identifying information. Use anonymized cohort analysis only.**

| Aging Bucket | Action |
|---|---|
| 0–30 days (current) | No action; monitor |
| 31–60 days | Automated reminder; confirm receipt of invoice |
| 61–90 days | Phone follow-up; confirm payment date |
| 91–120 days | Senior collections escalation; payment plan discussion |
| 121–180 days | Legal review; put on credit hold; consider collection agency |
| 180+ days | Provision at 50–100%; write-off evaluation |

**IFRS 9 ECL (Expected Credit Loss) provisioning:** Under IFRS 9 §5.5, trade receivables without significant financing component must use the simplified approach — lifetime ECL using a provision matrix by aging bucket. The ECL rate for each bucket should be based on historical loss experience adjusted for forward-looking macro factors (IFRS 9.B5.5.35).

**US GAAP ASC 326 (CECL):** For trade receivables, entities may use a practical expedient — pool receivables by shared risk characteristics (e.g., geography, customer type, credit rating) and estimate lifetime credit losses using historical loss rates adjusted for current conditions.

### 2.3 AR Financing Structures

#### 2.3.1 Factoring

| Type | Description | Balance Sheet Treatment | Key Consideration |
|---|---|---|---|
| **Recourse factoring** | Seller retains credit risk; factor has recourse to seller if debtor defaults | Typically **on-balance-sheet** — seller retains substantially all risks and rewards | Cheaper; but no derecognition benefit |
| **Non-recourse factoring** | Factor bears credit risk; no recourse to seller | **Off-balance-sheet eligible** if transfer test met (IFRS 9 / ASC 860) | More expensive; requires true-sale legal opinion |
| **Invoice discounting** | Confidential; seller retains collections relationship | Usually **on-balance-sheet** — control retained | Confidentiality benefit; no derecognition |

#### 2.3.2 Receivables Derecognition — IFRS 9 vs. ASC 860

**IFRS 9 §3.2 — Derecognition of Financial Assets:**

Step 1: Have the contractual rights to cash flows expired? If yes → derecognize.

Step 2: Has the entity transferred the contractual rights? If yes:
- **Step 2a — Risks and rewards test (IFRS 9.3.2.6):**
  - Transferred substantially all risks and rewards → **derecognize** (IFRS 9.3.2.6(a))
  - Retained substantially all risks and rewards → **do not derecognize** (IFRS 9.3.2.6(b))
  - Neither transferred nor retained substantially all → **Step 2b**
- **Step 2b — Control test (IFRS 9.3.2.6(c)):** If entity has not retained control → derecognize. If it has → continuing involvement approach.

**ASC 860-10-40 (US GAAP) — Transfers of Financial Assets:**

A transfer is recognized as a sale when all of the following conditions are met (ASC 860-10-40-5):

1. The transferred assets have been isolated from the transferor (and its consolidated affiliates).
2. Each transferee has the right to pledge or exchange the assets (or the beneficial interests).
3. The transferor does not maintain effective control over the transferred assets.

**Key divergence:** IFRS 9 uses a risks-and-rewards analysis first; ASC 860 is control-based. A transaction that achieves derecognition under one standard may not under the other. Always obtain a true-sale legal opinion and auditor concurrence.

#### 2.3.3 AR Securitization

AR securitization involves transferring receivables to a Special Purpose Entity (SPE/SPV) that issues asset-backed securities (ABS) to investors.

Key accounting considerations:
- **IFRS 10 / ASC 810 — Consolidation of SPE:** If the originator controls the SPE (power + exposure to variable returns), the SPE consolidates and no derecognition occurs. Structuring to avoid consolidation requires genuine third-party equity at risk (typically ≥ 3% of SPV assets under US market practice).
- **IFRS 9 derecognition** of receivables transferred to the SPV follows the same §3.2 analysis.
- **Overcollateralization and credit enhancement** (seller's retained interest, reserve accounts) may indicate continuing involvement — derecognition only of the portion transferred without retention of risks/rewards.

**SPPI test (IFRS 9.4.1.2):** Receivables must meet the Solely Payments of Principal and Interest test to be classified at amortized cost. Standard trade receivables typically pass. Receivables with complex features (embedded derivatives, variable returns linked to equity) may fail — preventing amortized cost classification.

---

## Part 3 — Accounts Payable Optimization

### 3.1 Payment Term Extension

DPO improvement through payment term extension:
- **Negotiation approach:** Offer suppliers alternative value (volume commitment, preferred status, streamlined PO-to-payment process) in exchange for extended payment terms.
- **Market standard by industry:** Manufacturing commonly 45–75 days; retail 45–90 days; technology 30–60 days. Benchmarking against Hackett Group / REL data recommended.
- **Legal and ethical constraints:** UK Prompt Payment Code, EU Late Payment Directive (Directive 2011/7/EU — max 60 days for commercial transactions unless explicitly agreed), and similar rules in Australia (APRA guidance), Japan (Subcontracting Act), and US (state-level prompt payment statutes for public contracts).

**EU Late Payment Directive — key provisions (Directive 2011/7/EU Art. 3–4):**
- B2B payment terms: 60 days maximum unless parties expressly agree and it is not grossly unfair to creditor.
- Statutory interest rate: ECB reference rate + 8 percentage points.
- Compensation for recovery costs: minimum €40 per invoice.

### 3.2 Supply Chain Finance / Reverse Factoring

**Structure:** Buyer arranges a program with a bank or SCF platform (Taulia, PrimeRevenue, C2FO, Greensill successor platforms). Supplier presents approved invoices on the platform and can request early payment at a discounted rate (buyer's credit rating, not supplier's). Buyer pays the bank at the original or extended due date.

**Economic benefit:**
- Supplier: early liquidity at the buyer's lower cost of credit.
- Buyer: extended DPO without damaging supplier relationships; potentially better pricing from suppliers.

**IAS 7.44A–44D — Supplier Finance Arrangements Disclosure (IASB amendments effective 1 January 2024):**

Entities must disclose for each supplier finance arrangement:
- Terms and conditions (including extended payment terms; whether confirmed invoices are collateral).
- Carrying amount of trade payables included in the arrangement.
- Carrying amount of financial liabilities confirmed as part of the arrangement.
- Range of payment due dates for trade payables and the related financial liabilities.
- Liquidity risk information if the arrangement is significant.

**Classification — trade payable vs. financing liability:**
- If the SCF arrangement merely accelerates supplier payment at buyer's option without changing the character of the payable → trade payable presentation is supportable.
- If the SCF instrument substitutes for the original payable (novation; new obligor; extended terms beyond original invoice due date that are markedly different) → classification as a financing liability (non-trade payable) may be required, with cash flows presented as financing outflows under IAS 7.17(e).

**ASU 2022-04 (FASB) — Supplier Finance Program Disclosures (effective for fiscal years beginning after December 15, 2022):**
- Buyers must disclose: key terms of supplier finance programs; amount outstanding at period-end; roll-forward of amounts outstanding during the period.
- US GAAP does not require reclassification of payables in a supplier finance program as financing liabilities (unlike the IAS 7 potential reclassification risk). Disclosure is the primary obligation.

### 3.3 Dynamic Discounting

**Structure:** Buyer offers suppliers the ability to receive early payment in exchange for a discount on the invoice value. The buyer funds the early payment from its own cash (unlike reverse factoring where a bank funds it). Discount rate is typically an annualized rate applied pro-rata for the days early.

**Example:** Net-60 invoice of $1,000,000; buyer offers 1.5% annualized discount for payment at Day 10 → discount = 1.5% × (50/365) × $1,000,000 = $2,055. Supplier receives $997,945.

**Accounting treatment (buyer):**
- Discount earned reduces cost of goods purchased (or is recognized as finance income depending on substance); not a revenue item.

**Benefit vs. reverse factoring:**
- Dynamic discounting uses buyer's own cash → no banking arrangement; no IAS 7.44A disclosure trigger.
- Reverse factoring uses bank funding → disclosure required; classification risk if terms materially extended.

---

## Part 4 — Inventory Management

### 4.1 EOQ — Economic Order Quantity

**Formula:** EOQ = √(2DS / H)

Where:
- D = Annual demand (units)
- S = Ordering cost per order ($)
- H = Holding cost per unit per year ($)

**Assumptions:** Constant demand; instantaneous replenishment; no quantity discounts; no shortages. Relax assumptions for real-world modifications (EOQ with quantity discounts; production run model; probabilistic demand).

### 4.2 Safety Stock

**Formula (simple):** Safety Stock = Z × σ_LT × √LT

Where:
- Z = Service level factor (e.g., 1.65 for 95%; 2.05 for 98%; 2.33 for 99%)
- σ_LT = Standard deviation of daily demand
- LT = Lead time in days

**Reorder Point:** ROP = (Average daily demand × Lead time) + Safety Stock

### 4.3 ABC Analysis

| Category | % of SKUs | % of Inventory Value | Control Level |
|---|---|---|---|
| **A** | ~10–20% | ~70–80% | Tight: frequent review, low safety stock tolerance, senior oversight |
| **B** | ~20–30% | ~15–20% | Moderate: periodic review |
| **C** | ~50–70% | ~5–10% | Light: bulk order; high safety stock relative to value |

### 4.4 Inventory Valuation — IFRS (IAS 2) vs. US GAAP (ASC 330)

| Feature | **IAS 2 (IFRS)** | **ASC 330 (US GAAP)** |
|---|---|---|
| **Permitted cost formulas** | FIFO or weighted average only | FIFO, weighted average, **LIFO** permitted |
| **LIFO** | **Prohibited** (IAS 2.25) | Permitted; LIFO reserve required in notes |
| **Write-down basis** | Lower of cost and **net realizable value** (NRV) (IAS 2.9) | Lower of cost and **net realizable value** (ASC 330-10-35-1B; FASB ASU 2015-11 aligned with IFRS) |
| **Write-down reversal** | **Permitted** if circumstances that caused write-down no longer exist (IAS 2.33) | **Not permitted** (ASC 330-10-35-14) |
| **Abnormal idle facility cost, freight, handling** | Excluded from inventory cost; expensed as incurred (IAS 2.16–2.18) | Excluded from inventory cost (ASC 330-10-30-7) |
| **Borrowing costs** | Excluded unless qualifying asset (IAS 23); most inventory is not a qualifying asset | Generally excluded (ASC 835-20) |

**LIFO → IFRS conversion impact:** US companies reporting under IFRS must eliminate LIFO inventory layers. In periods of rising prices, LIFO carries lower inventory balances than FIFO/weighted average → IFRS conversion typically increases inventory and retained earnings (LIFO reserve unwinding). This is a significant consideration for cross-border M&A and IFRS adoption projects.

### 4.5 Inventory Turnover and DIO

**Inventory Turnover = COGS / Average Inventory**

**DIO = 365 / Inventory Turnover** (or: (Average Inventory / COGS) × 365)

High inventory turnover / low DIO → lean operations; but excessive reduction creates stockout risk. Balance against service level targets.

---

## Part 5 — 13-Week Rolling Cash Flow Forecasting

### 5.1 Direct Method Structure

The 13-week rolling cash flow forecast uses the **direct method** — actual receipts and disbursements:

```
Operating Receipts
  + Customer collections (based on AR aging + payment terms)
  + Other operating receipts (government grants, insurance proceeds)
= Total Operating Receipts

Operating Disbursements
  - Payroll and benefits
  - Supplier payments (based on AP aging + DPO)
  - Rent and occupancy
  - Utilities
  - Tax payments (estimated quarterly payments, VAT/GST settlements)
  - Debt service (interest and scheduled principal)
  - Capital expenditures (discretionary vs. committed)
= Total Operating Disbursements

Net Operating Cash Flow (before financing)

Financing Activities
  + Revolving credit facility draws
  - Revolving credit facility repayments
  + Other financing inflows
= Net Cash Change

Opening Cash Balance
+ Net Cash Change
= Closing Cash Balance
```

### 5.2 IAS 7 — Cash Flow Statement Presentation

| Item | IAS 7 Classification Options | ASC 230 (US GAAP) |
|---|---|---|
| **Interest paid** | Operating **or** Financing (IAS 7.31–33) | **Operating only** (ASC 230-10-45-17) |
| **Interest received** | Operating **or** Investing (IAS 7.31–33) | **Operating only** (ASC 230-10-45-16) |
| **Dividends paid** | Operating **or** Financing (IAS 7.34) | **Financing only** (ASC 230-10-45-15) |
| **Dividends received** | Operating **or** Investing (IAS 7.34) | **Operating only** (ASC 230-10-45-16) |
| **Direct method** | **Encouraged** (IAS 7.19) | Permitted (ASC 230-10-45-25) |
| **Indirect method** | Permitted (IAS 7.18) | **Most common** in practice |

**IAS 7.44–44D — Supplier Finance Arrangement Disclosures (2024 amendments):** Entities must provide a reconciliation showing movements in supplier finance arrangement balances, to help users understand the impact on liquidity and cash flows.

### 5.3 Common Forecasting Failure Modes

| Failure Mode | Description | Remedy |
|---|---|---|
| **AR timing mismatch** | Assuming payment on terms rather than actual payment behavior | Use actual payment lag from AR aging analysis |
| **Seasonal demand ignored** | Uniform daily run rate applied to seasonal businesses | Model seasonal peaks and troughs explicitly |
| **Tax payment timing** | Missing estimated tax, VAT/GST, payroll tax due dates | Calendar-driven tax schedule overlay |
| **RCF draw/repayment** | Revolving credit facility mechanics not modeled | Include borrowing base availability and covenant headroom |
| **One-time items excluded** | CapEx, restructuring, M&A payments not included | Separate line item for committed non-recurring items |
| **Over-optimistic collections** | DSO improvement assumed too quickly | Conservative collection lag; sensitivity scenario |

---

## Part 6 — Working Capital Financing Structures

### 6.1 Asset-Based Lending (ABL)

**Structure:** Revolving credit facility secured by eligible assets — typically accounts receivable and inventory. Borrowing availability is driven by the borrowing base.

**Typical Borrowing Base:**
```
Eligible Receivables × Advance Rate (typically 80–85%)
+ Eligible Inventory × Advance Rate (typically 50–65% for raw materials/finished goods)
= Borrowing Base
Less: Reserves (dilution reserve, ineligibility reserves)
= Net Borrowing Availability
```

**Eligible Receivable criteria (typical):**
- Not more than 90 days past invoice date (some lenders: 120 days)
- Not more than 60 days past due
- Not a contra account (customer is also a supplier)
- Not a government receivable (unless included by election)
- Not concentration-limited (typically: single debtor > 25% of eligible AR is ineligible above threshold)
- Not foreign (unless foreign AR borrowing base included)
- No valid counterclaims or offsets

**Inventory ineligibility:**
- Work-in-process (WIP) — typically excluded
- Slow-moving or obsolete inventory (>12 months)
- Consigned inventory not owned by borrower
- Inventory not in borrower's possession

### 6.2 Receivables Financing

| Structure | Funder | On/Off Balance Sheet | Accounting Driver |
|---|---|---|---|
| **Factoring (non-recourse)** | Factor (bank or specialized) | Off-balance-sheet if ASC 860 / IFRS 9 derecognition met | True-sale legal opinion; isolation of assets |
| **Receivables securitization** | ABS investors via SPV | Off-balance-sheet if SPV not consolidated (IFRS 10 / ASC 810); receivables derecognized per ASC 860 / IFRS 9 §3.2 | Control/consolidation analysis of SPV |
| **Pledged receivables (ABL)** | Bank (ABL lender) | On-balance-sheet — pledge, not transfer; borrowing base facility | No derecognition; collateral pledge |
| **Invoice discounting** | Bank or specialty lender | On-balance-sheet — seller retains collections; control retained | No derecognition |

### 6.3 Supply Chain Finance Platforms

| Platform | Parent / Type | Key Feature |
|---|---|---|
| **Taulia** | SAP | Deep ERP integration (SAP); dynamic discounting and reverse factoring |
| **PrimeRevenue** | Independent | Global SCF network; multi-bank; strong in manufacturing |
| **C2FO** | Independent | Dynamic discounting marketplace; early pay auction model |
| **Kyriba** | Independent | TMS with integrated SCF; strong analytics |
| **JPMorgan ACCESS** | JPMorgan | Bank-led; strong for large corporates with JPM banking |

**Key diligence questions for SCF platform selection:**
1. Does the program require novation of the payable (creating reclassification risk under IAS 7)?
2. Are payment terms extended beyond original invoice terms (potential financing liability classification)?
3. What is the concentration of funder banks (single-bank programs create refinancing risk)?
4. Does the platform support multi-currency? APAC currencies?
5. What are the disclosure requirements for your reporting standard (IAS 7.44A; ASU 2022-04)?

### 6.4 Trade Finance Instruments

| Instrument | Description | Governing Standard | Key Use Case |
|---|---|---|---|
| **Letter of Credit (LC)** | Bank guarantee of payment upon presentation of conforming documents | UCP 600 (ICC) | Cross-border trade with new / high-risk counterparties |
| **Documentary Collection (D/C)** | Bank handles document transit but does not guarantee payment | URC 522 (ICC) | Established trade relationships; lower cost than LC |
| **Bank Guarantee / SBLC** | Conditional payment undertaking from bank | ISP98 (for SBLC) | Performance bonds; bid bonds |
| **Supply Chain Finance (reverse factoring)** | Buyer-led early payment arrangement | IAS 7.44A; ASU 2022-04 | Extend DPO while preserving supplier access to early payment |
| **Forfaiting** | Non-recourse purchase of medium-term receivables (typically 6 months–7 years) | ITFA standard terms | Capital goods exporters; medium-term receivables |

---

## Part 7 — Working Capital Release Program Design

### 7.1 Program Design Framework

A working capital release program typically follows this structure:

1. **Diagnostic phase:** Baseline current CCC by component (DSO, DPO, DIO). Segment by business unit, geography, customer/supplier tier. Identify top opportunities.
2. **Target setting:** Benchmark against industry best-in-class. Set 12-month and 24-month improvement targets by component.
3. **Initiative design:** Select levers per component (see Part 1.3). Sequence by speed of impact and implementation effort.
4. **Governance:** Assign working capital KPIs to functional owners (AR → Credit/Collections; AP → Procurement/Treasury; Inventory → Supply Chain). Weekly/monthly tracking.
5. **Financing layer:** If balance sheet improvement is needed faster than operational improvement allows, layer in ABL or SCF program as a bridge.
6. **Reporting:** Cash conversion cycle metrics in management accounts; IAS 7 / ASC 230 cash flow statement provides external reporting anchor.

### 7.2 Working Capital Release Estimation

| Initiative | Estimated Cash Release Formula | Example |
|---|---|---|
| DSO reduction | (DSO reduction in days × Annual revenue) / 365 | 5-day DSO reduction × $500M revenue / 365 = **$6.8M** |
| DPO extension | (DPO extension in days × Annual COGS) / 365 | 10-day DPO extension × $350M COGS / 365 = **$9.6M** |
| DIO reduction | (DIO reduction in days × Annual COGS) / 365 | 7-day DIO reduction × $350M COGS / 365 = **$6.7M** |

---

## Part 8 — APAC-Specific Working Capital Considerations

| Jurisdiction | Key Working Capital Consideration | Regulatory Reference |
|---|---|---|
| **China** | RMB-denominated receivables: conversion to USD requires SAFE approval. Cross-border factoring: SAFE registration. E-invoicing (Fapiao): required for VAT deduction; affects AR aging compliance. | SAFE Circular 19; VAT Law (2024 update) |
| **Japan** | Subcontracting Act (Shitauke-ho): mandates payment to subcontractors within 60 days of goods/service receipt; penalties for late payment. Trade bill (tegata) usage declining; electronic transfer systems (でんさい/densai) replacing. | Subcontracting Act (Act No. 120 of 1956); Densai regulations |
| **India** | MSMED Act 2006: payment to Micro/Small Enterprises within 45 days; buyers must report outstanding amounts to Ministry quarterly (Form-1). GST input tax credit timing affects AP optimization. | MSMED Act 2006 §15–16; GST Act 2017 |
| **Australia** | Payment Times Reporting Act 2020: large businesses (>$100M revenue) must report their small business payment times twice yearly. ATO-approved early payment discount programs. | Payment Times Reporting Act 2020 |
| **Singapore** | MAS guidelines on supply chain financing; SGX disclosure requirements for listed companies on trade receivables. | MAS Notice; SGX Listing Rules |

---

## Part 9 — Official Documentation URLs

| Standard / Regulation | URL | Access |
|---|---|---|
| **IAS 7** (Statement of Cash Flows — incl. 2024 SCF amendments) | [ifrs.org/issued-standards/list-of-standards/ias-7-statement-of-cash-flows/](https://www.ifrs.org/issued-standards/list-of-standards/ias-7-statement-of-cash-flows/) | Free with registration |
| **IFRS 9** (Financial Instruments — derecognition §3.2) | [ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/](https://www.ifrs.org/issued-standards/list-of-standards/ifrs-9-financial-instruments/) | Free with registration |
| **IAS 2** (Inventories) | [ifrs.org/issued-standards/list-of-standards/ias-2-inventories/](https://www.ifrs.org/issued-standards/list-of-standards/ias-2-inventories/) | Free with registration |
| **IFRS 10** (Consolidated Financial Statements — SPE consolidation) | [ifrs.org/issued-standards/list-of-standards/ifrs-10-consolidated-financial-statements/](https://www.ifrs.org/issued-standards/list-of-standards/ifrs-10-consolidated-financial-statements/) | Free with registration |
| **IASB Supplier Finance Amendments (IAS 7 / IFRS 7, January 2023)** | [iasb.org/news-and-events/news/2023/january/iasb-amends-ias-7-and-ifrs-7-supplier-finance-arrangements/](https://www.iasb.org/news-and-events/news/2023/january/iasb-amends-ias-7-and-ifrs-7-supplier-finance-arrangements/) | Fully public |
| **ASC 860** (Transfers and Servicing) | [asc.fasb.org/860](https://asc.fasb.org/860) | Free with registration |
| **ASC 330** (Inventory) | [asc.fasb.org/330](https://asc.fasb.org/330) | Free with registration |
| **ASC 230** (Statement of Cash Flows) | [asc.fasb.org/230](https://asc.fasb.org/230) | Free with registration |
| **ASU 2022-04** (Supplier Finance Program Disclosures) | [storage.fasb.org/ASU%202022-04.pdf](https://storage.fasb.org/ASU%202022-04.pdf) | Fully public |
| **ASU 2016-13 / ASC 326** (CECL — credit losses on AR) | [asc.fasb.org/326](https://asc.fasb.org/326) | Free with registration |
| **EU Late Payment Directive** (2011/7/EU) | [eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32011L0007](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32011L0007) | Fully public |
| **UCP 600** (Letters of Credit — ICC) | [iccwbo.org/resources-for-business/incoterms-rules/ucp-600/](https://iccwbo.org/resources-for-business/incoterms-rules/ucp-600/) | ICC member access; summaries public |
| **Japan Subcontracting Act** | [jftc.go.jp/en/legislation_gls/antimonopoly_law/subcontract/index.html](https://www.jftc.go.jp/en/legislation_gls/antimonopoly_law/subcontract/index.html) | Fully public |
| **India MSMED Act 2006** | [msme.gov.in/sites/default/files/MSMED-Act2006.pdf](https://msme.gov.in/sites/default/files/MSMED-Act2006.pdf) | Fully public |
| **Australia Payment Times Reporting Act 2020** | [legislation.gov.au/Details/C2020A00100](https://www.legislation.gov.au/Details/C2020A00100) | Fully public |

---

## Mandatory Advisory Note

This analysis is advisory and based solely on the facts described. Working capital financing structures, receivables derecognition eligibility, and supply chain finance classification involve fact-specific legal and accounting judgments that vary by jurisdiction and entity type. Verify accounting conclusions with your qualified external auditors, financing structures with legal counsel, and benchmark data with current industry sources before making operational or financing decisions. This skill does not constitute investment advice, financial advice, or a commitment to arrange financing, and does not form a financial-advisor or investment-advisor relationship.
