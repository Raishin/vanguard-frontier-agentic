# Safety checklist

Use this reference before any recommendation that involves group-based licensing changes, license removal, SKU downgrade actions, or any other Microsoft 365 license assignment configuration change in production.

## Non-negotiables

- Advisory only — never make or imply purchase commitments, guarantee cost savings, or provide binding contract pricing. State this limitation plainly if pressed.
- Never recommend removing licenses from active users without first confirming the user is inactive or the service is not depended upon — service interruption risk is immediate.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Microsoft 365 admin center evidence or Microsoft Graph license API read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent license counts, assignment states, group membership, or contract pricing.
- Require explicit user approval before recommending group-based licensing configuration changes in production — group-based licensing changes are live-guard gated.
- Never recommend nested groups for license assignment — nested group members do not receive licenses and this is a silent failure mode.
- Treat any large-scale license removal recommendation (100+ users) as requiring staged validation before full execution.

## Stress checks

- Which users have licenses assigned but have not signed in for 90+ days — are they truly inactive or on leave?
- Which license assignments are manual-only with no group-based automation, creating de-provisioning gaps when users leave?
- Are nested groups used anywhere in license assignment, silently excluding nested members?
- Which add-ons are assigned to users whose base SKU already includes the same capability — duplicate cost?
- Is the E3-versus-E5 decision documented against specific capability requirements, or was it a default choice?
- Does the EA true-up timeline align with current headcount trend to avoid surprise annual overage?
- Are users without a usage location set present in any licensed group — causing silent assignment failures?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual license assignment counts, group membership state, or contract terms. Never derive contract pricing from public documentation.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Assigning or removing group-based licensing configurations in production tenant security groups
- Removing licenses from user accounts (individual or bulk)
- Changing SKU assignments for 50 or more users simultaneously
- Initiating a group-based licensing reprocess operation via Microsoft Graph or PowerShell
- Recommending a SKU downgrade that removes capabilities currently in use
- Making any recommendation that touches EA true-up commitments or contract renewal terms — escalate to Microsoft account team or licensing specialist
