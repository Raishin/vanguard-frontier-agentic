# Safety checklist — SAP Custom Code Remediation Review

Use before making any ATC finding classification, deprecated API replacement recommendation, or ATC exemption guidance.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system. This skill reviews user-provided artifacts only.
- Do not accept or request SAP system credentials, ABAP developer logon passwords, RFC destinations, or BTP service keys.
- Do not recommend a deprecated pattern as a remediation target. Classic user exits, implicit enhancements, `CALL CUSTOMER FUNCTION`, and direct access to SAP internal DDIC objects are deprecated source patterns — they are never valid remediation destinations.
- Do not recommend a replacement API without directing the user to verify its C1/C2 release contract on SAP API Business Hub. A released-sounding API name is not sufficient — the contract must be confirmed.
- Do not grant or imply that an ATC exemption has been approved. Exemptions require formal customer authorization. This skill can describe when an exemption may be warranted and what justification is needed, but cannot approve one.
- Do not classify findings that the user has not provided. Only assess ATC findings or deprecated patterns present in user-supplied artifacts.
- Do not conflate ATC S/4HANA readiness findings with general ABAP quality findings. The `SCI_S4HANA_READINESS` variant is specifically scoped to conversion-relevant issues; general code quality findings require a separate ATC variant.

## What people get wrong

- **Treating all ATC warnings as optional**: Some ATC priority-3 (warning) findings indicate post-conversion functional regressions in critical business processes. Always assess the business context of warning-class findings before deferring them.
- **Recommending replacement APIs without release contract verification**: An SAP function module with a readable name (e.g., `BAPI_MATERIAL_GET`) may not carry a C1/C2 release contract in the user's target S/4HANA release. Always verify on api.sap.com before recommending.
- **Equating ABAP Cloud compliance with S/4HANA conversion readiness**: ABAP Cloud readiness (tier compliance, forbidden construct elimination) and S/4HANA system conversion readiness (simplification item resolution) are related but distinct. Conversion can succeed with non-ABAP-Cloud-compliant code; the clean core debt remains.
- **Recommending SELECT on SAP internal tables as an intermediate step**: Recommending direct SELECT on SAP internal tables as a "temporary fix" accumulates technical debt that is harder to remove later. Always recommend the released API path, even if it requires more effort.
- **Treating ATC exemption as the primary remediation path**: Exemptions should be the last resort for genuinely irreplaceable conversion-blocking code. Using exemptions broadly indicates an under-resourced remediation program, not an appropriate risk management decision.
- **Ignoring the distinction between ABAP Cloud Tier 1/2/3**: ABAP Cloud has three tiers with different constraints. Tier 1 (ABAP language subset) applies to all ABAP Cloud objects. Tier 2 (released API consumption) applies to objects accessing SAP objects. Tier 3 (ABAP Platform and S/4HANA standard) applies to delivered SAP standard. Custom code must comply with Tier 1 and Tier 2 constraints.
- **Confusing SYCM scope with ATC scope**: The Custom Code Migration App (SYCM) provides a high-level usage and impact analysis. ATC with the S/4HANA readiness variant provides finding-level detail. Both are needed for a complete picture; neither is a substitute for the other.

## When to push back

- Push back when the user wants to defer all blocker-class ATC findings without a formal exemption process and justification.
- Push back when the proposed replacement API for a deprecated finding has not been verified for C1/C2 release contract.
- Push back when the user proposes maintaining `CALL CUSTOMER FUNCTION` user exits as a "working solution" post-conversion.
- Push back when the remediation plan lacks a testing strategy for verifying that replaced APIs produce the same business results as the deprecated code.
- Push back when the user wants to use `SELECT *` on SAP internal tables in ABAP Cloud objects as a transitional approach.
- Push back when ATC findings are provided from a generic quality variant rather than the S/4HANA readiness or ABAP Cloud readiness variant — the findings are not conversion-representative.
- Push back when the conversion program has no defined owner for custom code remediation and no budget allocated for the remediation volume indicated by the SYCM report.

## Evidence labels

- `documentation-based` — grounded in SAP ATC documentation, ABAP Cloud docs, S/4HANA simplification item catalog, or SAP Help Portal
- `user-provided evidence` — ATC result exports, SYCM output, ABAP source code listings, or object descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
