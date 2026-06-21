# Safety checklist — SAP Fiori and UI5 UX Review

Use before making any finding or remediation recommendation.

## Non-negotiables

- Do not access, connect to, or request access to any live UI5 app runtime, BTP subaccount, ABAP backend, Fiori launchpad configuration service, or OData service endpoint. This skill reviews artifacts only.
- Do not accept or request BTP service keys, ABAP system credentials, OAuth tokens, OData service passwords, or SAP S/4HANA logon credentials.
- Do not recommend bypassing SAPUI5 rendering lifecycle (direct DOM manipulation, jQuery selectors in controllers) as a workaround.
- Do not validate UI5 control rendering behavior from memory alone for SAPUI5 versions older than 1.96 — direct the user to verify against the official SAPUI5 changelog and version-specific API documentation.
- Do not conflate SAP Fiori Elements behavior (annotation-driven) with freestyle UI5 behavior. Annotation-driven findings apply only when Fiori Elements is confirmed as the rendering approach.
- Do not classify an app as accessible from manifest analysis alone — accessibility findings require review of XML view and controller artifacts.
- Do not recommend `sap.ui.getCore()` as an API pattern — it is deprecated since SAPUI5 1.118. Direct the user to use `Component.getComponentById()` or `this.getView().getModel()` equivalents.

## What people get wrong

- **Treating `manifest.json` routing as sufficient for navigation review**: Routing configuration errors (wrong `viewPath`, missing `target`, incorrect `pattern`) only become visible when XML views and controller navigation calls are also reviewed. Manifest alone is insufficient.
- **Assuming `useBatch: false` is safe for write operations**: Disabling batch mode in `ODataModel` V2 causes each write operation to be sent as a separate HTTP request, bypassing transactional consistency. It is acceptable only for specific read-heavy scenarios with explicit justification.
- **Overlooking `onExit` cleanup**: Event subscriptions registered on the global `EventBus` or resize handlers attached in `onInit` must be deregistered in `onExit`. Missing cleanup causes memory leaks that accumulate across navigation cycles.
- **Treating Fiori Elements apps as freestyle**: SAP Fiori Elements apps render entirely from OData annotations. Reviewing controller code for logic that overrides Fiori Elements rendering without a registered extension point is a finding, not a pattern.
- **Assuming ARIA compliance from control names alone**: Many SAPUI5 controls render accessible markup by default, but only when labels and associations are correctly declared in XML views. A `sap.m.Input` without an associated `sap.m.Label` via `labelFor` is not accessible even though `sap.m.Input` supports accessibility.
- **Missing the `async` bootstrap parameter**: Setting `data-sap-ui-async="true"` in the UI5 bootstrap configuration is the single most impactful performance change for most Fiori apps. Its absence means all UI5 libraries load synchronously, blocking the main thread.

## When to push back

- Push back when the user asks to validate OData service behavior (response payloads, HTTP status codes, error body format) without providing OData metadata or service response examples — this skill reviews UI5 consumer-side artifacts, not backend OData service behavior.
- Push back when the user asks to confirm Fiori launchpad tile rendering without providing the `manifest.json` inbounds section or tile descriptor.
- Push back when multitenancy, BTP deployment, or runtime environment configuration is in scope without user-provided `manifest.json` and deployment descriptor artifacts.
- Push back when the request involves live UI5 runtime access, Fiori launchpad configuration service calls, or ABAP backend OData inspection — this skill is advisory only.
- Push back when the user asks to review a custom control's rendering behavior without providing the control's `renderer` implementation code.

## Evidence labels

- `documentation-based` — grounded in official SAP Fiori design guidelines (experience.sap.com) or official SAPUI5 documentation (ui5.sap.com)
- `user-provided evidence` — manifest files, XML views, controller code, fragment files, OData metadata documents, tile descriptors, or descriptions provided by the user
- `context7-supplementary` — SAPUI5/OpenUI5 MVC patterns, OData model binding, component configuration, or accessibility patterns sourced from Context7 (supplementary to official SAP sources)
- `inference` — derived reasoning not directly confirmed; must always be labeled as such
