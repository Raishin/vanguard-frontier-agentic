# M365/D365/Power Platform/Copilot Agent Board — Architecture & Design Plan

**Status:** PLAN ONLY — no files created, no implementation started
**Date:** 2026-06-16
**Evidence scale:** E0=assumption | E1=user | E2=repo pattern | E3=MS docs (verify) | E4=MS cert (verify)

---

## Phase 0 — Prerequisites (must land before any agent validates)

1. Register provider `"microsoft"` in the provider registry — validation gate rejects unknown providers (E2).
2. Confirm `npm run validate` passes with zero agents before adding any (E2).
3. All agents below ship as `execution_tier: static-review` in v1. See §D.

---

## A. Five Maestro Agents

> Maestros route. They never answer domain questions, never call live APIs, max 4 parallel dispatches, and escalate on ambiguity. (E2 repo constraint)

---

### A1 — microsoft-maestro-agent

**Folder:** `agents/microsoft/microsoft-maestro-agent/`
**Purpose:** Top-level router across all MS workloads — M365, D365, Power Platform, Copilot. Entry point for any MS-scoped prompt.
**Business Pain:** Users don't know which product tower owns their problem; wrong specialist = wrong answer.
**In-Scope:** Triage + route to m365-, d365-, power-platform-, or copilot-governance-maestro-agent.
**Out-of-Scope:** Domain answers, tenant data access, licensing quotes, roadmap commitments.
**Routing Rules:**
- Identity/security/compliance signal → m365-maestro-agent
- Finance/SCM/CRM/Field/BC signal → d365-maestro-agent
- Canvas/model-driven/flows/Dataverse signal → power-platform-maestro-agent
- Copilot deployment, governance, AI policy signal → copilot-governance-maestro-agent
- Ambiguous multi-tower: ask one clarifying question before routing
**Escalation:** Route to human if prompt contains PII, live-tenant credentials, or blast-radius >100 users.
**Refusal:** Any prompt requesting credentials, impersonation, or tenant-wide destructive action.
**Evidence Required:** Product name, tenant context (optional), task description.
**Output Contract:** `{ Route: <sub-maestro>, Reason: <1 sentence>, Mode: read-only|advisory }` — never mutating.
**Companion Skill:** `microsoft-maestro-skill` (E2 — must be created in skills/)
**KPIs:** (1) Mis-route rate <5%, (2) Escalation-to-human rate tracked, (3) Median time-to-route <3s, (4) Refusal false-positive rate <2%.
**Anti-Patterns:** Answering domain questions directly; dispatching >4 specialists simultaneously; routing to mutating-tier agents without human confirm.

---

### A2 — m365-maestro-agent

**Folder:** `agents/microsoft/m365-maestro-agent/`
**Purpose:** Second-tier router across all M365 specialists (identity, endpoint, collaboration, compliance, security, Copilot readiness, licensing, adoption).
**Business Pain:** M365 surface is too broad for one agent; 10 specialists need clean dispatch logic.
**In-Scope:** Route to any of the 10 M365 specialists. Aggregate multi-specialist outputs.
**Out-of-Scope:** D365, Power Platform, direct tenant calls.
**Routing Rules:**
- Identity/CA/MFA/Entra → entra-identity-conditional-access-architect
- Devices/MDM/Compliance policy → intune-endpoint-administrator-agent
- Teams/calling/rooms → teams-collaboration-communications-architect
- Exchange/SPO/ODB → exchange-sharepoint-onedrive-information-steward
- DLP/labels/retention/eDiscovery → purview-data-security-compliance-officer
- Threats/incidents/XDR → defender-xdr-security-operations-analyst
- Copilot data exposure/readiness → m365-copilot-readiness-data-exposure-governor
- Licensing/SKU → m365-licensing-value-realization-analyst
- Adoption/change → m365-adoption-change-enablement-lead
- Tenant config/governance → m365-tenant-governance-architect
**Escalation:** Concurrent identity + security incident → page human SOC.
**Refusal:** Live tenant GA commands, credential injection.
**Evidence Required:** M365 workload signal, tenant tier (E1).
**Output Contract:** `{ Route: <specialist>, Reason, Mode: advisory }`.
**Companion Skill:** `m365-maestro-skill`
**KPIs:** Specialist mis-route <5%; cross-workload collision rate; escalation accuracy; 90th-pct dispatch latency.
**Anti-Patterns:** Collapsing identity + compliance into single dispatch; skipping escalation on confirmed security incidents.

---

### A3 — d365-maestro-agent

**Folder:** `agents/microsoft/d365-maestro-agent/`
**Purpose:** Second-tier router across 13 D365 specialists covering Finance, SCM, BC, CE, Field, Sales, CI/J, developer, integration, migration, testing, and security/SoD.
**Business Pain:** D365 projects fail when Finance architects make Field Service decisions; wrong specialist wastes sprint cycles.
**In-Scope:** Functional area routing, implementation-phase awareness (design/build/test/cutover/live).
**Out-of-Scope:** M365, Power Platform non-D365, live ERP writes.
**Routing Rules:**
- General solution design → d365-business-applications-solution-architect (first stop)
- Finance/GL/AP/AR → d365-finance-functional-consultant-agent
- SCM/inventory/manufacturing → d365-supply-chain-functional-consultant-agent
- SMB/BC → d365-business-central-functional-consultant-agent
- Contact center/CS → d365-customer-service-contact-center-consultant
- Field ops → d365-field-service-operations-architect
- CRM/pipeline → d365-sales-revenue-operations-architect
- Marketing/journeys → d365-customer-insights-journeys-architect
- Code/extensions/PCF → d365-fno-developer-extension-engineer
- Integration/dual-write → d365-integration-dual-write-architect
- Data migration → d365-data-migration-cutover-lead
- Testing/UAT/perf → d365-test-performance-go-live-readiness-lead
- Security/SoD → d365-security-segregation-of-duties-steward
**Escalation:** Cutover or go-live context → require human sponsor confirmation before any dispatch.
**Refusal:** Production ERP data requests; financial data extraction.
**Evidence Required:** D365 app (F&O/CE/BC), project phase, functional area.
**Output Contract:** `{ Route: <specialist>, Phase: <design|build|test|cutover>, Mode: advisory }`.
**Companion Skill:** `d365-maestro-skill`
**KPIs:** Phase-mismatch rate; escalation on cutover; specialist collision; time to first specialist route.
**Anti-Patterns:** Dispatching migration and cutover agents simultaneously without sequencing; routing BC questions to F&O specialist.

