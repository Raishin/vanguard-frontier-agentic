---
name: sap-custom-code-remediation-review
description: Review custom ABAP code remediation plans and ATC findings for S/4HANA readiness. Assesses SAP simplification item violations, ABAP Test Cockpit results using S/4HANA readiness check variants, custom code migration app output, deprecated API usage, replacement API mapping, and clean-core alignment of proposed remediation paths. Advisory only — does not access or modify live SAP systems.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: architecture
  lifecycle: experimental
---

# SAP Custom Code Remediation Review

## Purpose

Review the remediation approach for custom ABAP code against SAP S/4HANA compatibility and clean core requirements. Assess ABAP Test Cockpit (ATC) findings using the S/4HANA readiness check variant, interpret custom code migration app output, classify deprecated API usage and its replacement path, evaluate custom code migration plans against SAP simplification item requirements, and validate that proposed remediation paths align with clean core principles. Does not access, run, or modify live SAP systems.

## When to use

Use this skill when the user asks to:

- review ATC (ABAP Test Cockpit) findings from the S/4HANA readiness check variant (e.g., `SCI_ABAP_CLOUD_READINESS`, `SCI_S4HANA_READINESS`) and classify findings by severity and remediation action,
- assess the output of the SAP Custom Code Migration App (transaction `SYCM`) or equivalent tooling for a set of ABAP custom objects,
- identify which deprecated SAP APIs, function modules, tables, or structures a custom ABAP program uses and what the supported replacement is,
- evaluate whether a proposed custom code remediation plan correctly prioritizes upgrade-blocking findings over clean-core-non-compliant findings,
- review a custom code migration project plan for completeness: scope, tooling, testing, and timeline alignment with the S/4HANA transformation program,
- validate that proposed replacement ABAP patterns follow ABAP Cloud programming model constraints (no SELECT on SAP internal tables, no CALL FUNCTION with deprecated FMs, no CALL SCREEN/CALL DIALOG in ABAP Cloud objects),
- assess which custom objects should be retired versus remediated versus replaced with standard SAP functionality.

## When not to use

- When the user needs a broader clean core extensibility assessment (BAdIs, RAP business objects, key-user extensibility) — use `sap-clean-core-debt-review`.
- When the user needs a transformation strategy assessment (brownfield vs greenfield, SAP Activate phase alignment) — use `sap-s4hana-transformation-architecture-review`.
- When the user needs live system enumeration of custom objects — use `sap-live-readonly-landscape-discovery` first, then return here for advisory review.
- When the user needs ABAP Cloud RAP modeling guidance — use `sap-abap-cloud-rap-review`.

## Does not touch live systems

This skill operates on user-provided ATC result exports, custom code migration app reports, ABAP program source code, object lists, or written descriptions of custom code patterns. It does not connect to any SAP ABAP system, run ATC checks, execute SE38/SE84/SYCM transactions, or access the SAP API Business Hub programmatically. All live enumeration must be provided by the user.

## Lean operating rules

- ATC finding classification first. Every ATC finding must be classified by check category and priority before a remediation action is recommended.
- S/4HANA readiness variant is the authoritative ATC scope. Findings from the `SCI_S4HANA_READINESS` or `SCI_ABAP_CLOUD_READINESS` variant take precedence over generic ATC check results.
- Upgrade-blocking findings must be resolved before conversion. ATC findings classified as `BLOCKER` or `ERROR` priority against the S/4HANA readiness variant represent conversion-blocking risks. They must be resolved or explicitly exempted before brownfield conversion can proceed.
- Deprecated API replacement must be SAP-released. When recommending a replacement for a deprecated function module, BAPI, or table, the replacement must carry a C1 or C2 release contract. Direct users to verify on SAP API Business Hub.
- Retire before remediate where justified. If a custom object serves a business function now covered by SAP standard S/4HANA, retirement is preferred over remediation. Assess business value vs. remediation cost.
- Do not fabricate ATC finding counts or severity levels. Only classify findings the user has provided from their actual ATC export or custom code migration app output.
- Evidence from official SAP sources takes precedence over memory or training data.
- Load only the reference needed for the scope in question.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in SAP ATC documentation, ABAP Cloud docs, S/4HANA simplification item catalog, or SAP Help Portal
- `user-provided evidence` — ATC result exports, custom code migration app output, ABAP source code, or object lists provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled

## Live-environment rules

**This skill does not touch live systems.** There is no credential, API call, RFC connection, or ABAP system access in this skill's execution path. Users must supply ATC exports, SYCM output, ABAP program source, or written descriptions of custom objects for this skill to review.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — ATC finding taxonomy, remediation decision tree, deprecated API replacement patterns, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, common remediation mistakes, when to push back.
- [Official sources](references/official-sources.md) — SAP ATC, custom code migration, S/4HANA simplification items, ABAP Cloud, deprecated API documentation.

## Response minimum

Return, at minimum:

- **Problem classification**: ATC check variant used, finding severity distribution, and primary deprecated API or violation pattern identified.
- **Evidence used**: documentation-based / user-provided evidence / inference.
- **Risk level**: upgrade-blocking / clean-core-non-compliant / technically-functional-but-non-standard / compliant.
- **Recommended action**: remediation path per finding class (replacement API, ATC exemption with justification, key-user extensibility, ABAP Cloud RAP rewrite, or retire).
- **Refusal / escalation triggers**: if live object enumeration is required, redirect to `sap-live-readonly-landscape-discovery`; if replacement API release contract needs verification, direct to SAP API Business Hub.
- **Business impact**: conversion timeline risk, upgrade support cost, clean core compliance posture after remediation.
- **Next verification step**: which ATC findings or deprecated APIs to validate against current SAP API Business Hub release contracts and S/4HANA simplification item catalog before implementing the remediation plan.
