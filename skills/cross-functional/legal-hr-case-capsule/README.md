# 📦 Legal-HR Case Capsule

The **case capsule** is the single, auditable handoff contract for Legal and HR agentic coordination. When a matter crosses a boundary — an HR investigation that needs privileged legal review, or a legal litigation hold that triggers an HR data freeze — the case capsule carries a structured, redacted payload between agents while preserving privilege, minimizing personal data, and recording the escalation path.

## What is a case capsule?

A 30-field structured handoff contract that:
- **Carries facts and risk posture**, never an authorization or final decision
- **Minimizes personal and sensitive data** through explicit redaction rules
- **Preserves legal and HR privilege** with mandatory do-not-disclose markers
- **Names one decision owner** — the human responsible for the next action
- **Enforces a do-not-do list** — irreversible actions that are explicitly forbidden until human sign-off

See [`SKILL.md`](SKILL.md) for the skill itself, and [`references/capsule-schema.md`](references/capsule-schema.md) for the complete 30-field specification, redaction rules, and a worked example.

## Why a case capsule?

Without a case capsule, agents would improvise handoffs — dumping raw case context, leaking personal data, losing track of who said what, and creating audit gaps. The case capsule is the guardrail: it enforces a shape, a payload contract, and a privilege boundary.

Every handoff is auditable: an auditor, a regulator, or opposing counsel can trace exactly what facts moved between agents, why, and who approved the motion.

## For Legal agents

When your review reveals an HR issue (wrongful termination exposure, discrimination risk, retaliation red flag), you hand off to HR via a case capsule. The capsule says *"here's what I found, here are the privilege markers, here's who owns the next step, here's what we must NOT do without consent."*

## For HR agents

When your review reveals a legal issue (whistleblower report, conflict of interest, anti-bribery exposure), you hand off to Legal via a case capsule. The capsule carries the HR context without leaking the employee's medical history, protected-characteristic data, or confidential HR files.

## Vault principles

- **Redaction is mandatory.** Every field has a redaction rule; no field carries unredacted personal data unless explicitly required.
- **Privilege is explicit.** Mandatory `attorney_client_privilege` and `work_product_doctrine` labels control what can be disclosed downstream.
- **One human owner.** Every handoff names a single `decision_owner` (a real person's title or role) who is accountable for the next irreversible action.
- **Refusal by default.** The case capsule carries a `do_not_do_list` — explicit, non-empty actions that are forbidden until human approval.

## Cross-references

- [`SKILL.md`](SKILL.md) — the skill prompt and scaffold for case-capsule composition
- [`references/capsule-schema.md`](references/capsule-schema.md) — 30-field specification, redaction rules, worked example, and validation checklist
- [`docs/architecture/legal-hr-agent-communication.md`](/docs/architecture/legal-hr-agent-communication.md) — cross-functional coordination principles, case capsule as the only channel, audit trail

---

*The case capsule is part of the vanguard frontier's cross-functional protocol layer. It enables agentic coordination that survives audit and legal scrutiny.*
