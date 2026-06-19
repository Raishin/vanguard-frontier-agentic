# Workflow and output contract — SAP S/4HANA Transformation Architecture Review

Use this reference for all strategy classification, risk assessment, and output formatting.

## Transformation strategy taxonomy

| Strategy | Definition | Primary use case | Key risk |
|----------|-----------|-----------------|---------|
| `brownfield` | System conversion of existing ECC or S/4HANA system in-place | Organizations wanting to retain historical data, custom configurations, and existing processes with minimal disruption | Simplification item volume; custom code and modification debt surfaced at conversion time |
| `greenfield` | New S/4HANA implementation starting from a clean system | Organizations willing to re-implement processes to standard; strong fit-to-standard discipline | Business re-design effort; data migration from legacy; organizational change management scope |
| `selective-data-transition` | Hybrid approach migrating selected data subsets from legacy to a new S/4HANA system | Organizations needing both a clean system baseline and selective historical data retention | Highest complexity; dual tooling (LT Replication + Migration Cockpit); cutover sequencing risk |
| `hybrid-phased` | Multi-wave program combining strategies across entities, geographies, or modules | Large enterprises with heterogeneous landscapes (mixed ECC versions, multiple SIDs) | Governance risk; data consistency across wave boundaries; integration architecture between legacy and S/4HANA systems during transition |

## Deployment model taxonomy

| Model | ABAP extensibility | Update cycle | Operations responsibility |
|-------|------------------|-------------|--------------------------|
| `cloud-public-edition` | Key-user + side-by-side only; no custom ABAP in-system | Quarterly (mandatory) | SAP-managed SaaS |
| `cloud-private-edition` | ABAP developer extensibility permitted; clean core recommended | Aligned with SAP S/4HANA on-premise release rhythm | Customer-managed on hyperscaler or SAP Managed Cloud |
| `rise-with-sap` | Depends on underlying edition (Public or Private); RISE is a bundled commercial offering, not a separate technical variant | Inherits from underlying edition | SAP or partner managed |
| `on-premise` | Full ABAP developer extensibility; clean core optional | Customer-controlled; SAP support packages and enhancement packages | Customer IT operations |

## SAP Activate phase alignment assessment

For each phase, assess whether the user's program has completed required deliverables:

| Phase | Key deliverables | Common gaps |
|-------|-----------------|------------|
| Discover | Business case, transformation roadmap, deployment model decision | No formal deployment model decision; roadmap not SAP Activate structured |
| Prepare | Project plan, system landscape, governance model, initial Readiness Check run | Readiness Check not yet executed; governance model not defined |
| Explore | Fit-to-standard workshops, delta design for gaps, simplification item resolution plan | Fit-to-standard workshops incomplete; gaps not triaged; simplification items not classified |
| Realize | Configuration, development of approved extensibility, data migration mock run 1 | Mock runs not scheduled; extensibility developed outside clean core boundaries |
| Deploy | Final data migration, cutover rehearsal, go-live readiness gate | No formal go/no-go criteria; cutover rehearsal not completed |
| Run | Hypercare, operations handover, post-go-live review | No hypercare plan; operations handover not defined |

## Simplification item impact classification

| Impact class | Definition | Assessment action |
|-------------|-----------|------------------|
| `conversion-blocking` | System cannot convert until this item is resolved (e.g., deprecated business function active) | Must resolve before conversion; add to critical path |
| `functional-change` | Process or configuration change required post-conversion; data or process behavior changes | Plan and test in Realize phase; update fit-to-standard scope |
| `manual-reimplementation` | Custom code or configuration must be manually re-implemented in new SAP approach | Add to custom code remediation scope; classify via `sap-custom-code-remediation-review` |
| `informational` | No blocking or functional impact; awareness item only | Document; no action required |

## Workflow

1. **Classify transformation strategy** — confirm brownfield / greenfield / SDT / hybrid from user-provided context.
2. **Confirm deployment model** — confirm target edition (Cloud Public, Cloud Private, RISE, on-premise); flag extensibility implications.
3. **Assess SAP Activate phase alignment** — map user's program state to SAP Activate phase; identify missing deliverables per phase.
4. **Classify Readiness Check findings** — if Readiness Check output is provided, classify each finding by impact class above.
5. **Review fit-to-standard posture** — assess workshop coverage, gap resolution approach, and scope of approved extensibility vs. standard configuration.
6. **Identify architectural risks** — surface strategy mismatches, deployment model constraint violations, and timeline risks.
7. **Prioritize recommendations** — conversion-blocking items first; then functional-change and manual-reimplementation; then advisory and optimization.
8. **Return output** per the output contract below.

## Output contract

Return:

1. Transformation strategy classification and deployment model confirmation
2. Evidence label per dimension (documentation-based / user-provided evidence / inference)
3. SAP Activate phase alignment assessment with gap list
4. Simplification item classification table (if Readiness Check output provided)
5. Fit-to-standard posture assessment (workshop completeness, gap handling approach)
6. Risk level per dimension (transformation-blocking / high / medium / low)
7. Prioritized recommendations with SAP Activate deliverable reference
8. Escalation trigger if live system data, custom code analysis, or data migration scope is needed before proceeding
