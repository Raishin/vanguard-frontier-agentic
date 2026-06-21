# Safety checklist — SAP Clean Core Debt Review

Use before making any remediation recommendation, especially for upgrade-blocking or business-critical custom objects.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system. This skill reviews artifacts only.
- Do not accept or request SAP system credentials, RFC destinations, BTP service keys, client secrets, or OAuth tokens.
- Do not propose implicit enhancements, customer modifications, or `CALL CUSTOMER FUNCTION` as remediation targets. These are deprecated extensibility patterns.
- Do not recommend consuming NOT_RELEASED SAP APIs as a remediation step. Only C1/C2 released APIs are clean core compliant.
- Do not validate release contract status from memory alone. Direct the user to verify on SAP API Business Hub for each proposed released API.
- Do not conflate ABAP Cloud (in-system) with BTP CAP (side-by-side). They serve different use cases and have different deployment models.
- Do not make upgrade-blocking declarations for objects the user has not provided. Only classify objects that appear in user-provided artifacts.

## What people get wrong

- **Recommending `EXIT_*` or user exits as a migration target**: User exits (`CALL CUSTOMER FUNCTION`) are the source of the problem, not a valid remediation. The migration target is a new ABAP Objects BAdI.
- **Treating all Z-objects as clean core violations**: A Z-program that only consumes C1/C2 released APIs and uses no SAP internal objects is clean core compliant.
- **Confusing key-user extensibility with developer extensibility**: Key-user extensibility is configuration-based (no ABAP). Developer extensibility requires ABAP Cloud or side-by-side BTP development.
- **Skipping API Business Hub validation**: Proposing a released API without verifying its C1/C2 contract on api.sap.com may lead the user to consume an API that is not actually released for their S/4HANA version.
- **Ignoring system version**: Clean core requirements and released API availability differ between S/4HANA on-premise, S/4HANA Cloud (RISE), and S/4HANA Cloud Public Edition. Always confirm with the user which deployment model is in scope.
- **Recommending CAP for in-system ABAP logic**: SAP CAP (Node.js/Java) runs on BTP, not inside the ABAP system. It cannot replace in-system ABAP business logic directly — it creates a side-by-side process.

## When to push back

- Push back when the user asks to "just leave the modification" without understanding upgrade risk.
- Push back when the user proposes a remediation that itself uses NOT_RELEASED APIs.
- Push back when the artifact is unavailable and the user asks for a compliance ruling from memory alone.
- Push back when the request involves live object inspection — redirect to `sap-live-readonly-landscape-discovery` first.

## Evidence labels

- `documentation-based` — grounded in SAP Clean Core docs, ABAP Cloud docs, or S/4HANA extensibility guides
- `user-provided evidence` — code snippets, ATC output, custom code migration app results provided by the user
- `context7-supplementary` — CAP/RAP framework patterns from Context7 (supplementary to official SAP docs)
- `inference` — derived reasoning not directly confirmed; must always be labeled as such