---

### A4 — power-platform-maestro-agent

**Folder:** `agents/microsoft/power-platform-maestro-agent/`
**Purpose:** Second-tier router across 7 Power Platform/Fabric specialists.
**Business Pain:** Low-code sprawl and ungoverned Dataverse environments create hidden blast radius; wrong specialist compounds the problem.
**In-Scope:** Route across Power Apps, Power Automate, Copilot Studio, Dataverse, ALM pipelines, Fabric/Power BI.
**Out-of-Scope:** M365 apps (not Power Platform surface), D365 backend.
**Routing Rules:**
- Solution architecture/design → power-platform-solution-architect-agent
- Dataverse security/roles/RLS → dataverse-security-model-architect
- Environment strategy/DLP/CoE → power-platform-governance-environment-strategy-lead
- ALM/pipelines/deployment → power-platform-alm-pipelines-engineer
- Copilot Studio bots/governance → copilot-studio-agent-governance-architect
- Flow risk/automation review → power-automate-automation-risk-reviewer
- Fabric/Power BI/lakehouse → fabric-power-bi-business-insights-architect
**Escalation:** Production environment deletion, DLP policy changes affecting >50 connectors.
**Refusal:** Live environment mutations, connector secrets.
**Evidence Required:** Platform component (Apps/Automate/Studio/Dataverse/BI), environment tier (dev/test/prod).
**Output Contract:** `{ Route: <specialist>, EnvironmentRisk: low|medium|high, Mode: advisory }`.
**Companion Skill:** `power-platform-maestro-skill`
**KPIs:** Environment-risk mis-classification; DLP escalation rate; CoE coverage score; dispatch accuracy.
**Anti-Patterns:** Routing governance questions to ALM engineer; treating prod and dev environments identically.

---

### A5 — copilot-governance-maestro-agent

**Folder:** `agents/microsoft/copilot-governance-maestro-agent/`
**Purpose:** Governs AI/Copilot deployment decisions across M365 Copilot, Copilot Studio, Fabric AI, and D365 Copilot features. Routes to data-exposure, Copilot Studio, or back to relevant domain maestro with a governance annotation.
**Business Pain:** Copilot features ship fast; governance lags; data oversharing incidents occur before readiness assessments run.
**In-Scope:** Copilot readiness, data exposure review, prompt policy, AI-use acceptable-use enforcement, cross-product Copilot dependency mapping.
**Out-of-Scope:** Non-AI features of M365/D365; execution of Copilot config changes.
**Routing Rules:**
- M365 Copilot data exposure → m365-copilot-readiness-data-exposure-governor (via m365-maestro)
- Copilot Studio bot design → copilot-studio-agent-governance-architect (via pp-maestro)
- D365 Copilot features → d365-maestro with governance annotation
- AI policy/RAI questions → emit advisory + escalate to human AI governance owner
**Escalation:** Any Copilot feature touching HR, legal, or financial data requires human sign-off.
**Refusal:** Disabling tenant-wide Copilot policies; bypassing RAI controls.
**Evidence Required:** Copilot product, data sensitivity classification, tenant Copilot license status (E1).
**Output Contract:** `{ Route: <specialist|human>, GovernanceRisk: low|medium|high|block, PolicyGap: <list>, Mode: advisory }`.
**Companion Skill:** `copilot-governance-skill`
**KPIs:** Data-exposure gaps flagged pre-deployment; RAI escalation rate; policy coverage score; time-to-governance-review.
**Anti-Patterns:** Treating all Copilot features as identical risk; skipping data classification before routing; approving RAI-adjacent changes without human.

---

## B. Full Agent Board Table

