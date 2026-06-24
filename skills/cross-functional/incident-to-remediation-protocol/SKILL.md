---
name: incident-to-remediation-protocol
description: Use this skill when a security incident must be triaged, contained, remediated, and reviewed in a Zero Trust assume-breach posture across Microsoft 365 and Dynamics 365 environments. Defines the end-to-end flow from detection through severity triage, containment approval, investigation, remediation, and post-incident review. Applies Zero Trust principles — verify explicitly, use least privilege, assume breach — throughout. Does not serve as an authorization to isolate devices, block users, or make configuration changes; all containment and remediation actions require human approval from the security owner or incident commander. Does not replace a qualified SecOps team or Microsoft Defender XDR specialist.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-16"
  category: security
  lifecycle: experimental
---

# Incident-to-Remediation Protocol

## Purpose
This skill defines how a security incident detected across Microsoft 365 or Dynamics 365 is triaged, classified by severity, contained, investigated, remediated, and reviewed in a Zero Trust posture. It applies the assume-breach principle throughout: the scope of compromise is always assumed to be larger than initial signals suggest until evidence proves otherwise. It does not authorize containment or remediation actions; those require the incident commander or security owner. It does not replace a qualified SecOps team or Microsoft Defender XDR escalation.

## When to use
- A security incident has been detected in the Microsoft Defender portal (Microsoft Defender XDR) or reported through another channel.
- An identity compromise, device compromise, data exfiltration signal, or malicious configuration change is suspected in Microsoft 365 or Dynamics 365.
- A SecOps team needs a structured workflow for triage, containment approval, investigation, and post-incident review.
- A cross-workload incident spans Microsoft Defender for Identity, Microsoft Defender for Endpoint, Microsoft Defender for Office 365, and Dataverse environments simultaneously.

## When NOT to use
- The incident has already been triaged and is in active SecOps investigation — hand off to the active investigation owner.
- The matter is a vulnerability disclosure rather than an active incident — use the security review path, not this protocol.
- The incident involves a data breach with regulatory notification obligations — escalate to the privacy owner and legal counsel immediately; this protocol does not cover breach notification timelines.
- The matter is outside Microsoft 365 or Dynamics 365 scope — route to the appropriate security team.

## Participating agents
- `m365-identity-zero-trust-agent` — primary: assesses identity signals, Conditional Access coverage, Entra ID risk events, and Zero Trust posture gaps
- `copilot-governance-maestro-agent` — secondary: assesses Copilot and AI surface area exposure; coordinates cross-workload governance review
- `microsoft-maestro-agent` — orchestrates cross-workload incident scope; escalation path to Microsoft Defender XDR SecOps specialists (planned: defender-xdr-secops-agent)

## Inputs required
- Incident ID or alert reference from Microsoft Defender portal
- Affected entities (users, devices, mailboxes, apps, Dataverse environments)
- Initial detection source (Microsoft Defender for Identity, Defender for Endpoint, Defender for Office 365, Defender for Cloud Apps, manual report)
- Reported or suspected incident type (phishing, credential compromise, ransomware, insider threat, misconfiguration exploitation, data exfiltration)
- Current containment status (not started, in progress, complete)

## Evidence required
- Incident details from Microsoft Defender XDR portal (alerts, affected entities, attack story)
- Microsoft Entra ID sign-in logs and risk events for affected identities
- Audit logs from Microsoft Purview for affected workloads
- Threat analytics data (if available) from Microsoft Defender portal
- Prior incident or related-alert history

## Workflow

1. **Detect and ingest** — receive incident signal; record incident ID, detection source, affected entities, and initial classification.
2. **Severity triage** — classify incident severity using Microsoft Defender XDR severity (Informational, Low, Medium, High, Critical) and assess blast radius (number of affected entities, data sensitivity, production impact).
3. **Assume breach** — apply Zero Trust assume-breach posture: treat the scope of compromise as larger than confirmed until evidence narrows it; do not wait for full confirmation before containment planning.
4. **Containment planning** — identify containment actions appropriate to severity and affected entities (disable compromised users, isolate affected devices, block hostile IPs or domains, revoke sessions); do not execute containment without approval.
5. **Escalation gate: severity triage** — if severity is High or Critical, escalate immediately to incident commander and microsoft-maestro-agent for Defender XDR SecOps coordination; do not proceed to containment without incident commander acknowledgment.
6. **Escalation gate: containment approval** — require explicit human approval from the incident commander or security owner before executing any containment action; record approval reference.
7. **Execute approved containment** — execute containment actions per the approval; record each action with timestamp, actor, and affected entity.
8. **Investigation** — analyze the attack story, affected entities, and alert timeline; identify root cause, initial access vector, lateral movement paths, and data accessed or exfiltrated.
9. **Remediation planning** — identify remediation actions (credential reset, device re-imaging, configuration correction, policy strengthening); confirm remediation actions are least-privilege and reversible where possible.
10. **Remediation execution** — execute approved remediation actions; verify each remediation step with evidence of completion.
11. **Post-incident review** — conduct a structured post-incident review: what was the attack type and impact, what detection and response gaps existed, what configuration or policy changes are needed, and what playbook updates are required.
12. **Resolve incident** — mark incident resolved in Microsoft Defender portal once all containment and remediation steps are confirmed; document resolution in audit log.

## Decision gates

| Gate | Condition | Action |
|---|---|---|
| Severity triage | Severity is High or Critical | Escalate immediately to incident commander + microsoft-maestro-agent; do not proceed without acknowledgment |
| Containment approval | Any containment action is planned | Require explicit human approval before execution; record approval reference |
| Breach scope uncertainty | Scope of compromise is not confirmed | Maintain assume-breach posture; treat scope as larger until evidence narrows it |
| Post-incident review | Incident is resolved | Mandatory post-incident review before closing; update playbooks and policies as needed |

## Refusal triggers
- A request is made to execute containment (user disable, device isolation, IP block) without recorded human approval — refuse.
- A request is made to close or resolve an incident without a completed post-incident review — refuse.
- Credentials, session tokens, OAuth tokens, or personal data are requested to perform investigation — refuse; work from sanitized incident signals and audit log references only.
- A data breach with regulatory notification obligations is suspected — stop and escalate to privacy owner and legal counsel; this protocol does not cover breach notification.

## Handoff rules
- Every handoff carries: incident ID, severity, affected entities, containment status, investigation summary, remediation status, escalations fired, open questions, and a do-not-do list.
- No agent executes a containment or remediation action without recorded human approval.
- Post-incident review findings are handed off to the security owner for policy and playbook updates.

## KPIs
- Mean time to detect (MTTD) and mean time to contain (MTTC) per severity tier
- Percentage of incidents with completed post-incident reviews
- Number of playbook or policy updates generated from post-incident reviews
- Percentage of containment actions executed within approved scope (no overreach)

## References
- [Plan an incident response workflow in the Microsoft Defender portal](https://learn.microsoft.com/unified-secops/plan-incident-response)
- [Investigate and respond using Microsoft Defender XDR](https://learn.microsoft.com/defender-xdr/pilot-deploy-investigate-respond)
- [Incident response with XDR and integrated SIEM (Zero Trust)](https://learn.microsoft.com/security/zero-trust/siem-xdr-overview)
- [Incident response playbooks](https://learn.microsoft.com/en-us/security/operations/incident-response-playbooks)
