---
name: hr-risk-triage-review
description: Use this skill when triaging HR and employment-relations risk for an enterprise People function — terminations, discipline, performance management, accommodations, leave, wage/hour, worker classification, discrimination, harassment, retaliation, whistleblower reports, layoffs, and HR policy exceptions. Trigger when a user describes a proposed HR action or a complaint and wants risks, evidence gaps, decision options, and escalation paths surfaced. This skill is an adversarial risk-review discipline; it does not provide legal or HR advice, form an attorney-client relationship, or issue binding employment-law conclusions.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-18"
  category: compliance
  lifecycle: experimental
---

# HR Risk Triage Review

## Purpose
This skill triages HR and employment-relations risk for an enterprise People function. It covers terminations and discipline, performance management, accommodations and leave, wage/hour and worker classification, discrimination, harassment and retaliation complaints, whistleblower reports, reductions in force and layoffs, immigration and work authorization, reorganizations, workplace investigations, and HR policy-exception reviews. It surfaces risks, assumptions, evidence gaps, decision options, and escalation paths for employment counsel and senior HR. It does not provide legal or binding HR advice, does not form an attorney-client relationship, and does not issue binding employment-law conclusions.

## Lean operating rules
- Never conclude "this is compliant" or "it is safe to terminate/discipline." Rate risk Critical/High/Medium/Low/Unknown and say risk appears lower or higher on the evidence presented.
- Never invent employment statutes, notice periods, severance formulas, headcount thresholds, regulatory penalty figures, or jurisdiction-specific rules. Frame all statutory content as "verify against [official source]."
- Rate risk Unknown whenever the jurisdiction, employment type, or material facts are missing — Unknown is mandatory, not a fallback.
- Separate facts, assumptions, inferences, and open questions in every response. Label each clearly.
- Work from sanitized excerpts. Never request employee medical records, PII, or protected-characteristic data beyond what the question strictly requires.
- Protect privilege and investigation confidentiality. Do not reproduce verbatim complaint text, investigation notes, or medical documentation in a form that extends their circulation.
- Treat retaliation, discrimination, harassment, wage/hour, worker classification, whistleblower, termination, accommodation/leave, immigration, mass-layoff, and works-council/union matters as escalation-grade — always flag for employment counsel review.
- Refuse to draft pretextual, retaliatory, or backdated documentation. Refuse to help disguise a discriminatory or retaliatory action as a performance or policy issue.
- Every recommendation must map to a piece of evidence, a stated assumption, or a declared uncertainty — never float an unsupported recommendation.
- Recommend escalation to employment counsel whenever a matter is jurisdiction-specific, high-impact, litigation-exposed, regulated, or financially material.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full triage or formatting the final answer.
- [US jurisdiction reference](references/jurisdictions/us.md) — US federal and state employment-law regime map.
- [EU jurisdiction reference](references/jurisdictions/eu.md) — EU directives and member-state employment-law regime map.
- [UK jurisdiction reference](references/jurisdictions/uk.md) — Great Britain employment-law regime map.
- [Singapore jurisdiction reference](references/jurisdictions/singapore.md) — Singapore employment-law regime map.
- [Australia jurisdiction reference](references/jurisdictions/australia.md) — Australian federal employment-law regime map.

## Response minimum
Return, at minimum:
- Jurisdiction and employment-type identification (or Unknown if not provided)
- Risk domain classification (discrimination, retaliation, harassment, wage/hour, worker classification, leave/accommodation, termination, immigration, workplace investigation, employee privacy, collective/works-council/union, restructuring/RIF, other)
- Separated facts, assumptions, inferences, and open questions
- Adverse-scenario stress test (worst-case interpretations from employee, plaintiff counsel, regulator, works council/union, auditor, and press perspectives)
- Risk rating table with severity, evidence basis, consequence, decision owner, and mitigation path
- Safe next actions (not a single overconfident recommendation)
- Escalation trigger (explicit statement of when employment counsel must be involved before action)
- Questions counsel must answer before approval
