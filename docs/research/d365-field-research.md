# Dynamics 365 (D365) Field Research

> Deep-research report grounding the `vanguard-frontier-agentic` Dynamics 365 agent board.
> Method: fan-out web search + Microsoft Learn MCP, first-party sources prioritized, claims
> labeled by evidence and confidence. Certifications treated as volatile and verified against
> current Microsoft Learn pages.
>
> **Date:** 2026-06-17 · **Orchestration note:** subagent fan-out was session-limited at run
> time, so the overseer executed the searches directly against Microsoft Learn / web.
> **Evidence scale:** E3 = official Microsoft docs · E4 = Microsoft Learn certification page.
> **Confidence:** High / Medium / Low (Low = re-verify before acting).

---

## 1. Certification & Applied Skills currency (2026)

| Exam / Cert | Status | Evidence | Confidence |
|---|---|---|---|
| **MB-700 — F&O Apps Solution Architect Expert** | **Active**; English updated **2025-03-19**; 3-cert prerequisite. | E4 — [MB-700 cert](https://learn.microsoft.com/credentials/certifications/d365-finance-and-operations-apps-solution-architect-expert/) | High |
| **MB-335 — Supply Chain Management Functional Consultant Expert** | **Active**; English updated **2025-06-20**; recommends MB-330 first. | E4 — [MB-335 cert](https://learn.microsoft.com/credentials/certifications/d365-supply-chain-management-functional-consultant-expert/) | High |
| **MB-330 (SCM FC)**, **MB-310 (Finance FC)**, **MB-500 (F&O Apps Developer)**, **MB-800 (Business Central FC)** | **Active** | E4 — Microsoft Learn certification catalog | High |
| **MB-230 — Customer Service Functional Consultant** | **Active**; skills measured **as of 2026-03-11** (cases, representative experience & routing, extend). | E4 — [MB-230 study guide](https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-230) | High |
| **MB-240 — Field Service Functional Consultant** | **RETIRING 2026-06-30** (11:59 PM CST). | E4 — [MB-240 study guide](https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-240) | High |
| **MB-210 — Sales** | **Retired (2024)**; succeeded by **MB-280**. | E4 — [MB-280 cert](https://learn.microsoft.com/credentials/certifications/d365-customer-experience-analyst-associate/) | High |
| **MB-280 — Dynamics 365 Customer Experience Analyst Associate** (covers **Sales + Customer Insights**) | **Active** (current CE/Sales anchor). A web result indicated a possible 2026-07-31 retirement — **treat as Low/verify** (likely a study-guide version note). | E4 — [MB-280 study guide](https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-280) | High (active) / Low (retirement claim) |
| **MB-260 — Customer Insights (Data) Specialist** | **Retired 2024-11-30**. | E4 — [MB-260 study guide](https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-260) | High |
| **MB-220 — Marketing** | **Retired** (legacy Marketing exam; capability folded into Customer Insights – Journeys). | E4 — [D365 cert name changes post](https://learn.microsoft.com/credentials/certifications/posts/dynamics-365-certification-and-exam-names-are-changing) | Medium |

**Board implications (cert-map fixes):**
- `d365-sales-revenue-operations-agent` — anchor is now **MB-280 (Customer Experience Analyst)**, *not* the retired MB-210. Update the agent's role/cert reference.
- `d365-field-service-to-cash-agent` — **MB-240 retires 2026-06-30**; flag the cert anchor as retiring and note Field Service skilling is moving toward Applied Skills / role content.
- `d365-customer-service-contact-center-agent` — **MB-230** is current (already cited). ✓
- Solution-architect / migration / SoD agents — **MB-700 / MB-500** current. ✓
- A future **Customer Insights – Journeys** agent has **no current data-specialist exam** (MB-260 retired); cert anchor would be weak — treat as Applied-Skills/role-content grounded.

---

## 2. Product & governance frameworks (current Microsoft guidance)

| Claim | Evidence | Confidence |
|---|---|---|
| **Success by Design** + **FastTrack** is Microsoft's implementation framework (Strategize → Initiate → Implement → Prepare → Operate) for D365 — the basis of the `d365-success-by-design-governance` agent. | E3 — [Dynamics 365 implementation guide](https://learn.microsoft.com/dynamics365/guidance/implementation-guide/overview) | High |
| The end-to-end **"service to cash" scenario was renamed "service to deliver"** in the **February 2025** Business Process Catalog (Learn articles not yet fully updated). | E3 — [Service-to-deliver: manage service work](https://learn.microsoft.com/dynamics365/guidance/business-processes/service-to-cash-create-process-service-work) | High |
| **Field Service** uses **Universal Resource Scheduling** (work order → requirement → bookable-resource booking; schedule board / schedule assistant / **Resource Scheduling Optimization** add-in); work-order close drives inventory consumption + invoice. | E3 — [URS for Field Service](https://learn.microsoft.com/dynamics365/field-service/universal-resource-scheduling-for-field-service), [Work order architecture](https://learn.microsoft.com/dynamics365/field-service/field-service-architecture) | High |
| **Customer Service** centers on case management, **unified routing**, **Omnichannel**, queues, entitlements, **SLAs** (enhanced/standard), and knowledge management; administered in the **Copilot Service admin center** (formerly Customer Service admin center). | E3 — [Case management overview](https://learn.microsoft.com/dynamics365/customer-service/administer/overview-cases), [Enhanced SLAs](https://learn.microsoft.com/dynamics365/customer-service/administer/create-enhanced-sla) | High |
| **Data migration / go-live** uses the **Data Management Framework** (data entities, packages, staging) with mock migration, reconciliation, and cutover strategy — the basis of `d365-data-migration-cutover`. | E3 — [Prepare to go live](https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-to-go-live), [data packages](https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/data-entities-data-packages) | High |
| **Copilot is embedded across D365** apps (e.g., the SCM Functional Consultant Expert explicitly lists "Copilot in Dynamics 365 Supply Chain Management"; Copilot in Field Service / Customer Service / Finance). | E4/E3 — [MB-335 cert](https://learn.microsoft.com/credentials/certifications/d365-supply-chain-management-functional-consultant-expert/), [Field Service overview](https://learn.microsoft.com/dynamics365/field-service/overview) | High |

**Board implication:** the D365 specialist + protocol set (finance close-to-report, supply-chain plan-to-produce, field-service-to-cash, customer-service, SbD, SoD, data-migration) is grounded on current Microsoft frameworks. Keep the **service-to-deliver** terminology note (already in the field-service skill) and add **Copilot-in-D365** as a cross-cutting review dimension.

---

## 3. Capability gaps / missing roles (future wave)

| Candidate agent | Why it's enterprise-painful | Anchor | Confidence |
|---|---|---|---|
| **D365 Customer Insights – Journeys** (real-time marketing, consent, segmentation) | Marketing/CDP is a distinct, high-value surface; MB-220/MB-260 retired left a coverage hole; planned in the board but not built. | Customer Insights docs (no current cert anchor) | High |
| **D365 Finance & Operations developer (X++ / extensions)** | Upgrade-safe customization and ALM is a top transformation risk; MB-500 exists. | MB-500; [F&O dev docs](https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/) | Medium |
| **D365 Project Operations** | Project-based revenue/resourcing is unaddressed. | Project Operations docs | Medium |
| **D365 Commerce / retail** | POS + omnichannel commerce unaddressed. | Commerce docs | Low |
| **D365 integration / dual-write & Power Platform boundary** | ERP↔CRM sync drift (dual-write) is a recurring failure; partially covered by protocols, not a standalone agent. | [Dual-write docs](https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/data-entities/dual-write/dual-write-overview) | Medium |

These confirm the board's "future wave" backlog (Customer Insights – Journeys, F&O developer, integration/dual-write specialist).

---

## 4. Market / competitive landscape

| Claim | Evidence | Confidence |
|---|---|---|
| Microsoft is embedding **Copilot/agents across the D365 ERP/CRM suite** and aligning skilling to the **Business Process Catalog** (process-first, e.g., service-to-deliver, order-to-cash). | E3 — [Business process catalog](https://learn.microsoft.com/dynamics365/guidance/business-processes/overview) | High |
| D365 competes with SAP S/4HANA, Oracle Fusion/NetSuite, and Salesforce (CRM); first-party search did not surface verifiable competitive specifics. | — | Low (re-verify) |

---

## Verification debt / re-verify before publishing agent cert maps

- **MB-280 retirement date** (Low) — confirm whether a 2026-07-31 retirement is real or a study-guide version artifact.
- **MB-220 retirement specifics** (Medium) — confirm exact date/replacement.
- **Project Operations / Commerce** cert anchors (Low-Medium) — confirm current exams before adding agents.
- Re-confirm "skills measured as of" dates and any new **Applied Skills** before stamping `last_verified` in agent metadata.

## Sources

- https://learn.microsoft.com/credentials/certifications/d365-finance-and-operations-apps-solution-architect-expert/
- https://learn.microsoft.com/credentials/certifications/d365-supply-chain-management-functional-consultant-expert/
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-230
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-240
- https://learn.microsoft.com/credentials/certifications/d365-customer-experience-analyst-associate/
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-280
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/mb-260
- https://learn.microsoft.com/credentials/certifications/posts/dynamics-365-certification-and-exam-names-are-changing
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/overview
- https://learn.microsoft.com/dynamics365/guidance/business-processes/service-to-cash-create-process-service-work
- https://learn.microsoft.com/dynamics365/field-service/universal-resource-scheduling-for-field-service
- https://learn.microsoft.com/dynamics365/field-service/field-service-architecture
- https://learn.microsoft.com/dynamics365/customer-service/administer/overview-cases
- https://learn.microsoft.com/dynamics365/customer-service/administer/create-enhanced-sla
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-to-go-live
