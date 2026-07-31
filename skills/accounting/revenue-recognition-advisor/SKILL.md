---
name: revenue-recognition-advisor
description: Apply the ASC 606 / IFRS 15 five-step revenue recognition model to described arrangements. Provides the complete five-step framework with paragraph citations, judgment-area reference tables, confidence-scoring guidance, common restatement triggers, GAAP/IFRS delta checklist, and official documentation URLs. Use when analyzing revenue recognition treatment for SaaS, licenses, professional services, multi-element arrangements, and channel partnerships. Advisory only — all outputs require external auditor review for material amounts.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-01"
  category: finance
  lifecycle: experimental
---

# Revenue Recognition Advisor — Reference Skill

## Purpose

Provide the complete analytical framework for applying ASC 606 / IFRS 15 to described revenue arrangements, with specific paragraph citations, judgment-area reference tables, confidence-scoring guidance, and official documentation URLs.

## Official Documentation

| Standard | Source | Access |
|---|---|---|
| FASB ASU 2014-09 Section A (full standard text) | https://www.fasb.org/page/document?pdf=ASU+2014-09_Section+A.pdf | **Fully public PDF** — no account required |
| FASB ASU 2014-09 (storage mirror) | https://storage.fasb.org/ASU%202014-09_Section%20A.pdf | **Fully public PDF** |
| FASB/IASB Comparison: Topic 606 vs. IFRS 15 | https://storage.fasb.org/Comparison%20of%20Topic%20606%20and%20IFRS%2015.pdf | **Fully public PDF** |
| ASC 606 (FASB Codification) | https://asc.fasb.org/606 | Free account required for full text |
| IFRS 15 HTML (2024 issued) | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs15.html | **Fully public** |
| IFRS 15 PDF (2022 issued) | https://www.ifrs.org/content/dam/ifrs/publications/pdf-standards/english/2022/issued/part-a/ifrs-15-revenue-from-contracts-with-customers.pdf?bypass=on | **Fully public PDF** |
| IFRS 15 supporting materials | https://www.ifrs.org/supporting-implementation/supporting-materials-by-ifrs-standards/ifrs-15/ | **Fully public** |
| IAS Plus IFRS 15 summary | https://iasplus.com/en/standards/ifrs/ifrs15 | **Fully public** |
| PCAOB Staff Audit Practice Alert 15 (Revenue) | https://pcaobus.org/Standards/QandA/SAPA-15-revenue-accounting-standard.pdf | **Fully public PDF** |
| PCAOB AS 2810 (evaluating misstatement) | https://pcaob-assets.azureedge.net/pcaob-dev/docs/default-source/rules-standards/standards/auditing/as2810.pdf | **Fully public PDF** |
| AICPA Illustrative ASC 606 Disclosures | https://www.aicpa-cima.com/resources/download/asc-606-illustrative-annual-and-transition-disclosures | **Fully public** |
| SEC EDGAR ASC 606 comment letters (2022–2025) | https://efts.sec.gov/LATEST/search-index?q=%22ASC+606%22&dateRange=custom&startdt=2022-01-01&enddt=2025-12-31&forms=CORRESP | **Fully public** |

## The Five-Step Model

### Step 1 — Identify the Contract(s) with a Customer
**ASC 606-10-25-1 through 606-10-25-8 | IFRS 15.9 through 15.16**

A contract exists when ALL five criteria are met:
1. Parties have approved the contract and are committed to perform
2. Each party's rights can be identified
3. Payment terms can be identified
4. The contract has commercial substance
5. Collection of substantially all consideration is probable

Key judgment: "Probable" under US GAAP = more likely than not (>50%). Under IFRS 15, "probable" = more likely than not — converged.

Contract combination (ASC 606-10-25-9): Combine when entered contemporaneously with the same customer, when negotiated as a package, or when consideration is interdependent.

### Step 2 — Identify the Performance Obligations
**ASC 606-10-25-14 through 606-10-25-22 | IFRS 15.22 through 15.30**

A promised good or service (or bundle) is a performance obligation if it is **distinct**:
- **Capable of being distinct** (ASC 606-10-25-19a): Customer can benefit from it on its own or with readily available resources.
- **Distinct within the context of the contract** (ASC 606-10-25-19b): Not highly interrelated with or dependent on other promises.

**BOTH tests must be met.** Failure of either test → combine with related promises and reassess.

Series provision (ASC 606-10-25-14b / IFRS 15.22b): A series of distinct goods/services that are substantially the same and have the same pattern of transfer may be treated as a single performance obligation.

High-frequency judgment areas:
- SaaS: Is implementation a separate PO from the subscription? Usually fails the "distinct within context" test if heavily customized and the customer cannot benefit from the license without the implementation.
- Professional services: When standalone value exists and customer can engage others for similar services → likely distinct.
- Post-contract support / maintenance: Typically a separate PO (customer can benefit on its own).

