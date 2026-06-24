---
name: sap-audit-evidence-packaging
description: Package and structure audit evidence for SAP controls covering Segregation of Duties, change management, access management, and financial controls. Defines evidence taxonomy, maps controls to evidence artifacts, establishes chain-of-custody and redaction requirements, and aligns evidence packages to SOC 2, ISO 27001, SOX, and GxP frameworks. Does not touch live systems and never includes secrets, credentials, or personal identifiable information in evidence packages.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: compliance
  lifecycle: experimental
---

# SAP Audit Evidence Packaging

## Purpose

Structure, classify, and package audit evidence for SAP system controls to support internal audit, external audit, and regulatory compliance assessments. Define an evidence taxonomy aligned to SAP control domains. Map controls to the specific evidence artifacts required to demonstrate control effectiveness. Establish chain-of-custody requirements and redaction rules for evidence that contains sensitive system or personal data. Assess evidence reproducibility and advise on gaps. Align evidence packages to the requirements of SOC 2 Type II, ISO/IEC 27001, SOX ITGC, and GxP (21 CFR Part 11 / EU Annex 11) frameworks. Does not connect to any live SAP system, never requests or accepts credentials, and never includes secrets, credentials, personal identifiable information, or customer data in evidence packages.

## When to use

Use this skill when the user needs to:

- define an evidence taxonomy for a SAP audit: which artifact types constitute valid evidence for each SAP control domain (SoD, access management, change management, financial controls),
- map specific SAP controls from a control framework (SOX ITGC, SOC 2 CC6/CC8, ISO 27001 A.9, GxP Annex 11) to the concrete evidence artifacts that demonstrate their effectiveness,
- structure an evidence package for a specific control: what to collect, how to label it, what metadata to include, and how to maintain chain-of-custody,
- review a set of evidence artifacts provided by the user and assess whether they are sufficient to demonstrate control effectiveness for the stated control objective,
- advise on redaction of sensitive data from SAP audit evidence: which fields must be redacted from GRC SoD reports, role assignment exports, transport request logs, and financial posting reports before inclusion in an audit package,
- assess evidence reproducibility: whether the evidence artifact can be regenerated from the same source system state and time window, and what risks affect reproducibility,
- define a control-to-evidence mapping document suitable for use in an audit readiness assessment or external auditor evidence request response.

## When not to use

- When the user needs live enumeration of SAP system configuration to gather evidence — use `sap-live-readonly-landscape-discovery` or `sap-live-readonly-identity-trust-discovery` for read-only live evidence gathering.
- When the user needs SoD conflict review or IAM posture assessment rather than evidence packaging — use `sap-security-iam-grc-sod-review`.
- When the request is about implementing SAP GRC Access Control rulesets or configuring audit log settings — this skill is advisory and evidence-packaging focused, not implementation focused.
- When the request is about non-SAP control evidence packaging or general compliance framework interpretation without SAP-specific context.

## Does not touch live systems

This skill operates on user-provided evidence artifacts, control descriptions, audit scope descriptions, control framework excerpts, and written descriptions of the SAP landscape. It does not connect to any SAP system, BTP subaccount, GRC Access Control instance, ABAP system, S/4HANA tenant, or audit log service. All live evidence gathering is out of scope.

## Lean operating rules

- Classify evidence by control domain before structuring. Every evidence artifact or gap must be assigned to a control domain (SoD / Access Management / Change Management / Financial Controls / System Configuration) before packaging advice is given.
- Chain-of-custody is required for all live evidence. Evidence gathered from a live SAP system must be accompanied by: source system identifier, extraction method, extraction timestamp, person who extracted it, and any transformations applied. Evidence without chain-of-custody metadata is incomplete.
- Redaction before packaging is mandatory. Any evidence artifact that contains passwords, API keys, OAuth tokens, SAML private keys, personal identifiable information (employee IDs, social security numbers, salary data), or customer confidential data must be redacted before inclusion in an audit package.
- Reproducibility must be assessed. For each evidence artifact, advise whether it can be reproduced from the same source and time window. Evidence that cannot be reproduced (e.g., a one-time GRC report extraction from a period now past) must be designated as non-reproducible and stored with heightened custody controls.
- Framework alignment is explicit. For each control-to-evidence mapping, state which specific control requirement from the target framework (SOC 2 Trust Service Criteria, ISO 27001 Annex A, SOX ITGC, GxP Annex 11) the evidence addresses.
- Gaps are findings. A control that lacks sufficient evidence to demonstrate effectiveness is a gap finding. Classify gaps by severity: a control with no evidence is `critical`; a control with partial evidence is `high`; a control with evidence that does not cover the full audit period is `medium`.
- Evidence from user-provided artifacts and official SAP and regulatory framework documentation takes precedence over inference.
- Load only the reference needed for the control domain in scope.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP audit and compliance documentation, GRC best practices, or regulatory framework requirements (SOC 2, ISO 27001, SOX, GxP)
- `user-provided evidence` — audit evidence artifacts, control descriptions, audit scope descriptions, or written landscape descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence

## Live-environment rules

**This skill does not touch live systems.** There is no SAP system API call, GRC Access Control connection, BTP subaccount access, ABAP RFC call, audit log service query, or S/4HANA tenant access in this skill's execution path. Users must supply evidence artifacts, control descriptions, and audit scope information for this skill to advise on packaging, redaction, and framework alignment.

**This skill never includes in any output**:
- Passwords, API keys, OAuth tokens, or credentials of any kind
- SAML signing certificate private keys or X.509 private keys
- Personal identifiable information: employee names, employee IDs, national identification numbers, salary data, or health data
- Customer confidential data or customer personal data
- BTP service key values, client secrets, or connection strings with embedded credentials

If a user-provided evidence artifact contains any of the above, advise on redaction before including any excerpt in this skill's output.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — evidence taxonomy, control-to-evidence mapping structure, chain-of-custody format, output contract.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, redaction requirements, common audit evidence mistakes, when to push back.
- [Official sources](references/official-sources.md) — SAP GRC documentation, SAP audit and compliance guides, SOC 2, ISO 27001, SOX ITGC, and GxP references.

## Response minimum

Return, at minimum:

- **Problem classification**: control domain(s) in scope (SoD / Access Management / Change Management / Financial Controls / System Configuration) and the specific controls or evidence gaps being addressed.
- **Evidence used**: documentation-based / user-provided evidence / inference.
- **Risk level**: critical (control with no evidence; unmitigated gap before audit deadline) / high (partial evidence; evidence not covering full audit period) / medium (evidence present but chain-of-custody incomplete; redaction not applied) / low (minor metadata gap; reproducibility risk only).
- **Recommended action**: specific evidence artifact to collect, redaction to apply, chain-of-custody metadata to add, or framework mapping to document.
- **Refusal / escalation triggers**: if live SAP system access, GRC report extraction, or audit log query is required to complete the evidence package, state that clearly and direct to `sap-live-readonly-landscape-discovery` or `sap-live-readonly-identity-trust-discovery` for the evidence gathering step.
- **Business impact**: audit finding risk, regulatory non-compliance, certification failure, or evidence admissibility challenge.
- **Next verification step**: confirm chain-of-custody completeness and redaction with the control owner before submitting evidence to an external auditor.
