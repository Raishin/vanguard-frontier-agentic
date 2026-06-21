# Workflow and output contract — SAP Hypercare and Incident Commander Review

Use this reference for all hypercare governance assessment, severity triage evaluation, war-room review, root-cause process classification, and output formatting.

## Hypercare governance domain taxonomy

| Domain | Description |
|--------|-------------|
| `severity-triage` | Severity tier definitions with objective business impact criteria; triage authority assignment; triage SLA per tier |
| `war-room-governance` | War-room activation threshold; named incident commander; functional and technical stream lead assignment; communication cadence; decision-making protocol |
| `incident-problem-workflow` | Incident record creation for all disruptions; problem record creation for recurring or systemic issues; incident lifecycle enforcement; resolution verification |
| `root-cause-investigation` | Root-cause analysis initiation criteria; structured methodology; evidence collection process; corrective action tracking; finding distribution |
| `business-impact-classification` | Business impact dimensions in severity assignment (revenue, user population, regulatory deadline, process criticality); business process owner involvement |
| `escalation-paths` | Internal escalation triggers and contacts (project director, CIO, executive sponsor); SAP support escalation mapping; premium engagement escalation; coverage hours reachability |
| `exit-from-hypercare-criteria` | Measurable exit thresholds; time-bounded criteria; formal governance body for exit review; handover to steady-state operations |

## Severity triage model

| Severity tier | Business impact criteria (required for objective triage) | Triage SLA | War-room trigger |
|---------------|--------------------------------------------------------|------------|-----------------|
| `severity-1` | Complete production outage affecting all users OR financial posting capability lost OR regulatory reporting deadline at risk within 24 hours | Triage owner acknowledged within 15 minutes; incident commander engaged within 30 minutes | Immediate war-room activation |
| `severity-2` | Major business process disruption affecting a significant user population OR critical interface down affecting order-to-cash, procure-to-pay, or payroll OR SLA breach imminent for business-critical milestone | Triage within 30 minutes; functional lead engaged within 1 hour | War-room activation if unresolved within 2 hours |
| `severity-3` | Partial process disruption with workaround available OR non-critical interface degradation OR reporting anomaly without operational impact | Triage within 2 hours; assigned and in progress within 4 hours | No war-room; escalate to severity-2 if workaround fails |
| `severity-4` | Minor issue with no operational impact OR cosmetic defect OR enhancement request surfaced during hypercare | Assessed and backlogged within 1 business day | No war-room; tracked in defect backlog for post-hypercare release |

## War-room governance assessment criteria

A governed war room during SAP hypercare must include:

- Activation threshold: objective criteria that trigger war-room activation (minimum: any severity-1 incident; optionally: concurrent severity-2 incidents above a defined count)
- Named incident commander: one named individual with final authority on priority and resource allocation during active war room; one named backup
- Functional stream leads: named representatives for each business process workstream in scope (finance, logistics, procurement, HR, etc.)
- Technical stream leads: named representatives for ABAP/Basis, integration, BTP platform, and infrastructure
- Communication cadence: defined status update frequency during active incidents (minimum: every 30 minutes for severity-1; every 60 minutes for severity-2 war room)
- Decision-making protocol: documented escalation path for decisions that exceed incident commander authority (system rollback, emergency transport, business process workaround authorization)
- Bridge and communication tool: defined conference bridge, collaboration channel (Microsoft Teams, SAP Cloud ALM war-room feature, etc.), and communication log

## Incident and problem workflow assessment criteria

A governed incident workflow during hypercare must include:

- Universal incident recording: all service disruptions, regardless of severity, result in an incident record in the designated ITSM tool
- Severity assignment at creation: severity tier assigned at incident creation time using the defined triage model — not retroactively
- Problem record trigger: three or more incidents with the same symptoms or root cause within a 7-day window must trigger problem record creation and formal root-cause investigation
- Resolution criteria: incident closure requires a verified fix confirmation step — confirming that the business process or system function that failed is now operating within normal parameters
- Post-incident review: severity-1 and severity-2 incidents require a documented post-incident review with root-cause summary and corrective action list within a defined timeframe after incident closure

## Root-cause investigation process criteria

A governed root-cause investigation during hypercare must include:

