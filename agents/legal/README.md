# ⚖️ Legal Agents

Legal, compliance, and regulatory risk review agent catalog for this marketplace.

## 📋 Legal agent ecosystem

A three-layer ecosystem: a maestro that classifies and routes, specialist
reviewers, and a shared cross-functional protocol layer. All agents are
static-review — they triage, analyze, and escalate; they never give legal
advice, make final legal decisions, or form an attorney-client relationship.

| Agent | Primary use | Layer |
|---|---|---|
| `legal-maestro-agent` | Classifies a legal matter, routes it to the right specialist, coordinates multi-agent review | maestro |
| `legal-counsel-review-agent` | Adversarial risk review for contracts, privacy, regulatory, employment, litigation, and policy-exception questions | specialist |
| `legal-contract-review-agent` | Contract clauses, indemnity, liability, termination, renewal, warranties, audit rights, governing law | specialist |
| `legal-privacy-data-protection-agent` | Data protection, retention, cross-border transfer, DPIA/PIA readiness, vendor DPAs, employee-data processing | specialist |
| `legal-employment-law-risk-agent` | Employment-law risk in HR matters — exposure, documentation gaps, counsel-review triggers | specialist |
| `legal-litigation-discovery-hold-agent` | Litigation holds, discovery preservation, subpoena intake, retention, spoliation risk | specialist |
| `legal-regulatory-compliance-agent` | Regulatory obligation mapping, compliance gaps, licensing, agency guidance, enforcement risk | specialist |
| `legal-ip-open-source-agent` | Copyright, trademark, patent-risk triage, open-source license obligations, third-party IP exposure | specialist |
| `legal-vendor-procurement-risk-agent` | Vendor contracts, third-party risk, audit rights, DPAs, SLAs, subcontractor obligations | specialist |
| `legal-ethics-investigations-agent` | Whistleblower, conflict of interest, anti-bribery, sanctions, executive-misconduct intake triage | specialist |
| `legal-policy-governance-agent` | Corporate policies, approval matrices, delegated authority, records retention, board/audit triggers | specialist |
| `legal-public-disclosure-agent` | Disclosure-risk inputs, materiality escalation, securities-law sensitivity, board visibility | specialist |
| `legal-knowledge-management-agent` | Legal playbooks, clause libraries, escalation matrices, matter taxonomies, precedents | specialist |

## 🛡️ Operating note

- These agents perform **static review only** — they read sanitized excerpts, surface risks, assumptions, evidence gaps, and escalation paths. They never execute code, contact regulators, or trigger live systems.
- **These agents give no legal advice and form no attorney-client relationship.** All outputs are risk-structured analysis for review by qualified counsel, not binding legal conclusions.
- No agent approves, denies, settles, files, or makes a public disclosure — every adverse or irreversible action routes to a named human owner.
- Escalation-grade matters (retaliation, discrimination, harassment, whistleblower, sanctions, bribery, data breach, public disclosure) are flagged immediately and routed to counsel.
- Cross-domain matters move as a `legal-hr-case-capsule`; see `skills/cross-functional/` and `docs/architecture/legal-hr-agent-routing.md`.
- Never supply secrets, credentials, personal data, employee medical detail, or trade secrets to these agents.

## 📦 Install

```bash
npx vfa-export-agents --platform claude-code --role legal-hr-risk-reviewer --repo .
```
