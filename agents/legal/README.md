# ⚖️ Legal Agents

Legal, compliance, and regulatory risk review agent catalog for this marketplace.

## 📋 Legal review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `legal-counsel-review-agent` | Adversarial risk review for contracts, privacy, regulatory, employment, litigation, compliance, and policy-exception questions; surfaces risks and escalation paths for qualified counsel | static-review | asked to issue binding legal conclusions or form an attorney-client relationship |

## 🛡️ Operating note

- This agent performs **static review only** — it reads sanitized excerpts, surfaces risks, assumptions, evidence gaps, and escalation paths. It never executes code, contacts regulators, or triggers live systems.
- **This agent gives no legal advice and forms no attorney-client relationship.** All outputs are risk-structured analysis for review by qualified counsel, not binding legal conclusions.
- Escalation-grade matters (retaliation, discrimination, harassment, whistleblower, sanctions, bribery, data-breach, public-disclosure) are flagged immediately and routed to counsel.
- Never supply secrets, credentials, personal data, employee medical detail, or trade secrets to this agent.

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --agents legal-counsel-review-agent --repo .
```