- Initiation trigger: root-cause analysis initiated at incident creation for severity-1; within 4 hours for severity-2
- Methodology: structured root-cause analysis approach applied (5-Whys, fishbone diagram, or equivalent); method must be documented and consistent
- Evidence collection: systematic collection of transport history, system event logs, business process execution records, and user reports relevant to the incident timeline
- Corrective action tracking: each root cause generates a corrective action with a named owner, completion deadline, and verification method
- Finding distribution: root-cause summary shared with technical teams, business process owners, and project leadership within a defined timeframe

## Exit-from-hypercare criteria taxonomy

Each criterion must be: measurable (specific numeric threshold), time-bounded (observed over a defined window), and governed (approved by a named authority before hypercare begins).

| Criterion | Example measurable threshold | Risk if absent |
|-----------|-----------------------------|--------------------|
| `incident-frequency` | Zero severity-1 incidents in the past 7 consecutive calendar days | Hypercare exit with unresolved systemic instability |
| `severity-distribution` | No more than two severity-2 incidents per week for two consecutive weeks | Premature exit while recurring high-severity issues persist |
| `open-critical-defects` | Zero open severity-1 defects; all open severity-2 defects have accepted workarounds or fix transport scheduled | Exiting hypercare with known unresolved critical issues |
| `business-process-throughput` | Business process transaction volume at or above 90% of expected daily volume for 5 consecutive business days | System technically stable but business process adoption below target |
| `system-performance` | Key transaction response times within defined SLA thresholds for 5 consecutive business days | Performance degradation masked by low initial load |
| `support-handover-complete` | Steady-state operations team trained, runbook received, and hypercare escalation contacts transitioned to standard support model | Abrupt handover with unprepared operations team |

## Workflow

1. **Identify hypercare scope** — confirm system types in scope, go-live date or approximate elapsed hypercare period, SAP support engagement level (standard / Preferred Success / MaxAttention).
2. **Assess severity triage model** — confirm objective business impact criteria, triage authority assignment, and triage SLA definition per tier.
3. **Evaluate war-room governance** — confirm activation threshold, named incident commander, stream lead assignments, communication cadence, and decision protocol.
4. **Review incident and problem workflow** — assess universal recording, severity assignment timeliness, problem record triggers, and resolution criteria.
5. **Assess root-cause investigation process** — confirm initiation criteria, methodology, evidence collection, corrective action tracking, and finding distribution.
6. **Review business impact classification** — confirm that business impact dimensions are factored into severity triage and that business process owners participate in assessment.
7. **Evaluate escalation paths** — confirm internal escalation triggers, SAP support priority mapping, premium engagement escalation, and coverage hours reachability.
8. **Assess exit-from-hypercare criteria** — confirm measurability, time bounds, governance authority, and handover completeness criteria.
9. **Assign overall hypercare governance posture** — `hypercare-ready` (all domains assessed as governed) / `conditional` (gaps with documented mitigation and accepted risk) / `hypercare-at-risk` (one or more domains with hypercare-critical or high-risk finding).
10. **Return output** per the output contract below.

## Output contract

Return:

1. Hypercare scope (system types, go-live date or elapsed period, SAP support engagement level)
2. Evidence label per domain (documentation-based / user-provided evidence / inference)
3. Hypercare governance assessment table: domain, posture (GOVERNED/PARTIAL/UNGOVERNED/NOT-YET-ASSESSED), finding description
4. Severity triage model completeness: objective vs. subjective criteria, triage authority assignment, SLA definition
5. War-room governance completeness: activation threshold, named incident commander, stream lead coverage, communication cadence
6. Exit-from-hypercare criteria completeness: measurable thresholds present, time-bounded, governance body named
7. Overall hypercare governance posture: hypercare-ready / conditional / hypercare-at-risk
8. Risk level per domain (hypercare-critical / high / medium / low)
9. Prioritized governance gap remediation recommendations with SAP Activate hypercare methodology or Cloud ALM incident management reference
10. Escalation notice for hypercare-critical findings — explicit statement that the program leadership must be made aware and the gap must be resolved before go-live or before the next hypercare phase
11. Explicit advisory boundary reminder: this review does not create incident records, escalate to SAP, authorize system changes, or operate live incident response
