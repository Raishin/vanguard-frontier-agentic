---
name: procure-to-pay-advisor
description: Multi-jurisdiction procure-to-pay accounting reference covering PO matching, AP accruals, vendor management, and related compliance.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Procure-to-Pay Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for procure-to-pay (P2P) accounting advisory — from purchase order matching through AP accruals, vendor management, VAT/GST input credit recovery, and procurement fraud controls.

---

## Part 1: PO Matching — 2-Way, 3-Way, and 4-Way

### Matching Types

| Match Type | Documents Compared | What Is Verified |
|---|---|---|
| **2-way match** | PO vs. supplier invoice | Price, quantity, terms |
| **3-way match** | PO vs. goods receipt (GRN) vs. supplier invoice | Price, quantity, and physical receipt confirmed |
| **4-way match** | PO vs. GRN vs. inspection record vs. supplier invoice | Price, quantity, receipt, and quality/inspection sign-off |

**Best practice**: 3-way matching is the minimum standard for goods purchases. 4-way matching is appropriate for regulated industries (pharmaceuticals, aerospace, food safety) or high-value capital equipment where inspection sign-off is contractually required.

### Purchase Price Variance (PPV)

PPV arises when the actual invoiced price differs from the standard or PO price:

```
PPV = (Actual Price − Standard/PO Price) × Quantity Received
```

- **Favorable PPV**: actual price < PO price → credit to PPV account.
- **Unfavorable PPV**: actual price > PO price → debit to PPV account.

**Accounting treatment by jurisdiction:**

| Jurisdiction | Standard | PPV Treatment |
|---|---|---|
| US GAAP | ASC 330-10 (Inventory) | Capitalize into inventory cost or expense if immaterial; significant variances may require allocation to COGS and ending inventory |
| IFRS | IAS 2.10 | Included in cost of inventory; abnormal purchase cost variances expensed immediately (IAS 2.16) |
| German HGB | §255 HGB (Herstellungskosten) | Cost of purchase (Anschaffungskosten) — invoiced price + ancillary costs; price variances absorbed into inventory at actual cost |
| JGAAP | ASBJ Statement No. 9 | Actual cost or standard cost (with variance adjustment to COGS) |

### Quantity Variance

Quantity variance arises when goods received quantity differs from invoiced quantity:

- **Short delivery**: invoice quantity > GRN quantity → dispute or accrue liability only for quantity received.
- **Over-delivery**: invoice quantity < GRN quantity → accrue full received quantity; issue debit note or request corrected invoice.

**Control**: Quantity variances should be cleared within the period or escalated to the buyer for vendor dispute resolution. Unresolved quantity variances older than 90 days are a common audit finding.

### Tolerance Policies

Typical industry tolerance bands for PO matching:

| Type | Common Tolerance |
|---|---|
| Price tolerance | ±1–3% of PO unit price |
| Quantity tolerance | ±2–5% of PO quantity |
| Amount tolerance | ±$500 or ±2% of invoice total (whichever is lower) |

Tolerances exceeding these bands typically require buyer/controller approval before payment release.

---

## Part 2: AP Accruals — GRNI and Period-End Cutoff

### Goods Received Not Invoiced (GRNI) Accrual

At period end, goods may be received but the supplier invoice not yet processed. GAAP requires accrual of the liability:

**Journal at period end (accrual):**
```
DR  Inventory / Expense (at PO price or estimated cost)
  CR  GRNI Accrual (Accrued Liabilities)
```

**Journal in subsequent period (reversal):**
```
DR  GRNI Accrual
  CR  Inventory / Expense
```
Then the actual invoice is processed normally through AP.

### GRNI Accrual — Multi-Jurisdiction Standard Basis

| Jurisdiction | Standard | Accrual Basis |
|---|---|---|
| US GAAP | ASC 420, ASC 450-20-25 | Probable and reasonably estimable obligation; accrue when obligation incurred (receipt of goods transfers risk/reward) |
| IFRS | IAS 37.14 | Present obligation as result of past event (receipt); probable outflow; reliably estimable → recognize provision/accrual |
| German HGB | §249 Abs. 1 HGB | Rückstellungen (provisions) must be recognized for uncertain liabilities — HGB is more prudent: recognize even when probability < 50% in some interpretations; Imparitätsprinzip (imparity principle) requires recognition of risks even before they are fully crystallized |
| JGAAP | ASBJ Conceptual Framework | Accrual basis broadly consistent with IFRS; no formal IAS 37 equivalent but practice aligns |
| India Ind AS | Ind AS 37 (= IAS 37) | Same as IFRS (IAS 37.14) |
| China CAS | CAS No. 13 (Contingencies) | Broadly consistent with IAS 37 |