| Agent | Folder Path | Tier | Pain Solved | Business-Impact Hypothesis | MS Role/Cert Alignment (E4 verify) | Products/Workloads | Companion Skill | Execution Tier | Refusal Trigger | KPIs |
|---|---|---|---|---|---|---|---|---|---|---|
| microsoft-maestro-agent | agents/microsoft/microsoft-maestro-agent/ | Maestro | Wrong-tower routing | Reduce mis-routed MS tickets by >40% | N/A — router | All MS | microsoft-maestro-skill | static-review | Credentials, destructive, PII | Mis-route rate, escalation rate |
| m365-maestro-agent | agents/microsoft/m365-maestro-agent/ | Maestro | M365 specialist dispatch | Correct first dispatch >90% | N/A — router | M365 | m365-maestro-skill | static-review | Live tenant GA, credential injection | Mis-route, collision, latency |
| d365-maestro-agent | agents/microsoft/d365-maestro-agent/ | Maestro | D365 functional routing | Phase-aligned specialist >85% accuracy | N/A — router | D365 F&O/CE/BC | d365-maestro-skill | static-review | Production ERP data, cutover without sponsor | Phase-mismatch, collision |
| power-platform-maestro-agent | agents/microsoft/power-platform-maestro-agent/ | Maestro | Low-code sprawl routing | Environment-risk classification correct >90% | N/A — router | Power Platform, Fabric | power-platform-maestro-skill | static-review | Prod mutations, connector secrets | Risk mis-class, dispatch accuracy |
| copilot-governance-maestro-agent | agents/microsoft/copilot-governance-maestro-agent/ | Maestro | AI governance gap | Pre-deployment exposure review on 100% of Copilot rollouts | N/A — router | M365 Copilot, Studio, D365 AI | copilot-governance-skill | static-review | Disabling RAI controls, HR/legal data without sign-off | Policy coverage, RAI escalation |
| m365-tenant-governance-architect | agents/microsoft/m365-tenant-governance-architect/ | Specialist | Tenant config drift, ungoverned settings | Reduce configuration debt incidents 30% | MS-900/MS-102 (E4 verify) | M365 Admin, Tenant | m365-tenant-governance-skill | static-review | Live admin center mutations | Drift alerts, baseline coverage |
| entra-identity-conditional-access-architect | agents/microsoft/entra-identity-conditional-access-architect/ | Specialist | CA policy gaps, MFA bypass risk | Close >80% identity risk gaps in review | SC-300/AZ-500 (E4 verify) | Entra ID, CA, PIM, MFA | entra-ca-skill | static-review | Policy push to prod tenant without human | Gaps closed, MFA coverage |
| intune-endpoint-administrator-agent | agents/microsoft/intune-endpoint-administrator-agent/ | Specialist | Unmanaged endpoints, compliance gaps | Compliance policy coverage >95% of enrolled devices | MD-102 (E4 verify) | Intune, Autopilot, Endpoint | intune-endpoint-skill | static-review | Wipe commands, policy force-push | Enrollment coverage, compliance % |
| teams-collaboration-communications-architect | agents/microsoft/teams-collaboration-communications-architect/ | Specialist | Ungoverned Teams sprawl | Reduce orphaned teams/channels 40% | MS-700 (E4 verify) | Teams, Calling, Rooms | teams-collab-skill | static-review | Live call queue mutation | Sprawl score, guest access risk |
| exchange-sharepoint-onedrive-information-steward | agents/microsoft/exchange-sharepoint-onedrive-information-steward/ | Specialist | Data exposure in email/SPO/ODB | Oversharing risk reduced 50% before Copilot rollout | MS-203/MS-102 (E4 verify) | Exchange, SharePoint, OneDrive | exo-spo-skill | static-review | Mailbox data extraction | Oversharing surface, label coverage |
| purview-data-security-compliance-officer | agents/microsoft/purview-data-security-compliance-officer/ | Specialist | DLP, retention, eDiscovery gaps | Compliance posture score +20 points | SC-400 (E4 verify) | Purview, DLP, Sensitivity Labels, Retention | purview-compliance-skill | static-review | Policy delete, hold removal | DLP policy coverage, label adoption |
| defender-xdr-security-operations-analyst | agents/microsoft/defender-xdr-security-operations-analyst/ | Specialist | Slow incident triage, alert fatigue | MTTD/MTTR reduced 25% through structured review | SC-200 (E4 verify) | Defender XDR, Sentinel, MDE | defender-xdr-skill | static-review | Isolating endpoints, blocking accounts without human | Alert coverage, MTTD, MTTR |
| m365-copilot-readiness-data-exposure-governor | agents/microsoft/m365-copilot-readiness-data-exposure-governor/ | Specialist | Copilot oversharing sensitive data | 100% readiness gate before Copilot license grant | AZ-900/AI-102 (E4 verify) | M365 Copilot, SPO, Entra | copilot-readiness-skill | static-review | Enabling Copilot on unreviewed tenant | Exposure surface, readiness score |
| m365-licensing-value-realization-analyst | agents/microsoft/m365-licensing-value-realization-analyst/ | Specialist | License waste, wrong SKU assignment | License cost optimized >15% | MS-900 (E4 verify) | M365 Admin, License | licensing-skill | static-review | Bulk license removal | Active/assigned gap, cost delta |
| m365-adoption-change-enablement-lead | agents/microsoft/m365-adoption-change-enablement-lead/ | Specialist | Low feature adoption post-rollout | Adoption score +25% in 90 days | MS-900 (E4 verify) | M365 Apps, Viva | adoption-skill | static-review | None — advisory only | Adoption %, training coverage |
| d365-business-applications-solution-architect | agents/microsoft/d365-business-applications-solution-architect/ | Specialist | Bad architectural decisions early | Design anti-patterns caught pre-build | MB-700 (E4 verify) | D365 F&O, CE, BC | d365-arch-skill | static-review | Production schema changes | Design risk score, ADR coverage |
| d365-finance-functional-consultant-agent | agents/microsoft/d365-finance-functional-consultant-agent/ | Specialist | Finance config errors, audit risk | Finance go-live defects reduced 30% | MB-310 (E4 verify) | D365 Finance | d365-finance-skill | static-review | Live GL posting, period close | Config coverage, audit trail |
| d365-supply-chain-functional-consultant-agent | agents/microsoft/d365-supply-chain-functional-consultant-agent/ | Specialist | SCM flow gaps, inventory errors | Supply chain defects reduced 25% | MB-330 (E4 verify) | D365 SCM | d365-scm-skill | static-review | Production inventory adjustments | Flow coverage, integration errors |
| d365-business-central-functional-consultant-agent | agents/microsoft/d365-business-central-functional-consultant-agent/ | Specialist | SMB BC mis-configuration | BC go-live readiness score >90 | MB-800 (E4 verify) | D365 Business Central | d365-bc-skill | static-review | Production posting | Readiness score, open issues |
| d365-customer-service-contact-center-consultant | agents/microsoft/d365-customer-service-contact-center-consultant/ | Specialist | CS routing/SLA gaps | SLA breach rate reduced 20% | MB-230 (E4 verify) | D365 Customer Service, Omnichannel | d365-cs-skill | static-review | Live case closure | SLA coverage, routing accuracy |
| d365-field-service-operations-architect | agents/microsoft/d365-field-service-operations-architect/ | Specialist | Field schedule/resource gaps | First-time fix rate +15% | MB-240 (E4 verify) | D365 Field Service, Resource Scheduling | d365-fs-skill | static-review | Live dispatch changes | FTFR, schedule efficiency |
| d365-sales-revenue-operations-architect | agents/microsoft/d365-sales-revenue-operations-architect/ | Specialist | CRM data quality, pipeline gaps | Pipeline accuracy +20% | MB-210 (E4 verify) | D365 Sales | d365-sales-skill | static-review | Live opportunity deletion | Data quality score, pipeline coverage |
| d365-customer-insights-journeys-architect | agents/microsoft/d365-customer-insights-journeys-architect/ | Specialist | Fragmented customer data, GDPR risk | Segment accuracy +30%, consent compliance 100% | MB-260 (E4 verify) | D365 Customer Insights, Journeys | d365-ci-skill | static-review | PII export, bulk delete | Segment quality, consent coverage |
| d365-fno-developer-extension-engineer | agents/microsoft/d365-fno-developer-extension-engineer/ | Specialist | Over-customization, upgrade risk | ISV/customization footprint reduced 20% | MB-500 (E4 verify) | D365 F&O developer, X++, PCF | d365-dev-skill | static-review | Deploying untested extensions to prod | Extension risk score, test coverage |
| d365-integration-dual-write-architect | agents/microsoft/d365-integration-dual-write-architect/ | Specialist | Dual-write sync failures, data corruption | Integration error rate <1% | MB-700/MB-500 (E4 verify) | D365 F&O+CE, Dataverse, Dual-write | d365-integration-skill | static-review | Enabling dual-write without mapping review | Sync error rate, mapping coverage |
| d365-data-migration-cutover-lead | agents/microsoft/d365-data-migration-cutover-lead/ | Specialist | Data quality failures at cutover | Cutover defects <5 critical | MB-700 (E4 verify) | D365 F&O/CE/BC, DIXF, DMF | d365-migration-skill | static-review | Production data load without rehearsal sign-off | Migration accuracy, cutover rehearsal score |
| d365-test-performance-go-live-readiness-lead | agents/microsoft/d365-test-performance-go-live-readiness-lead/ | Specialist | Untested go-live, perf degradation | Performance baseline met before go-live | MB-700 (E4 verify) | D365 F&O/CE, LCS, RSAT | d365-testing-skill | static-review | Go-live sign-off without completed perf test | Test coverage %, perf baseline delta |
| d365-security-segregation-of-duties-steward | agents/microsoft/d365-security-segregation-of-duties-steward/ | Specialist | SoD violations, audit findings | SoD conflicts reduced to zero critical in audit | MB-700/SC-900 (E4 verify) | D365 F&O/CE security, SoD | d365-sod-skill | static-review | Granting conflicting role combos in prod | SoD conflict count, audit finding rate |
| power-platform-solution-architect-agent | agents/microsoft/power-platform-solution-architect-agent/ | Specialist | Ungoverned solution design | Solution complexity score within guardrails | PL-600 (E4 verify) | Power Apps, Dataverse, Connectors | pp-arch-skill | static-review | Production solution import without review | Complexity score, tech debt index |
| dataverse-security-model-architect | agents/microsoft/dataverse-security-model-architect/ | Specialist | Dataverse role/RLS sprawl | Role conflicts zero in prod | PL-400/PL-600 (E4 verify) | Dataverse, Business Units, Security Roles | dataverse-security-skill | static-review | Role assignment in prod without review | Role conflict count, RLS coverage |
| power-platform-governance-environment-strategy-lead | agents/microsoft/power-platform-governance-environment-strategy-lead/ | Specialist | Ungoverned environment proliferation | Environment count within CoE policy | PL-600/PL-900 (E4 verify) | Power Platform CoE, DLP, Environments | pp-governance-skill | static-review | Environment deletion, DLP changes | Env count vs policy, DLP connector coverage |
| power-platform-alm-pipelines-engineer | agents/microsoft/power-platform-alm-pipelines-engineer/ | Specialist | Manual deployments, pipeline gaps | Deployment success rate >98% | PL-400 (E4 verify) | Power Platform Pipelines, Azure DevOps | pp-alm-skill | static-review | Deploying untested solution to prod | Pipeline coverage, deployment failure rate |
| copilot-studio-agent-governance-architect | agents/microsoft/copilot-studio-agent-governance-architect/ | Specialist | Ungoverned bots, data leakage | 100% of published bots have governance review | PL-200/AI-102 (E4 verify) | Copilot Studio, Power Virtual Agents | copilot-studio-skill | static-review | Publishing bot to prod without review | Bot governance coverage, data-leak incidents |
| power-automate-automation-risk-reviewer | agents/microsoft/power-automate-automation-risk-reviewer/ | Specialist | High-risk flows with no review | Risk-rated flow inventory 100% complete | PL-900/PL-400 (E4 verify) | Power Automate, Premium Connectors | automate-risk-skill | static-review | Enabling high-risk flows in prod | High-risk flow count, review coverage |
| fabric-power-bi-business-insights-architect | agents/microsoft/fabric-power-bi-business-insights-architect/ | Specialist | Ungoverned BI, inconsistent metrics | Single metric definition adopted >80% | DP-600/PL-300 (E4 verify) | Fabric, Power BI, Lakehouse | fabric-bi-skill | static-review | Publishing unreviewed reports to prod workspace | Metric consistency score, certified report % |

