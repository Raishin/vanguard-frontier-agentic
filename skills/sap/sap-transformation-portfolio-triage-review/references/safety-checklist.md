# Safety checklist — SAP Transformation Portfolio Triage Review

Use before making any triage recommendation, especially for findings that affect workstream sequencing, dependency resolution, RAID escalation, or readiness gate decisions with go-live implications.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system, SAP Cloud ALM tenant, project management tool, or transformation program repository. This skill reviews artifacts only.
- Do not accept or request SAP system credentials, SAP Cloud ALM API tokens, Jira access tokens, or any live project tool authentication.
- Do not recommend changing workstream sequencing without first understanding which downstream workstreams depend on the current sequence. Sequence changes in a running program have cascade effects.
- Do not classify a RAID item as low risk without checking whether it appears in the critical path. A technically low-risk issue on the critical path is a program risk.
- Do not recommend descoping a workstream without first confirming what regulatory, contractual, or go-live dependency that workstream satisfies.
- Do not use inference alone to conclude that a workstream is blocked. Blocking dependency findings must be traceable to user-provided evidence or official SAP Activate methodology.
- Do not conflate methodology phase with calendar phase. A program may label its phases differently from SAP Activate. Identify the equivalent phase before applying readiness gate criteria.

## What people get wrong

- **Starting configuration before fit-to-standard is complete**: This is the most common S/4HANA workstream sequencing error. Configuration decisions made before fit-to-standard workshops are complete are built on unvalidated scope and frequently require rework.
- **Treating integration as a sub-workstream of S/4HANA**: Integration is a cross-cutting workstream that must be scoped, designed, and tested independently. Programs that subordinate integration to the S/4HANA track consistently underplan interface inventory and integration testing.
- **RAID log treated as a status report rather than a risk instrument**: RAID logs that record issues without mitigation plans or owners are governance documents, not risk management tools. Flag this pattern explicitly.
- **Readiness gates treated as checkboxes rather than gates**: A quality gate where all items are marked complete but supporting evidence is not reviewed is not a gate — it is a milestone stamp. Push back on programs that cannot produce evidence for gate criteria.
- **Change management starting in Deploy phase**: Change management that begins during deployment is structurally too late. Stakeholder engagement and training needs analysis must begin in Prepare or Explore at the latest.
- **Data migration treated as a Deploy-phase activity**: Data migration preparation (extraction, cleansing, transformation design) must begin in Explore. Programs that start data migration in Realize are at high risk of cutover failure.
- **Underestimating security and compliance workstream duration**: Authorization concept design, SoD matrix definition, and GRC configuration are consistently underestimated. They depend on fit-to-standard output and take longer than expected in complex organizations.

## When to push back

- Push back when the user asks to triage a program without providing any program artifact (project plan, RAID log, workstream charter, or dependency map). Triage from memory alone is inference, not assessment.
- Push back when the user asks to confirm a workstream is on track without providing evidence of readiness gate completion.
- Push back when a request requires live access to SAP Cloud ALM, Jira, or another project tool — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or descriptions.
- Push back when the user asks to recommend a specific go-live date without providing a complete view of the critical path across workstreams.

## Evidence labels

- `documentation-based` — grounded in SAP Activate methodology, SAP Cloud ALM program management docs, or official SAP transformation guidance
- `user-provided evidence` — project plans, RAID logs, workstream charters, dependency maps, or steering decks provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
