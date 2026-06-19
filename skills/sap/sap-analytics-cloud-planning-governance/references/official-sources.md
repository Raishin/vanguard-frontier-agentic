# Official sources — SAP Analytics Cloud Planning Governance Review

Use this reference when grounding SAC story design, planning model governance, data action review, allocation assessment, connection type evaluation, data access control audit, and performance posture review.

**Evidence level**: documentation-based (SAP Analytics Cloud Help Portal). No live-system evidence is collected by this skill.

## Stories and models

- Stories in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/stories
  source_owner: SAP SE
  topic_supported: Story design, page layout, widget types, filter configuration, model binding, calculated measures in stories
  why_needed: Primary reference for classifying story design findings including widget-to-model binding correctness, filter scope, and story-level performance governance
  evidence_level: primary
  last_verified: 2026-06-19

## Planning models

- Planning models in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/planning-models
  source_owner: SAP SE
  topic_supported: Planning model creation, version category structure (actual, budget, forecast, rolling forecast), account dimension, date dimension, measure definition, model locking
  why_needed: Defines the planning model structure — required to classify version category design gaps, dimension configuration errors, and missing version locking as planning integrity findings
  evidence_level: primary
  last_verified: 2026-06-19

## Data actions

- Data actions in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/data-actions
  source_owner: SAP SE
  topic_supported: Data action step types (Advanced Formula, copy, allocation, date-based distribution), step sequencing, trigger configuration, multi-action orchestration
  why_needed: Defines the data action execution model — required to classify step sequencing errors, incorrect scope definitions, and missing trigger configuration as planning pipeline findings
  evidence_level: primary
  last_verified: 2026-06-19

## Allocations

- Allocations in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/allocations
  source_owner: SAP SE
  topic_supported: Allocation methods (spread, distribution, breakback), driver dimension selection, allocation hierarchy scope, allocation result validation
  why_needed: Defines the allocation step model — required to classify incorrect allocation method selection, unbounded hierarchy scope, and missing validation as planning process findings
  evidence_level: primary
  last_verified: 2026-06-19

## Connections

- Live data connections in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/live-data-connections
  source_owner: SAP SE
  topic_supported: Live connection vs. import connection capabilities, supported source systems, connection type trade-offs, import data refresh scheduling, incremental load configuration
  why_needed: Defines the connection type model — required to classify live vs. import connection misuse, missing refresh schedules on import models, and incremental load configuration gaps
  evidence_level: primary
  last_verified: 2026-06-19

## Data access controls

- Data access control in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/data-access-control
  source_owner: SAP SE
  topic_supported: Role-based access (BI Admin, Planner, Viewer, custom roles), team-level data access, dimension member-level access restrictions in planning models, folder and story sharing permissions
  why_needed: Defines the SAC access control model — required to classify over-permissive role assignments, missing dimension-level data restrictions, and ungoverned story sharing as access control findings
  evidence_level: primary
  last_verified: 2026-06-19

## Performance

- Performance optimization in SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/performance
  source_owner: SAP SE
  topic_supported: Story performance optimization, widget count guidelines, query reduction strategies, model size limits, import model caching, live connection query push-down behavior
  why_needed: Defines the performance model for SAC stories and models — required to classify stories with excessive widget density, unbounded query scope, or import model refresh cadence gaps as performance findings
  evidence_level: primary
  last_verified: 2026-06-19

## Overview

- What is SAP Analytics Cloud
  https://help.sap.com/docs/sap-analytics-cloud/sap-analytics-cloud/what-is-sap-analytics-cloud
  source_owner: SAP SE
  topic_supported: SAP Analytics Cloud product overview, feature set, BI and planning capabilities, tenant model
  why_needed: Provides the product-level context for all governance assessment; defines capability boundaries and tenant model
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Analytics Cloud Help Portal documentation describes the designed behavior of stories, planning models, data actions, allocations, connections, and access controls. It does not prove which models exist in the user's tenant, whether data actions are correctly configured, or whether version locking is in effect. Users must supply story screenshots, model configuration exports, data action scripts, or written descriptions for concrete assessment.
