# Workflow and output contract — SAP Procurement / License / FinOps / Vendor Protocol

Use this reference for trigger classification, role activation logic, finding severity, handoff sequencing, and output format.

## Trigger classification taxonomy

| Trigger class | Description | Primary role activated |
|---------------|-------------|----------------------|
| `consumption-spike` | BTP credits or entitlement quota consumed faster than forecast | sap-license-btp-consumption-finops-agent |
| `entitlement-change` | New service enablement, quota increase, or entitlement transfer requested | sap-btp-entitlement-guarded-operator-agent (via handoff only) |
| `license-compliance` | SAP named user audit, indirect access review, or measurement preparation | sap-license-btp-consumption-finops-agent |
| `rise-sla-event` | SLA breach, support escalation, renewal negotiation, or exit evaluation | sap-rise-sla-vendor-risk-agent |
| `vendor-lock-in` | Architectural dependency on single vendor without documented exit path | sap-rise-sla-vendor-risk-agent |
| `contractual-exposure` | Penalty clause, auto-renewal, minimum consumption guarantee, or true-up deadline | sap-procurement-ariba-value-leakage-agent + sap-rise-sla-vendor-risk-agent |
| `idle-subscription` | BTP subscriptions with zero consumption for 60+ days | sap-license-btp-consumption-finops-agent |
| `over-provisioning` | Entitlement quota significantly exceeds measured consumption | sap-license-btp-consumption-finops-agent + sap-btp-entitlement-guarded-operator-agent (via handoff) |

## Finding severity classification

| Severity | Criteria |
|----------|---------|
| `critical` | Immediate contractual breach, active SLA violation with penalty exposure, license non-compliance with audit in progress, or credit depletion within current billing period |
| `high` | Projected credit depletion within 30 days, entitlement quota exceeding consumption by more than 300%, RISE exit clause triggering unplanned vendor dependency, or maverick procurement bypassing approved vendor agreements |
| `medium` | Idle subscriptions consuming credit allocation, entitlement over-provisioning between 150-300% of consumption, upcoming true-up with unresolved user count discrepancy, or missing vendor lock-in mitigation plan |
| `low` | Best practice deviation: no consumption tagging by cost center, no documented emergency credit top-up procedure, missing quarterly FinOps review cadence |

## Workflow

1. **Classify trigger** — identify which trigger class(es) apply from the user-provided description and evidence.
2. **Activate relevant roles** — determine which of the four participating agent roles are relevant to this session.
3. **Inventory evidence** — list all evidence artifacts provided; confirm redaction compliance; request missing mandatory items.
4. **Produce per-role findings** — for each activated role, produce advisory findings with evidence labels and severity classification.
5. **Map decision rights** — for each finding that requires action, identify the named human approver from the decision rights table in SKILL.md.
6. **Gate irreversible actions** — for any finding that involves a listed irreversible action, confirm human approval is required before proceeding; do not pre-approve.
7. **Sequence handoff** — determine the order in which findings should be resolved (critical first, then high, then medium structural gaps, then low).
8. **Produce audit package** — assemble the audit package as defined in SKILL.md.

## Output contract

Return:

1. **Trigger classification** — which trigger class(es) activated, with evidence label.
2. **Roles activated** — which participating agent domains are in scope for this session.
3. **Evidence inventory** — types of artifacts received; redaction confirmation; any missing mandatory evidence.
4. **Per-domain findings** — severity, evidence label, and recommended advisory action for each finding.
5. **Decision rights table** — pending decisions mapped to named approver roles.
6. **Irreversible-action gate status** — list of any irreversible actions in scope; whether human approval has been confirmed (yes / not yet / not applicable).
7. **Escalation log** — escalation owners to notify and what they must action.
8. **Audit package readiness** — complete / incomplete (with blocking items listed).
9. **Next human step** — the specific action the human must take to proceed (e.g., "Present this package to the Global Account Administrator and FinOps lead for quota reduction approval before submitting to the entitlement operator gate").

## Handoff to guarded-mutating operator gate

This protocol does not invoke `sap-btp-entitlement-guarded-operator-agent` directly. When the audit package is complete and human approval has been confirmed for all irreversible actions, the protocol produces a handoff summary containing:

- Approved action type and scope (quota increase / reduction / subscription termination).
- Evidence basis for the action.
- Named approvers who confirmed the action.
- Timestamp of approval confirmation.
- Reference to the change record where approval is documented.

A human presents this handoff summary to the operator gate. The operator gate independently verifies approval before executing any mutation.