**Key HGB divergence**: Under §249 HGB, provisions for uncertain liabilities (ungewisse Verbindlichkeiten) must be recognized as soon as the obligation is probable — the HGB Imparitätsprinzip means losses and obligations are recognized earlier than under IFRS/US GAAP in some cases. GRNI at HGB balance sheet date must be provisioned regardless of materiality threshold.

### AP Cutoff Procedures

Period-end AP cutoff checklist:

- [ ] All GRNs dated on or before the last day of the period are matched against invoices or accrued as GRNI.
- [ ] All invoices dated on or before the period end are posted to AP, even if payment terms extend beyond the period.
- [ ] Purchase orders with partial deliveries: accrue only for the received portion.
- [ ] Service accruals: accrue for services performed to period end per contract milestones even without an invoice.
- [ ] Credit notes and debit notes: match and post within the period they relate to.
- [ ] Prepayments: reclassify advance payments from AP to prepaid asset (ASC 340 / IAS 38).

---

## Part 3: Accounts Payable Accounting

### Invoice Validation

Standard AP invoice validation steps:

1. Three-way match confirmation (PO, GRN, invoice).
2. Duplicate invoice check (same vendor, same invoice number, same amount — any two of three match = hold for review).
3. Vendor master verification (correct bank details, active status, no debarment flags).
4. VAT/GST validity (invoice number format, supplier tax registration, correct rate applied).
5. Currency and payment terms confirmation.
6. Approval routing per delegation of authority matrix.

### Early Payment Discounts — Net Method vs. Gross Method

**Gross method (default US practice):**
- Record payable at full invoice amount.
- Discount taken credited to "Purchase Discounts" income at payment date.
- Standard: ASC 310-10-35-2 (gross recording is acceptable).

**Net method:**
- Record payable net of expected discount (e.g., 2/10 net 30 → record at 98%).
- If discount not taken, debit "Discounts Lost" (financing cost).
- Standard: ASC 310-10-35-3 (net method preferred by some theorists as it reflects economic substance).

**IFRS approach (IFRS 9 / IAS 39):**
- Trade payables at amortised cost (IFRS 9.4.2.1).
- For short-term payables (< 1 year), discount effect is typically immaterial → record at transaction price.
- Early payment discounts reduce the carrying amount of the payable; discounts not taken increase interest cost.
- Source: IFRS 9.B5.1.1 — short-term receivables/payables at invoice amount if effect of discounting is immaterial.

**German HGB (§253 HGB):**
- Record at settlement amount (Erfüllungsbetrag).
- Skonto (cash discount) deducted from cost of asset/inventory if taken.

### Dynamic Discounting and Supply Chain Financing

**Dynamic discounting**: buyer offers early payment at variable discount rates calculated on a sliding scale tied to days-early. The earlier the payment, the higher the discount captured. These programs are typically funded from the buyer's own cash — no reclassification from AP required.

**Supply chain financing / reverse factoring**: A bank or financial intermediary pays the supplier early; the buyer repays the bank at the original invoice maturity (or extended terms). This can alter the classification of the payable:

**IFRS IC Agenda Decision (November 2020) — key conclusions:**
- If the terms of the arrangement are substantially different from normal trade payables (e.g., extended payment terms beyond normal commercial terms, bank acts as principal rather than agent), the liability should be reclassified from **trade payables** to **borrowings** on the balance sheet.
- "Substantially modified" indicators: extended payment terms compared to peer entities; arrangement is contingent on the buyer's relationship with the bank rather than the supplier relationship; the supplier's claim is discharged upon bank payment (novation).
- Source: IFRS IC Update November 2020 — https://www.ifrs.org/news-and-events/updates/ifric/2020/ifric-update-november-2020/

**US GAAP (ASC 470-10-45):**
- No specific standard equivalent to the IFRS IC decision.
- Indicators that a supply chain finance arrangement constitutes a financing arrangement (and should be classified as short-term debt rather than AP):
  - Extended payment terms significantly beyond industry norms.
  - Buyer negotiated the arrangement with the bank directly.
  - Buyer provides payment guarantee to the bank.
  - Terms are linked to the buyer's credit standing, not the supplier's.
- Source: ASC 470-10-45; ASU 2022-04 (Supplier Finance Program disclosures, effective for fiscal years beginning after December 15, 2022).

**Disclosure — ASU 2022-04 (US GAAP)**:
Entities must disclose: key terms of supplier finance programs, outstanding amount at period end, balance sheet line where the liability is presented, and rollforward of activity.

