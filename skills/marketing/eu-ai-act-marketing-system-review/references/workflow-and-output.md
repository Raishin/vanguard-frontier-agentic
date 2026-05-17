# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide an AI system description card as a sanitized document (no model weights, no training data, no vendor credentials). The description card should cover:
- System purpose and primary use case (what decision or output does it produce?)
- Input data types (behavioral signals, demographic data, purchase history, engagement metrics, etc.)
- Output type (score, ranking, binary decision, content recommendation, audience segment)
- Human-oversight mechanism (is there a human review gate before the output is acted upon?)
- Deployment geography (EU deployment or EU-resident data subjects?)
- Whether the system profiles natural persons (produces an evaluation of personal aspects)
- Internal risk classification, if any
- Whether the system is integrated into a downstream automated decision chain

If the user provides only a partial description card, note which elements are absent and scope findings accordingly.

### Step 2 — Determine EU AI Act applicability

Confirm whether the Act applies:
- The system is placed on the market in the EU, used in the EU, or its outputs affect EU-resident natural persons.
- The operator or deployer is subject to EU jurisdiction, or the provider targets the EU market.

If applicability is uncertain, flag as MEDIUM and recommend a legal-jurisdiction assessment.

### Step 3 — Article 5 prohibited-practice screening

Screen the system description for candidate prohibited practices before proceeding to risk-tier classification:

```text
# Article 5(1)(a) — Subliminal manipulation
System uses techniques that influence behavior below the threshold of conscious awareness,
causing decisions persons would not have made otherwise — e.g., urgency signals calibrated
to anxiety response patterns without the user's knowledge.

# Article 5(1)(b) — Exploitation of vulnerabilities
System exploits specific vulnerabilities of a group (age, disability, social/economic situation)
to distort behavior in a way that causes harm — e.g., targeting financially distressed segments
with high-interest offers optimized on engagement signals from that population.

# Article 5(1)(e)/(f) — Social scoring / emotion recognition in workplace or public space
```

If any candidate applies, flag as HIGH and route the prohibited-practice determination to qualified legal counsel. Do not determine prohibition yourself.

### Step 4 — Annex III high-risk classification

Map system characteristics to Annex III categories relevant to marketing AI:

```text
Annex III(1) — Biometric categorisation that infers sensitive characteristics
Annex III(3) — AI in education or vocational training affecting access
Annex III(4) — Employment, workers management, access to self-employment
Annex III(5) — Access to and enjoyment of essential private services and public services
              → creditworthiness scoring, insurance risk, financial product access
Annex III(6) — Law enforcement (typically out of scope for marketing)
Annex III(8) — Administration of justice / democratic processes
```

A marketing AI system that profiles natural persons to determine or influence their access to credit, insurance, employment, or essential services maps to Annex III(5) or Annex III(4). Classify as HIGH-RISK.

```text
# HIGH — lead-quality scorer using behavioral + demographic signals, output routes to credit team
System purpose: score leads for mortgage pre-qualification routing
Input: browsing behavior, inferred income tier, device type, engagement rate
Output: lead-quality score → routed to underwriting queue or rejected
Classification: Annex III(5) — access to essential private services (credit/mortgage)
Obligation: Technical documentation (Art. 11), conformity assessment (Art. 43),
            EU AI database registration (Art. 71), transparency to affected persons (Art. 13)
```

### Step 5 — Limited-risk and transparency-only assessment

For systems that do not meet Annex III criteria, assess whether limited-risk transparency obligations apply:

- Article 52(1): Systems interacting with natural persons must disclose they are AI (chatbots, virtual advisors).
- Article 52(3): Deep fake / synthetic content must be disclosed as artificially generated.
- Article 52(4): Emotion recognition or biometric categorisation systems must notify the persons exposed.

```text
# MEDIUM — AI chatbot on marketing site with no AI-disclosure notice
Obligation: Article 52(1) transparency notice required before interaction begins.
```

### Step 6 — Human oversight and Article 14 assessment

Assess whether the system's declared human-oversight mechanism satisfies Article 14 for high-risk systems:

```text
# HIGH — "human in the loop" flag declared but system routes decisions to automated downstream agents
The human review gate must be meaningful: the human must be able to understand the output,
detect failures, and override or halt the system. Rubber-stamp review with no override capability
does not satisfy Article 14.
```

### Step 7 — Documentation gap inventory

For any non-minimal-risk system, enumerate required documentation and flag gaps:

| Obligation | Article | Status |
|---|---|---|
| Technical documentation | Art. 11 | Present / Absent / Partial |
| Conformity assessment | Art. 43 | Present / Absent / Planned |
| EU AI database registration | Art. 71 | Present / Absent / Not started |
| Transparency notice (users) | Art. 13/52 | Present / Absent |
| Fundamental rights impact assessment | Art. 27 | Present / Absent |
| Responsible person designation | Art. 26 | Present / Absent |

### Step 8 — August 2026 enforcement readiness check

Regulation 2024/1689 entered into force August 1, 2024. Key milestones:
- February 2, 2025: Prohibited practices (Article 5) enforceable.
- August 2, 2025: GPAI and governance provisions enforceable.
- August 2, 2026: All provisions including high-risk obligations enforceable.

Flag any high-risk system with no documented conformity-assessment timeline, no responsible person, or no EU AI database registration as MEDIUM (if enforcement date is future) or HIGH (if enforcement date has passed at time of review).

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<description card provided | documentation-based | inference>

## Risk-tier classification
<Prohibited (Art. 5) candidate | High-risk Annex III | Limited-risk (Art. 52) | Minimal-risk>
<rationale: which Annex III category or Article 5 provision applies and why>

## Documentation gap inventory
<table: obligation | article | status>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## August 2026 enforcement readiness
<summary of gaps and timeline pressure>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security and scope notes

- This is a static review. Never request model weights, training datasets, internal performance logs, or vendor system-access credentials. Work from sanitized description cards only.
- The prohibited-practice determination under Article 5 is a legal conclusion — flag the candidate risk and route to qualified legal counsel rather than deciding it.
- EU AI Act obligations are in addition to, not instead of, GDPR obligations. A system that triggers Annex III also implicates GDPR Article 22, Article 35 DPIA obligations, and special-category data restrictions.
- August 2026 is a hard enforcement deadline; systems requiring conformity assessments need lead time. Flag timeline pressure explicitly.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
