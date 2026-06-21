# Context7 framework docs — SAP Fiori and UI5 UX Review

**Role**: supplementary. Official SAP Fiori design guidelines at experience.sap.com and official SAPUI5 documentation at ui5.sap.com are the primary sources for all review guidance. Context7-sourced OpenUI5/SAPUI5 documentation supplements with code-level examples and implementation detail for MVC patterns, OData model binding, component configuration, and accessibility.

**Library used**: OpenUI5
Context7 library ID: `/ui5/docs`
Lookup targets: MVC controller patterns, OData V2/V4 model binding, async component loading, i18n ResourceModel, XML view and fragment patterns, accessibility labeling
Skill: `sap-fiori-ui5-ux-review`
Classification: supplementary — strongly applies for MVC, OData consumption, component configuration, and accessibility domains

---

## UI5 MVC — Controller initialization and ResourceModel (supplementary)

Source: OpenUI5 documentation (Context7 `/ui5/docs`)
Reference: https://github.com/ui5/docs/blob/main/docs/03_Get-Started/step-8-translatable-texts-df86bfb.md

Correct controller `onInit` pattern: instantiate the ResourceModel for i18n and set it as a named model on the view. Business data models should also be set on the view, not on the component or global core, unless cross-view sharing is required.

```javascript
sap.ui.define([
   "sap/ui/core/mvc/Controller",
   "sap/ui/model/json/JSONModel",
   "sap/ui/model/resource/ResourceModel"
], (Controller, JSONModel, ResourceModel) => {
   "use strict";

   return Controller.extend("myApp.controller.Main", {
      onInit() {
         const oModel = new JSONModel({ recipient: { name: "World" } });
         this.getView().setModel(oModel);

         const i18nModel = new ResourceModel({ bundleName: "myApp.i18n.i18n" });
         this.getView().setModel(i18nModel, "i18n");
      }
   });
});
```

**Review relevance**: Assess whether models are set correctly on the view (not on `sap.ui.getCore()`), whether the i18n ResourceModel uses a bundle name rather than a hardcoded URL, and whether the `onInit` hook is used appropriately rather than doing model setup in the constructor.

---

## UI5 XML view — Shell control and Fiori launchpad embedding (supplementary)

Source: OpenUI5 documentation (Context7 `/ui5/docs`)
Reference: https://github.com/ui5/docs/blob/main/docs/03_Get-Started/step-12-shell-control-as-container-4df1d91.md

The `sap.m.Shell` control provides a letterbox layout for standalone app deployment. When the app runs inside the SAP Fiori launchpad, the Shell control is automatically omitted by the launchpad container. Wrapping the `sap.m.App` inside a `Shell` is the correct pattern for apps that must work both standalone and embedded in the launchpad.

```xml
<mvc:View
   controllerName="myApp.controller.App"
   xmlns="sap.m"
   xmlns:mvc="sap.ui.core.mvc"
   displayBlock="true">
   <Shell>
      <App>
         <pages>
            <Page title="{i18n>homePageTitle}">
               <content>
                  <!-- page content -->
               </content>
            </Page>
         </pages>
      </App>
   </Shell>
</mvc:View>
```

**Review relevance**: If the root XML view wraps `sap.m.App` without a `Shell` and the app is expected to run standalone (outside the Fiori launchpad), it will not adapt correctly to larger screens. Classify as a `medium` finding for apps with standalone deployment requirement.

---

## UI5 OData V2 model — Template view and metadata loading (supplementary)

Source: OpenUI5 documentation (Context7 `/ui5/docs`)
Reference: https://github.com/ui5/docs/blob/main/docs/04_Essentials/xml-templating-5ee619f.md

OData V2 model creation with async metadata loading and annotation URI. The `loadMetadataAsync: true` flag is required for correct async initialization; without it, metadata loading blocks the main thread.

```javascript
const oModel = new ODataModel(
   "/sap/opu/odata/IWBEP/GWSAMPLE_BASIC/", {
      annotationURI: "/path/to/annotations.xml",
      json: true,
      loadMetadataAsync: true
   }
);
const oMetaModel = oModel.getMetaModel();
oMetaModel.loaded().then(() => {
   // Safe to access metadata here
});
```

**Review relevance**: Assess whether `loadMetadataAsync: true` is set on OData V2 model instances declared in `manifest.json` or instantiated in controllers. A V2 ODataModel without this flag blocks metadata loading synchronously. Classify as a `high` performance finding.

---

## UI5 app development best practices (supplementary)

Source: OpenUI5 documentation (Context7 `/ui5/docs`)
Reference: https://github.com/ui5/docs/blob/main/docs/03_Get-Started/best-practices-for-developers-28fcd55.md

SAP/OpenUI5 best practices for app development cover asynchronous loading, component setup, controller patterns, view and fragment handling, model usage, and resource bundle configuration. Key points for this skill:

- **Asynchronous loading**: All UI5 apps should use the `async` bootstrap parameter and declare `async: true` in the component configuration. Synchronous loading is deprecated and a performance finding.
- **Components**: Use `UIComponent.extend()` with `metadata.interfaces: ["sap.ui.core.IAsyncContentCreation"]` to enable async content creation and avoid rendering blockers.
- **Controllers**: Avoid storing references to UI controls in controller properties — use `byId` each time. Store only model references or simple state in controller instance properties.
- **Models**: Prefer declaring models in `manifest.json` over programmatic instantiation in controllers. Programmatic instantiation is acceptable for local view models not shared across views.
- **ResourceBundles**: Always reference i18n text keys via `{i18n>key}` binding in XML views rather than resolving them programmatically unless dynamic text construction is required.

**Review relevance**: Use these best practices as the basis for `medium` and `low` findings in the App Structure, XML View, and Controller review domains. Flag violations of asynchronous loading as `high` findings.

---

## Scope boundaries for Context7 usage

Context7 OpenUI5/SAPUI5 documentation applies to the following review domains in this skill:

- **App Structure** (manifest.json, component configuration, async loading): directly applicable
- **XML View** (view patterns, Shell control, fragment usage): directly applicable
- **Controller** (onInit patterns, model setup, lifecycle hooks): directly applicable
- **OData Consumption** (ODataModel V2 creation, metadata async loading, model binding): directly applicable
- **Accessibility** (ARIA labeling in XML views, accessible control patterns): applicable for control-level patterns
- **Fiori Design** and **Launchpad**: use official SAP Fiori design guidelines (experience.sap.com) and SAP Help Portal as primary sources; Context7 provides supplementary implementation detail only

Always label Context7-sourced guidance as `context7-supplementary` in responses. For SAPUI5 version-specific control behavior or deprecated API lists, direct the user to verify against the official SAPUI5 changelog at ui5.sap.com.
