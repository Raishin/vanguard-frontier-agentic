---
name: consolidation-intercompany-advisor
description: Multi-jurisdiction consolidation scope and intercompany elimination reference framework covering ASC 810 / IFRS 10 control models, VIE (Variable Interest Entity) primary beneficiary analysis, NCI measurement, equity method accounting (ASC 323 / IAS 28), intercompany eliminations (sales, profit-in-inventory, debt, interest, dividends), deferred tax on IC eliminations (ASC 740 / IAS 12), and adversarial group reporting scenarios across US GAAP, IFRS, German HGB, JGAAP, CAS, and Ind AS.
allowed-tools: Skill Read WebFetch Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-02"
  category: finance
  lifecycle: experimental
---

# Consolidation & Intercompany Advisor — Reference Skill

## Purpose

Provide the complete multi-jurisdiction framework for consolidation scope determination and intercompany elimination advisory — from controlling financial interest analysis through VIE (Variable Interest Entity — ASC 810-10-15) primary beneficiary tests, NCI measurement, equity method accounting, elimination of intercompany transactions, deferred tax on eliminations, and adversarial group reporting scenarios.

---

## Part 1: Consolidation Scope — ASC 810 vs. IFRS 10

### 1.1 The Control Models Compared

| Dimension | US GAAP (ASC 810) | IFRS 10 |
|---|---|---|
| Core principle | Controlling financial interest | Control: power over investee + exposure to variable returns + ability to use power to affect returns |
| Primary framework | Two-tier: Voting Interest Entity (VIE) model + Variable Interest Entity (VIE) model | Single principle-based control model |
| Structured entities / SPEs | Detailed VIE guidance (ASC 810-10-15) — entity is a VIE if equity at risk is insufficient or lacks decision-making rights | IFRS 10 structured entity concept (less prescriptive; applies the same three-element control test) |
| Majority voting threshold | >50% of voting interest → consolidate (absent other indicators) | Majority voting rights are one indicator of power; substantive potential voting rights also considered |
| Potential voting rights | Generally not considered unless currently exercisable | Substantive potential voting rights must be considered (IFRS 10.B38-B50) |

### 1.2 Variable Interest Entities (VIEs) — ASC 810-10-15 <!-- VIE: US GAAP term for an entity consolidated based on economic exposure rather than majority voting ownership -->

An entity is a VIE if **any** of the following conditions is present (ASC 810-10-15-14):

1. The equity investment at risk is not sufficient to permit the entity to finance its activities without additional subordinated financial support.
2. The equity holders lack the direct or indirect ability to make decisions about the entity's activities that most significantly impact its economic performance through voting rights or similar rights.
3. The equity holders do not absorb the expected losses of the entity.
4. The equity holders do not receive the expected residual returns of the entity.

**Primary beneficiary test** (ASC 810-10-25-38): An enterprise is the primary beneficiary — and must consolidate — if it has **both**:
- **Power**: the ability to direct the activities of the VIE that most significantly impact its economic performance; AND
- **Losses/benefits**: the obligation to absorb losses of, or the right to receive benefits from, the VIE that could potentially be significant to the VIE.

Note: There is no explicit numeric threshold in ASC 810 for VIE primary beneficiary determination — it is entirely judgment-based on power and economics.

### 1.3 IFRS 10 Control — Three-Element Test (IFRS 10.7)