### Step 3 — Determine the Transaction Price
**ASC 606-10-32-2 through 606-10-32-27 | IFRS 15.47 through 15.72**

Transaction price = consideration the entity expects to be entitled to in exchange for transferring goods or services. Includes:

**Variable consideration** (ASC 606-10-32-5 through 32-13 | IFRS 15.50 through 15.55):
- Estimate using: Expected value (probability-weighted) OR Most likely amount — whichever better predicts the amount (ASC 606-10-32-8).
- **Variable consideration constraint** (ASC 606-10-32-11 through 32-16): Include variable consideration only to the extent it is probable that a significant reversal in cumulative revenue will NOT occur. Key factors: high sensitivity to external factors, long estimation horizon, broad range of outcomes, entity's experience limited. This is the #1 most-cited area in SEC comment letters (2022–2025 per RevenueHub analysis).

**Significant financing component** (ASC 606-10-32-15 through 32-20): Adjust if payment timing differs significantly from transfer of goods/services. Practical expedient: not required if the financing period is ≤ 12 months.

**Non-cash consideration** (ASC 606-10-32-21 through 32-24): Measure at fair value of consideration received.

**Consideration payable to customer** (ASC 606-10-32-25 through 32-27): Reduce the transaction price unless it is for a distinct good or service from the customer.

### Step 4 — Allocate the Transaction Price to Performance Obligations
**ASC 606-10-32-28 through 606-10-32-44 | IFRS 15.73 through 15.90**

Allocate in proportion to **standalone selling prices (SSPs)** of each performance obligation.

**Determining SSP** (ASC 606-10-32-32 through 32-39):
- Observable: use it directly.
- Not directly observable — estimate using:
  - Adjusted market assessment approach
  - Expected cost plus margin approach
  - Residual approach (only when SSP is highly variable or uncertain and at least one other PO has an observable SSP)

**Allocation of discounts** (ASC 606-10-32-36): Allocate to all POs proportionately unless observable evidence shows the discount relates to specific POs only.

**Variable consideration allocation** (ASC 606-10-32-39 through 32-41): Allocate entirely to a single PO only if the allocation condition and reasonableness condition are both met.

### Step 5 — Recognize Revenue When (or As) a Performance Obligation Is Satisfied
**ASC 606-10-25-23 through 606-10-25-37 | IFRS 15.31 through 15.38**

**Over time** if any one of three criteria met (ASC 606-10-25-27):
1. Customer simultaneously receives and consumes the benefits as the entity performs
2. Entity's performance creates or enhances an asset that the customer controls as it is created/enhanced
3. Entity's performance does not create an asset with an alternative use AND entity has an enforceable right to payment for performance completed to date

If none of the three criteria are met → **Point in time** (ASC 606-10-25-30).

**Point-in-time indicators** (ASC 606-10-25-30): Customer has obtained control — consider: right to payment, legal title transferred, physical possession transferred, risks and rewards transferred, customer has accepted the asset.

**Licenses** (ASC 606-10-55-54 through 55-65 | IFRS 15.B52 through B63):
- **Functional IP** (point in time): Has significant standalone functionality; customer's value not significantly affected by entity's ongoing activities. Software, completed media, patents.
- **Symbolic IP** (over time): Value derived from entity's ongoing activities. Brand licenses, franchise agreements, licenses to evolving IP.

---

## High-Risk Judgment Areas (Fortune-50 Frequency)

| Judgment Area | Key Paragraphs | Common Error | Confidence Signal |
|---|---|---|---|
| Identifying distinct POs in bundled SaaS | ASC 606-10-25-19 | Treating highly interdependent implementation as separate PO → deferring too much revenue | Low: implementation is >30% of contract value and no standalone market |
| Variable consideration constraint | ASC 606-10-32-11 | Including constrained variable consideration prematurely → revenue reversal required | Low: volume rebate >20% of base price, first contract with customer |
| Principal vs. agent (gross vs. net) | ASC 606-10-55-36 through 55-40 | Presenting as principal when agent → gross overstatement of revenue | Medium: entity does not control inventory/service before delivery to customer |
| License type (functional vs. symbolic) | ASC 606-10-55-58 through 55-65 | Treating evergreen brand license as functional → point-in-time recognition → revenue accelerated | Low: entity has ongoing obligations that affect the licensed IP's value |
| Contract modifications | ASC 606-10-25-10 through 25-13 | Treating modification as new contract when it should be a continuation → incorrect cumulative catch-up vs. prospective treatment | Medium: modification adds distinct goods at non-SSP |
| Bill-and-hold | ASC 606-10-55-83 through 55-84 | Recognizing before customer controls the asset | Low: customer has not made formal request; product not separately identified |
| Consignment | ASC 606-10-55-78 through 55-80 | Recognizing revenue at consignment shipment | Low: entity bears risk of loss; customer has right of return without consuming |

