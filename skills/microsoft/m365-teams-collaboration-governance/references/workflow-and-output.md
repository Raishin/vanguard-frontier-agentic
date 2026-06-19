# Workflow and output contract

Use this reference only when performing the full Teams collaboration governance review or formatting the final review.

## Review domains

Check these areas before giving a verdict:

- **Teams and group lifecycle**: Team creation controls, naming policies, expiration policies for Microsoft 365 groups, archival, deletion, restore, and ownerless team remediation
- **Sprawl control**: Total team and group count, inactive team identification, group creation restriction, and ownership gap analysis
- **External access and guest sharing**: Tenant-wide external access settings, per-team guest access, B2B collaboration configuration, cross-tenant access policies, and external user lifecycle (governed vs. ungoverned)
- **Sensitivity labels on Teams and groups**: Label enablement for containers, privacy and external user access enforcement, external sharing from labeled sites, Conditional Access for labeled containers, and label deployment coverage
- **Meeting policies**: Meeting recording, lobby controls, who can present, watermarking, end-to-end encryption, meeting templates, three-tier protection model (baseline/sensitive/highly sensitive)
- **Messaging policies**: Chat settings, external chat access, read receipts, and content moderation
- **App permission policies**: Org-wide app settings, per-group or per-user app permission policies, third-party app trust boundaries, and custom app governance
- **Phone and voice governance**: Calling policies, emergency calling configuration, call park, call queues, auto attendants, and voice routing review
- **Information barriers**: Policy existence and coverage for regulated segments requiring communication restriction

## Safe workflow

1. **Frame scope**
   - Tenant / environment / licensing tier (Teams Essentials, E3, E5):
   - Team count and guest user count (approximate):
   - Regulatory requirements (information barriers, communication compliance):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer read-only Teams admin center evidence or Microsoft Graph read output for current-state claims when available.
   - Otherwise inspect repository IaC/config, sanitized user evidence, or official docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What path allows a guest or external user to access sensitive team content without review or expiration?
   - What sprawl or ownerless team contains sensitive data with no active governance?
   - What sensitivity label gap leaves a sensitive team without privacy or external sharing enforcement?
   - What app permission policy allows untrusted third-party apps to access team or channel data?
   - What meeting policy gap allows external participants to record or access sensitive meeting content without controls?
   - What rollback path exists if a tenant-wide external access change breaks existing partner collaboration?
4. **Recommend the smallest safe action**
   - Prefer staged rollout for policy changes, pilot group testing for new sensitivity labels, and report mode for information barrier policies.
   - If the safest action is to stop and gather evidence before making changes, say that plainly.

## Output contract

Return this structure:

```markdown
# M365 Teams Collaboration Governance Review: <scope>
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
