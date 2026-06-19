---
name: audit-evidence-mapping-protocol
description: Use this skill when compliance controls must be mapped to audit evidence, when evidence collection, retention policy, and legal-hold status need to be assessed across Microsoft 365 workloads, or when an attestation package must be assembled for an auditor or regulator. Defines the end-to-end flow from control identification through evidence discovery, gap analysis, retention verification, legal-hold confirmation, and attestation sign-off. Does not serve as an authorization to release evidence to external parties; that requires a human compliance or legal owner. Does not replace qualified legal or audit counsel.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: compliance
  lifecycle: experimental
---

# Audit Evidence Mapping Protocol

## Purpose
This skill defines how compliance controls are traced to concrete audit evidence, how that evidence is validated for completeness and retention, and how an attestation package is assembled and handed off to the compliance or legal owner. It exists so that evidence gaps are surfaced before an audit begins, retention and legal-hold status are verified against Microsoft Purview policy, and every handoff carries a documented chain of custody. It does not authorize evidence release, does not replace legal or audit counsel, and does not modify retention policies or legal-hold configuration.

## When to use
- A compliance or audit team must map controls to evidence artifacts across Microsoft 365 workloads.
- Audit log retention status needs to be verified against organizational policy before a regulatory review.
- A legal hold must be confirmed or flagged as incomplete before an eDiscovery matter proceeds.
- An attestation package must be assembled from distributed evidence sources (Exchange, SharePoint, Entra ID, Dataverse).
- Evidence completeness gaps are suspected and must be inventoried before the auditor window opens.

## When NOT to use
- Evidence has already been assembled and validated — hand off directly to the compliance owner.
- The matter is an active legal proceeding requiring litigation-hold decisions — escalate to legal counsel immediately.
- A retention policy change or legal-hold placement is required — that is a production-impacting action and must be escalated to the Purview compliance administrator and the legal owner.
- The matter involves special-category personal data and jurisdiction is unknown — stop and escalate.

## Participating agents
- `d365-security-sod-governance-agent` — primary: maps segregation-of-duties controls to evidence artifacts in Dynamics 365 and Power Platform audit logs
- `m365-maestro-agent` — orchestrates cross-workload evidence discovery; escalation path to Microsoft Purview specialists (planned: purview-compliance-specialist-agent)

## Inputs required
- Control framework or control list (e.g., ISO 27001, SOC 2, NIST CSF control IDs)
- Workload scope (Exchange Online, SharePoint, Entra ID, Dynamics 365, Power Platform, Teams)
- Retention requirements (regulatory, contractual, or internal policy)
- Legal-hold case reference (if applicable)
- Audit window dates

## Evidence required
- Existing audit log retention policies from Microsoft Purview (Audit Standard or Audit Premium tier)
- Current legal-hold case list and hold scope from Microsoft Purview eDiscovery
- Control register or risk register from the compliance team
- Prior audit findings or evidence gaps (if available)

## Workflow

1. **Ingest control framework** — receive the control list and map each control to a workload and evidence type (audit log, configuration export, access review, policy document).
2. **Scope workloads** — confirm which Microsoft 365 services and Dynamics 365 environments are in scope; note licensing tier (Audit Standard vs. Audit Premium) as it affects default retention (180 days vs. 1 year vs. 10 years with add-on).
3. **Discover evidence artifacts** — for each control, identify the evidence artifact location (Purview Audit log, SharePoint document library, Entra ID access review, Power Platform DLP report).
4. **Verify retention status** — confirm that audit log retention policies cover the required period; flag any control whose evidence falls under the 180-day standard default that requires a longer period.
5. **Check legal-hold coverage** — if a legal hold is active, confirm that evidence in scope is covered by an eDiscovery hold; flag uncovered artifacts.
6. **Identify evidence gaps** — produce a gap register: controls with missing artifacts, expired retention, or uncovered legal-hold scope.
7. **Escalation gate: evidence completeness** — if gap register is non-empty, pause and escalate to compliance owner and m365-maestro-agent for Purview remediation.
8. **Escalation gate: retention or legal-hold deficiency** — if retention is insufficient or a legal hold is not in place, stop and escalate to the Purview compliance administrator and legal owner before proceeding.
9. **Assemble attestation package** — for each control, compile: control ID, evidence artifact reference, evidence location, retention expiry, legal-hold status, evidence quality rating (complete / partial / missing), and open questions.
10. **Review for privilege and privacy sensitivity** — label package sections with privilege sensitivity and privacy sensitivity before handoff.
11. **Hand off to compliance owner** — deliver attestation package with a do-not-do list and open questions; require human sign-off before any evidence is transmitted to an external auditor or regulator.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Evidence completeness | Any control has missing or partial evidence | Pause; escalate to compliance owner; open remediation task |
| Retention / legal-hold | Retention < required period OR legal hold not confirmed | Stop; escalate to Purview admin and legal owner before proceeding |
| Privilege sensitivity | Evidence touches legally privileged communications | Flag; route to legal counsel before inclusion in package |
| Special-category data | Evidence includes health, biometric, or regulated personal data | Stop; confirm jurisdiction and privacy owner before proceeding |

## Refusal triggers
- A request is made to delete, truncate, or shorten retention of in-scope evidence — refuse and escalate.
- A request is made to release evidence directly to an external auditor without human compliance owner sign-off — refuse.
- Credentials, tenant IDs, or personal data are requested to perform evidence discovery — refuse; work from sanitized signals only.
- Jurisdiction of regulated personal data is unknown — refuse to proceed; escalate.

## Handoff rules
- Every handoff carries: control ID list, evidence gap register, retention status summary, legal-hold status, privilege sensitivity label, privacy sensitivity label, open questions, and a do-not-do list.
- No agent transmits evidence to external parties. Human compliance owner authorizes all external transmissions.
- Post-handoff, the receiving agent or human confirms receipt and records the handoff in the audit log.

## KPIs
- Percentage of controls with complete, verified evidence before audit window opens
- Number of retention gaps identified and remediated before audit
- Time from control mapping to attestation package delivery
- Number of escalations requiring human intervention

## References
- [Learn about auditing solutions in Microsoft Purview](https://learn.microsoft.com/purview/audit-solutions-overview)
- [Manage audit log retention policies](https://learn.microsoft.com/purview/audit-log-retention-policies)
- [Microsoft Purview eDiscovery solutions](https://learn.microsoft.com/en-us/purview/ediscovery)
- [Microsoft Purview Data Lifecycle Management](https://learn.microsoft.com/en-us/purview/data-lifecycle-management)
