# 🧑‍💼 HR Agents

HR, employment-risk, and People-function agent catalog for this marketplace.

## 📋 HR risk review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `hr-risk-triage-review-agent` | Triage terminations, discipline, accommodations, wage/hour, discrimination, harassment, retaliation, layoffs, and HR policy exceptions; surface risks and escalation paths for employment counsel | static-review | asked to issue a binding compliance opinion, draft pretextual or retaliatory documentation, or bypass counsel or a works council |

## 🛡️ Operating note

- This agent performs **static review only** — it works from sanitized excerpts and never requests employee medical records, personal data, or protected-characteristic data beyond what the question strictly requires.
- **This agent gives no legal or HR advice and forms no attorney-client relationship.** All outputs are analytical inputs for employment counsel and senior HR, not binding determinations.
- Escalation to employment counsel is the default recommendation for any jurisdiction-specific, high-impact, litigation-exposed, regulated, or financially material matter.

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --agents hr-risk-triage-review-agent --repo .
```
