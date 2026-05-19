# 🧑‍💼 HR Agents

HR, employment-risk, and People-function agent catalog for this marketplace.

## 📋 HR agent ecosystem

A three-layer ecosystem: a maestro that classifies and routes, specialist
reviewers, and a shared cross-functional protocol layer. All agents are
static-review — they triage, analyze, and escalate; they never give HR or
legal advice, make final HR decisions, or recommend adverse action.

| Agent | Primary use | Layer |
|---|---|---|
| `hr-maestro-agent` | Classifies an HR matter, routes it to the right specialist, coordinates cross-functional review | maestro |
| `hr-risk-triage-review-agent` | Triage terminations, discipline, accommodations, wage/hour, discrimination, harassment, retaliation, layoffs | specialist |
| `hr-employee-relations-agent` | Misconduct allegations, grievances, manager behavior, interpersonal conflict, escalation readiness | specialist |
| `hr-workplace-investigations-agent` | Investigation planning, evidence mapping, witness sequencing, neutrality and confidentiality controls | specialist |
| `hr-performance-management-agent` | Performance documentation, coaching plans, PIPs, calibration, manager bias risk, defensibility | specialist |
| `hr-termination-readiness-agent` | Documentation sufficiency, consistency, retaliation risk, final-pay and access-removal dependencies | specialist |
| `hr-leave-accommodation-agent` | Leave, disability accommodation, return-to-work, medical-information minimization, interactive process | specialist |
| `hr-recruiting-selection-agent` | Recruiting workflows, job descriptions, selection criteria, assessment fairness, adverse-impact risk | specialist |
| `hr-compensation-equity-agent` | Compensation, promotion, leveling, pay equity, incentives, calibration, adverse-impact risk | specialist |
| `hr-benefits-payroll-agent` | Benefits, payroll-process risk, deductions, classification dependencies, final-pay dependencies | specialist |
| `hr-workforce-planning-rif-agent` | Restructuring, reductions in force, redeployment, selection criteria, notice triggers, fairness | specialist |
| `hr-learning-policy-agent` | HR policy training, manager enablement, compliance training, comprehension and completion controls | specialist |
| `hr-analytics-people-data-agent` | People analytics, data minimization, access controls, algorithmic bias, employee monitoring | specialist |
| `hr-culture-dei-agent` | Inclusion, culture, engagement, anti-harassment prevention, DEI program governance, employee trust | specialist |
| `hr-hris-process-controls-agent` | HRIS workflow controls, access permissions, approval chains, audit logs, separation of duties | specialist |

## 🛡️ Operating note

- These agents perform **static review only** — they work from sanitized excerpts and never request employee medical records, personal data, or protected-characteristic data beyond what the matter strictly requires.
- **These agents give no legal or HR advice and form no attorney-client relationship.** All outputs are analytical inputs for employment counsel and senior HR, not binding determinations.
- No agent terminates, disciplines, denies leave or accommodation, or sends an employee communication — every adverse or irreversible action routes to a named human owner.
- Escalation to employment counsel is the default recommendation for any jurisdiction-specific, high-impact, litigation-exposed, regulated, or financially material matter.
- Cross-domain matters move as a `legal-hr-case-capsule`; see `skills/cross-functional/` and `docs/architecture/legal-hr-agent-routing.md`.

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --role legal-hr-risk-reviewer --repo .
```
