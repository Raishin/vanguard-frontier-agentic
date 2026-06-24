# Workflow and output contract — SAP Fiori and UI5 UX Review

Use this reference for all classification, severity assignment, and output formatting.

## Review domain taxonomy

| Domain | Scope |
|--------|-------|
| `App Structure` | manifest.json completeness, component configuration, library dependencies, routing setup, data source declarations, model declarations |
| `XML View` | view nesting correctness, control usage compliance, aggregation binding, layout patterns, fragment reuse, view instantiation |
| `Controller` | MVC separation, lifecycle hooks, event handler design, controller extension, direct DOM access, memory leak patterns |
| `OData Consumption` | ODataModel V2/V4 configuration, binding paths, batch mode, error callbacks, filter/sort expressions, deferred groups |
| `Fiori Design` | floorplan selection, SAP Fiori design guideline adherence, header/footer bar patterns, responsive design, Fiori Elements versus freestyle decision |
| `Accessibility` | ARIA landmark usage, form field labeling, table header correctness, keyboard navigation, high-contrast theme compatibility, screen reader support |
| `Launchpad` | tile configuration, semantic object and action definition, target mapping, cross-application navigation intent, app descriptor sap.app and sap.ui5 section correctness |
| `Performance` | async component loading, lazy view instantiation, OData batch grouping, initial payload size, synchronous module loading, unnecessary re-renders |

## Severity classification

| Severity | Meaning | Examples |
|----------|---------|---------|
| `critical` | Security or data integrity risk originating from the UI5 app | Direct DOM manipulation that bypasses CSP; XSS vector via unsanitized binding; client-side authorization bypass |
| `high` | Functional, operational, or accessibility failure | Missing OData error callback; synchronous component loading; missing ARIA label on interactive control; broken launchpad navigation intent; incorrect routing configuration causing navigation failure |
| `medium` | Fiori design deviation, maintainability gap, or UX standard violation | Wrong floorplan for the use case; missing footer toolbar in an Object Page; controller code accessing model data outside MVC separation; non-standard control layout |
| `low` | Best practice deviation or code quality concern | Inconsistent controller naming; unused i18n keys; hardcoded text instead of i18n bundle reference; suboptimal OData batch group naming |

## Workflow

1. **Receive artifacts** — manifest.json, XML views, controller code, fragment files, OData metadata document, launchpad tile descriptor, or user descriptions.
2. **Classify each finding** by review domain above.
3. **Assign severity** (critical / high / medium / low).
4. **Identify evidence level** (documentation-based / user-provided evidence / context7-supplementary / inference).
5. **Recommend specific remediation** — annotation or configuration to add, control to replace, callback to implement, or pattern to correct.
6. **Prioritize** — critical and high severity first; accessibility and OData error handling before design and performance findings.
7. **Return output** per the output contract below.

## Common finding patterns

### App Structure
- Missing `async: true` in `manifest.json` `sap.ui5.componentUsages` or component `index.js` (high performance finding)
- Data source URL missing or pointing to a hardcoded absolute URL instead of a relative or destination-based path (high)
- Missing `sap.ui` section or incorrect minimum SAPUI5 version declaration (medium)
- Routing configuration missing a default target or using incorrect `viewPath` (high — causes navigation failure)

### XML View
- Hardcoded text in XML view instead of i18n binding (low)
- Missing `growing="true"` and `growingThreshold` on large `sap.m.List` or `sap.m.Table` controls (high performance finding)
- Using deprecated controls (e.g., `sap.ui.commons.*`) instead of `sap.m.*` equivalents (medium)
- Fragment instantiated inside a controller without caching — creates duplicate fragment on each navigation (high)

### Controller
- Direct DOM access via `document.getElementById()` or jQuery selectors in controller methods (high)
- Missing `onExit` lifecycle hook cleanup for event subscriptions or resize handlers (high — memory leak)
- Business logic (data transformation, validation) mixed into controller instead of separated into a model or utility module (medium)
- Using `sap.ui.getCore()` to access models instead of `this.getView().getModel()` (medium — deprecated pattern)

### OData Consumption
- ODataModel `read()` or `create()` call with no `error` callback (high)
- Submitting changes without calling `ODataModel.submitChanges()` for V2 deferred batch groups (high — silent data loss)
- Using `sap.ui.model.odata.v2.ODataModel` with `useBatch: false` in production without justification (medium — performance)
- V4 ODataListBinding context accessed before the binding is resolved (high — runtime error)

### Fiori Design
- Using a custom freestyle layout where a Fiori Elements List Report or Object Page is the correct pattern for the use case (medium)
- Object Page missing a dynamic page header with the required header content area (high)
- Missing "Determine Changes" or "Save" / "Cancel" button pair in an editable Object Page footer (high)
- Worklist app missing the search field and variant management controls required by the Worklist floorplan (medium)

### Accessibility
- Interactive control (`sap.m.Button`, `sap.m.Input`) missing `ariaLabelledBy` or `ariaDescribedBy` association and no visible label (high)
- `sap.m.Table` missing column headers or using empty `sap.m.Column` label (high)
- Form fields in `sap.ui.layout.form.SimpleForm` without `sap.m.Label` with `labelFor` association (high)
- Custom control not implementing `getAccessibilityInfo()` method (medium)

### Launchpad
- `sap.app.crossNavigation.outbounds` missing required semantic object and action for cross-application navigation (high)
- `sap.app.crossNavigation.inbounds` missing `signature.parameters` definition for intent-based navigation (high)
- Tile title or subtitle hardcoded instead of using i18n reference in `manifest.json` (low)
- Missing `sap.app.ach` (Application Component Hierarchy) assignment for support and monitoring (medium)

### Performance
- `sap/ui/core/Core` loaded synchronously in `index.html` without `data-sap-ui-async="true"` bootstrap parameter (high)
- Non-critical XML views instantiated eagerly in `onInit` instead of lazily on first navigation (medium)
- OData `$expand` used with unbounded expand depth — fetches entire entity graph (high — network performance)
- Large JSON model payload loaded into a `sap.ui.model.json.JSONModel` on app startup instead of paged OData binding (medium)

## Output contract

Return:

1. Artifacts reviewed and domains in scope
2. Finding(s) per domain with severity and evidence label
3. Specific remediation recommendation per finding (control to replace, configuration key to set, callback to add, annotation to apply, or pattern to adopt)
4. Accessibility summary — list of interactive controls and form fields with their labeling and ARIA coverage status
5. Prioritized remediation sequence (critical → high → medium → low)
6. Escalation trigger if live UI5 runtime, BTP deployment, or ABAP backend OData inspection is required before proceeding
