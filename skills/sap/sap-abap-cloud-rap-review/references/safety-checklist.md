# Safety checklist — SAP ABAP Cloud RAP Review

Use before making any finding or remediation recommendation, especially for authorization and released API compliance findings.

## Non-negotiables

- Do not access, connect to, or request access to any live ABAP system, SAP S/4HANA tenant, or BTP ABAP Environment. This skill reviews artifacts only.
- Do not accept or request system credentials, RFC connection parameters, logon data, BTP service keys, or client secrets.
- Do not classify `update_granted = abap_true` (simulated full access) as acceptable in production-bound code. This pattern is explicitly documented as a development-only simulation that must be removed before production deployment.
- Do not validate release contract status from memory alone. Direct the user to verify proposed C1/C2 alternatives on SAP API Business Hub (api.sap.com) for the specific S/4HANA or BTP ABAP Environment version in scope.
- Do not recommend consuming NOT_RELEASED SAP internal objects as a remediation step, even if they are technically functional. Only C1/C2 released objects are clean-core compliant in ABAP Cloud.
- Do not classify a RAP business object's authorization coverage as complete without reviewing the `CHECK_AUTHORIZATION` method implementation. A BDEF with `authorization master ( global )` is meaningless if the method contains pass-through logic.
- Do not confuse RAP managed and unmanaged implementation types. Managed RAP uses the RAP framework for persistence; unmanaged RAP delegates to custom code. The wrong selection affects OData behavior, draft support, and lock handling.

## What people get wrong

- **Leaving AUTHORITY-CHECK simulation in production**: The `update_granted = abap_true` simulation pattern is a documented development shortcut from openSAP exercises. Any behavior implementation containing this in a production-bound branch is a critical security finding.
- **Treating `authorization master ( global )` as sufficient**: Declaring `authorization master ( global )` in the BDEF is a structural declaration, not an authorization guarantee. The `CHECK_AUTHORIZATION` method must implement actual `AUTHORITY-CHECK` statements for it to be meaningful.
- **Assuming draft works without draft table**: `with draft;` in the BDEF requires a matching `draft table` definition in the CDS interface view. Missing the CDS-side draft table causes a runtime error on activation.
- **Using interface views directly in service bindings**: Interface views should not appear directly in service bindings. The correct pattern is: interface view → projection view → service definition → service binding.
- **Confusing ROLLBACK WORK and ROLLBACK ENTITIES**: In RAP unit tests, `ROLLBACK ENTITIES` is the correct teardown mechanism, not `ROLLBACK WORK`. Using `ROLLBACK WORK` does not clean up the RAP buffer.
- **Asserting only initial result in unit tests**: A test that only asserts `assert_initial( failed-travel )` without asserting expected field values provides weak coverage — the test passes even if the result is empty.

## When to push back

- Push back when the user asks for an authorization compliance ruling without providing the behavior implementation class source code for `CHECK_AUTHORIZATION`.
- Push back when the user proposes to keep a `update_granted = abap_true` simulation in code that will go to production.
- Push back when released API compliance is in scope but no ABAP source code or object list is provided for review.
- Push back when the request involves live ABAP system access, ATC execution, or object navigation — redirect to SAP ABAP Test Cockpit or `sap-clean-core-debt-review` for live object enumeration first.
- Push back when the user proposes consuming a NOT_RELEASED SAP object as a workaround and presents performance or convenience as justification.

## Evidence labels

- `documentation-based` — grounded in official SAP ABAP Cloud docs (help.sap.com/docs/abap-cloud) or SAP Help Portal RAP documentation
- `user-provided evidence` — ABAP source code, CDS DDL, BDEF source, metadata extensions, or written descriptions provided by the user
- `context7-supplementary` — RAP authorization and unit test patterns from Context7 openSAP sample repository (supplementary to official SAP docs)
- `inference` — derived reasoning not directly confirmed; must always be labeled as such