---

## Part 4: Vendor Management

### Vendor Master Controls

Critical controls over the vendor master file:

| Control Area | Risk Mitigated | Key Control |
|---|---|---|
| New vendor creation | Ghost vendor / fictitious supplier | Dual approval for new vendor setup; independent verification of bank details |
| Bank account changes | Payment diversion fraud | Segregated approval (cannot be changed by AP processor); callback verification to vendor using number on file — not number provided in change request |
| Duplicate vendor detection | Duplicate payment | System-level duplicate check (same name + TIN; same address + bank account) |
| Vendor deactivation | Payments to unapproved vendors | Periodic review and deactivation of vendors with no activity in prior 12 months |
| Vendor master reconciliation | Unauthorized changes | Monthly comparison of vendor master change log against approved change requests |

### 1099/1042-S (United States)

**Form 1099-MISC / 1099-NEC**: Required for US persons receiving $600+ in a calendar year for services (non-employees), rent, prizes, or other income. Filed with IRS by January 31 of following year. Backup withholding (24%) required if vendor fails to provide TIN (Form W-9).

**Form 1042-S**: Required for payments to foreign persons (non-US tax residents) subject to US withholding tax. Chapter 3 (portfolio interest, dividends, royalties) and Chapter 4 (FATCA) withholding rules apply. Filed by March 15 of following year.

**Source**: IRS Publication 15-B; IRS Instructions for Forms 1099, 1042-S.

### GDPR — Vendor Data Retention

Under EU GDPR Article 17 (Right to Erasure) and Article 5(1)(e) (Storage Limitation):
- Vendor data (contact persons, personal email addresses) should not be retained beyond the period necessary for the commercial relationship plus applicable statutory limitation period.
- Financial records (invoices, payment confirmations) must be retained for statutory periods (e.g., 10 years in Germany — §257 HGB; 7 years in the UK — Companies Act 2006; 7 years in the US — IRS requirements).
- Vendor personal data in the vendor master should be anonymized or deleted once the retention period expires, while retaining the transaction records themselves.

---

## Part 5: Expense Accruals, Prepaid Assets, and Purchase Commitments

### Prepaid Assets

Payments made in advance of the period to which they relate must be classified as prepaid assets:

**US GAAP**: ASC 340-10 — prepaid expenses recognized as assets; amortized on a straight-line basis over the benefit period.

**IFRS**: IAS 38 (intangibles) and IAS 1 (presentation) — advance payments classified as current assets; amortized as the service/benefit is consumed.

**Common examples**: prepaid insurance, prepaid maintenance contracts, prepaid software licenses, prepaid rent.

**German HGB (§250 HGB)**: Rechnungsabgrenzungsposten (RAP) — mandatory prepayment recognition on both asset and liability sides; no materiality threshold exemption.

### Accrued Liabilities vs. Accounts Payable

| Item | Accounts Payable | Accrued Liabilities |
|---|---|---|
| Definition | Obligation from a received invoice for goods/services already delivered | Obligation for goods/services received but not yet invoiced |
| Basis | Invoice in hand | Estimate based on contractual terms or best estimate |
| Balance sheet presentation | Current liabilities — trade payables | Current liabilities — accrued expenses |
| US GAAP standard | ASC 210-10-S99 | ASC 450-20 (loss contingencies) / ASC 420 |
| IFRS standard | IAS 1.54(k) — trade and other payables | IAS 37.11 — accruals (distinguished from provisions: accruals = more certain; provisions = uncertain timing/amount) |

### Purchase Commitments

**US GAAP (ASC 440-10-50)**:
- Non-cancellable purchase commitments should be disclosed in the notes if material.
- If the purchase price under the commitment exceeds the current market price (onerous commitment), a loss should be recognized immediately for the excess: ASC 330-10-35-17 (for inventory commitments) and ASC 440-10-25-1.

**IFRS (IAS 37.66–37.69)**:
- An onerous contract (one in which unavoidable costs of meeting the obligation exceed economic benefits expected) requires immediate provision for the least net cost of exiting the contract.
- Purchase commitments that are onerous must be provisioned; the provision = minimum of (cost of fulfilling - expected benefit) vs. (penalty for cancellation): IAS 37.68.

---

## Part 6: VAT/GST on Purchases

### Input Tax Credit Recovery

VAT/GST paid on business purchases can generally be recovered as input tax credit against output VAT/GST collected, subject to the following conditions:

| Condition | Requirement |
|---|---|
| Valid tax invoice | Invoice must contain supplier tax registration number, invoice number, date, VAT/GST amount separately stated |
| Business purpose | Purchase must be used for taxable business activities |
| Not a blocked input | Input tax is not in a blocked category (see below) |

