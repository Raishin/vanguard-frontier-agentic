# Official sources

Use this reference only when you need source grounding for Microsoft Purview data security and compliance — sensitivity labels, DLP, Insider Risk Management, eDiscovery, retention, Audit, and DSPM for AI — or the detailed source list.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft Purview tenant state:

- https://learn.microsoft.com/purview/dlp-learn-about-dlp
- https://learn.microsoft.com/purview/sensitivity-labels
- https://learn.microsoft.com/purview/insider-risk-management
- https://learn.microsoft.com/purview/data-security-posture-management-learn-about
- https://learn.microsoft.com/purview/ediscovery
- https://learn.microsoft.com/purview/retention
- https://learn.microsoft.com/purview/audit-solutions-overview
- https://learn.microsoft.com/purview/dlp-policy-reference
- https://learn.microsoft.com/purview/dlp-adaptive-protection-learn
- https://learn.microsoft.com/purview/insider-risk-management-policies
- https://learn.microsoft.com/purview/data-security-posture-management-oversharing

## Grounding rule

Official documentation explains Microsoft Purview service behavior. It does not prove the user's current tenant DLP policy set, sensitivity label taxonomy, retention policy coverage, eDiscovery hold state, or Insider Risk Management configuration. Prefer read-only Microsoft Purview compliance portal evidence, Graph API read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-17)

Key service facts from official Microsoft Learn documentation:

**Sensitivity labels and information protection (per learn.microsoft.com/purview/sensitivity-labels):**
- Sensitivity labels classify and protect content across Microsoft 365 services, endpoints, and third-party apps
- Labels apply encryption, content marking, and access controls; label priority order determines inheritance for the highest-priority label
- Auto-labeling policies apply labels without user interaction based on sensitive information types or trainable classifiers
- Microsoft 365 Copilot inherits and surfaces sensitivity labels; newly created content from Copilot can inherit the highest priority label from source items
- SC-401 Information Security Administrator Associate (replaced SC-400 on 2025-05-31) is the certification anchor for information protection and data security administration

**Data Loss Prevention (per learn.microsoft.com/purview/dlp-learn-about-dlp):**
- DLP uses deep content inspection and contextual analysis to identify sensitive items and enforce protection policies across Microsoft 365 services, endpoints, and cloud apps
- Endpoint DLP extends DLP to Windows devices onboarded to Microsoft Purview, monitoring and blocking sensitive data activities including upload, copy, print, and clipboard use
- Adaptive Protection integrates Insider Risk Management risk levels (Elevated, Moderate, Minor) as DLP policy conditions, enabling dynamic policy enforcement based on user risk score
- DLP policy conditions include sensitive information types, sensitivity labels, retention labels, and trainable classifiers

**Data lifecycle and retention (per learn.microsoft.com/purview/retention):**
- Retention policies and retention labels control whether content is retained, deleted, or both for a specified period
- Records management supports regulatory records with preservation locks that prevent policy modification or deletion
- Disposition reviews allow human approval before final deletion of content at the end of a retention period
- Preservation locks prevent administrators from turning off a policy, weakening restrictions, or deleting the policy

**Insider Risk Management (per learn.microsoft.com/purview/insider-risk-management):**
- Detects, investigates, and mitigates internal risks including IP theft, data leakage, and security violations using machine learning and Microsoft 365 signals
- Policy templates: data theft by departing users, data leakage, security policy violations, and more
- Privacy controls include pseudonymization, role-based access (Insider Risk Analysts vs. Investigators), and notice templates
- Cases can be escalated to Microsoft Purview eDiscovery (Premium) for formal investigation
- Adaptive Protection feeds user risk levels into DLP conditions for dynamic enforcement

**eDiscovery and legal hold (per learn.microsoft.com/purview/ediscovery):**
- eDiscovery (Premium) supports custodian management, legal hold notifications, content search, review sets, and disposition with KQL and advanced filtering
- Legal holds preserve content in place without affecting users; releasing a hold before litigation is resolved is a critical compliance risk
- Review sets support KQL querying, tagging, redaction, and export for legal review

**Audit (Premium) (per learn.microsoft.com/purview/audit-solutions-overview):**
- Audit (Premium) provides 365-day default log retention (extendable to 10 years) and intelligent insights for high-value events
- Supports forensic investigation of user and admin activities across Microsoft 365 services

**DSPM for AI (per learn.microsoft.com/purview/data-security-posture-management-learn-about):**
- Data Security Posture Management (DSPM) provides a central view of data security risks including oversharing, unlabeled sensitive data, and unprotected AI interactions
- Data risk assessments identify oversharing in SharePoint and OneDrive for Business, surfacing content that is obsolete, over-permissioned, or lacks governance controls
- DSPM for AI monitors sensitive data interactions with Microsoft 365 Copilot, Copilot Studio agents, and third-party AI apps (ChatGPT Enterprise, Google Gemini)

**Common failure modes:**
- Sensitivity label gaps leaving large volumes of sensitive content unclassified and unprotected
- DLP policies in test/audit mode that have never been promoted to enforcement mode
- Endpoint DLP not deployed to managed Windows devices, leaving sensitive data exfiltration via USB and cloud upload undetected
- Insider Risk Management not enabled or missing high-risk policy templates for departing users and data leakage
- eDiscovery holds not scoped to all relevant custodians and data sources at the time of litigation trigger
- Retention policies not covering all regulated content types (financial, legal, HR) per applicable regulations
- DSPM for AI showing high oversharing scores for SharePoint sites with sensitive content accessible to Microsoft 365 Copilot

Review implications:
- Do not approve DLP policy changes that weaken existing rules, broaden exclusions, or remove sensitive information types from scope without documented business justification and compensating controls.
- Releasing an eDiscovery legal hold before the litigation or investigation is formally closed is a critical compliance breach — escalate immediately.
- Documentation cannot prove the user's actual DLP policy enforcement state, retention policy coverage, or eDiscovery hold scope.
