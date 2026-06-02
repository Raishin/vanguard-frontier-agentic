---
name: business-combinations-advisor
description: Multi-jurisdiction business combinations reference framework covering acquisition accounting, purchase price allocation, goodwill, and post-combination integration under ASC 805 and IFRS 3.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Business Combinations Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for business combinations advisory — from acquirer identification and acquisition date determination through purchase price allocation, goodwill measurement, NCI, deferred tax in PPA, post-combination accounting, and common-control transactions.

---

## Part 1: The Acquisition Method — ASC 805 / IFRS 3

### Step 1: Identify the Acquirer

The acquirer is the entity that obtains control of the acquiree (ASC 805-10-55 / IFRS 3.B14–B18).

**Reverse acquisition indicators** (when legal acquiree is the accounting acquirer):
- The former owners of the legal acquiree obtain the majority of the voting rights of the combined entity.
- The composition of the governing body is dominated by former owners of the legal acquiree.
- Management of the legal acquiree comprises the majority of management of the combined entity.
- The combination is effected at a premium over the fair value of the legal acquirer.
- The larger entity is the legal acquiree.

Source: ASC 805-10-55-11 / IFRS 3.B14–B18

### Step 2: Determine the Acquisition Date

The acquisition date is the date on which the acquirer obtains control — generally the closing date when consideration is transferred and the acquiree's assets are received and liabilities assumed (ASC 805-10-25-6 / IFRS 3.9).

### Step 3: Recognise and Measure Identifiable Assets and Liabilities

At the acquisition date, recognise:
- **All identifiable assets acquired and liabilities assumed** that meet the Framework definitions, even if not recognised by the acquiree (e.g., internally developed intangibles).
- **At fair value** as of the acquisition date, except for specific exceptions (e.g., deferred tax — IAS 12/ASC 740; employee benefits — IAS 19/ASC 715; operating leases — IFRS 16/ASC 842).
- **No recognition** of restructuring provisions or future losses of the acquiree as of the acquisition date (ASC 805-20-25-1 / IFRS 3.11).

---

## Part 2: Consideration Transferred and PPA

### Consideration Components

| Component | Measurement | Key Notes |
|---|---|---|
| Cash | Face value | Straightforward |
| Equity issued | Fair value of shares at **acquisition date** market price | Not at announcement date |
| Contingent consideration | **Fair value at acquisition date** | Subsequent changes through P&L (not goodwill) under ASC 805-30-35 / IFRS 3.58 |
| Replacement awards (share-based) | Portion attributable to pre-combination service = purchase price; portion for post-combination service = compensation cost | ASC 805-30-25-3 / IFRS 3.B55–B62 |

### Step Acquisitions (Business Combination Achieved in Stages)

1. Remeasure the previously held equity interest in the acquiree to **fair value at the acquisition date**.
2. Recognise the resulting gain or loss in P&L.
3. Include that remeasured fair value in the consideration transferred to calculate goodwill.

Source: ASC 805-10-25-10 / IFRS 3.42

### Contingent Consideration — Post-Acquisition Remeasurement

- Equity-classified contingent consideration: **Not remeasured** (settled in own shares, classified as equity).
- Liability-classified contingent consideration: **Remeasured at fair value each reporting date; change goes through P&L**.
- Common error: routing post-acquisition changes in contingent consideration to goodwill (prohibited under both ASC 805 and IFRS 3).

---

## Part 3: Identifiable Intangibles

### Recognition Criteria (ASC 805-20 / IFRS 3.B31–B40)

Recognise separately from goodwill if the asset meets either:
- **Separability criterion**: can be separated from the entity and sold, transferred, licensed, rented, or exchanged.
- **Contractual-legal criterion**: arises from contractual or other legal rights, regardless of separability.

### Common Intangibles Recognised in PPA

| Category | Examples | Typical Valuation Method |
|---|---|---|
| Customer-related | Customer lists, customer relationships, order backlog | Multi-period excess earnings (MPEEM) |
| Technology-based | Developed technology, patents, databases | Relief from royalty; cost approach |
| Marketing-related | Trade names, trademarks, internet domain names | Relief from royalty |
| Contract-based | Licensing agreements, non-compete agreements, franchise agreements | Incremental cash flow approach |
| Artistic-related | Copyrights, literary works | Relief from royalty |

### In-Process Research & Development (IPR&D)

