# Microsoft 365 (M365) Field Research

> Deep-research report grounding the `vanguard-frontier-agentic` Microsoft 365 agent board.
> Method: fan-out web search + Microsoft Learn MCP, first-party sources prioritized, claims
> labeled by evidence and confidence. Certifications are treated as volatile and verified
> against current Microsoft Learn pages.
>
> **Date:** 2026-06-17 · **Orchestration note:** subagent fan-out was session-limited at
> run time, so the overseer executed the searches directly against Microsoft Learn / web.
> **Evidence scale:** E3 = official Microsoft docs · E4 = Microsoft Learn certification page.
> **Confidence:** High / Medium / Low (Low = re-verify before acting).

---

## 1. Certification & Applied Skills currency (2026)

| Claim | Status | Evidence | Confidence |
|---|---|---|---|
| **SC-401 "Administering Information Security in Microsoft 365"** is the current information-security exam → *Microsoft Certified: Information Security Administrator Associate*. Skills measured **as of April 27, 2026**; passing score 700. | **Active** (no retirement date listed) | E4 — [SC-401 exam](https://learn.microsoft.com/credentials/certifications/exams/sc-401/), [study guide](https://learn.microsoft.com/credentials/certifications/resources/study-guides/sc-401) | High |
| **SC-400 / Information Protection and Compliance Administrator Associate** is **retired** (exam + cert + renewal) on **May 31, 2025**; SC-401 is the successor. | **Retired 2025-05-31** | E4 — [SC-400 cert page](https://learn.microsoft.com/credentials/certifications/information-protection-administrator/) | High |
| **MS-102 (Microsoft 365 Administrator Expert)** active; **MD-102 (Endpoint Administrator)** is a prerequisite associate cert and active. | **Active** | E4 — Microsoft Learn certification catalog (MS-102 / MD-102) | High |
| **SC-300 (Identity & Access Administrator)** and **SC-200 (Security Operations Analyst)** active; **SC-100 (Cybersecurity Architect)** active. | **Active** | E4 — Microsoft Learn certification catalog | High |
| SC-401 scope now explicitly covers **Purview DLP, sensitivity labels, retention, Insider Risk, Audit, and "Protect data used by AI services" / DSPM for AI** — i.e., the modern M365 data-security surface. | **Active** | E4 — [SC-401 study guide](https://learn.microsoft.com/credentials/certifications/resources/study-guides/sc-401) | High |

**Board implication:** the board's identity/Copilot agents are cert-aligned. The planned **Purview data-security/compliance** specialist (in the plan, not yet built) maps cleanly to **SC-401** — and SC-401's "Protect data used by AI services / DSPM for AI" objective directly reinforces the `m365-copilot-readiness-governance` agent. Any board doc still referencing **SC-400** must be updated to **SC-401**.

---

## 2. Product & governance frameworks (current Microsoft guidance)

| Claim | Evidence | Confidence |
|---|---|---|
| **Copilot Control System** is Microsoft's governance framework for M365 Copilot & agents, organized in three pillars: **security & governance, management controls, measurement & reporting**. | E3 — [Copilot Control System](https://learn.microsoft.com/microsoft-365/copilot/copilot-control-system/overview), [security-governance](https://learn.microsoft.com/microsoft-365/copilot/copilot-control-system/security-governance) | High |
| **Oversharing** is addressed by the Microsoft **oversharing blueprint** powered by **Microsoft Purview** + **SharePoint Advanced Management (SAM)**; **Purview Data Security Posture Management (DSPM) for AI** added **item-level investigation and bulk remediation of overshared links** (Ignite 2025). | E3 — [Ignite 2025 security & governance for Copilot](https://techcommunity.microsoft.com/blog/microsoft365copilotblog/security-and-governance-innovations-for-microsoft-365-copilot-and-agents-from-ig/4476172), [configure a secure & governed foundation](https://learn.microsoft.com/microsoft-365/copilot/configure-secure-governed-data-foundation-microsoft-365-copilot) | High |
| Foundational governance is available at **E3/A3/G3** (M365 admin center, SAM, Purview); optimized controls at **E5/A5/G5** (Purview + Defender for Cloud Apps). | E3 — Ignite 2025 blog (above) | Medium |
| **Zero Trust for Microsoft 365 Copilot** uses a **7-layer model** (data protection, identity & access, app protection, device management, threat protection, secure Teams collaboration, user permissions to data). | E3 — [Apply Zero Trust to M365 Copilot](https://learn.microsoft.com/security/zero-trust/copilots/zero-trust-microsoft-365-copilot) | High |
| **Entra** least-privilege guidance: phishing-resistant MFA for admins via Conditional Access, **PIM** for just-in-time privileged roles, access reviews; least-privileged-roles-by-task. | E3 — [Entra least privilege](https://learn.microsoft.com/entra/id-governance/scenarios/least-privileged), [Zero Trust identity prerequisites](https://learn.microsoft.com/security/zero-trust/zero-trust-identity-device-access-policies-prerequisite) | High |
| Adoption/value is measured via **Copilot Analytics / Copilot Dashboard (Viva Insights)**, the **M365 Copilot readiness/usage report**, and the **AI adoption score** (target ≈3 active days/week). | E3 — [Copilot Control System measurement & reporting](https://learn.microsoft.com/microsoft-365/copilot/copilot-control-system/measurement-reporting), [AI adoption score](https://learn.microsoft.com/microsoft-365/admin/adoption/ai-adoption-score) | High |

**Board implication:** the `m365-copilot-readiness-governance`, `m365-identity-zero-trust`, `power-platform-governance-dataverse-security`, and `microsoft-business-impact-value-realization` agents are grounded on current frameworks. Add **DSPM for AI item-level / bulk remediation** and the **E3 vs E5 control tiers** to the Copilot agent's reference pack.

---

## 3. Capability gaps / missing roles (future wave)

| Candidate agent | Why it's enterprise-painful | Anchor | Confidence |
|---|---|---|---|
| **Purview data-security & compliance officer** (DLP, sensitivity labels, Insider Risk, eDiscovery, Audit, legal hold) | The single largest M365 governance surface; **SC-401** is built around it; only partially covered today by the Copilot-readiness agent. | SC-401; [Purview docs](https://learn.microsoft.com/purview/) | High |
| **Defender XDR SecOps analyst** (incident triage, hunting, response) | Threat detection/response is a distinct discipline (SC-200); the board only references it via protocol escalation. | SC-200; [Defender XDR](https://learn.microsoft.com/defender-xdr/) | High |
| **M365 backup / BCDR & data resilience** | Microsoft 365 Backup is GA; ransomware/retention recovery has no dedicated agent. | [M365 data resilience](https://learn.microsoft.com/microsoft-365/enterprise/) | Medium |
| **External collaboration / guest-access governance** | Teams/SharePoint external sharing + Entra B2B guest sprawl is a recurring oversharing root cause. | Entra External ID; SAM | Medium |
| **Intune endpoint / app-protection** specialist | Device compliance as the Conditional Access signal (MD-102) is referenced but not a standalone agent. | MD-102; [Intune Zero Trust](https://learn.microsoft.com/mem/intune/fundamentals/zero-trust-with-microsoft-intune) | Medium |

These confirm the board's documented "future wave" (Defender XDR, Purview insider-risk/eDiscovery, M365 BCDR, EA licensing) is the right backlog.

---

## 4. Market / competitive landscape

| Claim | Evidence | Confidence |
|---|---|---|
| Microsoft's **first-party** governance stack (Copilot Control System + Purview DSPM for AI + SAM + Defender) is positioned as the primary way to secure/govern M365 Copilot; the core message is "Copilot reflects your existing permissions posture — fix oversharing first." | E3 — [Secure & govern M365 Copilot](https://www.microsoft.com/security/business/solutions/data-security-b), Ignite 2025 blog | High |
| A third-party **SaaS security posture management (SSPM)** / data-access-governance market exists around the same oversharing problem, but first-party search surfaced mostly Microsoft sources — competitive specifics were not first-party-verifiable here. | — | Low (re-verify) |

---

## Verification debt / re-verify before publishing agent cert maps

- **E3 vs E5 control-tier** specifics (Low-Medium) — confirm against current Microsoft licensing guidance.
- **Third-party SSPM competitive landscape** (Low) — not first-party; verify with analyst sources if needed.
- Microsoft cert pages change frequently; re-confirm SC-401 "skills measured as of" date and any new **Applied Skills** before stamping `last_verified` in agent metadata.

## Sources

- https://learn.microsoft.com/credentials/certifications/exams/sc-401/
- https://learn.microsoft.com/credentials/certifications/resources/study-guides/sc-401
- https://learn.microsoft.com/credentials/certifications/information-protection-administrator/
- https://learn.microsoft.com/microsoft-365/copilot/copilot-control-system/overview
- https://learn.microsoft.com/microsoft-365/copilot/copilot-control-system/security-governance
- https://learn.microsoft.com/microsoft-365/copilot/copilot-control-system/measurement-reporting
- https://learn.microsoft.com/microsoft-365/copilot/configure-secure-governed-data-foundation-microsoft-365-copilot
- https://techcommunity.microsoft.com/blog/microsoft365copilotblog/security-and-governance-innovations-for-microsoft-365-copilot-and-agents-from-ig/4476172
- https://learn.microsoft.com/security/zero-trust/copilots/zero-trust-microsoft-365-copilot
- https://learn.microsoft.com/entra/id-governance/scenarios/least-privileged
- https://learn.microsoft.com/microsoft-365/admin/adoption/ai-adoption-score
- https://www.microsoft.com/security/business/solutions/data-security-b