### Blocked Input Tax Categories

| Category | Jurisdiction | Blocked? | Notes |
|---|---|---|---|
| Business entertainment | EU VAT (Directive 2006/112/EC Art. 176) | Yes — member states may block | Most EU states block entertainment input VAT; UK allows partial recovery with strict criteria |
| Passenger vehicles | Most jurisdictions | Yes (full or partial) | Germany: blocked unless exclusively for business use; UK: 50% block for cars used privately; US: luxury auto limits under §179/168 IRC (not VAT, but similar principle) |
| Employee meals | EU | Generally blocked | Unless canteen provided as welfare benefit to all employees |
| Residential property | Most VAT systems | Blocked | VAT on residential accommodation generally irrecoverable |

### Partial Exemption — Mixed-Use Entities

Entities that make both taxable and VAT-exempt supplies (e.g., a bank with both fee-based services and interest income) can only recover a proportion of input VAT:

- **Standard method (EU)**: taxable turnover / total turnover × input VAT = recoverable portion.
- **Special methods**: sector methods, transaction count methods — agreed with tax authority for specific business models.
- **Annual adjustment**: provisional recovery during the year is adjusted to actual taxable use at year end.

**India GST (CGST Act, 2017, Section 17)**:
- Blocked credits listed in Section 17(5): motor vehicles (except for specified purposes), food/beverages, club membership, health services.
- Proportionate credit for mixed-use: Rule 42 and Rule 43 of CGST Rules.
- Fapiao equivalent: valid tax invoice (GST invoice) required with supplier GSTIN.

**China VAT (VAT Reform — Caishui [2016] No. 36)**:
- Input VAT credit requires a valid fapiao (special VAT invoice — 增值税专用发票) issued by the supplier.
- Fapiao must be verified through the Golden Tax System (金税系统) before input credit is claimed.
- Blocked inputs: passenger transportation for private use, catering services, resident daily entertainment, gifts.

---

## Part 7: Procurement Fraud Controls

### Segregation of Duties (SoD) in P2P

A core internal control principle: no single individual should control more than one of the following P2P functions:

| Function | Role | SoD Requirement |
|---|---|---|
| PO creation | Requisitioner / Buyer | Must be separate from approval |
| PO approval | Purchasing Manager / Controller | Must be separate from creation and payment |
| Goods receipt (GRN) | Warehouse / Receiving | Must be separate from PO creation and payment |
| Invoice processing | AP Processor | Must be separate from PO approval and payment |
| Payment approval | Finance Manager / Treasury | Must be separate from invoice processing |
| Vendor master maintenance | AP Master Data | Must be separate from payment processing |

**Key SoD violation patterns to detect:**
- Same individual creates PO and approves payment.
- Same individual creates vendor and processes invoice.
- Same individual receives goods and approves invoice.

### Three-Lines-of-Defence Framework

| Line | Role | P2P Application |
|---|---|---|
| First line | Management / Operations | Procurement policies, PO approval limits, SoD controls in ERP |
| Second line | Risk/Compliance / Finance | Transaction monitoring; vendor master review; AP aging analysis; duplicate payment detection |
| Third line | Internal Audit | Independent testing of P2P controls; SoD conflict review; vendor due diligence audits |

### Vendor Due Diligence

Minimum vendor due diligence for significant new vendors:

- [ ] Business registration verification (company registry check).
- [ ] Sanctions screening (OFAC, EU consolidated list, UN sanctions).
- [ ] Politically Exposed Persons (PEP) check for beneficial owners.
- [ ] Tax registration verification (VAT number, TIN/EIN).
- [ ] Anti-bribery questionnaire for high-risk categories and jurisdictions.
- [ ] Adverse media search.

### FCPA and UK Bribery Act — Procurement Interaction

**US FCPA (Foreign Corrupt Practices Act, 15 U.S.C. §78dd-1 et seq.)**:
- Prohibits payments to foreign government officials to obtain or retain business.
- P2P risk: inflated procurement invoices as a mechanism to generate slush funds; payments to agent/intermediary vendors who are related to government officials.
- Books and records provision: requires accurate books; false entries to disguise bribes are a separate FCPA violation regardless of whether the bribery charge is proven.

**UK Bribery Act 2010 (Section 7 — Failure to Prevent)**:
- Stricter than FCPA: covers commercial bribery (not just public officials) and applies to facilitation payments (which FCPA permits with limited exceptions).
- Section 7: corporate offence of failing to prevent bribery by associated persons — including third-party vendors acting on the company's behalf.
- P2P risk: vendor kickback schemes; procurement agent bribery.
- Defence: adequate procedures (documented procurement controls, training, monitoring) — mapped to Ministry of Justice guidance (six principles).