| Standard | Treatment |
|---|---|
| ASC 805 (US GAAP) | Capitalise as an indefinite-lived intangible asset at acquisition date; do not amortise until project complete or abandoned; test for impairment annually (ASC 805-20-25-9) |
| IFRS 3 | Recognise at fair value if it meets identifiability criteria; capitalise if criteria in IAS 38.57 are met (technical feasibility, intention to complete, ability to use/sell, probable future economic benefits, adequate resources); expense if criteria not met |

---

## Part 4: Goodwill

### Goodwill Calculation

```
Goodwill = Consideration Transferred
         + Fair Value of NCI
         + Fair Value of Previously Held Interest (if step acquisition)
         − Fair Value of Net Identifiable Assets Acquired
```

### Full Goodwill vs. Partial Goodwill (IFRS 3.19)

| Method | US GAAP | IFRS 3 |
|---|---|---|
| Full goodwill | **Required**: NCI measured at FV → goodwill includes NCI's share | **Choice** (election per transaction): NCI at FV |
| Partial goodwill | Not permitted | **Choice** (election per transaction): NCI at proportionate share of net identifiable assets → goodwill = only acquirer's share |

**Impact**: Full goodwill increases both goodwill and NCI on the balance sheet. Impairment losses on full-goodwill entities are allocated between the parent and NCI.

### Bargain Purchase (Negative Goodwill)

If consideration + NCI FV + previously held interest < fair value of net identifiable assets:
1. Reassess fair values of all identified assets and liabilities.
2. If a deficit remains after reassessment, recognise a **gain in P&L** at the acquisition date.

Source: ASC 805-30-25-2 / IFRS 3.34–36

### Goodwill Subsequent Accounting

| Standard | Subsequent Measurement |
|---|---|
| US GAAP (ASC 350-20) | **No amortisation**; annual impairment test (or more frequently if triggering events occur); private companies may elect amortisation over 10 years (ASU 2014-02) |
| IFRS (IAS 36) | **No amortisation**; annual impairment test; impairment losses not reversed |
| German HGB | **Amortise** over useful life (default 10 years if not reliably estimable) |
| JGAAP (ASBJ No.21) | **Amortise** over ≤ 20 years on a straight-line basis |
| CAS 20 (China) | **No amortisation**; annual impairment test (similar to IFRS) |
| Ind AS 103 | **No amortisation**; annual impairment (identical to IFRS 3 / IAS 36) |

---

## Part 5: Non-Controlling Interests (NCI)

| Option | Measurement | Goodwill Impact |
|---|---|---|
| Fair value (full goodwill) | NCI FV = NCI % × implied entity value | Full goodwill includes NCI's proportionate share |
| Proportionate share of net identifiable assets | NCI % × FV of net identifiable assets | Partial goodwill = only acquirer's share |

**IFRS 3.19 choice** is made on a transaction-by-transaction basis; an entity may use full goodwill for one acquisition and partial for another.

US GAAP requires full goodwill in all cases (ASC 805-20-30-7).

---

## Part 6: Deferred Tax in Purchase Price Allocation

**No initial recognition exception in business combinations** (ASC 740-10-25-16 / IAS 12.15 exception does not apply to business combinations).

### Key Steps

1. **Identify temporary differences** created by fair value step-ups (e.g., intangibles stepped up above tax basis).
2. **Recognise deferred tax liability** on the fair value excess over tax basis of acquired assets.
3. **Gross up the intangible**: The DTA/DTL from the step-up changes the fair value allocation and goodwill.
   - Example: Intangible FV = $100, tax basis = $0. DTL at 25% = $25. Intangible reported = $100; DTL = $25; goodwill increases by $25 (to absorb the DTL).
4. **No iterative calculation required** under ASC 740; under IAS 12, the gross-up creates a circular calculation that must be resolved directly.

Source: ASC 740-10-25-16; IAS 12.66

---

## Part 7: Post-Combination Accounting

### Integration Costs

Acquisition-related costs (advisory fees, legal fees, due diligence) are **expensed as incurred** — not added to consideration transferred (ASC 805-10-25-23 / IFRS 3.53).

Restructuring costs expected post-acquisition are **expensed as incurred** in post-combination periods. They are not recognised as a liability at acquisition date unless the acquiree had an existing obligation at the acquisition date.

### Pre-Existing Relationships

If the acquirer and acquiree had a pre-existing relationship (e.g., supplier contract, licensing arrangement):
- Settle the relationship separately from the business combination.
- Gain/loss on settlement recognised in P&L.
- Do not include in consideration transferred.

Source: ASC 805-10-25-20 / IFRS 3.B50–B53

### Indemnification Assets

