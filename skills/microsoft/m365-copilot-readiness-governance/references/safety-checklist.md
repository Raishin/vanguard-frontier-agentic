# Safety checklist

Use this reference before any recommendation that changes sharing settings, label policies, DLP rules, Copilot enablement toggles, connector permissions, or any other Microsoft 365 tenant configuration.

## Non-negotiables

- Never recommend enabling Microsoft 365 Copilot without evidence of a completed oversharing assessment and permissions baseline. State this refusal plainly and block until baseline is done.
- Never ask users to paste secrets, admin credentials, tenant IDs, connection strings, client secrets, or customer data into chat.
- Use read-only Microsoft 365 Admin Center, SharePoint Admin Center, or Graph API read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent tenant configuration states, sensitivity label coverage, sharing link counts, connector grants, or DLP policy scope.
- Require explicit user approval before recommending enabling Copilot, publishing sensitivity labels, creating or modifying DLP policies, changing tenant-wide sharing settings, or granting connector permissions.
- Keep remediation least-privilege, reversible, and scoped to the requested workload or site boundary.
- Treat EEEU (Everyone Except External Users) exposure as high severity until proven remediated with evidence.
- Treat any connector or plugin with unscoped Graph permissions (Mail.ReadWrite, Files.ReadWrite.All without site scope, etc.) as high risk until scoped.

## Stress checks

- What data can Microsoft 365 Copilot surface to users beyond their intended access?
- What stale permissions or overly broad sharing links amplify oversharing blast radius?
- What connectors or plugins have Graph permissions that exceed their stated use case?
- What compliance or audit evidence is missing from the DSPM for AI assessment?
- What rollback path exists if Copilot is paused or disabled post-enablement?
- What site owners are missing and cannot attest to their site's data sensitivity?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Microsoft 365 tenant configuration, label coverage, or sharing state.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Enabling Microsoft 365 Copilot for any user population
- Publishing or modifying sensitivity labels tenant-wide
- Creating or modifying DLP policies that affect Copilot grounding
- Changing tenant-wide sharing settings (EEEU, external sharing, anonymous links)
- Granting or modifying Microsoft Graph application permissions for connectors or plugins
- Enabling or modifying Restricted SharePoint Search or Restricted Content Discovery settings
