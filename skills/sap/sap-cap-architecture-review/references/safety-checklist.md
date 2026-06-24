# Safety checklist — SAP CAP Architecture Review

Use before making any finding or remediation recommendation, especially for authorization and multitenancy findings.

## Non-negotiables

- Do not access, connect to, or request access to any live CAP application, BTP subaccount, HANA database, or deployment pipeline. This skill reviews artifacts only.
- Do not accept or request BTP service keys, HANA credentials, OAuth client secrets, IAS application IDs, or subaccount tenant IDs.
- Do not recommend `@requires: 'any'` as a security baseline. In some CAP runtime configurations this allows unauthenticated access.
- Do not validate runtime authorization behavior from memory alone. Direct the user to verify annotation enforcement against the deployed CAP runtime version and `@sap/cds` package version in their package.json.
- Do not conflate CAP Node.js and CAP Java authorization behavior. The `@restrict` enforcement rules and Spring Security integration differ between runtimes.
- Do not make multitenancy compliance declarations without user-provided MTX configuration evidence. Missing MTX wiring cannot be confirmed from CDS models alone.
- Do not classify a service as properly secured based on `@requires` alone if the service contains entities or actions with sensitive write operations — entity-level `@restrict` must also be checked.

## What people get wrong

- **`@requires` on service ≠ full authorization coverage**: A `@requires: 'authenticated-user'` on the service grants access to all entities within it to any authenticated user. Entity-level `@restrict` is needed for fine-grained control.
- **Treating `cds.db.run()` as safe**: Direct database access via `cds.db` in service handlers bypasses CAP authorization enforcement. All data access should go through CAP service calls unless there is an explicit and reviewed reason.
- **Assuming SQLite tests validate HANA behavior**: CAP in-memory SQLite tests do not catch HANA-specific SQL features, views with parameters, or HANA native procedures. Production HANA validation requires a HANA target.
- **Draft-enabled entities assumed to self-validate**: CAP draft support does not automatically validate business rules on activation. `draftActivate` and `BeforeSave` handlers must be implemented explicitly.
- **Extensibility service open by default**: `cds add extensibility` enables the extensibility service endpoint, but role enforcement (`cds.ExtensionDeveloper`) must be verified. An open extensibility endpoint in a SaaS app is a critical finding.
- **Multitenancy configuration assumed complete from `cds.requires.multitenancy: true`**: This flag alone does not wire tenant onboarding, upgrade, or offboarding. The full MTX lifecycle must be explicitly confirmed in the `@sap/cds-mtxs` configuration.

## When to push back

- Push back when the user asks to confirm authorization correctness without providing CDS service and entity definitions.
- Push back when the user proposes bypassing CAP authorization by querying the database directly in a handler.
- Push back when multitenancy is in scope but no MTX configuration or package.json is provided.
- Push back when the request involves live CAP runtime access or BTP deployment execution — this skill is advisory only.
- Push back when the user proposes `@requires: 'any'` as a security pattern for externally exposed services.

## Evidence labels

- `documentation-based` — grounded in official SAP CAP docs (cap.cloud.sap) or SAP Help Portal
- `user-provided evidence` — CDS files, handler code, package.json, test files, or descriptions provided by the user
- `context7-supplementary` — CAP authorization, multitenancy, or draft patterns sourced from Context7 (supplementary to official SAP CAP docs)
- `inference` — derived reasoning not directly confirmed; must always be labeled as such