---

## C. Detailed Specs — 8 Highest-Impact Specialists

---

### C1 — entra-identity-conditional-access-architect

**Folder:** `agents/microsoft/entra-identity-conditional-access-architect/`
**Tier:** Specialist
**Primary Business Pain:** CA policy gaps leave MFA bypasses, legacy auth open, and privileged accounts unprotected — each is an audit finding and a breach vector.
**Business-Impact Hypothesis:** Closing critical CA policy gaps (MFA enforcement, block legacy auth, PIM for privileged roles) reduces identity-related breach risk by >60% based on Microsoft Security Intelligence data (E3 verify).
**MS Role/Cert Alignment:** SC-300 Identity and Access Administrator (E4 verify — confirm active, not retired); AZ-500 Azure Security Engineer overlaps PIM scope (E4 verify).
**Products/Workloads:** Microsoft Entra ID, Conditional Access, Privileged Identity Management (PIM), Microsoft Authenticator, Entra ID Protection.
**Companion Skill:** `entra-ca-skill`
**Minimum Evidence Required:** Exported CA policy list (JSON or screenshot); Entra sign-in risk report; PIM role assignments; named location list.
**Allowed Tools:** Read files, grep policy exports, web fetch MS docs, advisory output. No write to tenant. (E2 least-privilege)
**Forbidden Actions:** Push CA policy changes directly; disable policies; modify PIM settings; access live Entra data.
**Refusal Conditions:** Requests to bypass MFA for any account; requests to disable named-location blocking; live tenant credential presented.
**Escalation Partners:** defender-xdr-security-operations-analyst (active incident); m365-tenant-governance-architect (baseline drift); human identity owner for any policy push.
**Output Contract:** `{ PolicyGaps: [{id, risk, recommendation}], PIMFindings: [], EstimatedRiskReduction: %, Mode: advisory, RequiresHumanApproval: true }`
**KPIs:** (1) CA gap coverage — % of recommended policies reviewed; (2) MFA enforcement rate across user population; (3) Legacy auth blocked %; (4) PIM activation request anomalies flagged.
**Anti-Patterns:** Recommending "report-only" mode as sufficient; ignoring break-glass account exclusions; treating all CA policies as identical risk level.

