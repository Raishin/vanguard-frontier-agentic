# Workflow and output contract — SAP MDG Master Data Quality Review

Use this reference for all MDG finding classification, risk level assignment, remediation path selection, and output formatting.

## MDG finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `data-model` | `non-standard-node-structure` | Custom entity type uses a node structure that deviates from the MDG standard single-object or multi-object model without a documented justification — creates maintenance and upgrade risk |
| `data-model` | `missing-key-field-constraint` | Entity type key field has no uniqueness constraint or reuse object linkage — allows duplicate key values to be created in the MDG staging area |
| `data-model` | `extensibility-approach-risk` | Data model extended via unsupported enhancement approach (direct table modification) rather than EEWB/AXT — creates upgrade incompatibility |
| `validations` | `mandatory-field-check-missing` | BRFplus validation rule does not enforce a field that the business data standard requires as mandatory — incomplete governance leaves required data absent |
| `validations` | `referential-integrity-gap` | Cross-field referential check (e.g., cost center valid for company code, profit center assigned to controlling area) is absent or inactive |
| `validations` | `duplicate-threshold-too-loose` | Matching algorithm threshold is set so low that known duplicate patterns are not flagged — duplicates enter the active data without governance review |
| `validations` | `duplicate-threshold-too-strict` | Matching algorithm threshold produces excessive false positives — consolidation workflow is bottlenecked by non-duplicate candidates requiring manual resolution |
| `derivations` | `silent-overwrite-no-notification` | Derivation rule overwrites a user-entered field without notifying the change request initiator — user cannot detect when the system replaces their input |
| `derivations` | `derivation-dependency-wrong-order` | Derivation rules execute in an order where a later rule depends on a field populated by an earlier rule, but that dependency is not enforced — derivation produces incorrect values on the first pass |
| `derivations` | `derivation-coverage-gap` | A field that should be auto-derived (e.g., tax jurisdiction from postal code, purchasing organization from plant) has no derivation rule — users must manually populate it, creating inconsistency |
| `workflow` | `step-no-owner` | Workflow step has no assigned agent (user, role, or organizational unit) — change request can stall indefinitely at this step |
| `workflow` | `missing-escalation` | Workflow template has no deadline enforcement or escalation routing for overdue steps — process bottlenecks have no governance response |
| `workflow` | `routing-logic-gap` | Workflow routing logic does not cover all valid attribute combinations for the change request type — some change requests fall to an incorrect or no approver |
| `workflow` | `bypass-path-via-direct-activation` | Change request type allows direct activation without workflow approval for certain conditions — bypasses the governance review layer |
| `consolidation` | `best-record-logic-undocumented` | Best record calculation logic is not documented — reviewers cannot determine which source system fields take precedence when multiple records conflict |
| `consolidation` | `mass-change-unrestricted-access` | MDG mass change template execution is not restricted by authorization object or role — any user with basic MDG access can run bulk master data changes |
| `consolidation` | `consolidation-error-not-monitored` | Consolidation run errors are not routed to a monitored queue or alert — failed consolidations may go undetected |
| `data-quality-kpis` | `domain-not-covered` | An in-scope MDG domain (e.g., business partner, supplier, material) has no data quality KPI defined — governance blind spot for that domain |
| `data-quality-kpis` | `dimension-gap` | KPI framework measures completeness but not conformance, uniqueness, or timeliness for one or more in-scope domains — partial quality view |
| `data-quality-kpis` | `threshold-not-set` | Data quality KPI has no threshold configured — score is calculated but no alert or workflow is triggered when quality falls below acceptable levels |
| `key-mapping` | `source-system-mapping-gap` | Source system is not mapped in the MDG key mapping object for one or more entity types — replication from that system cannot be routed correctly |
| `key-mapping` | `replication-error-not-monitored` | MDG replication errors (SOA fault messages, ALE/IDoc errors) are not monitored — replication failures silently accumulate in the error queue |

## Risk level assignment

| Risk level | Criteria |
|-----------|---------|
| `critical` | Governance bypass via direct activation without workflow; unrestricted mass change template execution enabling bulk unauthorized master data modification; undetected duplicate propagation into transactional systems |
| `high` | Derivation silent overwrite without notification; missing mandatory field validation for a regulatory or financially material field; key mapping gap causing replication failures to a critical target system; missing escalation path causing indefinite workflow stall |
| `medium` | Workflow step with no owner; duplicate threshold misconfiguration (too loose or too strict); KPI domain not covered; consolidation error not monitored; derivation dependency order issue |
| `low` | Best practice deviation in data model design; non-standard extensibility approach with low immediate risk; cosmetic KPI threshold gap in a low-risk domain |

## Remediation path decision tree

For each finding:

1. **Is this a governance bypass (direct activation without workflow, or unrestricted mass change execution)?**
   - Yes → `critical`. Immediately escalate to the data governance team. Disable direct activation for the affected change request type or restrict mass change template execution by role. Do not approve until the bypass path is closed and tested in a non-production system.
   - No → continue.

2. **Is this a derivation that silently overwrites a user-entered field without notification?**
   - Yes → `high`. Add a notification step to the derivation that informs the change request initiator when their input is overridden by the derivation. Review whether the derivation logic is correct and whether a notification is sufficient or whether the overwrite should be blocked pending user confirmation.
   - No → continue.

3. **Is this a mandatory field validation gap for a regulatory or financially material field?**
   - Yes → `high`. Add a BRFplus validation rule for the missing mandatory field. Test the rule in a development system against representative change requests before activating in production. Coordinate with the data steward team to define the enforcement scope.
   - No → continue.

4. **Is this a key mapping gap causing replication failures to a critical target system?**
   - Yes → `high`. Complete the key mapping for the affected entity type and source system. Configure replication error monitoring to alert the MDG operations team. Reconcile any master data records in the target system that were not replicated during the gap period.
   - No → continue.

5. **Is this a workflow step with no owner or no escalation path?**
   - Yes → `medium`. Assign an agent (user, role, or organizational unit) to the unowned step. Configure a deadline and escalation routing (to manager or data governance team) for overdue steps. Review the workflow agent determination BAdI if routing is logic-driven.
   - No → continue.

6. **Is this a data quality KPI gap (domain not covered, dimension missing, threshold not set)?**
   - Yes → `medium`. Extend the KPI framework to cover the missing domain or dimension. Set enforcement thresholds for each KPI. Configure a scheduled data quality run and dashboard alert for scores below threshold.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — BRFplus rule exports or summaries, workflow configuration descriptions, data model design documents, KPI dashboard exports, key mapping configuration notes, consolidation design summaries, or written descriptions of the MDG landscape.
2. **Classify each finding** by MDG governance domain and finding class.
3. **Assign risk level** per the table above (critical / high / medium / low).
4. **Flag governance bypass risks** (direct activation, unrestricted mass change) immediately — escalate before other remediation.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — governance bypass and mass change risks first; then high derivation, validation, key mapping, and escalation gap findings; then medium workflow and KPI gap findings; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. MDG governance domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Finding detail: affected MDG domain, change request type, entity type, BRFplus rule ID (if provided), workflow step, KPI name, or key mapping object
5. Recommended remediation action (BRFplus rule addition, derivation notification step, workflow agent assignment, escalation configuration, matching threshold adjustment, KPI extension, key mapping completion, mass change authorization tightening, etc.)
6. MDG governance posture after remediation
7. Escalation notice for any governance bypass risk — explicit statement that this requires data governance team sign-off before proceeding
8. Prioritized remediation sequence
9. Confirmation that no live MDG system was accessed and no master data records were created, modified, approved, or activated in this review