---

## Part 8: Multi-Jurisdiction Summary Table

| Topic | US GAAP | IFRS | German HGB | JGAAP | India Ind AS / GST | China CAS / VAT |
|---|---|---|---|---|---|---|
| GRNI accrual basis | ASC 450-20-25: probable + estimable | IAS 37.14: present obligation + probable + estimable | §249 HGB: mandatory; Imparitätsprinzip — earlier recognition than IFRS | Accrual basis; aligns with IFRS in practice | Ind AS 37 = IAS 37 | CAS No. 13 ≈ IAS 37 |
| Early payment discount | ASC 310: net or gross method acceptable | IFRS 9.B5.1.1: immaterial if short-term; amortised cost | §253 HGB: settlement amount; Skonto reduces cost | Broadly consistent with IFRS | Ind AS 109 = IFRS 9 | CAS No. 22 ≈ IFRS 9 |
| Supply chain finance reclassification | ASC 470-10-45 + ASU 2022-04 disclosure | IFRS IC Nov 2020: reclassify if substantially modified | No specific guidance; apply substance over form | No specific guidance | Ind AS 109; ICAI guidance | No specific guidance |
| Purchase commitments disclosure | ASC 440-10-50: note disclosure + loss if onerous | IAS 37.66: provision if onerous contract | §285 No. 3a HGB: note disclosure of significant obligations | ASBJ: note disclosure | Ind AS 37.66 = IAS 37 | CAS No. 13 |
| Input VAT blocking — entertainment | §15 Abs. 1a UStG (Germany): generally blocked | EU Directive Art. 176: member state discretion | Blocked under §15 Abs. 1a UStG | Japan Consumption Tax Act: partially deductible | Section 17(5) CGST Act: blocked | Caishui [2016] No. 36: blocked |

---

## Part 9: Official Documentation — Publicly Accessible URLs

| Standard / Resource | URL | Access |
|---|---|---|
| ASC 210 (Balance Sheet) | https://asc.fasb.org/210 | Public (free registration) |
| ASC 310 (Receivables) | https://asc.fasb.org/310 | Public (free registration) |
| ASC 340 (Prepaid / Deferred Costs) | https://asc.fasb.org/340 | Public (free registration) |
| ASC 440 (Commitments) | https://asc.fasb.org/440 | Public (free registration) |
| ASC 450 (Contingencies) | https://asc.fasb.org/450 | Public (free registration) |
| ASC 470 (Debt) | https://asc.fasb.org/470 | Public (free registration) |
| ASU 2022-04 (Supplier Finance Programs) | https://www.fasb.org/page/document?pdf=ASU+2022-04.pdf | Public |
| IAS 37 (Provisions) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias37.html | Public |
| IFRS 9 (Financial Instruments) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs9.html | Public |
| IFRS IC Update Nov 2020 (Supply Chain Finance) | https://www.ifrs.org/news-and-events/updates/ifric/2020/ifric-update-november-2020/ | Public |
| German HGB (§249 Rückstellungen) | https://www.gesetze-im-internet.de/hgb/__249.html | Public (German) |
| German UStG (§15 Input VAT) | https://www.gesetze-im-internet.de/ustg_1980/__15.html | Public (German) |
| India CGST Act 2017 (Section 17 — blocked credits) | https://www.cbic.gov.in/resources//htdocs-cbec/gst/cgst-act.pdf | Public |
| FCPA (15 U.S.C. §78dd-1) | https://www.justice.gov/criminal/criminal-fraud/foreign-corrupt-practices-act | Public |
| UK Bribery Act 2010 | https://www.legislation.gov.uk/ukpga/2010/23/contents | Public |
| UK Bribery Act — Adequate Procedures Guidance | https://www.justice.gov.uk/downloads/legislation/bribery-act-2010-guidance.pdf | Public |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described above. Local statutory, tax, and regulatory requirements vary by jurisdiction and change frequently. This analysis does not constitute authoritative accounting guidance, a compliance opinion, a tax opinion, or a legal opinion in any jurisdiction. Supply chain financing reclassification conclusions under IFRS IC 2020 and ASC 470 require entity-specific assessment with external auditors. Procurement fraud and anti-bribery conclusions require assessment by qualified legal counsel familiar with applicable law. Verify all conclusions with qualified local auditors, tax advisors, and legal counsel before relying on this analysis for compliance purposes. This analysis does not form an accountant-client relationship.
