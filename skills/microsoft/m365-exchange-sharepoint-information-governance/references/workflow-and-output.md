# Workflow and output contract

Use this reference only when performing the full Exchange Online and SharePoint information governance review or formatting the final review.

## Review domains

Check these areas before giving a verdict:

- **Mailbox lifecycle**: Archive mailbox enablement, inactive mailbox policies, shared mailbox governance (no interactive sign-in, licensed, reviewed), resource mailbox hygiene, and mailbox size and quota management
- **Site lifecycle**: SharePoint site ownership policies, inactive site detection and remediation (simulation vs. active policy mode), site attestation, Microsoft 365 Archive for stale content, and orphaned site cleanup
- **External and anonymous sharing controls**: Tenant-level SharePoint and OneDrive sharing settings (Anyone/New and existing guests/Existing guests/Only org), site-level overrides, Anyone link expiration, link permission defaults, and EEEU access scope
- **SharePoint Advanced Management (SAM)**: Data access governance (DAG) reports, Restricted Content Discovery (RCD) for high-risk sites, Restricted Access Control (RAC) for membership-gated sites, site access reviews, block download policies, and content management assessment
- **Oversharing remediation and Copilot readiness**: EEEU insights, sharing link activity reports, permission state reports, sensitivity label distribution, prioritized high-risk site list for RCD or RAC, and pre-Copilot deployment checklist
- **Retention and records management**: Microsoft Purview retention policies covering Exchange Online and SharePoint Online, retention labels for records declaration, event-based retention, adaptive scopes, and retention policy gap identification
- **Hold and eDiscovery readiness**: Litigation hold coverage, eDiscovery hold assignment, Recoverable Items folder health, and inactive mailbox policy for departed employees
- **Information architecture**: Hub site structure, site collection boundaries, sensitivity label application to SharePoint sites, content type governance, and information hierarchy alignment to data classification

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier (E3, E5, Copilot, SAM license):
   - Approximate site count and mailbox count:
   - Copilot deployment status or target timeline:
   - Regulatory or legal hold requirements:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only SharePoint admin center or Exchange admin center evidence, or Microsoft Graph read output, for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, DAG report exports, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What sharing configuration allows unauthenticated (Anyone link) or org-wide (EEEU) access to sensitive site content?
   - What high-risk site is not protected by RCD or RAC and will surface unintended content in Copilot responses?
   - What inactive or orphaned site holds sensitive data with no active owner and no lifecycle policy?
   - What mailbox or site has no applicable retention policy — creating eDiscovery or compliance gaps?
   - What litigation hold or eDiscovery hold may be missing for content under legal obligation?
   - What rollback path exists if a tenant-wide sharing policy restriction breaks existing partner collaboration or anonymous link sharing workflows?
4. **Recommend the smallest safe action**
   - Prefer simulation mode for site lifecycle policies before enabling active mode, staged RCD rollout starting with highest-risk sites, and retention policy report mode before enforcement.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Exchange and SharePoint Information Governance Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Control area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or reports to run:
- Expected result:
## Residual risk
- <risk or explicit none>
```
