# Safety checklist — SAP S/4HANA Transformation Architecture Review

Use before making any transformation strategy recommendation or risk classification, especially for program commitments or deployment model decisions.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system. This skill reviews documentation and user-provided artifacts only.
- Do not accept or request SAP system credentials, RFC destinations, S-user passwords, BTP service keys, or cloud platform API tokens.
- Do not assume the deployment model. Always confirm with the user whether the target is Cloud Public Edition, Cloud Private Edition, RISE with SAP, or on-premise managed. These have fundamentally different extensibility and operations constraints.
- Do not classify simplification item impact from memory alone. Only classify findings the user has supplied from their actual Readiness Check output.
- Do not recommend brownfield conversion without confirming the user's source release is a supported conversion start point for their target S/4HANA release.
- Do not recommend Cloud Public Edition for a program with significant approved custom ABAP requirements — Cloud Public Edition restricts in-system ABAP extensibility to key-user tools only.
- Do not recommend selective data transition without acknowledging the additional cutover complexity and dual-tooling requirement compared to brownfield or greenfield.
- Do not validate fit-to-standard completeness without knowing which modules and lines of business the workshops covered.

## What people get wrong

- **Treating RISE with SAP as a deployment model**: RISE with SAP is a commercial bundle (subscription model including cloud migration, SAP S/4HANA Cloud Private Edition or Public Edition, BTP entitlements). The underlying deployment model is either Cloud Private Edition or Cloud Public Edition. Architectural constraints depend on the underlying edition, not the RISE designation.
- **Conflating brownfield with minimal-change**: Brownfield conversion still requires simplification item resolution, custom code remediation, and post-conversion fit-to-standard alignment. It is not a "lift and shift" of ECC to S/4HANA.
- **Assuming greenfield eliminates custom code risk**: Greenfield eliminates the conversion-time custom code issue but does not eliminate the need to re-implement business requirements that SAP standard does not cover. Those requirements still need clean-core-compliant extensibility.
- **Skipping fit-to-standard workshops**: Fit-to-standard workshops are mandatory in the SAP Activate Explore phase to identify genuine gaps before development begins. Programs that skip them often discover scope in the Realize phase when it is expensive to change direction.
- **Treating SDT as brownfield-plus-data-selection**: SDT is a distinct approach with its own tooling (SAP LT Replication Server or LTMC-based migration), data selection logic, and cutover complexity. It is not a brownfield conversion with a filter applied.
- **Ignoring quarterly update discipline for Cloud Public Edition**: Cloud Public Edition mandates quarterly updates with no opt-out. Custom code and integrations must be tested each quarter. Programs that ignore this accumulate testing debt rapidly.
- **Recommending parallel landscapes without acknowledging cost**: Selective data transition requires running both legacy and S/4HANA systems in parallel during migration, which has infrastructure and license implications the user should budget for explicitly.

## When to push back

- Push back when the user wants to commit to a strategy before the SAP Readiness Check has been executed.
- Push back when the user wants to adopt Cloud Public Edition but has described requirements that require in-system custom ABAP development.
- Push back when the transformation roadmap does not include a formal simplification item resolution plan.
- Push back when fit-to-standard workshops have not been planned or completed for the Explore phase.
- Push back when the user presents a brownfield cutover plan without a documented rollback / fallback procedure.
- Push back when the user treats SAP Activate as optional or as a documentation exercise rather than a structural delivery framework.
- Push back when custom code remediation scope is unknown and the user wants to commit to a Realize phase start date.

## Evidence labels

- `documentation-based` — grounded in SAP Activate methodology, S/4HANA deployment guides, Readiness Check documentation, or SAP Help Portal
- `user-provided evidence` — SAP Readiness Check output, project plans, fit-to-standard workshop results, or architectural descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
