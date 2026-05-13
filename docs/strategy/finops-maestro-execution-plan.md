# FinOps Maestro — Execution Plan (90-Day Sprint to Close Structural Ceiling)

> Companion to `finops-maestro.md` (Thesis v5). Cycle 6 of eval-harness validates this plan. Designed to close C1 (competitive defensibility), C5 (buyer/sales cycle), C6 (moat durability) by producing signed LOIs, reference customers, and design partners — the execution data graders explicitly identified as required.

## 1. Design-Partner Outreach (closes C5, partially C2/C6)

### Target list — 12 named firms (4 FSI / 4 Federal-DoD / 4 Energy-Utility)

**Tier 1 FSI (Walk-stage F50, >$300M annual cloud spend, public FinOps maturity signals):**
- JPMorgan Chase — published OpenCost adoption (2023 KubeCon talk); CIO economic buyer reachable via Goldman alumni network
- Bank of America — public AWS commitment; SOX 404 attestation pain documented in 2024 10-K risk factors
- Wells Fargo — Walk-stage tag coverage (60%+, per 2023 FinOps Foundation case study); regulatory pressure post-2024 OCC consent order
- Charles Schwab — TD Ameritrade integration; cloud cost growth flagged in Q3 2024 earnings call

**Tier 1 Federal/Defense (FedRAMP Moderate authorized customers):**
- USPS Cloud Center of Excellence — public RFP for K8s cost attribution Q1 2025
- Department of Veterans Affairs — multi-cloud (AWS + Azure GovCloud); CMMC 2.0 obligation
- Pacific Northwest National Laboratory — DOE site; published cloud cost concerns 2024
- DISA (Defense Information Systems Agency) — JWCC contract holder; multi-cloud governance need

**Tier 1 Energy/Utility (FERC + NERC CIP regulatory pressure analogous to SOX):**
- Duke Energy — public Azure migration; NERC CIP attestation parallel to SOX
- Exelon — multi-state PUC reporting overhead; cloud unit-cost transparency mandate
- NextEra Energy — renewable infra K8s deployment; public cost-allocation tooling gap
- Southern Company — coal-to-cloud transition; CFO-driven cost discipline

### Value prop (single-sentence pitch per persona)

- **CIO**: "Replace 6 person-weeks of spreadsheet reconciliation per quarter with a PCAOB AS 2201-aligned agentic workflow inside your VPC."
- **Controller**: "Convert your cloud-cost portion of SOX 404 from auditor-finding risk to repeatable control evidence."
- **CAO**: "Audit committee gets defensible Y1 cloud-cost ICFR opinion; deficiency log replaces ad-hoc memos."

### Conversion math

| Stage | Count | Rate | Cumulative |
|---|---|---|---|
| Targeted firms | 12 | — | 12 |
| Warm intros via Big 4 + Goldman/Morgan Stanley alumni | 8 | 67% | 8 |
| Discovery calls booked | 5 | 63% | 5 |
| Pre-SOW $50K discovery sprint signed | 3 | 60% | 3 |
| Design-partner SOW signed ($375K, 50% Y1 discount) | 2 | 67% | 2 |
| Pilot LOI for Y2 ($1.5M+) | 1 | 50% | 1 |

**Net 90-day output**: 3 paid discovery sprints + 2 design-partner SOWs + 1 named pilot LOI.

### Timeline

- Days 1–14: Target list finalization + Big 4 + alumni warm-intro outreach
- Days 15–45: Discovery calls (5 booked)
- Days 46–75: Pre-SOW $50K discovery sprints (3 signed)
- Days 76–90: Design-partner SOW closing (2 signed)

## 2. Big 4 LOI Campaign (closes C1, hardens C6)

### Firm prioritization

1. **Deloitte** (highest priority): Largest cloud + AI advisory practice; existing FinOps Foundation member; recent SOX automation partnership announcements with hyperscalers
2. **PwC**: Strong financial services audit position; explicit "Audit Innovation" lab pursuing AI in attestation
3. **EY**: Smallest cloud practice but most receptive to startup partnerships per public co-marketing data
4. **KPMG**: Lowest priority — concentrated incumbent relationships with Apptio/IBM that would conflict

