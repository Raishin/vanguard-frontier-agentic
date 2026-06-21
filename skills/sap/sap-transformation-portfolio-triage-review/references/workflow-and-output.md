# Workflow and output contract — SAP Transformation Portfolio Triage Review

Use this reference for workstream classification, dependency assessment, readiness gate evaluation, RAID log review, and output formatting.

## Workstream classification taxonomy

| Workstream domain | Scope | Typical triage findings |
|-------------------|-------|------------------------|
| `s4hana-conversion` | System conversion, greenfield, or selective data transition; fit-to-standard workshops; configuration; unit and integration testing; cutover | Fit-to-standard skipped, configuration started before gap analysis, cutover plan missing, no regression test scope defined |
| `btp-platform` | BTP account model, subaccount design, environment provisioning, extensibility foundation | BTP provisioned after S/4HANA go-live planning, extensibility design not aligned to clean core decisions, no BTP governance model defined |
| `integration` | Integration architecture, middleware selection, API design, interface inventory, integration testing | Interface inventory incomplete, integration platform not selected before S/4HANA configuration, no end-to-end integration test plan |
| `data-migration` | Legacy data extraction, cleansing, transformation, loading, reconciliation, data archiving | Data migration started too late, no data quality baseline established, reconciliation criteria undefined |
| `security-and-compliance` | Role design, authorization concept, GRC, SoD matrix, audit readiness, regulatory compliance | Authorization concept not started before configuration, SoD rules not defined, no security acceptance criteria for go-live |
| `change-management` | Stakeholder engagement, training design, communication plan, readiness assessment, hypercare | Change management not started until Deploy phase, no training needs analysis, hypercare plan missing |
| `infrastructure` | Hosting, network, connectivity, SAP infrastructure readiness, cloud migration | Infrastructure sizing not complete before Realize, connectivity to legacy systems not tested, DR not defined |

## Risk severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Dependency or sequencing failure that directly threatens the program go-live date, a regulatory deadline, or creates a mandatory rework loop (e.g., configuration started without fit-to-standard, cutover planned without data migration reconciliation) |
| `high` | Workstream without a defined readiness gate entering the next phase, unresolved cross-workstream blocking dependency, RAID risk with no mitigation plan and high program impact |
| `medium` | RAID gap (unowned risk, undated assumption, issue without resolution path), workstream prioritization imbalance (high-value workstream under-resourced), missing but non-blocking dependency |
| `low` | Best practice deviation in program structure (no naming convention, inconsistent workstream reporting cadence, RAID log not reviewed in more than two steering cycles) |

## RAID log assessment criteria

For each RAID item category:

- **Risks**: Does each risk have an owner, a likelihood and impact rating, a mitigation plan, and a review date? Unmitigated high-impact risks with no owner are `high` findings.
- **Assumptions**: Does each assumption have an owner and a validation date? Stale assumptions (no validation in more than one SAP Activate phase) are `medium` findings.
- **Issues**: Does each issue have an owner, a target resolution date, and a status? Issues open beyond the resolution date with no escalation are `high` findings.
- **Decisions**: Is each decision recorded with the decision maker, the date, and the rationale? Undocumented decisions create ambiguity that manifests as rework in Realize and Deploy.

## Readiness gate evaluation

Per SAP Activate phase exit:

| Phase exit | Key readiness evidence required |
|------------|--------------------------------|
| Prepare exit | Project charter signed, team mobilized, system landscape confirmed, sandbox provisioned |
| Explore exit | Fit-to-standard workshops complete, gap list approved, extensibility decisions recorded, integration inventory complete |
| Realize exit | Configuration complete, unit test sign-off, integration test complete, data migration dry run executed, training materials drafted |
| Deploy exit | User acceptance testing sign-off, cutover rehearsal complete, hypercare plan approved, go/no-go criteria met |

A workstream that advances to the next phase without completing the prior phase exit evidence set is a `high` readiness gate finding.

## Workflow

1. **Receive artifacts** — project plan, RAID log, workstream charters, dependency maps, steering committee decks, or program descriptions.
2. **Classify each workstream** by domain.
3. **Map dependencies** — identify predecessor, shared-outcome, and cross-workstream integration dependencies.
4. **Assess readiness gates** per SAP Activate phase exit for each workstream.
5. **Review RAID log** for completeness, ownership, and resolution status.
6. **Classify findings** by risk level (critical / high / medium / low).
7. **Prioritize** — critical sequencing or gate failures first; then high dependency and RAID issues; then medium gaps; then low best-practice items.
8. **Return output** per the output contract below.

## Output contract

Return:

1. Workstream domain and specific triage finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Recommended action (resequence, add readiness gate, escalate RAID item, restructure dependency, adjust resourcing)
5. Dependency chain affected (if applicable)
6. Prioritized remediation sequence
7. Escalation trigger if live project tool access is required to confirm the finding