---

## GAAP vs. IFRS 15 Key Differences

| Area | ASC 606 | IFRS 15 | Impact |
|---|---|---|---|
| Licenses of IP — sales/usage-based royalties | Recognize as earned, subject to constraint | Same constraint, but allocate based on nature of underlying PO (ASC route vs. IFRS 15.B63A) | May differ when PO is a bundle of license + service |
| Contract modifications — combining contracts | Explicit guidance in ASC 606-10-25-9 | IFRS 15.17 — same criteria, slightly different illustrative examples | Minimal practical difference |
| Collectability threshold | "Probable" = more likely than not | "Probable" = more likely than not (IFRS 15.9e) | Converged |
| Incremental costs of obtaining a contract | Must capitalize if expected to recover (ASC 340-40-25-1) | Same requirement (IFRS 15.91) | Converged; both allow practical expedient for ≤1-year amortization period |
| Disclosure requirements | Extensive (ASC 606-10-50) | Extensive (IFRS 15.110 through 15.129); broadly converged | Multinational entities with both reporting requirements face doubled disclosure burden |

---

## Confidence-Scoring Framework

Apply to each step conclusion before including in the response:

| Score | Definition | Trigger for Mandatory Flag |
|---|---|---|
| High | Conclusion does not change under any reasonable interpretation of the facts as described | None — include without flag |
| Medium | Conclusion holds for the most likely interpretation but alternative interpretations are possible | Flag: "Alternative interpretation exists — confirm with external auditor" |
| Low | Conclusion depends on a fact not provided or a judgment call that reasonable accountants would disagree on | Flag: "Conclusion is highly fact-dependent; external auditor review required before concluding" |

---

## Enforcement Calibration — Real Cost Benchmarks

These are live enforcement actions to calibrate what "getting this wrong" costs:

| Company | Year | Violation | Cost |
|---|---|---|---|
| Fluor Corporation | 2023 | Percentage-of-completion errors on fixed-price construction (Step 5, over-time measurement, ASC 606-10-25-31). Net earnings overstated up to 37% over 3 years. | **$14.5M civil penalty** + restatement of FY2016–FY2018 |
| Newell Brands / Former CEO Polk | 2023 | Channel stuffing — pulling future sales into current quarters by obtaining early delivery consent; bill-and-hold shipments without control transfer (ASC 606-10-25-30). | **$12.5M penalty**; CEO separately charged |
| Telecom executives (unnamed, SEC) | 2023 | Improperly inflating and recognizing revenue; former controller among defendants | **3 executives permanently barred** from serving as officers or directors |
| Total SEC accounting enforcement FY2023 | 2023 | 83 enforcement actions; 49% involved revenue recognition; $583M total monetary settlements | Industry-wide |

Source: https://www.sec.gov/newsroom/press-releases/2023-170 | https://www.sec.gov/newsroom/press-releases/2023-210 | https://www.sec.gov/newsroom/press-releases/2023-234

PCAOB data: Revenue recognition deficiencies found in ~48% of audits with deficiencies in 2023 (up from 40% in 2022). Source: https://pcaobus.org/news-events/news-releases/news-release-detail/pcaob-report-audits-with-deficiencies-rose-for-second-year-in-a-row-to-40-in-2022

---

## Common Restatement Triggers

Signal these as risk flags when present:

1. **Channel stuffing**: Recognizing at shipment to distributors with right of return and no evidence of sell-through (ASC 606-10-25-1(e) collectability; 606-10-25-30 control transfer)
2. **Bill-and-hold**: Insufficient separation of customer's product; entity still bears risk (ASC 606-10-55-83 through 55-84)
3. **Cutoff errors**: Revenue recognized in wrong period due to system timing (≠ point of control transfer per ASC 606-10-25-30)
4. **Improper capitalization**: Capitalizing costs that should be expensed (or vice versa under ASC 340-40)
5. **One-time items classified as recurring**: Restructuring gains, asset sales, or out-of-period items included in operating revenue
6. **Premature inclusion of variable consideration**: Including constrained amounts before the uncertainty resolves (ASC 606-10-32-11)
7. **Gross-to-net misclassification**: Presenting net arrangements gross (or vice versa) — principal vs. agent error (ASC 606-10-55-36 through 55-40)

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the facts described above. It does not constitute authoritative accounting guidance or a professional opinion. Consult your external auditor, qualified accounting professional, or Big-4 advisor before concluding on material transactions. Revenue recognition judgments are fact-specific and changes in facts may change the conclusion.