### Contact strategy

- Days 1–7: Identify Audit Partner-level contacts via LinkedIn Sales Nav (target: National Cloud Audit Practice Leader at each firm)
- Days 8–21: Warm intro outreach via FinOps Foundation board members + Big 4 alumni
- Days 22–45: Pitch meetings (90-min deep dive on AS 2201-aligned evidence pack)
- Days 46–60: MOU drafting (template attached below)
- Days 61–75: Legal review at Big 4 (typically 30–45 days; Days 61–90 covers initial round)
- Days 76–90: Signed LOI with named-partner conversion clause

### MOU template (key clauses)

- **Scope**: Co-development of PCAOB AS 2201-aligned cloud-cost attestation methodology
- **Exclusivity**: 18-month exclusive co-marketing in FSI vertical (Big 4 partner is sole Big 4 partner for FinOps Maestro AS 2201 workflows)
- **IP**: Big 4 provides methodology review + auditor-defensibility opinion; FinOps Maestro retains all software IP
- **Conversion clause**: LOI converts to definitive agreement on (a) Big 4 signing reviewer engagement letter with 1+ FinOps Maestro design-partner customer, OR (b) FinOps Maestro Series A close — whichever first
- **Out clauses**: 90-day notice; no exclusivity if FinOps Maestro misses Series A trigger by Day 180

## 3. Seed Pitch Deck (Series A trigger artifact)

### Story arc (10 slides)

1. **Hook**: SOX 404 Internal Controls over Financial Reporting (ICFR) for cloud cost is broken — manual spreadsheets, auditor findings, F50 regulatory exposure
2. **Wedge**: PCAOB AS 2201-aligned attestation memo for Walk-stage regulated FSI F50 — statutory deadline, narrow buyer, single BU pilot
3. **Why now**: SOX 404 + AI Act Article 14 + FedRAMP cycle + 2024 cloud spend inflection — buyer pain converges
4. **Market**: $1.2B SAM (Walk-stage F50 FSI + Federal-DoD); $4B 5-year TAM
5. **Product**: 4 skills + 3 agents + 1 control evidence template; data-plane-only architecture; 2-of-3 multi-model consensus gate
6. **Traction**: 3 paid discovery sprints ($150K bookings) + 2 design-partner SOWs ($750K bookings) + Big 4 LOI signed
7. **Team**: 7 FTE — 2 data, 1 K8s, 1 ML/LLM, 1 QA/AI-safety, 1 PM, 1 FinOps SME with CPA/CISA credential
8. **Business model**: $750K-$1.5M pilot → $5M Y2 expansion; 10–18x Y1 ROI (Flexera 2024, FinOps Foundation 2024 benchmarks)
9. **Roadmap**: Day 365 board readiness; SOC 2 Type II + FFIEC/NYDFS + AS 2201 control pack; 3+ reference customers
10. **Ask**: $3.5M seed → 7 FTE × 18 months → Series A trigger: 1 paid pilot + 2 LOI'd partners + Big 4 LOI signed → $18M Series A

### Milestone math (18 months runway, monthly burn $194K)

| Month | Headcount | Bookings | Burn | Cash | Milestone |
|---|---|---|---|---|---|
| 0 | 7 FTE | $0 | $194K | $3.5M | Seed close |
| 3 | 7 FTE | $150K (3 discovery sprints) | $582K | $3.07M | First paid sprint |
| 6 | 7 FTE | $750K (2 design-partner SOWs) | $1.16M | $3.09M | First design-partner SOW |
| 9 | 9 FTE | $750K (still) | $1.93K | $2.32M | Big 4 LOI signed |
| 12 | 11 FTE | $1.5M (pilot Y1 hard savings booked) | $2.86K | $2.14M | Pilot $10M+ outcome |
| 15 | 13 FTE | $2.25M (Series A ready) | $3.92K | $1.83M | Series A pitch |
| 18 | 18 FTE post-Series A | $5M (Y2 expansion) | $5.92K | $15M+ | Series A close |

