# Official sources — SAP Transformation Portfolio Triage Review

Use this reference when grounding workstream classification, phase sequencing, readiness gate criteria, RAID assessment, and SAP Activate methodology alignment.

**Evidence level**: documentation-based (SAP Activate methodology, SAP Cloud ALM, SAP Help Portal). No live-system evidence is collected by this skill.

## SAP Activate methodology

- SAP Activate Methodology Guide
  https://help.sap.com/docs/activate-methodology/sap-activate-methodology/sap-activate-methodology-guide
  source_owner: SAP SE
  topic_supported: SAP Activate phases (Discover, Prepare, Explore, Realize, Deploy, Run), phase exit criteria, accelerators, methodology principles
  why_needed: Primary methodology reference for assessing whether transformation workstreams are sequenced correctly relative to SAP Activate phase gates
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Activate Workstreams
  https://help.sap.com/docs/activate-methodology/sap-activate-methodology/workstreams
  source_owner: SAP SE
  topic_supported: Standard SAP Activate workstream taxonomy, workstream-level task lists, dependencies between workstreams, workstream ownership
  why_needed: Defines the canonical workstream structure against which user-provided program workstreams are classified and assessed for completeness
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Activate Quality Gates
  https://help.sap.com/docs/activate-methodology/sap-activate-methodology/quality-gates
  source_owner: SAP SE
  topic_supported: Quality gate criteria per SAP Activate phase, exit checklist items, gate review process, readiness evidence requirements
  why_needed: Defines the readiness gate standard for each phase exit — required to assess whether workstreams have sufficient readiness evidence before proceeding
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Activate Methodology (SAP Support Portal)
  https://support.sap.com/en/alm/sap-activate.html
  source_owner: SAP SE
  topic_supported: SAP Activate roadmap viewer, accelerator catalog, methodology updates, links to scenario-specific roadmaps (S/4HANA, cloud, hybrid)
  why_needed: Entry point for scenario-specific roadmaps and accelerators — required when the transformation scenario (greenfield, brownfield, selective data transition) affects workstream structure
  evidence_level: primary
  last_verified: 2026-06-19

## S/4HANA fit-to-standard and scope

- Fit-to-Standard Analysis
  https://help.sap.com/docs/s4hana-cloud/sap-s-4hana-cloud/fit-to-standard-analysis
  source_owner: SAP SE
  topic_supported: Fit-to-standard workshop process, scope item selection, gap identification, delta design output as input to extensibility decisions
  why_needed: Defines the mandatory sequence of fit-to-standard before solution design and configuration — key to identifying S/4HANA workstream sequencing violations
  evidence_level: primary
  last_verified: 2026-06-19

## SAP Cloud ALM program management

- SAP Cloud ALM Portfolio and Project Management
  https://help.sap.com/docs/cloud-alm/applicationhelp/portfolio-and-project-management
  source_owner: SAP SE
  topic_supported: Portfolio-level program structure in SAP Cloud ALM, project hierarchy, milestone tracking, cross-project dependency visibility
  why_needed: Reference for organizations using SAP Cloud ALM as their program management tool — enables assessment of whether program artifacts are being captured correctly
  evidence_level: primary
  last_verified: 2026-06-19

- SAP Cloud ALM Project Management
  https://help.sap.com/docs/cloud-alm/applicationhelp/project-management
  source_owner: SAP SE
  topic_supported: Task management, RAID log management, project timeline, workstream progress tracking, deliverable status within SAP Cloud ALM
  why_needed: Defines the SAP-native approach to RAID log management and project task tracking — required when assessing whether the user's RAID log structure aligns with SAP Cloud ALM conventions
  evidence_level: primary
  last_verified: 2026-06-19

## Grounding rule

SAP Activate methodology and SAP Cloud ALM documentation describe the designed transformation methodology and program management approach. They do not prove what workstreams exist in the user's program, how dependencies are managed, or whether readiness gates have been passed. Users must supply project plans, RAID logs, workstream charters, or architecture decision records for concrete triage assessment.