---

### C2 — m365-copilot-readiness-data-exposure-governor

**Folder:** `agents/microsoft/m365-copilot-readiness-data-exposure-governor/`
**Tier:** Specialist
**Primary Business Pain:** M365 Copilot surfaces any file a user has read access to in semantic search — overshared SharePoint/OneDrive files become AI-amplified data leaks.
**Business-Impact Hypothesis:** Tenants without a readiness gate expose 30-60% of sensitive files to broader audiences through Copilot (E3 verify — Microsoft Secure Productivity guidance). Blocking Copilot license grant until readiness score >threshold prevents class of incidents.
**MS Role/Cert Alignment:** SC-400 (Purview, sensitivity labels) + MS-102 (M365 admin) — no single cert covers full scope (E4 verify caveat). AI-102 for AI service governance (E4 verify).
**Products/Workloads:** M365 Copilot, SharePoint Online, OneDrive, Microsoft Purview (sensitivity labels, DLP), Entra ID (group-based license assignment).
**Companion Skill:** `copilot-readiness-skill`
**Minimum Evidence Required:** Tenant SharePoint sharing settings export; sensitivity label deployment status; Purview DLP policy list; number of Copilot licenses requested; data classification inventory (if any).
**Allowed Tools:** Read policy exports, advisory analysis, web fetch MS Copilot guidance. No live tenant calls.
**Forbidden Actions:** Enabling Copilot licenses; modifying sharing settings; removing sensitivity labels; accessing file contents.
**Refusal Conditions:** Copilot rollout requested on tenant with no sensitivity labels and oversharing not addressed; HR/legal/financial data scoped without Purview controls.
**Escalation Partners:** purview-data-security-compliance-officer (label deployment); entra-identity-conditional-access-architect (group-based access); copilot-governance-maestro-agent.
**Output Contract:** `{ ReadinessScore: 0-100, BlockingGaps: [], RecommendedActions: [], GoLiveRecommendation: block|conditional|approve, Mode: advisory }`
**KPIs:** (1) Readiness score at assessment vs. at license grant; (2) Oversharing surface area (sites/files with broad permissions); (3) Sensitivity label coverage %; (4) Copilot incidents attributable to pre-rollout gaps.
**Anti-Patterns:** Treating Copilot readiness as a license-only question; ignoring SharePoint external sharing settings; assessing only email and ignoring SPO/ODB.

---

### C3 — purview-data-security-compliance-officer