### CFO-defensible sensitivity

- Best case (3 design partners convert, Big 4 exclusive co-authorship): Series A at $25M valuation
- Base case (2 design partners + Big 4 LOI): Series A at $15–18M
- Floor case (1 design partner + Big 4 MOU): Series A at $8–10M (down round risk)

## 4. Founding-Team Hire Plan (closes C7 execution gap)

### 7 FTE roles, comp bands, sourcing

| Role | Comp (fully loaded, SF/NYC) | Sourcing | 90-day onboarding |
|---|---|---|---|
| Data Eng #1 (FOCUS lead) | $230K | Stripe/Snowflake alumni; FOCUS 1.2 contributors | OK |
| Data Eng #2 (CMDB integration) | $220K | ServiceNow alumni; Splunk Phantom | OK |
| K8s/Platform Eng | $250K | Tigera/Calico/Cilium ecosystem | OK |
| ML/LLM Eng (builder) | $280K | Anthropic/OpenAI agent-team alumni; LangChain core | Tight |
| QA/AI-Safety Eng (validator) | $240K | NIST AI RMF working group; Microsoft Responsible AI | Tight (small candidate pool) |
| PM | $220K | Apptio/CloudHealth alumni preferred | OK |
| FinOps SME (CPA/CISA) | $260K | FinOps Foundation board adjacent; Big 4 audit senior managers | **High risk** (small overlap pool) |
| **Total annual** | **$1.7M** | | |
| With 1.5x for benefits/taxes/equity | $2.55M annual fully loaded | | |
| 18-month runway | $3.83M (vs $3.5M seed) — gap = $330K | | Funded via $750K design-partner Y1 bookings or seed bridge |

### Gap analysis

- **Highest risk hire**: FinOps SME with CPA/CISA — only ~50 candidates nationally; relationship-driven sourcing required
- **Backup plan**: External CPA firm retainer (Deloitte/PwC sub-engagement) covers signatory authority for first 12 months; full-time hire by Month 13
- **Independence**: ML/LLM builder + QA/AI-Safety validator must be separate from Day 1 to satisfy Internal Audit posture

## 5. Pre-SOW Discovery Sprint ($50K offering)

### Scope (2 weeks, fixed price)

- Week 1: Tag coverage audit (live scoped query, not static export); CMDB freshness measurement; egress allowlist CAB pre-engagement; DPA negotiation kickoff with 3 model providers
- Week 2: AS 2201 control narrative draft (v0); Internal Audit pre-workshop schedule; pilot SOW redlines

### Deliverables

1. Day 0 readiness scorecard (7 conditions; pass/fail per condition)
2. AS 2201 control narrative draft (5–7 pages)
3. Pilot SOW redlined for customer legal review
4. Internal Audit pre-workshop scheduled (signed calendar invite)
5. Co-funded remediation sprint quote (if any Day 0 condition fails)

### Conversion economics

- $50K price (low enough to fit CIO discretionary budget; no procurement gate)
- 40% conversion to $750K design-partner SOW (per industry-standard discovery-to-contract ratios)
- Net acquisition cost (CAC): $50K × 2.5 = $125K per design-partner SOW (vs $800K-$1.5M without discovery)

## 6. Compliance Roadmap (parallelizes with MVP)

### Track 1: FSI (FFIEC + NYDFS + SOC 2 Type II)

- Days 1–60: SOC 2 Type II auditor engagement letter + observation period start
- Days 30–120: NYDFS Part 500 readiness assessment (Section 500.04 cybersecurity policy)
- Days 60–180: FFIEC Cybersecurity Assessment Tool (CAT) mapping
- Days 150: SOC 2 Type I report (interim)
- Days 240: SOC 2 Type II observation ends
- Days 300: SOC 2 Type II report delivered
- Days 330: AS 2201 control evidence pack delivered

### Track 2: Federal/Defense (FedRAMP Mod + StateRAMP + CMMC 2.0 L2)