An investor controls an investee when it has **all three** of the following:
1. **Power** over the investee (IFRS 10.10): existing rights that give the investor the current ability to direct the relevant activities (activities that significantly affect the investee's returns).
2. **Exposure** to variable returns from its involvement with the investee (IFRS 10.15).
3. **Ability** to use its power over the investee to affect the amount of the investor's returns (IFRS 10.17).

**De-facto control** (IFRS 10.B41-B46): An investor may have power with less than a majority of voting rights if, considering the size and dispersion of other shareholders' holdings, the investor has the practical ability to direct the relevant activities unilaterally.

**Substantive potential voting rights** (IFRS 10.B38-B50): When assessing control, consider potential voting rights only if they are substantive — i.e., the holder has a practical ability to exercise them (not merely a legal right without practical means).

### 1.4 Voting Interest Entities — US GAAP Majority Voting Model

For entities that are not VIEs, ASC 810-10-15-8 applies the majority voting model:
- Consolidate when the parent owns, directly or indirectly, more than 50% of the voting stock.
- Exception: Effective control may be absent despite majority ownership (e.g., severe long-term restrictions on ability to transfer funds — ASC 810-10-15-10).

### 1.5 Convergence and Divergence Summary

| Scenario | US GAAP Treatment | IFRS Treatment |
|---|---|---|
| 60% voting ownership, no VIE indicators | Consolidate (majority voting model) | Consolidate (power via voting majority) |
| 40% ownership, contractual power to direct key activities | VIE analysis required; likely consolidate if primary beneficiary | Consolidate if IFRS 10 three-element control exists (power via contractual right) |
| 40% ownership, widely dispersed other shareholders | VIE analysis; consolidate if primary beneficiary | De-facto control may exist (IFRS 10.B41) — consolidate |
| 30% with substantive warrants (not currently exercisable) | Potential voting rights generally not considered | Assess if rights are substantive (IFRS 10.B47) — may consolidate |
| Investment company / Fund (ASC 946 / IFRS 10.27-33) | Investment entity exception — do not consolidate portfolio companies; measure at fair value | Investment entity exception — same outcome; consolidate only investment entity subsidiaries that provide investment-related services |

---

## Part 2: Local GAAP Consolidation Thresholds

### 2.1 German HGB — Konzernabschluss (§ 290-315a HGB)

**Obligation to prepare consolidated financial statements** (§ 290 HGB):
- A parent undertaking must prepare consolidated financial statements if it **controls** one or more subsidiaries.
- Control presumed (§ 290 Abs. 2 HGB) if the parent holds a majority of voting rights, has the right to appoint or remove a majority of the supervisory/management board, or has a dominant influence agreement (Beherrschungsvertrag).

**Size exemptions** (§ 293 HGB):
- A group may be exempt from consolidated financial statements if it meets **two of three** thresholds for two consecutive years:
  - Total assets ≤ EUR 25 million (after netting)
  - Net revenues ≤ EUR 50 million
  - Average employees ≤ 250
- Exception: Listed groups must prepare consolidated accounts regardless of size.

**Sub-group exemption** (§ 291 HGB): A German parent that is itself a subsidiary of an EU parent preparing IFRS-compliant group accounts may be exempt from preparing its own Konzernabschluss, provided the sub-group is included in the higher-level consolidation.

### 2.2 JGAAP — ASBJ Statement No. 22

Japan's consolidated accounting standard (ASBJ Statement No. 22, effective 2009) adopts a control model broadly aligned with IFRS:
- Consolidate if the parent controls the subsidiary — holds majority voting rights, or can control the subsidiary's board through contractual arrangements or other means.
- Substantive potential voting rights are considered (analogous to IFRS 10.B38).
- Investment entity exception: Not fully adopted in JGAAP; fund structures are subject to specific guidance.

### 2.3 CAS 33 (China) — Consolidated Financial Statements

CAS 33 (revised 2014) substantially converged with IFRS 10:
- Control definition mirrors IFRS 10: power over investee + exposure to variable returns + ability to use power.
- Structured entities (SPEs): same three-element control test applies.
- **CSRC reporting**: A-share listed groups must file annual consolidated financial statements by April 30; semi-annual consolidated reports by August 31. CSRC requires the group consolidation to reflect all entities controlled per CAS 33, including VIE structures used by Chinese technology companies to satisfy foreign ownership restrictions.

**VIE structures in China**: Chinese companies listed offshore (e.g., on NYSE, HKEX) frequently use contractual VIE arrangements to consolidate entities holding operating licenses restricted from foreign ownership. Under CAS 33 and IFRS 10, consolidation of these VIEs is based on the control analysis — not ownership. Under US GAAP (ASC 810), primary beneficiary analysis applies. **This is a significant area of auditor judgment and regulatory scrutiny.**

### 2.4 Ind AS 110 (India)

Ind AS 110 is word-for-word identical to IFRS 10, including:
- Three-element control test.
- Structured entity guidance.
- Investment entity exception.
- De-facto control based on relative shareholding.

SEBI LODR Regulation 33 requires listed Indian entities to file quarterly and annual consolidated financial statements prepared under Ind AS.

---

## Part 3: Non-Controlling Interests (NCI)

### 3.1 Measurement at Acquisition

| Standard | NCI Measurement Options at Acquisition Date |
|---|---|
| US GAAP (ASC 805-20-30-7) | **Required**: fair value of NCI (full goodwill method only) |
| IFRS 3.B44 | **Choice** (policy election, applied transaction-by-transaction): (a) fair value of NCI (full goodwill method); OR (b) NCI's proportionate share of the acquiree's identifiable net assets (partial goodwill method — no goodwill attributed to NCI) |

**Practical impact**: Under IFRS partial goodwill, if a subsidiary with €100M fair value of identifiable net assets is 80% acquired, total goodwill recognized is less than under full goodwill. If a subsequent impairment occurs, the impairment charge differs between full and partial goodwill methods.

### 3.2 Subsequent NCI Accounting

- **Attribution of losses** (ASC 810-10-45-20): Losses are attributed to NCI even if this results in a deficit NCI balance. (Prior to SFAS 160, excess losses were absorbed by the parent.)
- **IFRS 10.B94**: Same principle — losses are attributed to NCI even if the NCI balance becomes negative, unless the NCI has a contractual obligation to, and can, make good the losses.
- **Transactions with NCI that do not result in loss of control** (ASC 810-10-45-23 / IFRS 10.B96): Treated as equity transactions. No gain or loss recognized in P&L; the difference between consideration paid/received and the carrying amount of NCI acquired/disposed is adjusted directly in equity.
- **Loss of control** (ASC 810-10-40-5 / IFRS 10.25): Derecognize assets/liabilities of the former subsidiary; recognize retained interest at fair value; recognize gain/loss in P&L.

---

## Part 4: Equity Method — ASC 323 / IAS 28

### 4.1 Significant Influence Threshold

| Standard | Presumption | Rebuttal |
|---|---|---|
| ASC 323-10-15-8 | 20% or more of voting stock → significant influence presumed | Rebuttable if evidence indicates inability to exercise significant influence (e.g., legal proceedings, agreements restricting influence) |
| IAS 28.6 | 20% or more of voting power → significant influence presumed | Same rebuttable presumption logic |

**Indicators of significant influence** (beyond 20% threshold): Board representation, participation in policy-making, material intercompany transactions, interchange of managerial personnel, provision of essential technical information (ASC 323-10-15-6 / IAS 28.7).

### 4.2 Investor-Level Accounting

- Record investment at cost; adjust for investor's share of investee's earnings/losses (net of dividends received) and OCI items.
- Adjust for **upstream eliminations** (investee sells to investor — eliminate investor's share of unrealized profit from inventory still held by investor): ASC 323-10-35-7 / IAS 28.28.
- Adjust for **downstream eliminations** (investor sells to investee — eliminate investor's share of unrealized profit on intercompany transfer still on investee's balance sheet): same paragraphs.
- **Losses in excess of investment** (ASC 323-10-35-20 / IAS 28.38): Discontinue equity method when investment reaches zero; resume only when the investor's share of net income exceeds the investor's share of previously unrecognized losses.

### 4.3 Impairment of Equity Method Investments

- **US GAAP**: ASC 323-10-35-31 — assess for other-than-temporary impairment. If carrying amount exceeds fair value and the decline is other than temporary, write down to fair value and recognize impairment loss in earnings.
- **IAS 28.40-43**: Apply IAS 36 impairment indicators. If there is objective evidence of impairment, test the entire investment as a single asset (cash-generating unit concept does not apply within equity method investment). Recoverable amount = higher of value in use and fair value less costs to sell.

---

## Part 5: Intercompany Elimination Requirements

### 5.1 Elimination Categories (ASC 810-10-45 / IFRS 10.B86)

At each consolidation close, eliminate **100%** of the following:

**Intercompany balances:**
- [ ] Intercompany receivables and payables (seller's receivable = buyer's payable at the same amount; mismatches signal cutoff errors or currency differences requiring investigation)
- [ ] Intercompany loans — principal balance in both entities
- [ ] Intercompany accrued interest on loans (interest income at lender = interest expense at borrower)
- [ ] Intercompany dividends declared but not yet received (dividend income at parent = dividends payable at subsidiary)

**Intercompany P&L transactions:**
- [ ] Intercompany sales (upstream and downstream) — seller's revenue = buyer's cost or capitalized amount
- [ ] Intercompany management fees, royalties, and service charges
- [ ] Intercompany rental income and expense

**Unrealized profit eliminations:**
- [ ] **Profit-in-inventory** (ASC 810-10-45-1 / IFRS 10.B86-B87): Eliminate intercompany profit embedded in inventory not yet sold to a third party. The full intercompany profit is eliminated regardless of NCI percentage (full elimination principle under both US GAAP and IFRS).
- [ ] **Profit on intercompany asset transfers** (e.g., PP&E sold between group entities): Eliminate the gain/profit and adjust the asset's carrying amount and depreciation schedule.
- [ ] **Profit on intercompany construction contracts** billed upstream.

### 5.2 Deferred Tax on Intercompany Eliminations

**ASC 740-10-25-3 / IAS 12.39**: When an intercompany profit is eliminated in consolidation, a temporary difference arises between the consolidated carrying amount and the tax base of the asset (which reflects the intercompany transfer price paid by the buying entity). This requires recognition of a deferred tax asset at the buyer's jurisdiction's enacted (US GAAP) or substantively enacted (IAS 12) tax rate.

**Example** (profit-in-inventory):
- Seller entity (Jurisdiction A, 25% tax rate) sells inventory to buyer entity (Jurisdiction B, 20% tax rate) at €1,000 cost + €200 profit.
- Buyer holds €200 unrealized profit in inventory at consolidation date.
- Consolidated elimination entry: Dr Retained Earnings (or COGS) €200; Cr Inventory €200.
- Deferred tax entry: Dr Deferred Tax Asset €40 (€200 × 20% — buyer's rate under IAS 12.39); Cr Income Tax Expense €40.
- Under US GAAP (ASC 740-10-25-3(e)): same deferred tax asset at buyer's enacted rate.

**Common error**: Using the seller's tax rate instead of the buyer's tax rate for the deferred tax on intercompany eliminations. IAS 12.39 explicitly states that deferred taxes arising from temporary differences in the consolidated financial statements are measured using the tax laws applicable to the entity that holds the asset.

### 5.3 Transfer Pricing and Elimination

**Arm's length pricing**: Intercompany transactions should be priced at arm's length for tax purposes (OECD Transfer Pricing Guidelines). When the transfer price differs from arm's length (e.g., cost-plus arrangements), the elimination in consolidation must still eliminate 100% of the intercompany profit. However:
- **Transfer pricing adjustments** by tax authorities post-period may require a retroactive adjustment to the intercompany transaction — creating a need to re-open eliminations.
- **Versioned elimination rules**: In a multi-year group structure, maintaining versioned transfer pricing policies per entity pair per period is recommended — enables traceability when tax authorities challenge a specific year's pricing.
- **Permanent establishment risk**: If an intercompany service arrangement creates a permanent establishment in a jurisdiction, additional local tax filings may be required — outside the consolidation scope but flagged as a risk dependency.

---

## Part 6: Multi-Jurisdiction IC Elimination Comparison

| Elimination Area | US GAAP (ASC 810) | IFRS (IFRS 10) | German HGB | JGAAP | CAS 33 | Ind AS 110 |
|---|---|---|---|---|---|---|
| Full vs. partial elimination of unrealized IC profit | **Full** elimination (100%) | **Full** elimination (IFRS 10.B86) | Full elimination (§ 304 HGB) | Full elimination (ASBJ Stmt 22) | Full elimination | Full (= IFRS 10) |
| Deferred tax on IC eliminations | Yes — ASC 740-10; buyer's enacted rate | Yes — IAS 12.39; buyer's substantively enacted rate | Yes — deferred taxes required (§ 306 HGB) | Yes — broadly aligned with IFRS | Yes — CAS 18 (deferred tax) | Yes — Ind AS 12 (= IAS 12) |
| NCI share of IC elimination | No adjustment to NCI for IC eliminations (full elimination regardless of NCI %) | Same — full elimination (NCI not relevant to elimination) | Full elimination | Full elimination | Full elimination | Full elimination |
| Upstream profit elimination (associate — equity method) | Investor's share only (ASC 323-10-35-7) | Investor's share only (IAS 28.28) | Investor's share only | Investor's share only | Investor's share only | Investor's share only |

---

## Part 7: EU Transparency Directive — Consolidated Group Filing

**Directive 2004/109/EC (as amended by 2013/50/EU)** requires issuers admitted to trading on an EU regulated market to file:
- **Annual Financial Report**: within 4 months of financial year end — must include audited consolidated financial statements.
- **Half-Year Financial Report**: within 3 months of end of first six months — may be unaudited but subject to review.

**Consolidation scope for EU reporting**: The consolidated financial statements must be prepared under IFRS as endorsed by the EU (EU IFRS). Any differences between IFRS as issued by the IASB and EU-endorsed IFRS are currently minimal (the EU has endorsed all major standards through 2024 with limited carve-outs historically in IAS 39 — since replaced by IFRS 9).

---

## Part 8: SAFE Cross-Border Intercompany Loan Constraints (China)

China's State Administration of Foreign Exchange (SAFE) imposes restrictions on cross-border intercompany lending:

**Key rules:**
- **SAFE Circular 19 (2015)**: Foreign-invested enterprises (FIEs) in China may on-lend foreign exchange received via equity contributions (foreign exchange funds under the capital account) to Chinese group entities, subject to SAFE registration and approval. However, lending to third parties or converting to RMB for on-lending requires specific approval.
- **Macro-prudential framework**: Outbound loans from China entities to overseas group companies require registration with SAFE under the outbound direct investment (ODI) framework or the intercompany loan framework. Loan principal, interest rate, and repayment terms must comply with SAFE registration requirements.
- **Impact on IC elimination**: Intercompany loans that are not SAFE-registered or that exceed approved limits may not be legally enforceable — creating a risk that the intercompany balance cannot be collected, potentially triggering impairment rather than elimination.
- **FX restrictions**: Repatriation of intercompany loan repayments from China requires SAFE approval for large amounts; delays create timing mismatches in intercompany reconciliation.

**Advisory flag**: Any intercompany loan involving a Chinese entity should be reviewed against current SAFE regulations before elimination is treated as routine. Engage local Chinese legal counsel for structural compliance.

---

## Part 9: Adversarial Consolidation Scenarios

### 9.1 M&A Closes Mid-Quarter-Close

**Scenario**: An acquisition closes on the 15th day of the third month of a quarter. The close cycle for Q3 is already in progress.

**Recommended workflow**:
1. **Quarantine the new entity**: Flag it as "provisional consolidation" in the consolidation system — prevents premature blending of incomplete data.
2. **Determine acquisition date** (ASC 805-10-25-6 / IFRS 3.9): The acquisition date is when the acquirer obtains control — typically the legal closing date.
3. **Include only post-acquisition date results**: Consolidated P&L includes the acquiree's results from the acquisition date only (ASC 805-10-45-2 / IFRS 3.B64).
4. **Fair value measurement period** (ASC 805-10-25-15 / IFRS 3.45): Up to one year from the acquisition date to finalize purchase price allocation — provisional amounts used in the first consolidation.
5. **Intercompany eliminations in the acquisition period**: Identify intercompany transactions between the acquirer and acquiree that occurred before the acquisition date — these are pre-acquisition and not eliminated in consolidation (the acquiree was not yet part of the group).

### 9.2 Cross-Border Intercompany Dispute — Two-Sided Matching Break

**Scenario**: The US parent records a €5.2M intercompany receivable from its German subsidiary. The German subsidiary records a €5.0M intercompany payable to the US parent. A €200K break exists.

**Investigation workflow**:
1. **Currency investigation**: Determine if the break is a FX translation difference. The US parent may have translated the original USD-denominated receivable at a different rate than the German subsidiary used to translate its EUR-denominated payable.
2. **Timing investigation**: Was the transaction recorded in the same period by both entities? Cross-border transactions often have a day's lag due to time zones.
3. **Dispute flag**: If neither FX nor timing explains the break, escalate to intercompany dispute resolution. Do not eliminate until the break is resolved — eliminate the agreed amount and leave the disputed amount as a reconciling item.
4. **Deferred tax impact**: If the break results in a permanent difference (dispute not resolved), consider whether a provision is needed.
5. **Standard reference**: ASC 810-10-45-1 (all intercompany balances eliminated) / IFRS 10.B86 — elimination requires both sides to agree.

### 9.3 Transfer Price Change Affecting Elimination

**Scenario**: A group changes its intercompany transfer pricing policy mid-year — moving from cost-plus 10% to cost-plus 15% for a major intercompany services arrangement.

**Recommended approach**:
1. **Version the elimination rules**: Create a new version of the intercompany elimination rule effective the date of the transfer price change. Apply the old rule to periods before the change and the new rule thereafter.
2. **Retroactive vs. prospective**: Determine whether the change is a new arrangement (prospective) or a restatement of prior period pricing (retroactive — requires re-opening prior period eliminations).
3. **Deferred tax recalculation**: The changed transfer price affects the temporary difference underlying the deferred tax on IC eliminations. Recalculate deferred tax asset/liability for each affected intercompany balance.
4. **Documentation**: Maintain the transfer pricing policy change documentation to support auditor inquiries and potential tax authority challenges.

---

## Part 10: Official Documentation — Publicly Accessible URLs

| Standard | Resource | URL | Access |
|---|---|---|---|
| ASC 810 (Consolidation) | FASB ASC | https://asc.fasb.org/810 | Public (registration may be required for full text) |
| ASC 323 (Equity Method) | FASB ASC | https://asc.fasb.org/323 | Public |
| IFRS 10 (Consolidated Financial Statements) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs10.html | **Fully public** |
| IFRS 3 (Business Combinations) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ifrs3.html | **Fully public** |
| IAS 28 (Investments in Associates and JVs) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias28.html | **Fully public** |
| IAS 12 (Income Taxes) | IASB HTML standard | https://www.ifrs.org/content/dam/ifrs/publications/html-standards/english/2024/issued/ias12.html | **Fully public** |
| German HGB | Federal Ministry of Justice | https://www.gesetze-im-internet.de/hgb/ | **Fully public (German)** |
| JGAAP — ASBJ Statement No. 22 | ASBJ English translations | https://www.asb.or.jp/en/accounting_standards/accounting_standards/ | **Fully public** |
| EU Transparency Directive | EUR-Lex | https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32004L0109 | **Fully public** |

---

## Mandatory Advisory Note

Every response from this agent must end with:

> **Advisory**: This analysis is advisory and based solely on the entity profile and facts described above. Consolidation scope determinations and intercompany elimination requirements depend on specific contractual arrangements, ownership structures, and local regulatory requirements that vary by jurisdiction and change over time. This analysis does not constitute authoritative accounting guidance, a compliance opinion, or a legal opinion in any jurisdiction. Verify all consolidation scope conclusions and intercompany elimination outputs with qualified external auditors before use in statutory consolidated financial statements.