**Folder:** `agents/microsoft/purview-data-security-compliance-officer/`
**Tier:** Specialist
**Primary Business Pain:** DLP policy gaps, missing retention policies, and absent sensitivity labels create regulatory exposure (GDPR, HIPAA, CCPA) and audit findings.
**Business-Impact Hypothesis:** Structured Purview policy review closes the highest-severity audit findings 60% faster than manual review (E0 — needs customer evidence).
**MS Role/Cert Alignment:** SC-400 Microsoft Information Protection Administrator (E4 verify — confirm active). MS-102 overlaps for tenant compliance settings (E4 verify).
**Products/Workloads:** Microsoft Purview (DLP, Sensitivity Labels, Retention, eDiscovery, Audit), Compliance Manager, Communication Compliance.
**Companion Skill:** `purview-compliance-skill`
**Minimum Evidence Required:** DLP policy export; sensitivity label taxonomy; retention schedule; Compliance Manager score; regulatory framework in scope (GDPR/HIPAA/etc.).
**Allowed Tools:** Read policy exports, advisory gap analysis, web fetch Purview docs.
**Forbidden Actions:** Delete holds; remove retention policies; modify eDiscovery cases; access content under legal hold.
**Refusal Conditions:** eDiscovery requests without legal instruction; requests to circumvent GDPR deletion obligations; hold removal without legal sign-off.
**Escalation Partners:** m365-copilot-readiness-data-exposure-governor (pre-Copilot); exchange-sharepoint-onedrive-information-steward (data location); legal/compliance team for hold decisions.
**Output Contract:** `{ ComplianceScore: delta, DLPGaps: [], RetentionGaps: [], LabelGaps: [], RegulatoryRisk: low|medium|high|critical, Mode: advisory }`
**KPIs:** (1) DLP policy coverage %; (2) Sensitivity label adoption % across M365; (3) Retention policy coverage %; (4) Compliance Manager score delta.
**Anti-Patterns:** Treating DLP and retention as independent; ignoring Communication Compliance for regulated industries; recommending label deletion without eDiscovery hold check.

---

### C4 — d365-business-applications-solution-architect

**Folder:** `agents/microsoft/d365-business-applications-solution-architect/`
**Tier:** Specialist
**Primary Business Pain:** Architecture decisions made in sprint 1 constrain the entire programme — wrong data model, over-customization, wrong integration pattern cost millions to unwind.
**Business-Impact Hypothesis:** Catching 3+ critical design anti-patterns in design phase saves >200 hours of rework per programme (E0 — estimate; customer data needed).
**MS Role/Cert Alignment:** MB-700 Microsoft Dynamics 365: Finance and Operations Apps Solution Architect (E4 verify — confirm active vs. retired/renamed). Note: MB-700 was retired and replaced; verify current cert name (E4 — active verification required).
**Products/Workloads:** D365 Finance, D365 CE, D365 Business Central, Dataverse, Power Platform, Azure Integration Services.
**Companion Skill:** `d365-arch-skill`
**Minimum Evidence Required:** Solution blueprint or design doc; customization list; integration map; data model draft; ISV list; hosting model (cloud/hybrid).
**Allowed Tools:** Read design artifacts, advisory review, web fetch MS docs.
**Forbidden Actions:** Approve production deployments; access production data; push code.
**Refusal Conditions:** Requests to architect around audit controls; designs that embed PII in unprotected fields; requests to suppress upgrade blockers.
**Escalation Partners:** d365-security-segregation-of-duties-steward (security model); d365-integration-dual-write-architect (integration); d365-test-performance-go-live-readiness-lead (pre-go-live).
**Output Contract:** `{ DesignRisks: [{area, severity, recommendation}], AntiPatterns: [], ArchitectureDecisionRecords: [], Mode: advisory }`
**KPIs:** (1) Critical design issues caught in design phase vs. build phase; (2) Customization footprint score; (3) ADR coverage %; (4) Architecture review completion rate.
**Anti-Patterns:** Approving ISV solutions without upgrade-path review; designing integrations without error-handling patterns; treating Business Central as a scaled-down F&O.

---

### C5 — d365-security-segregation-of-duties-steward

**Folder:** `agents/microsoft/d365-security-segregation-of-duties-steward/`
**Tier:** Specialist
**Primary Business Pain:** SoD conflicts in D365 F&O/CE security roles create fraud risk and audit findings — auditors reject go-live with critical SoD violations.
**Business-Impact Hypothesis:** Catching SoD conflicts pre-go-live eliminates the most common audit blocker in D365 F&O programmes (E0 — common pattern, not quantified).
**MS Role/Cert Alignment:** MB-700 (solution architect — security domain) + SC-900 (security fundamentals) (E4 verify). No dedicated D365-security-only cert confirmed (E3 verify).
**Products/Workloads:** D365 Finance (security roles, duties, privileges), D365 CE (security roles, business units), D365 Business Central (permission sets).
**Companion Skill:** `d365-sod-skill`
**Minimum Evidence Required:** Security role export; user-to-role assignment matrix; SoD conflict matrix (if available); audit framework (SOX, GDPR, local GAAP rules).
**Allowed Tools:** Read role exports, advisory SoD analysis, web fetch MS security docs.
**Forbidden Actions:** Assign roles in production; delete roles; modify privileges; access user financial transactions.
**Refusal Conditions:** Requests to grant a user roles that create a critical SoD conflict without compensating control documentation; requests to suppress audit logging.
**Escalation Partners:** d365-business-applications-solution-architect; d365-finance-functional-consultant-agent (finance-specific duties); human audit/compliance team.
**Output Contract:** `{ SoDConflicts: [{roles, duties, riskLevel, compensatingControl}], CriticalCount: n, GoLiveBlock: bool, Mode: advisory }`
**KPIs:** (1) Critical SoD conflict count at go-live gate; (2) Role count vs. recommended minimum; (3) SoD matrix coverage %; (4) Audit findings attributable to SoD post-go-live.
**Anti-Patterns:** Treating CE and F&O SoD models as identical; accepting compensating controls without documenting them; reviewing only finance roles and ignoring CE security model.

---

### C6 — d365-data-migration-cutover-lead

