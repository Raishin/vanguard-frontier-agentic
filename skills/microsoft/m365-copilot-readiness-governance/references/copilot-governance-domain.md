# Copilot Governance Domain Guide

Use this reference for Microsoft 365 Copilot readiness, oversharing assessment, Zero Trust layer review, data governance failure modes, safe workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> Assign the Copilot license and it only shows users what they already have access to.

Technically true — but dangerously incomplete. Microsoft 365 Copilot surfaces data that users *technically* can access but *practically* would never discover manually. Overly broad permissions, stale access grants, and EEEU sharing mean Copilot becomes a search accelerator for data that should have been restricted years ago.

Common bad assumptions:

- Copilot respects permissions, so oversharing is the user's problem, not IT's.
- Sensitivity labels on some documents are enough for the whole tenant.
- EEEU grants are harmless because they apply to internal users only.
- Graph connectors and plugins inherit the same access controls as SharePoint.
- Running DSPM for AI once at enablement is sufficient ongoing governance.
- Restricted SharePoint Search is a permanent solution rather than an interim control.

## Copilot governance failure modes

- **EEEU oversharing**: Sites with Everyone Except External Users grants expose all internal content to Copilot grounding, regardless of data sensitivity.
- **Broken inheritance**: Libraries and folders with broken permission inheritance are invisible to site-level audits but fully accessible to Copilot.
- **Stale access**: Former employees, vendors, or project members with lingering permissions expand the Copilot data surface unexpectedly.
- **Missing site owners**: Sites without active owners cannot be reviewed, remediated, or attested; SAM site access reviews cannot be actioned.
- **Unscoped connector permissions**: Graph connectors or Copilot extensibility agents with Mail.ReadWrite, Files.ReadWrite.All, or Calendars.ReadWrite at tenant scope exceed least privilege.
- **DLP gaps on Copilot location**: DLP policies not scoped to the Microsoft 365 Copilot location allow sensitive content to be grounded in Copilot responses.
- **Label gaps on high-value sites**: SharePoint sites containing financial, HR, or regulated data without site sensitivity labels are invisible to label-based DLP and access controls.
- **RSS as permanent state**: Restricted SharePoint Search is an interim control with site limits; treating it as permanent governance leaves the underlying oversharing problem unresolved.

## Zero Trust layer minimum safe workflow

1. **Layer 1 — Data protection**: Run DSPM for AI data risk assessment. Review sensitivity label coverage across SharePoint, OneDrive, Teams, Exchange. Run SAM Content Management Assessment. Identify EEEU-exposed sites and high-risk sharing links.
2. **Layer 2 — Identity and access**: Verify Conditional Access MFA baseline is in place. Confirm access reviews are scheduled for groups and applications with Copilot scope.
3. **Layer 3 — App protection**: Verify Intune app protection policies cover Copilot mobile surfaces if mobile use is in scope.
4. **Layer 4 — Device management**: Confirm device compliance policies are enforced for Copilot access if device-based CA conditions are intended.
5. **Layer 5 — Threat protection**: Confirm audit logging is enabled for Copilot interaction activity. Verify Defender for Office 365 and EOP baselines are active.
6. **Layer 6 — Secure Teams collaboration**: Review Teams external access policies, guest access settings, and shared channel governance.
7. **Layer 7 — User permissions to data**: Run SAM site access reviews for high-risk sites. Remove EEEU. Confirm site ownership. Rescope sharing links to approved users or security groups.
8. **Connectors and plugins**: For each Graph connector or Copilot extensibility agent, document the Graph permission scope, data accessed, and business justification. Require scoped, least-privilege permissions before approval.
9. **Enablement gate**: Only recommend enabling Copilot after evidence of completion (or documented accepted risk) for all applicable layers. Refuse if baseline is absent.

## Verification targets

- DSPM for AI data risk assessment output and flagged high-risk sites
- SAM Content Management Assessment — oversized audiences, EEEU usage, broken inheritance, inactive/ownerless sites
- Sensitivity label coverage report for SharePoint sites, OneDrive locations, and Teams channels
- DLP policy scope — confirm Microsoft 365 Copilot location is included where required
- Sharing link report — anonymous links, organization-wide links, EEEU grants on high-value sites
- Microsoft Graph permission inventory for all connectors, plugins, and Copilot extensibility agents
- Purview Audit log — Copilot interaction activity enabled and retained per compliance policy
- Conditional Access MFA baseline — confirm scope includes Copilot and Microsoft 365 services

## When to push back

Push back if the user asks to:

- Enable Copilot without a completed oversharing assessment or DSPM for AI review
- Treat Restricted SharePoint Search as the final governance solution
- Skip sensitivity labeling on the grounds that "the data isn't that sensitive"
- Grant broad Graph application permissions to connectors or plugins without scoped justification
- Accept EEEU exposure on sites with financial, HR, legal, or regulated data
- Skip site access reviews because site owners "are too busy"
- Disable DLP policies to avoid false positives in Copilot responses
