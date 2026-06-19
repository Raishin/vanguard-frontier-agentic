# Safety checklist — SAP MDG Master Data Quality Review

Use before making any MDG configuration or data governance remediation recommendation, especially for findings involving governance workflow, validation rules, mass processing, or key mapping.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP MDG system, S/4HANA backend, Fiori launchpad, BRFplus runtime, or replication target system. This skill reviews artifacts only.
- Do not accept or request SAP logon credentials, MDG admin access, RFC connection details, or direct database access.
- Do not create, modify, approve, reject, activate, or delete any master data change request, consolidation object, mass change template run, or governance workflow step. This skill is strictly advisory.
- Do not execute or trigger MDG mass change templates, consolidation runs, or replication jobs. These operations can affect large volumes of master data records and must be executed by authorized MDG administrators in a controlled change window.
- Do not recommend activating BRFplus validation or derivation rules directly in a production MDG system. All rule changes must first be tested in a development or quality system with representative change request data.
- Do not use memory alone to determine which BRFplus application or decision table governs a specific MDG validation scenario. BRFplus rule assessment must be grounded in user-provided rule exports or official MDG documentation.
- Do not conflate MDG staging area data with active master data. Changes in the MDG staging area are not effective until a change request is approved and activated. Governance gaps in the workflow layer can prevent activation — but data in the staging area is not yet live.

## What people get wrong

- **Treating all derivation rules as safe**: Derivation rules that silently overwrite fields the user has already populated are a data quality risk, not just a convenience feature. Organizations often add derivations to improve consistency but do not account for cases where the derivation logic is wrong for edge cases — and the user cannot tell that their input was overwritten.
- **Assuming BRFplus covers all validation**: BRFplus is the primary validation tool in MDG, but custom BAdI implementations (USMD_RULE_SERVICE or equivalent) can override or bypass BRFplus rules. Both layers must be reviewed for a complete validation coverage picture.
- **Overlooking the bypass risk in direct activation**: Some MDG change request type configurations allow a "direct activation" path for certain user groups or attribute conditions, bypassing the normal workflow approval chain. This is a common governance gap that is not always visible in the standard workflow template view — it requires checking the change request type customizing directly.
- **Confusing MDG consolidation with data migration**: MDG consolidation is an ongoing governance process for deduplicating and harmonizing master data from multiple source systems. It is not a one-time migration activity. Consolidation run errors that accumulate in a production system without monitoring are an ongoing data quality risk.
- **Treating data quality KPIs as a reporting-only concern**: MDG data quality KPIs are most valuable when they drive workflow — for example, triggering a data steward review task when a score falls below threshold. KPIs that are calculated but never acted on provide measurement without governance.
- **Missing replication monitoring as a governance gap**: When MDG replication to a target system fails silently, the target system continues to operate with stale or incorrect master data. Missing replication error monitoring is a governance gap because the organization cannot tell which records are out of sync.
- **Conflating MDG for Finance with MDG for Business Partner**: These are distinct MDG domains with separate data models, governance workflows, and validation frameworks. A well-governed MDG for Business Partner domain does not imply that MDG for Finance G/L account and cost center governance is equally mature — each domain must be assessed independently.

## When to push back

- Push back when a governance bypass risk (direct activation without workflow, unrestricted mass change execution) is identified — escalate to the data governance team before recommending any other remediation.
- Push back when the user asks to confirm MDG governance compliance from memory alone without providing BRFplus rule exports, workflow configuration descriptions, or data model documents.
- Push back when the request requires live MDG system access (active change request status, live data quality scores, replication queue state) — state clearly that live inspection is out of scope and ask the user to supply the relevant export or description.
- Push back when asked to create, modify, approve, activate, or run any master data operation — this is an absolute boundary and must be refused in all circumstances.
- Push back when the user asks to assess duplicate detection quality without providing the match profile configuration or a sample of known duplicate pairs — threshold assessment without data cannot be meaningful.

## Evidence labels

- `documentation-based` — grounded in SAP Master Data Governance Help Portal documentation, BRFplus documentation, MDG data model guides, or SAP MDG best practice guidance
- `user-provided evidence` — BRFplus rule exports, workflow configuration summaries, KPI dashboard descriptions, key mapping configuration notes, data model design documents, consolidation run logs, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