**Folder:** `agents/microsoft/d365-data-migration-cutover-lead/`
**Tier:** Specialist
**Primary Business Pain:** Data migration failures and untested cutover sequences cause go-live delays costing $100K–$1M/day in enterprise ERP programmes (E0 — industry estimate, verify with customer data).
**Business-Impact Hypothesis:** Structured cutover rehearsal review catches >70% of critical migration defects before go-live weekend (E0).
**MS Role/Cert Alignment:** MB-700 (E4 verify — see architect note above on retirement status). Lifecycle Services (LCS) expertise is operational, not cert-covered (E3 verify).
**Products/Workloads:** D365 Finance (DIXF/DMF), D365 CE (Data Import Wizard, SDK), D365 Business Central (configuration packages, RapidStart), Azure Data Factory (migration pipelines), LCS (for F&O).
**Companion Skill:** `d365-migration-skill`
**Minimum Evidence Required:** Migration strategy document; data entity list; cutover plan; rehearsal results (if available); data volume estimates; legacy system data quality report.
**Allowed Tools:** Read migration artifacts, advisory review, web fetch MS migration docs.
**Forbidden Actions:** Execute data loads in production; access legacy system data directly; approve go/no-go without human sponsor.
**Refusal Conditions:** Go-live approval without at least one complete cutover rehearsal documented; migration with unresolved critical data quality issues; financial period cutover without CFO sign-off.
**Escalation Partners:** d365-business-applications-solution-architect; d365-test-performance-go-live-readiness-lead; d365-finance-functional-consultant-agent (finance data validation); human programme sponsor.
**Output Contract:** `{ MigrationRisks: [], CriticalBlockers: [], RehearsalReadiness: score, GoLiveRecommendation: block|conditional|approve, Mode: advisory, RequiresHumanApproval: true }`
**KPIs:** (1) Migration defects found in rehearsal vs. production; (2) Cutover duration vs. plan; (3) Data quality score post-migration; (4) Rollback readiness documented (yes/no).
**Anti-Patterns:** Treating DIXF and DMF as interchangeable without testing; skipping rollback plan; approving cutover without rehearsal; treating all entities as equal-risk.

---

### C7 — dataverse-security-model-architect

**Folder:** `agents/microsoft/dataverse-security-model-architect/`
**Tier:** Specialist
**Primary Business Pain:** Dataverse security roles proliferate uncontrolled; Business Unit hierarchy mismatches cause data leakage or over-restriction; RLS gaps expose sensitive records.
**Business-Impact Hypothesis:** Security model review before production rollout catches >80% of role-conflict and data-leakage issues that would require costly post-go-live remediation (E0).
**MS Role/Cert Alignment:** PL-400 Microsoft Power Platform Developer + PL-600 Microsoft Power Platform Solution Architect (E4 verify — confirm PL-600 active). No dedicated Dataverse-security cert (E3 verify).
**Products/Workloads:** Microsoft Dataverse, Power Apps (model-driven), D365 CE apps, Business Units, Security Roles, Column-Level Security, Row-Level Security.
**Companion Skill:** `dataverse-security-skill`
**Minimum Evidence Required:** Security role export (solution export or admin center); Business Unit hierarchy diagram; column security profile list; record access requirements (functional spec).
**Allowed Tools:** Read exports and specs, advisory analysis, web fetch Dataverse docs.
**Forbidden Actions:** Assign security roles in production; modify Business Unit hierarchy; delete security profiles.
**Refusal Conditions:** Requests to grant system admin to non-admin service accounts; role designs that give all-records access to shared service accounts; column security bypass requests.
**Escalation Partners:** power-platform-governance-environment-strategy-lead (environment scope); d365-security-segregation-of-duties-steward (D365 CE SoD); d365-business-applications-solution-architect (solution architecture).
**Output Contract:** `{ RoleConflicts: [], DataLeakageRisks: [], BUHierarchyIssues: [], RemediationPlan: [], Mode: advisory }`
**KPIs:** (1) Security role count vs. recommended minimum; (2) Role conflict count; (3) Column security coverage on sensitive fields; (4) Post-go-live security incidents attributable to model.
**Anti-Patterns:** Using system admin role for all integrations; not modeling Business Unit inheritance; treating Dataverse and D365 F&O security as the same model.

---

### C8 — power-platform-governance-environment-strategy-lead

**Folder:** `agents/microsoft/power-platform-governance-environment-strategy-lead/`
**Tier:** Specialist
**Primary Business Pain:** Uncontrolled environment proliferation (hundreds of dev/sandbox envs), absent DLP policies, and no CoE Starter Kit = ungoverned low-code estate with hidden blast radius.
**Business-Impact Hypothesis:** CoE + DLP + environment strategy reduces ungoverned flow count 50% and connector risk incidents 30% (E0 — Microsoft CoE guidance exists but outcome data is E3 verify).
**MS Role/Cert Alignment:** PL-600 (E4 verify) + PL-900 (E4 verify). CoE Starter Kit is a Microsoft product but not cert-specific (E3 verify).
**Products/Workloads:** Power Platform Admin Center, CoE Starter Kit, DLP Policies, Environment Management, Power Platform Pipelines.
**Companion Skill:** `pp-governance-skill`
**Minimum Evidence Required:** Environment inventory (admin center export); DLP policy list; CoE Starter Kit deployment status; connector tier list; maker count.
**Allowed Tools:** Read admin exports, advisory analysis, web fetch Power Platform governance docs.
**Forbidden Actions:** Delete environments; modify DLP in production without human approval; disable connectors tenant-wide.
**Refusal Conditions:** Environment deletion without data backup confirmed; DLP change that blocks production solutions without impact assessment; enabling premium connectors tenant-wide without policy.
**Escalation Partners:** power-platform-alm-pipelines-engineer (deployment patterns); dataverse-security-model-architect (Dataverse security); power-platform-maestro-agent for cross-specialist coordination.
**Output Contract:** `{ EnvironmentRisk: score, DLPGaps: [], CoEReadiness: low|partial|full, RecommendedActions: [], Mode: advisory }`
**KPIs:** (1) Environment count within policy; (2) DLP connector coverage %; (3) CoE Starter Kit adoption %; (4) High-risk flow count with no DLP coverage.
**Anti-Patterns:** Treating dev and prod DLP policy as identical; ignoring trial environments in inventory; recommending CoE without scoping implementation effort.

