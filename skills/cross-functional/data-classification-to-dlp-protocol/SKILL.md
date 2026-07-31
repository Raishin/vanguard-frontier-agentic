---
name: data-classification-to-dlp-protocol
description: Use this skill when sensitive data must be discovered, classified with Microsoft Purview sensitivity labels, protected by Data Loss Prevention policies, and monitored for label adoption and DLP policy effectiveness across Microsoft 365 and Power Platform environments. Defines the end-to-end flow from data discovery through classification taxonomy design, sensitivity label deployment, DLP policy coverage, and adoption monitoring. Does not authorize the creation or modification of sensitivity labels, DLP policies, or auto-labeling policies; all production-impacting policy changes require human approval from the Purview compliance administrator and the data owner. Does not replace qualified information protection or compliance counsel.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-06-16"
  category: compliance
  lifecycle: experimental
---

# Data Classification to DLP Protocol

## Purpose
This skill defines how sensitive data is discovered, classified using Microsoft Purview sensitivity labels, protected by Data Loss Prevention policies, and monitored for classification coverage and DLP effectiveness across Microsoft 365, Power Platform, and Dataverse environments. It ensures classification decisions are driven by a defined taxonomy before policies are deployed, that DLP coverage matches the taxonomy, and that label adoption is tracked over time. No agent creates or modifies sensitivity labels or DLP policies; those are production-impacting actions requiring the Purview compliance administrator and the data owner.

## When to use
- An organization needs to establish or review a sensitivity label taxonomy and map it to DLP policies across Microsoft 365 workloads.
- DLP policy coverage gaps are suspected (workloads, data types, or locations not yet covered).
- Label adoption rates need to be assessed and improvement actions identified.
- A Power Platform or Dataverse environment needs to be assessed for data classification and DLP policy alignment.
- A new data type (e.g., a new category of regulated personal data) needs to be classified and protected.

## When NOT to use
- Sensitivity labels or DLP policies are already deployed and fully validated — hand off to the Purview admin for ongoing monitoring.
- The matter involves a suspected active data loss or exfiltration event — use the incident-to-remediation-protocol instead.
- The classification requirement involves health, financial, or regulated data requiring specialist legal interpretation — escalate to the appropriate compliance or legal counsel first.
- The Purview tenant is not yet configured — initial tenant configuration is a Purview admin task, not a protocol task.

## Participating agents
- `m365-copilot-readiness-governance-agent` — primary: assesses Microsoft 365 data classification readiness, sensitivity label taxonomy, and Copilot surface area data protection gaps
- `power-platform-governance-dataverse-security-agent` — secondary: assesses Dataverse environment data classification, column-level security, and Power Platform DLP policy alignment
- `m365-maestro-agent` — orchestrates cross-workload classification and DLP review; escalation path to Microsoft Purview information protection specialists (planned: purview-information-protection-specialist-agent)

## Inputs required
- List of data types in scope (e.g., customer PII, financial records, health data, intellectual property, confidential business data)
- Workload scope (Exchange Online, SharePoint, OneDrive, Teams, Dataverse, Power Platform connectors)
- Existing sensitivity label taxonomy (if any)
- Existing DLP policies (if any)
- Regulatory or contractual requirements driving classification (e.g., GDPR, HIPAA, internal policy)

## Evidence required
- Current sensitivity label list from Microsoft Purview Information Protection (exported summary)
- Current DLP policy list and policy locations from Microsoft Purview (exported summary)
- Label adoption report or usage data (if available)
- Power Platform DLP connector policy list for in-scope environments
- Data inventory or data map for in-scope workloads (if available)

## Workflow

