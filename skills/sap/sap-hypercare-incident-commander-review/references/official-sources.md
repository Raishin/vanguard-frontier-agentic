# Official sources — SAP Hypercare and Incident Commander Review

Use this reference when grounding severity triage assessment, war-room governance evaluation, incident and problem workflow review, root-cause investigation process assessment, escalation path validation, and exit-from-hypercare criteria evaluation.

**Evidence level**: documentation-based (SAP Help Portal, SAP Activate methodology documentation, SAP Active Global Support documentation, SAP Cloud ALM application help). No live-system evidence is collected by this skill.

## SAP Activate — Hypercare and Stabilization Methodology

- SAP Activate — Hypercare Plan and Run Phase Stabilization
  https://help.sap.com/docs/SAP_ACTIVATE/80d20672e1e74bde9f0c7f84cda1e3a6/f5f6e8c0c82043f3b33879ef88f26f2b.html
  source_owner: SAP SE
  topic_supported: SAP Activate Run phase hypercare deliverables: hypercare plan structure, stabilization period governance, incident triage during go-live, war-room setup, support coverage model, handover from project to operations, exit-from-hypercare criteria and governance
  why_needed: Primary reference for hypercare governance assessment — defines the SAP Activate-recommended hypercare plan deliverables, stabilization governance model, and exit criteria standards used to classify gaps in hypercare plan completeness and exit criteria definition
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Activate — Go-Live Readiness and Hypercare Planning in the Deploy Phase
  https://help.sap.com/docs/SAP_ACTIVATE/80d20672e1e74bde9f0c7f84cda1e3a6/3c3e8f1bb1e64df6a2e1f3ad1a1d9c0e.html
  source_owner: SAP SE
  topic_supported: Deploy phase hypercare preparation: hypercare team structure, on-call coverage model, escalation path definition, war-room charter, go-live communication plan, Day 1 and Week 1 governance
  why_needed: Required to assess whether hypercare preparation began in the Deploy phase — defines the pre-go-live hypercare planning deliverables, war-room charter requirements, and on-call coverage structure used to evaluate whether the hypercare model is adequately prepared before go-live
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM — Incident and Problem Management

- IT Task Management in SAP Cloud ALM
  https://help.sap.com/docs/cloud-alm/applicationhelp/it-task-management
  source_owner: SAP SE
  topic_supported: Incident and problem record management in SAP Cloud ALM, severity classification model, escalation path configuration, ITSM platform integration, knowledge article linkage, task assignment and workflow, incident lifecycle governance
  why_needed: Required to assess Cloud ALM incident and problem management process governance during hypercare — defines the severity model, problem record linkage capability, escalation workflow, and incident lifecycle states used to classify incident management gaps in the hypercare window
  evidence_level: primary
  last_verified: 2026-06-19

- Analytics and Reporting in SAP Cloud ALM
  https://help.sap.com/docs/cloud-alm/applicationhelp/analytics-and-reporting
  source_owner: SAP SE
  topic_supported: Hypercare and operations reporting in SAP Cloud ALM: incident frequency reporting, severity distribution dashboards, SLA performance reporting, business process health tracking, operations cockpit for stabilization monitoring
  why_needed: Reference for assessing whether hypercare governance reporting is configured in SAP Cloud ALM — defines the reporting and analytics capabilities available for tracking incident frequency, severity trends, and stabilization progress used to evaluate whether exit-from-hypercare criteria can be measured objectively
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Support — Active Global Support and Escalation

- SAP Support — Product Support and Incident Management
  https://support.sap.com/en/my-support/product-support.html
  source_owner: SAP SE
  topic_supported: SAP support incident creation and management, priority definitions (P1 through P4), SAP support SLA by priority, escalation procedures within SAP support, SAP ONE Support Launchpad access
  why_needed: Required to assess escalation path completeness — defines the SAP support priority model, SLA commitments per priority, and escalation procedures within SAP product support used to evaluate whether the hypercare escalation procedure correctly maps internal severity tiers to SAP support priorities
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Active Global Support — Critical Situation Handling and Premium Engagement
  https://support.sap.com/en/offerings-programs/support-services/active-global-support.html
  source_owner: SAP SE
  topic_supported: SAP Active Global Support (AGS) service portfolio, Critical Situation (CritSit) procedure, SAP MaxAttention and Preferred Success engagement models, escalation to SAP product management for blocking issues, SAP Go-Live Support services
  why_needed: Required to assess premium support escalation path completeness — defines the SAP CritSit procedure, MaxAttention and Preferred Success escalation model, and Go-Live Support services used to evaluate whether organizations with premium SAP support agreements have correctly configured and tested their escalation paths for hypercare
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Help Portal, SAP Activate methodology documentation, and SAP Active Global Support documentation describe the designed hypercare governance model, recommended incident management workflow, severity triage framework, and escalation procedure for SAP programs. They do not prove how the user's hypercare team is structured, what severity criteria the user's program has defined, whether the user's escalation contacts are reachable during hypercare hours, or whether the user's exit criteria are measurable. Users must supply hypercare plan documents, war-room governance charters, incident management process descriptions, severity triage definitions, escalation procedure documents, and exit criteria documentation for concrete governance assessment.
