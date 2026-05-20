---
name: salesforce-risk-taxonomy
description: Use this skill when a Salesforce matter must be assigned a standardized matter type, risk tier, or escalation gate before routing or handoff. Defines all matter types (org-config, automation, code, integration, security/IAM, data, sales/CPQ, service/SLA, experience-cloud, marketing/consent, agentforce-AI, analytics-metrics, slack-governance, industry-vertical, architecture, compliance, release-deploy, live-mutation), risk tiers (Critical/High/Medium/Low/Unknown), and escalation gates (production data exposure, guest-user expansion, autonomous AI action, finance/revenue logic, regulated-vertical, mass change, irreversible deploy). Trigger phrases: "classify this Salesforce risk", "what tier is this change", "does this trigger an escalation gate", "assign a risk rating to this Salesforce matter". Do not use when you need to route a matter to an agent (use salesforce-routing-protocol), when you need a structured handoff (use salesforce-case-capsule), or when you need to approve a live change (use salesforce-live-change-approval-protocol). This skill classifies risk; it does not assess compliance or give Salesforce advice.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-20"
  category: compliance
  lifecycle: experimental
---

# Salesforce Risk Taxonomy

## Purpose
This skill provides the shared vocabulary for classifying Salesforce matters by
type, risk tier, and escalation gate. It exists so that all Salesforce
specialist agents use consistent terminology when they classify, route, hand
off, and escalate matters. It does not assess compliance, give Salesforce
advice, or authorize any action.

## When to use
- A matter must be assigned a standardized type before routing or capsule creation.
- A risk tier must be determined for a proposed change, finding, or event.
- An escalation gate must be checked before a handoff or approval proceeds.
- Agents need shared vocabulary to avoid classification drift across sessions.

## When not to use
- You need to route the matter to an agent — use `salesforce-routing-protocol`.
- You need a structured handoff record — use `salesforce-case-capsule`.
- You need to approve or refuse a live org mutation — use `salesforce-live-change-approval-protocol`.
- You need to respond to a live data exposure event — use `salesforce-data-exposure-escalation-protocol`.

## Minimum payload (required inputs)
- Description of the Salesforce matter (sanitized, no credentials or PII).
- Available context: environment type, change scope, data sensitivity.

## Matter types

| Matter type | Description |
|---|---|
| `org-config` | Org settings, custom settings, custom metadata, sandboxes, licenses |
| `automation` | Flow, approval processes, validation rules, Process Builder <!-- verify-before-merge:2026-05-20 -->, assignment rules |
| `code` | Apex, LWC, triggers, async jobs, static resources, packages |
| `integration` | REST/SOAP/Bulk/Streaming APIs, Platform Events <!-- verify-before-merge:2026-05-20 -->, CDC, MuleSoft <!-- verify-before-merge:2026-05-20 -->, middleware |
| `security-iam` | Permission sets, profiles, sharing rules, OWD, role hierarchy, guest users, OAuth, Shield <!-- verify-before-merge:2026-05-20 --> |
| `data` | Data quality, data migration, data archival, retention policies, data classification |
| `sales-cpq` | CPQ <!-- verify-before-merge:2026-05-20 --> pricing, quoting, product catalog, Revenue Cloud <!-- verify-before-merge:2026-05-20 -->, order management |
| `service-sla` | Service Cloud <!-- verify-before-merge:2026-05-20 --> entitlements, SLAs, case management, omni-channel |
| `experience-cloud` | Experience Cloud <!-- verify-before-merge:2026-05-20 --> sites, guest-user access, sharing sets, digital experiences |
| `marketing-consent` | Marketing Cloud <!-- verify-before-merge:2026-05-20 -->, Account Engagement <!-- verify-before-merge:2026-05-20 -->, consent capture, preference centers, Data Cloud <!-- verify-before-merge:2026-05-20 --> |
| `agentforce-ai` | Agentforce <!-- verify-before-merge:2026-05-20 --> configuration, Einstein <!-- verify-before-merge:2026-05-20 --> features, AI grounding, autonomous actions |
| `analytics-metrics` | Reports, dashboards, CRM Analytics <!-- verify-before-merge:2026-05-20 -->, Tableau <!-- verify-before-merge:2026-05-20 -->, metric definitions |
| `slack-governance` | Slack <!-- verify-before-merge:2026-05-20 --> integration, workspace governance, Slack-Salesforce data flows |
| `industry-vertical` | Health Cloud <!-- verify-before-merge:2026-05-20 -->, Financial Services Cloud <!-- verify-before-merge:2026-05-20 -->, Government Cloud <!-- verify-before-merge:2026-05-20 -->, regulated-vertical configuration |
| `architecture` | Org strategy, multi-org topology, data model design, platform limits |
| `compliance` | Audit requirements, BAA/DPA obligations, regulatory mapping, evidence collection |
| `release-deploy` | Change sets, DX packages, CI/CD pipelines, destructive changes, sandbox refresh |
| `live-mutation` | Any proposed change to a production org; always triggers live-change-approval-protocol |

