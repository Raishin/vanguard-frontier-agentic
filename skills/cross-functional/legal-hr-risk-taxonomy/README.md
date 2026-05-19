# 🏷️ Legal-HR Risk Taxonomy

The **risk taxonomy** defines the shared vocabulary for legal and HR risk assessment — severity scales, sensitivity labels, escalation-grade matter types, escalation gates, and minimum-necessary audit-log schema. It enables agents to speak the same language and escalate with precision.

## What is the risk taxonomy?

A structured reference that specifies:
- **5-level severity scale** (Critical → High → Medium → Low → Informational) with clear business-impact definitions
- **Privilege, privacy, and retaliation sensitivity labels** — data classification to control what can be disclosed, who can view case context, and when escalation is mandatory
- **Escalation-grade matter types** — a list of matters (retaliation allegations, data breaches, whistleblower reports, executive misconduct, securities violations) that trigger automatic counsel/board escalation
- **Escalation gates** — thresholds and decision rules that mandate senior human involvement
- **Minimum-necessary audit-log schema** — structured fields every Legal and HR agent must emit to the audit trail

See [`references/risk-labels.md`](references/risk-labels.md) for the complete specification, enumeration of matter types, label definitions, and the audit-log schema.

## The severity scale

| Level | Definition | Legal example | HR example | Next action |
| ----- | ---------- | ------------- | ---------- | ----------- |
| **Critical** | Board-level incident; regulatory exposure; imminent legal jeopardy | Patent infringement by key competitor entering market; major data breach affecting thousands of customers | Executive misconduct allegation; whistleblower report of systemic discrimination; mass layoff with retaliation risk | Immediate counsel + board escalation |
| **High** | Material legal or HR risk; requires counsel or senior HR review; potential litigation or regulatory investigation | Wrongful-termination exposure; contract breach by vendor; regulatory licensing gap | Wrongful termination exposure; retaliation risk in termination; high-risk accommodation denial | Escalate to employment counsel or CHRO within 24 hours |
| **Medium** | Process improvement needed; no immediate jeopardy; senior review recommended | Contract clause ambiguity; IP assignment gap in contractor agreement | Pay-equity anomaly; performance-plan defensibility concern | Escalate to counsel/senior HR; schedule review within a week |
| **Low** | Informational; no immediate action required; routine guidance | Routine contract renewal; standard vendor audit | Routine scheduling conflict; policy-interpretation question | Route to the right specialist; no escalation required |
| **Informational** | Informational only; logged for completeness | Regulatory guidance update; industry trend | Engagement survey result; policy reminder | Log and distribute |

## Sensitivity labels

Every Legal and HR matter is labeled with one or more sensitivity markers that control disclosure:

- **attorney-client-privilege** — information protected from disclosure under legal privilege; must not be shared downstream without counsel approval
- **work-product-doctrine** — analysis, strategy, or litigation advice; protected under work-product doctrine
- **medical-information** — employee medical records, disability status, medical leave reason; must be minimized and access-controlled
- **protected-characteristic-data** — age, race, gender, religion, national origin, disability, genetic information; must be minimized and access-controlled
- **whistleblower-report** — information from a whistleblower; mandatory confidentiality and retaliation-risk escalation
- **retaliation-risk** — an action that, if taken without review, could trigger retaliation claims; automatic escalation
- **data-breach-material** — involves a data breach or loss of regulated data; automatic regulatory and counsel escalation
- **executive-misconduct** — conduct by executives or board members; automatic board and counsel escalation

## Escalation-grade matter types

The following matter types **automatically trigger escalation** (counsel, CHRO, CFO, or board as appropriate):

- Retaliation allegations or risk
- Discrimination or harassment allegations
- Whistleblower reports
- Executive or board-member misconduct
- Data breaches or regulatory notifications
- Litigation holds or subpoena response
- Securities-law materiality (IPO, acquisitions, public disclosures)
- Mass layoffs or restructurings with adverse-impact risk
- Third-party claims (breach of contract, IP infringement, product liability)
- Regulatory agency contact or investigation

## Escalation gates

Escalation gates are decision thresholds. When a matter hits a gate, human approval is mandatory:

1. **Severity ≥ High** → escalate to counsel (legal) or CHRO (HR)
2. **Any sensitivity label** → escalate to the gate owner (e.g., whistleblower → board; executive misconduct → CEO + board)
3. **Matter type is escalation-grade** → escalate automatically
4. **Time-critical or irreversible action** → escalate before execution
5. **Cross-domain matter** → escalate to maestro router; both domains approve
6. **Disagreement between specialists** → escalate to counsel + CHRO
7. **Regulatory or board trigger** → escalate to general counsel + CFO + board

## Audit-log schema

Every Legal and HR agent must emit the following minimum-necessary audit-log entry:

```json
{
  "agent_id": "legal-privacy-data-protection-agent",
  "matter_id": "MTR-2025-00041",
  "timestamp": "2025-05-19T14:23:45Z",
  "severity": "High",
  "sensitivity_labels": ["attorney-client-privilege", "data-breach-material"],
  "decision": "Flag DPIA gap; escalate to counsel before proceeding with cross-border transfer",
  "evidence_level": "Strong",
  "open_questions": ["Scope of DPIA if only metadata is transferred", "Adequacy decision status in target jurisdiction"],
  "safe_next_actions": ["Retain outside DPA counsel", "Schedule adequacy-decision research"],
  "decision_owner": "Chief Privacy Officer",
  "do_not_do_list": ["Do not proceed with cross-border transfer without adequate decision", "Do not disclose PII without DPIA completion"]
}
```

See [`references/risk-labels.md`](references/risk-labels.md) for the complete schema and worked examples.

## Cross-references

- [`SKILL.md`](SKILL.md) — the skill prompt for risk assessment and taxonomy application
- [`references/risk-labels.md`](references/risk-labels.md) — complete specification, matter-type enumeration, label definitions, audit-log examples
- [`docs/architecture/legal-hr-agent-routing.md`](/docs/architecture/legal-hr-agent-routing.md) — how escalation gates feed into routing decisions
- [`skills/cross-functional/legal-hr-case-capsule`](/skills/cross-functional/legal-hr-case-capsule/) — case capsule carries severity and sensitivity labels

---

*The risk taxonomy is part of the vanguard frontier's cross-functional protocol layer. It ensures Legal and HR agents rate risk in the same language and escalate with precision.*