If the seller contractually indemnifies the acquirer for a specific uncertainty (e.g., contingent liability): Recognise an indemnification asset at the same amount as the indemnified item. Measure on the same basis as the indemnified liability (ASC 805-20-25-27 / IFRS 3.27–28).

### Measurement Period Adjustments

**Measurement period**: Up to 12 months after the acquisition date to finalise the PPA (ASC 805-10-25-13 / IFRS 3.45).

- **Provisional PPA**: Recognise provisional amounts where fair values are not yet determined.
- **Adjustments**: Retrospectively adjust provisional amounts; restate comparatives.
- **After measurement period closes**: Changes are recognised prospectively in P&L (not retrospective).

**M&A closes mid-quarter-close — adversarial scenario**:
- Flag new entity in consolidation system immediately on acquisition date.
- Apply provisional PPA; do not delay consolidation.
- Restate prior-period comparatives when final PPA adjustments are made within the measurement period.
- Deferred tax on provisional intangibles: use best estimate; adjust when final PPA received.

---

## Part 8: Common Control Transactions

| Framework | Treatment |
|---|---|
| US GAAP (ASC 805-50) | **Predecessor / historical cost basis** — no fair value step-up; financial statements restated for all periods presented as if combination occurred at earliest period shown |
| IFRS | **No specific standard**; entity applies a policy choice (historical cost or acquisition method); IFRIC agenda decision (2019) confirmed acquisition method is permitted but not required |
| German HGB | Pooling of interests (Interessenzusammenführung) abolished; Erwerbsmethode (acquisition method) required for all business combinations including some common control |
| JGAAP (ASBJ No.21) | Common control transactions: carrying amount (historical cost) basis |
| CAS 20 | Common control: carrying amount basis; non-common control: acquisition method |
| Ind AS 103 | No guidance on common control transactions; entities typically apply predecessor basis |

---

## Part 9: Joint Ventures and Joint Operations

| Type | US GAAP | IFRS |
|---|---|---|
| Joint venture | ASC 323: **equity method** | IFRS 11: **mandatory equity method** (proportionate consolidation not permitted for JVs) |
| Joint operation | Proportionate share of assets/liabilities/revenue/expenses | IFRS 11: proportionate share (same) |
| Key standard | ASC 323-10 / ASC 808 | IFRS 11.24–26 |

---

## Part 10: Multi-Jurisdiction Summary

| Jurisdiction | Standard | Key Differences vs. IFRS 3 |
|---|---|---|
| US GAAP | ASC 805/350/720 | Full goodwill required; IPR&D capitalised; no goodwill amortisation; acquisition costs expensed |
| IFRS | IFRS 3/IAS 27/28/36 | Full or partial goodwill choice; IPR&D may be expensed; no amortisation |
| German HGB | HGB §§ 301–309 | Goodwill amortised (≤10 years default); simpler intangible identification rules |
| JGAAP | ASBJ No.21 | Goodwill amortised (≤20 years straight-line); pooling abolished 2008 |
| China CAS 20 | CAS 20 | Common control: carrying amount; non-common: acquisition method; no goodwill amortisation |
| Ind AS 103 | Ind AS 103 | Identical to IFRS 3; no amortisation of goodwill |

---

## Part 11: Official Documentation — Publicly Accessible URLs

| Standard | URL | Access |
|---|---|---|
| ASC 805 (FASB) | https://asc.fasb.org/805 | Public |
| IFRS 3 (IASB) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs3.html | Fully public |
| IAS 27 (IASB) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias27.html | Fully public |
| IAS 28 (IASB) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias28.html | Fully public |
| IAS 36 (Impairment) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias36.html | Fully public |
| IAS 12 (Deferred Tax) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias12.html | Fully public |
| IFRS 11 (Joint Arrangements) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs11.html | Fully public |
| German HGB | https://www.gesetze-im-internet.de/hgb/ | Fully public (German) |
| JGAAP ASBJ No.21 | https://www.asb.or.jp/en/accounting_standards/accounting_standards/ | Fully public |
| China CAS 20 | https://www.mof.gov.cn/ | Partially public (Chinese) |
| Ind AS 103 | https://www.icai.org/post/indian-accounting-standards | Fully public |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the transaction profile and facts described above. Business combinations accounting involves complex judgements about fair value, control, and tax that vary by jurisdiction and transaction structure. This analysis does not constitute a formal purchase price allocation report, fairness opinion, or valuation conclusion for any regulatory or transactional purpose. All conclusions require verification with qualified external auditors, valuation specialists, and legal advisors before relying on this analysis for any compliance, financial reporting, or transactional purpose.