## Risk tiers

| Tier | Definition | Default action |
|---|---|---|
| **Critical** | Production data at imminent risk; irreversible action without approval; regulatory breach likely; autonomous AI action beyond boundary | Immediate stop; escalate; do not proceed without human authorization |
| **High** | Significant production impact; non-trivial rollback; affects regulated data, revenue logic, or broad permissions | Pause; require documented approval; dual-agent review recommended |
| **Medium** | Controlled-environment risk; reversible; affects limited scope; no regulated data | Proceed with review; log rationale; single specialist sufficient |
| **Low** | Sandboxed, non-production, or purely advisory; no data exposure; fully reversible | Proceed with standard review; document findings |
| **Unknown** | Jurisdiction, scope, data sensitivity, or counterparty identity is missing or ambiguous | Treat as High; gather evidence before reclassifying |

**Rule:** Unknown must never be reclassified to Low without documented evidence. When in doubt, escalate.

## Escalation gates

An escalation gate, when fired, requires the matter to pause and be reviewed by a
named human owner before any agent takes further action. Gates are not advisory —
they are hard stops.

| Gate | Fires when |
|---|---|
| `production-data-exposure` | Any configuration, automation, or code change that could expose production records to unauthorized parties; guest-user OWD widening; sharing rule expansion on PII objects |
| `guest-user-expansion` | Experience Cloud <!-- verify-before-merge:2026-05-20 --> guest-user profile changes; sharing set modifications; public-site access widening |
| `autonomous-ai-action` | Agentforce <!-- verify-before-merge:2026-05-20 --> agent action allowlist expansion; unsupervised AI write-back to production records; AI model scope exceeding review boundary |
| `finance-revenue-logic` | Changes to CPQ <!-- verify-before-merge:2026-05-20 --> pricing rules, revenue recognition logic, billing integration, order management, or financial reporting fields |
| `regulated-vertical` | Changes to Health Cloud <!-- verify-before-merge:2026-05-20 -->, Financial Services Cloud <!-- verify-before-merge:2026-05-20 -->, or Government Cloud <!-- verify-before-merge:2026-05-20 --> orgs; HIPAA, PCI, FINRA, or FedRAMP scope |
| `mass-change` | Bulk permission assignment; data loader operations on > configurable threshold records; mass automation activation; org-wide setting change affecting all users |
| `irreversible-deploy` | Deployment of destructive changes (destructiveChanges.xml); field or object deletion; package uninstall; irreversible metadata removal |

## Workflow
1. Read the sanitized matter description.
2. Assign one or more `matter_type` values from the table above.
3. Assign `risk_tier` using the tier definitions; default Unknown if evidence is missing.
4. Check each escalation gate: does the matter description meet any gate condition?
5. List all fired gates.
6. Output classification: matter_type(s), risk_tier, escalation_gates_fired, rationale.

## Evidence requirements
- Sanitized matter description with scope, environment, and data sensitivity stated.
- Sufficient context to determine whether regulated data is in scope.
- If environment type is unknown, treat as production.

## Output format
```
matter_type: [one or more from the table]
risk_tier: Critical | High | Medium | Low | Unknown
escalation_gates_fired: [list, or "none"]
rationale: [one paragraph, evidence-based]
missing_evidence: [what would change the classification]
```

## Redaction rules
- Never request secrets, credentials, OAuth tokens, refresh tokens, session IDs, MFA seeds, customer PII.
- Sanitize org IDs, user IDs (replace with placeholders) before sharing in outputs.

## Privilege / data handling rules
- Classification uses sanitized descriptions only.
- If the description contains what appears to be production data, credentials, or PII,
  decline and ask for a redacted version.
- Regulated-vertical matter types always require privacy_sensitivity = Regulated-Data.

## Handoff rules
- Classification output is consumed by salesforce-case-capsule (capsule population)
  and salesforce-routing-protocol (matter routing).
- Required handoff fields: matter_type, risk_tier, escalation_gates_fired, rationale.

## Audit log fields
- matter_id, skill_id, skill_version, invoked_by, input_hash, evidence_quality, output_verdict, escalation_fired, timestamp

## Stop conditions
- Description contains live credentials, session tokens, or unredacted PII — stop and refuse.
- Matter type cannot be determined from available context — output Unknown tier and request clarification.
- Regulated-vertical scope is asserted but jurisdiction is unknown — escalate before classifying.

## Security notes
- This skill produces classification labels only; it does not authorize any change.
- Risk tier Unknown is a safe default; never suppress it to appear more conclusive.
- Escalation gates are hard stops, not suggestions. A gate that fires must pause the matter.
