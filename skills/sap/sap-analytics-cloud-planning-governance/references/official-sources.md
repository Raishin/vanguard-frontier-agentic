# Official sources — SAP Analytics Cloud Planning Governance Review

Use this reference when grounding SAC story design, planning model governance, data action review, allocation assessment, connection type evaluation, data access control audit, and performance posture review.

**Evidence level**: documentation-based (SAP Analytics Cloud Help Portal). No live-system evidence is collected by this skill.

## Stories and models

- Creating stories in SAP Analytics Cloud
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/e4a450f79e7b4218a1fb7dd78b13b7fd.html
  source_owner: SAP SE
  topic_supported: Story design, page layout, widget types, filter configuration, model binding, calculated measures in stories
  why_needed: Primary reference for classifying story design findings including widget-to-model binding correctness, filter scope, and story-level performance governance
  evidence_level: primary
  last_verified: 2026-06-19

## Planning models

- Planning in SAP Analytics Cloud
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/5b12c2e3b0f14e31bcee9b0b19d3b254.html
  source_owner: SAP SE
  topic_supported: Planning model creation, version category structure (actual, budget, forecast, rolling forecast), account dimension, date dimension, measure definition, model locking
  why_needed: Defines the planning model structure — required to classify version category design gaps, dimension configuration errors, and missing version locking as planning integrity findings
  evidence_level: primary
  last_verified: 2026-06-19

## Data actions

- Data actions in SAP Analytics Cloud
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/6e4a9b3cd3a44f85a7b6d1f6a3e3e9ef.html
  source_owner: SAP SE
  topic_supported: Data action step types (Advanced Formula, copy, allocation, date-based distribution), step sequencing, trigger configuration, multi-action orchestration
  why_needed: Defines the data action execution model — required to classify step sequencing errors, incorrect scope definitions, and missing trigger configuration as planning pipeline findings
  evidence_level: primary
  last_verified: 2026-06-19

- Advanced Formula reference for data actions
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/4d9b87e8c6504e84a7c6e19e6d9f8c5a.html
  source_owner: SAP SE
  topic_supported: Advanced Formula language reference, MEMBERSET, RESULTLOOKUP, IF/ELSE, FOREACH constructs, scope rules for formula execution
  why_needed: Defines Advanced Formula scope rules — required to flag unbounded MEMBERSET and RESULTLOOKUP expressions that operate on full model scope and cause performance or correctness issues
  evidence_level: primary
  last_verified: 2026-06-19

## Allocations

- Allocation steps in data actions
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/1c0082b826ed4437b2e0494e78d2fd68.html
  source_owner: SAP SE
  topic_supported: Allocation methods (spread, distribution, breakback), driver dimension selection, allocation hierarchy scope, allocation result validation
  why_needed: Defines the allocation step model — required to classify incorrect allocation method selection, unbounded hierarchy scope, and missing validation as planning process findings
  evidence_level: primary
  last_verified: 2026-06-19

## Connections

- Connecting to data sources in SAP Analytics Cloud
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/7c35e27de90c46e8b9e83fe21fce95a5.html
  source_owner: SAP SE
  topic_supported: Live connection vs. import connection capabilities, supported source systems, connection type trade-offs, import data refresh scheduling, incremental load configuration
  why_needed: Defines the connection type model — required to classify live vs. import connection misuse, missing refresh schedules on import models, and incremental load configuration gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Data access controls

- Managing user access and security in SAP Analytics Cloud
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/d944bf14c7c744e9b4f0499d36e99e39.html
  source_owner: SAP SE
  topic_supported: Role-based access (BI Admin, Planner, Viewer, custom roles), team-level data access, dimension member-level access restrictions in planning models, folder and story sharing permissions
  why_needed: Defines the SAC access control model — required to classify over-permissive role assignments, missing dimension-level data restrictions, and ungoverned story sharing as access control findings
  evidence_level: primary
  last_verified: 2026-06-19

## Performance

- Optimizing SAP Analytics Cloud model and story performance
  https://help.sap.com/docs/SAP_ANALYTICS_CLOUD/00f68c2e08b941f081002fd3691d86a7/b65b3a9a6e6447b29b9c9be1f3f5e7d2.html
  source_owner: SAP SE
  topic_supported: Story performance optimization, widget count guidelines, query reduction strategies, model size limits, import model caching, live connection query push-down behavior
  why_needed: Defines the performance model for SAC stories and models — required to classify stories with excessive widget density, unbounded query scope, or import model refresh cadence gaps as performance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Analytics Cloud Help Portal documentation describes the designed behavior of stories, planning models, data actions, allocations, connections, and access controls. It does not prove which models exist in the user's tenant, whether data actions are correctly configured, or whether version locking is in effect. Users must supply story screenshots, model configuration exports, data action scripts, or written descriptions for concrete assessment.