1. **Ingest data types and requirements** — receive the list of data types and regulatory requirements; map each data type to a classification level (e.g., Public, Internal, Confidential, Highly Confidential, Restricted).
2. **Review existing taxonomy** — compare the requested classification levels to the existing sensitivity label taxonomy; identify gaps (missing labels, overlapping scope, or outdated labels).
3. **Taxonomy design recommendation** — recommend a sensitivity label taxonomy aligned to Microsoft Purview best practices (hierarchical labels, sublabels for sensitivity tiers, encryption settings per tier); do not create or modify labels — recommend for human Purview admin review.
4. **Map labels to DLP coverage** — for each sensitivity label, identify which DLP policies cover it across which workloads; identify coverage gaps (labels with no DLP policy, workloads with no DLP policy, data types with no protective action).
5. **Assess auto-labeling opportunity** — identify data types that qualify for auto-labeling (sensitive information types, trainable classifiers, exact data match); recommend auto-labeling policies; do not create policies.
6. **Power Platform and Dataverse alignment** — assess Power Platform connector DLP policies for the in-scope environments; flag connectors that handle sensitive data but are not blocked or restricted by a DLP policy; assess Dataverse column security for fields containing sensitive data.
7. **Escalation gate: classification taxonomy** — if the taxonomy is undefined, incomplete, or contradictory, pause and escalate to the data owner and compliance owner before proceeding to DLP policy review.
8. **Escalation gate: DLP coverage** — if critical coverage gaps exist (regulated data types with no DLP policy or no protective action), stop and escalate to the Purview compliance administrator and the data owner for remediation.
9. **Escalation gate: label adoption** — if label adoption is below the agreed threshold (defined by the compliance owner), flag as a risk item and escalate to the data owner for adoption improvement actions.
10. **Assemble classification and DLP report** — compile: taxonomy gap list, DLP coverage map, auto-labeling opportunities, Power Platform DLP gaps, label adoption summary, and recommended next actions.
11. **Hand off to compliance owner** — deliver report with a do-not-do list and open questions; require human sign-off before any label or policy changes are implemented.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Classification taxonomy | Taxonomy is undefined, incomplete, or contradictory | Pause; escalate to data owner + compliance owner |
| DLP coverage | Regulated data type has no DLP policy or no protective action | Stop; escalate to Purview admin + data owner |
| Label adoption | Adoption rate below threshold for regulated data types | Flag; escalate to data owner for adoption improvement |
| Special-category data | Health, biometric, or sensitive personal data in scope | Confirm jurisdiction and privacy owner before proceeding |
| Copilot surface exposure | Sensitivity label configuration affects Copilot data grounding | Escalate to m365-copilot-readiness-governance-agent for Copilot-specific DLP assessment |

## Refusal triggers
- A request is made to create, modify, or delete a sensitivity label or DLP policy without human Purview admin approval — refuse.
- A request is made to remove encryption from a sensitivity label covering regulated data — refuse and escalate.
- Credentials, tenant IDs, or personal data are requested to perform the classification assessment — refuse; work from sanitized taxonomy and policy summary signals only.
- The classification taxonomy is being designed to evade a regulatory obligation (e.g., deliberately under-classifying regulated personal data) — refuse and escalate to compliance owner.

## Handoff rules
- Every handoff carries: taxonomy gap list, DLP coverage map, Power Platform DLP gaps, label adoption summary, escalations fired, open questions, and a do-not-do list.
- No agent creates, modifies, or deletes sensitivity labels or DLP policies. Human Purview compliance administrator owns all policy changes.
- Post-handoff, the Purview admin confirms planned policy changes and the data owner signs off on the taxonomy before implementation.

## KPIs
- Percentage of regulated data types with a defined sensitivity label and active DLP policy
- Label adoption rate for regulated data types across in-scope workloads
- Number of DLP coverage gaps identified and remediated per review cycle
- Time from classification taxonomy approval to DLP policy deployment

## References
- [Learn about sensitivity labels — Microsoft Purview Information Protection](https://learn.microsoft.com/purview/sensitivity-labels)
- [Learn about data loss prevention — Microsoft Purview](https://learn.microsoft.com/purview/dlp-learn-about-dlp)
- [Implement information protection and data loss prevention with Microsoft Purview (training)](https://learn.microsoft.com/training/paths/purview-implement-information-protection-data-loss-prevention/)
- [Power BI implementation planning: Information protection and DLP](https://learn.microsoft.com/power-bi/guidance/powerbi-implementation-planning-info-protection-data-loss-prevention-overview)