---

## D. Execution-Tier and Live-Guard Classification

| Agent Group | v1 Tier | Live-Guard? | Justification |
|---|---|---|---|
| All 5 Maestros | static-review | YES — never auto-dispatch to mutating agents | Routers only; no answers, no mutations |
| All 10 M365 Specialists | static-review | SOME — see below | M365 tenant config changes are irreversible at speed |
| entra-identity-conditional-access-architect | static-review | YES | CA policy errors lock out users tenant-wide |
| defender-xdr-security-operations-analyst | static-review | YES | Endpoint isolation / account blocking needs human |
| intune-endpoint-administrator-agent | static-review | YES | Wipe commands are destructive |
| All 13 D365 Specialists | static-review | SOME — see below | ERP mutations in prod have financial and legal consequences |
| d365-data-migration-cutover-lead | static-review | YES | Production data load requires human sponsor approval |
| d365-security-segregation-of-duties-steward | static-review | YES | Role changes in prod are audit events |
| d365-test-performance-go-live-readiness-lead | static-review | YES | Go-live sign-off cannot be automated |
| All 7 Power Platform/Copilot Specialists | static-review | SOME — see below | |
| power-platform-governance-environment-strategy-lead | static-review | YES | DLP/env deletion is high blast-radius |
| copilot-studio-agent-governance-architect | static-review | YES | Publishing bots is externally visible |
| Remaining specialists (M365 adoption, licensing, etc.) | static-review | NO | Advisory only; no mutation path exists |

**v1 Mandate:** ALL agents ship `execution_tier: static-review`. No agent in this board executes writes, pushes policies, or invokes production APIs in v1. Mutating-runtime tier deferred to Wave 2, gated on: (a) human-confirm protocol implemented in maestro, (b) rollback mechanism documented, (c) blast-radius assessment per agent. (E2 repo pattern — least privilege, safe rollback.)

---

## E. Keep / Merge / Split / Kill Recommendations

| Decision | Agents | Reason |
|---|---|---|
| MERGE | `exchange-sharepoint-onedrive-information-steward` + `purview-data-security-compliance-officer` | 60%+ overlap: both address data exposure, labeling, and DLP. SPO/ODB stewardship IS Purview label enforcement. Split only if org has distinct Exchange admin and Compliance admin roles — otherwise one agent, two sub-modes. **Recommendation: MERGE into `m365-information-protection-steward`; flag for v2 split if team structure demands it.** |
| MERGE (consider) | `d365-test-performance-go-live-readiness-lead` + `d365-data-migration-cutover-lead` | Both are go-live gates. Separation is valid for large programmes; at SMB/mid-market scale they collapse into one "go-live readiness" agent. **Recommendation: KEEP separate for enterprise; add routing condition in d365-maestro based on programme scale.** |
| SPLIT | `d365-business-applications-solution-architect` | Covers F&O, CE, and BC — three distinct architectures. One agent is correct for the top-level maestro's first stop, but it should clearly route sub-questions to the relevant functional consultant. Risk: agent tries to answer F&O and BC questions with equal depth. **Recommendation: KEEP as first-stop architect; enforce strict "I route, I don't answer deep functional questions" discipline.** |
| KILL (weak) | `m365-adoption-change-enablement-lead` | Useful but lowest technical leverage. Adoption advice is largely human-managed; agent adds limited value beyond a checklist. **Recommendation: KILL as standalone agent. Merge adoption metrics into `m365-licensing-value-realization-analyst` (adoption = license value). Revisit in Wave 2 if Viva Insights data integration available.** |
| KEEP | `copilot-governance-maestro-agent` (separate from copilot-studio) | Copilot governance spans M365, D365, and Power Platform — it genuinely needs a cross-tower maestro role. Do not merge into m365-maestro. |
| KEEP | `dataverse-security-model-architect` (separate from D365 SoD) | D365 F&O SoD (duties/privileges in AX security model) is fundamentally different from Dataverse security roles/BU hierarchy. Merging produces an agent that's expert in neither. |
| MISSING | `azure-integration-services-architect` | D365/Power Platform programmes invariably involve Logic Apps, API Management, or Service Bus for integration. The `d365-integration-dual-write-architect` covers dual-write but not Azure-native integration patterns. **Recommendation: Add in Wave 2.** |
| MISSING | `m365-external-collaboration-guest-access-steward` | Teams guest access, B2B/B2C, SPO external sharing are distinct enough from the information steward's remit to warrant a specialist in highly regulated sectors. **Recommendation: Evaluate in Wave 2 based on customer demand.** |
| FLAG | All cert alignments marked E4 | MB-700 retirement/rename status, PL-600 active status, and SC-400 scope must be verified against current Microsoft Learn certification catalog before agent AGENT.md is written. Do not publish cert claims as fact. |

---

## Implementation Sequencing (Not Implementation — Sequence Only)

1. **Phase 0:** Register `microsoft` provider. Validate baseline passes. (Blocker for everything.)
2. **Phase 1:** `microsoft-maestro-agent` only. Validate routing logic with stub downstream agents.
3. **Phase 2:** Four sub-maestros + companion skills stubs.
4. **Phase 3:** 8 highest-impact specialists (§C above) with full harness files.
5. **Phase 4:** Remaining 22 specialists in priority order per business demand.
6. **Wave 2 gate:** Any mutating-runtime tier agent — only after live-guard protocol and blast-radius assessment complete.

---

*Next: `02-skill-packs-and-templates.md` — companion skill structure, SKILL.md frontmatter contracts, allowed-tools baseline per tier, and skill-manifest update procedure.*
