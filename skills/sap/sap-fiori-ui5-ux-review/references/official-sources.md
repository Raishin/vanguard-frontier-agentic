# Official sources — SAP Fiori and UI5 UX Review

Use this reference when grounding Fiori design guideline adherence, SAPUI5 app structure, OData consumption, accessibility, and Fiori launchpad integration guidance.

**Evidence level**: documentation-based (SAP Fiori design guidelines at experience.sap.com, SAPUI5 documentation at ui5.sap.com, SAP Help Portal). No live-system evidence is collected by this skill.

## SAP Fiori Design Guidelines

- SAP Fiori Design Guidelines (Web)
  https://experience.sap.com/fiori-design-web/
  source_owner: SAP SE
  topic_supported: Fiori floorplan selection (List Report, Object Page, Overview Page, Worklist, Wizard), header and footer bar patterns, control usage guidelines, responsive design, UX pattern catalog
  why_needed: Primary source for classifying Fiori design guideline deviations; defines which floorplan is correct for a given use case and what header/footer patterns are required
  evidence_level: primary
  last_verified: 2026-06-19

## SAPUI5 App Descriptor (manifest.json)

- SAPUI5 App Descriptor Reference
  https://ui5.sap.com/#/topic/003f755d46d34dd1bbce9ffe08c8d46a
  source_owner: SAP SE
  topic_supported: manifest.json structure — sap.app, sap.ui, sap.ui5 sections; data source declarations; routing configuration; model declarations; component settings; i18n bundle registration
  why_needed: Authoritative reference for reviewing manifest.json completeness and identifying missing or incorrect configuration entries
  evidence_level: primary
  last_verified: 2026-06-19

## SAPUI5 MVC and Component Architecture

- SAPUI5 MVC Overview
  https://ui5.sap.com/#/topic/91f0a22d6f4d1014b6dd926db0e91070
  source_owner: SAP SE
  topic_supported: Model-View-Controller pattern in UI5 — view types (XML, JSON, HTML), controller lifecycle hooks (onInit, onExit, onBeforeRendering, onAfterRendering), model binding, controller extension
  why_needed: Defines correct MVC separation and lifecycle hook patterns; used to classify controller code that violates MVC separation or misuses lifecycle hooks
  evidence_level: primary
  last_verified: 2026-06-19

- SAPUI5 Components and Descriptor
  https://ui5.sap.com/#/topic/958ead51e2e94ab8bcdc90fb7e9d53d0
  source_owner: SAP SE
  topic_supported: UIComponent instantiation, async component loading, component configuration, index.html bootstrap setup, async rendering
  why_needed: Defines the async component loading pattern; used to classify synchronous loading as a high performance finding
  evidence_level: primary
  last_verified: 2026-06-19

## OData Consumption

- SAPUI5 OData V2 Model
  https://ui5.sap.com/#/topic/6c47b2b39db9404582994070ec3d57a2
  source_owner: SAP SE
  topic_supported: ODataModel V2 configuration, read/create/update/delete operations, error callbacks, batch mode, deferred groups, filter and sort expressions, metadata handling
  why_needed: Authoritative source for OData V2 consumption patterns; used to classify missing error callbacks, incorrect batch mode settings, and filter expression anti-patterns
  evidence_level: primary
  last_verified: 2026-06-19

- SAPUI5 OData V4 Model
  https://ui5.sap.com/#/topic/5338bd1f9afb45fb8b2af957c3530e8f
  source_owner: SAP SE
  topic_supported: ODataModel V4 configuration, list and context binding, auto-batch mode, PATCH-based updates, late properties, binding parameters
  why_needed: Authoritative source for OData V4 consumption patterns; used to classify V4-specific binding issues and incorrect auto-batch configuration
  evidence_level: primary
  last_verified: 2026-06-19

## Accessibility

- SAPUI5 Accessibility Guidelines
  https://ui5.sap.com/#/topic/68b9644a253741e8a4b9e4279a35c247
  source_owner: SAP SE
  topic_supported: ARIA landmark usage, labelFor associations, high-contrast theme support, keyboard navigation, screen reader compatibility, getAccessibilityInfo() implementation for custom controls
  why_needed: Defines SAPUI5 accessibility requirements; used to classify missing ARIA labels, incorrect form field labeling, and inaccessible custom controls as high findings
  evidence_level: primary
  last_verified: 2026-06-19

## Fiori Launchpad Integration

- SAP Fiori Launchpad Overview
  https://help.sap.com/docs/SAP_FIORI_LAUNCHPAD/fd8d03d7-14b9-40f7-9a07-1a594f20ac8d/what-is-sap-fiori-launchpad
  source_owner: SAP SE
  topic_supported: Fiori launchpad architecture, tile configuration, semantic object and action definition, target mapping, cross-application navigation, app descriptor inbound/outbound navigation declarations
  why_needed: Defines Fiori launchpad integration requirements; used to classify incorrect semantic object definitions, missing target mappings, and broken cross-application navigation intents
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Fiori Elements

- SAPUI5 Fiori Elements Overview
  https://ui5.sap.com/#/topic/c9ab34570cc14ea5ab72a6d1a4a03e3f
  source_owner: SAP SE
  topic_supported: Fiori Elements floorplans (List Report, Object Page, Analytical List Page, Overview Page), annotation-driven rendering, extension points, building blocks
  why_needed: Defines the Fiori Elements rendering model; used to distinguish annotation-driven findings from freestyle findings and to classify incorrect extension point usage
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Fiori design guidelines and SAPUI5 documentation describe design intent and recommended patterns. They do not prove which SAPUI5 controls are rendered correctly in the user's deployed UI5 version, which Fiori Elements features are available for their SAP system version, or whether the Fiori launchpad target mapping is correctly configured in their specific SAP BTP or on-premise launchpad. Users must supply manifest files, XML views, controller code, and OData metadata for concrete assessment.
