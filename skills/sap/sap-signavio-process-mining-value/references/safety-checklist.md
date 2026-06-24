# Safety checklist — SAP Signavio Process Mining Value Review

Use before making any process mining analytical or configuration recommendation, especially for findings involving event log coverage, conformance deviation classification, bottleneck attribution, or value realization measurement.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP Signavio Process Intelligence tenant, SAP S/4HANA system, Ariba, SuccessFactors, or any source system event log. This skill reviews artifacts only.
- Do not accept or request Signavio tenant credentials, SAP logon credentials, event log database access, or raw process mining data containing personally identifiable information (PII). Event logs from HR processes (Hire-to-Retire) may contain personal employee data — these must not be shared.
- Do not modify process models in Signavio Process Manager, investigation configurations in Signavio Process Intelligence, mining run parameters, conformance rules, KPI definitions, or dashboard layouts. This skill is strictly advisory.
- Do not trigger mining runs, conformance check executions, KPI recalculations, or data extractions in any Signavio or source system. These operations affect production analytical environments and must be executed by authorized Signavio administrators.
- Do not present process mining findings as definitive proof of process compliance or non-compliance without confirming event log completeness. Findings derived from an incomplete event log are analytical observations with limited reliability, not compliance determinations.
- Do not recommend acting on conformance deviation findings before confirming that a target process model has been established and agreed by process owners. Deviations measured against an informal or unreviewed reference model may incorrectly flag intentional regional or legal variants as non-conformances.

## What people get wrong

- **Treating "process is mined" as equivalent to "event log is complete"**: Signavio can produce a process map from a partial event log. A P2P investigation that covers only the S/4HANA invoice verification steps but misses the Ariba purchase requisition and purchase order steps is mined — but the map does not represent the full process. Coverage completeness must be explicitly verified.
- **Confusing process variant frequency with process variant importance**: Low-frequency process variants are often dismissed as noise. In reality, low-frequency variants frequently contain the highest-risk behaviors: emergency bypasses, error corrections, fraud patterns, and regulatory exceptions. A sound review assesses whether low-frequency variants have been investigated, not just filtered out.
- **Treating conformance score as a compliance score**: A Signavio conformance score measures how often the actual process matches the defined reference model. It does not measure regulatory compliance, internal policy compliance, or audit readiness unless the reference model explicitly encodes those requirements. The distinction matters for stakeholder communication.
- **Accepting bottleneck identification as a complete finding**: Identifying that a specific process step has a high average cycle time is the beginning of an analysis, not the end. The actionable finding requires attribution (what causes the wait), impact quantification (what is the cost of this wait), and a specific remediation lever (system change, automation, resource increase, process redesign).
- **Conflating Signavio Process Manager and Signavio Process Intelligence**: Process Manager is a BPMN modeling and collaboration tool. Process Intelligence is a process mining and analytics tool. They share the SAP Signavio brand but are distinct products with different purposes. A review of process model governance in Process Manager is not a review of process mining analytics in Process Intelligence.
- **Assuming value realization is complete when improvements are implemented**: Implementation of a process improvement (new S/4HANA Fiori app, automation bot, streamlined approval workflow) delivers the potential for value realization. Actual value realization requires post-implementation measurement: comparing the cycle time, rework rate, or exception rate before and after the change under comparable volume conditions.

## When to push back

- Push back when an event log coverage gap invalidates downstream process mining findings — advise stakeholders that findings for affected processes cannot be treated as reliable until coverage is confirmed.
- Push back when the user asks to confirm process compliance based on mining findings alone without a defined reference process model for conformance checking.
- Push back when the request requires live Signavio tenant access, active mining output data, or real event log records — state clearly that live inspection is out of scope and ask the user to supply the relevant export or description.
- Push back when asked to modify investigation configurations, trigger mining runs, or alter conformance rules — this is an absolute boundary and must be refused in all circumstances.
- Push back when asked to assess process efficiency from industry benchmarks without confirming that the user's event log covers the same process scope as the benchmark definition — benchmark comparison is misleading when scope differs.
- Push back when improvement initiative owners ask this skill to confirm that a registered initiative has "delivered its value" — post-implementation measurement requires data from the live Signavio investigation, which must be supplied by the user.

## Evidence labels

- `documentation-based` — grounded in SAP Signavio Process Intelligence documentation, SAP Help Portal Signavio content, or SAP process mining best practice guidance
- `user-provided evidence` — event log configuration descriptions, conformance check summaries, bottleneck analysis outputs, value tracking dashboard exports, investigation configuration descriptions, or written descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