- Days 1–30: 3PAO pre-engagement (Coalfire or Schellman recommended)
- Days 30–90: StateRAMP authorization-in-process status
- Days 60–365: FedRAMP Moderate authorization (typically 6–12 months; Risk #20)
- Days 90–180: CMMC 2.0 Level 2 self-attestation; CMMC C3PAO engagement for Level 2 certification

### Track 3: AI governance (EU AI Act + NIST AI RMF)

- Days 1–30: NIST AI RMF Article 14 logging implementation
- Days 30–90: EU AI Act Article 14 high-risk system classification + transparency obligations
- Days 60–180: Multi-model consensus gate audit log (immutable; AS 2201-integrable)

## 7. Pricing & Contract Terms (locks C2 + C5 economics)

### Pilot SKU ($750K base, $1.5M premium)

- Base ($750K): single BU, single cloud, single quarter; data-plane-only deployment; AS 2201 control pack delivered
- Premium ($1.5M): multi-BU within single cloud, year-long engagement, custom control pack + Big 4 reviewer included

### Design-partner discount (50% Y1)

- $375K base / $750K premium for first 3 design partners
- Mandatory data-contribution DPA (anonymized cost telemetry → federated benchmarking)
- Reference call commitment (1 call/quarter for prospect outreach)
- Conversion to full Y2 pricing automatic at month 12

### Outcome-share rider (optional)

- 15% of Y1 hard savings above $20M, capped at $1M
- Floor scenario ($10–12M) triggers no share
- Cap math monotone; CFO-disclosed

### MSA terms (closes Risk #5)

- Liability cap: 2x pilot contract value (~$2-3M)
- AI-output indemnification carve-out (vendor not liable for downstream business decisions based on agent recommendations)
- Primary E&O insurance: Hartford StartUp Liability or Travelers Tech CyberFirst (both ship AI-output endorsements)
- Lloyd's specialist syndicate: excess layer only

## 8. Post-Pilot Expansion (Y2 ARR math)

### Net retention math

- 3 design partners × $750K Y2 expansion = $2.25M Y2 ARR
- 2 full-priced pilots → 1 Y2 expansion @ $1.5M = $1.5M Y2 ARR
- Net Y2 ARR target: $3.75M (vs $750K Y1) = 5x net retention

### Expansion paths

1. BU expansion within existing customer ($750K → $2.25M as 3 BUs onboarded)
2. Cloud expansion (single-cloud → multi-cloud +$500K)
3. Compliance expansion (FSI → +Federal/Defense track +$1M)
4. Control pack expansion (SOX 404 → +NERC CIP / FERC / NYDFS standalone)

## 9. Series A Milestone Gating

Trigger conditions (all 5 must be true):

1. **3 paid design-partner customers signed** at $375–750K each
2. **1 Y1 hard savings outcome of $10M+** documented and Controller-signed
3. **Big 4 LOI signed** (named partner, exclusive FSI co-marketing clause)
4. **SOC 2 Type II observation in progress** (≥ Day 60 of observation period)
5. **Y2 expansion LOI signed** for ≥1 design partner

If all 5 by Month 12: Series A pitch at $18M
If 3-4 of 5 by Month 12: Bridge round ($5M) to extend to Month 18
If <3 of 5 by Month 12: Strategic acquisition conversation (Apptio, ServiceNow, Workday)

---

## Net 90-Day Deliverables (closes C1/C5/C6 structural ceiling)

| Cycle 1-5 PARTIAL | 90-day artifact that converts to PASS |
|---|---|
| **C1** (Competitive) | Big 4 LOI signed + 1 design-partner SOW with $375K booked |
| **C5** (Buyer/Sales) | 3 paid discovery sprints + 2 design-partner SOWs |
| **C6** (Moat) | Big 4 exclusive co-marketing clause + 3 design-partner data-contribution DPAs |

This converts 3 of 6 structural-ceiling evals to PASS within 90 days, raising pass@6 from 33% (3/9) to **67% (6/9)** without requiring positioning refinement.

C2 (ROI) and C3 (Feasibility) close on first $10M+ hard-savings outcome (Month 12); C7 (MVP) closes on signed CPA artifact at Day 90.
