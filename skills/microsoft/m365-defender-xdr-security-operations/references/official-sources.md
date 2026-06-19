# Official sources

Use this reference only when you need source grounding for Microsoft Defender XDR security operations — incident response, advanced hunting, AIR, attack disruption, Defender signal sources, and Microsoft Sentinel integration — or the detailed source list.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft Defender XDR tenant incident state:

- https://learn.microsoft.com/defender-xdr/microsoft-365-defender
- https://learn.microsoft.com/defender-xdr/advanced-hunting-overview
- https://learn.microsoft.com/defender-xdr/m365d-autoir
- https://learn.microsoft.com/defender-xdr/incident-queue
- https://learn.microsoft.com/defender-xdr/automatic-attack-disruption
- https://learn.microsoft.com/security/zero-trust/siem-xdr-overview
- https://learn.microsoft.com/defender-xdr/m365d-configure-auto-investigation-response
- https://learn.microsoft.com/defender-xdr/advanced-hunting-schema-tables
- https://learn.microsoft.com/defender-xdr/custom-detection-rules
- https://learn.microsoft.com/defender-xdr/pilot-deploy-investigate-respond

## Grounding rule

Official documentation explains Microsoft Defender XDR and Sentinel service behavior. It does not prove the user's current incident queue state, AIR automation level, advanced hunting coverage, or Sentinel analytics rule deployment. Prefer read-only Defender XDR portal evidence, Graph Security API read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Microsoft Defender XDR unified platform (per learn.microsoft.com/defender-xdr/microsoft-365-defender):**
- Unified incident queue in the Microsoft Defender portal correlates alerts from Defender for Endpoint, Defender for Office 365, Defender for Identity, and Defender for Cloud Apps into single incidents
- Incidents expose the complete attack story including affected entities, alert timeline, evidence, and automated investigation results
- Automatic attack disruption uses high-fidelity signals from incident correlation to contain active attacks at machine speed — isolating compromised devices and disabling compromised accounts without waiting for analyst action
- Zero Trust assume-breach principle: treat every unconfirmed incident as active; minimize lateral movement window by acting on disruption signals quickly

**Advanced hunting with KQL (per learn.microsoft.com/defender-xdr/advanced-hunting-overview):**
- Query-based proactive threat hunting across up to 30 days of raw data using Kusto Query Language (KQL)
- Schema tables include: EmailEvents, EmailAttachmentInfo, DeviceFileEvents, DeviceProcessEvents, IdentityDirectoryEvents, IdentityQueryEvents, CloudAuditEvents, CloudProcessEvents, CloudStorageAggregatedEvents
- Guided mode enables visual query construction without KQL for analysts learning the schema
- Custom detection rules run advanced hunting queries on a schedule and generate alerts or response actions automatically
- FileProfile() function enriches file SHA256 hashes with threat intelligence including prevalence, signer, and issuer data

**Automated investigation and response — AIR (per learn.microsoft.com/defender-xdr/m365d-autoir):**
- AIR automatically investigates alerts and produces verdicts: malicious, suspicious, or no threats found
- Remediation actions from AIR include: quarantine file, stop process, block URL, isolate device
- Action Center shows all pending and completed remediation actions requiring approval or review
- Automation level for device groups controls whether AIR remediates automatically (Full) or requires analyst approval (Semi or None)
- Recommended setting: Full — remediate threats automatically for mature SOC environments with tested playbooks

**Automatic attack disruption (per learn.microsoft.com/defender-xdr/automatic-attack-disruption):**
- High-confidence containment of active attacks at machine speed: isolates compromised endpoints, disables compromised user accounts
- Triggered by incident correlation signals; marked clearly in the Defender XDR incident queue
- Does not require analyst action to trigger; analysts review and can reverse disruption actions in Action Center

**Defender signal sources:**
- Defender for Endpoint: device risk, behavioral analytics, EDR, file and process telemetry
- Defender for Office 365: phishing, malware in email, safe links, safe attachments, threat explorer
- Defender for Identity: lateral movement, credential harvesting, domain controller activity, pass-the-hash/ticket
- Defender for Cloud Apps: shadow IT, OAuth app anomalies, cloud discovery, anomalous session activity

**Microsoft Sentinel integration (per learn.microsoft.com/security/zero-trust/siem-xdr-overview):**
- Microsoft Sentinel workspaces can be onboarded to the Defender portal for unified SIEM-XDR incident management
- Sentinel analytics rules generate alerts that correlate with Defender XDR incidents
- Sentinel playbooks (Logic Apps) automate response actions and can be triggered by Defender XDR incidents
- Advanced hunting in the unified portal queries both Defender XDR and Sentinel data sources

**SC-200 certification anchor:**
- SC-200 Security Operations Analyst Associate validates threat mitigation using Microsoft Defender XDR, Microsoft Sentinel, Defender for Endpoint, Defender for Identity, Defender for Office 365, and Defender for Cloud Apps

**Common failure modes:**
- AIR automation level set to None or Semi for all device groups, requiring manual approval for every remediation action and slowing response significantly
- Advanced hunting not used proactively; SOC relies only on alert-driven investigation without hunting for precursor indicators
- Incident queue not triaged by severity; high-severity incidents assigned alongside low-severity noise without prioritization
- Custom detection rules not deployed for organization-specific threat patterns, leaving KQL hunting insights unused
- Microsoft Sentinel analytics rules not tuned, generating high false-positive volumes that suppress analyst attention to true positives
- Automatic attack disruption actions not reviewed post-containment, leaving disrupted users or devices isolated beyond the threat window

Review implications:
- Do not recommend changing AIR automation levels without assessing the current false-positive rate and incident volume.
- Containment actions initiated outside of automatic attack disruption require explicit SecOps owner approval — escalate immediately.
- Documentation cannot prove the user's actual incident queue state, AIR automation level, or Sentinel analytics rule coverage.
