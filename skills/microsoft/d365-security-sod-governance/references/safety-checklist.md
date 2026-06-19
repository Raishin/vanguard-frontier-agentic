# Safety checklist

Use this reference before any recommendation involving production role changes, SoD conflict override approvals, privileged access grants, or compliance-impacting security configuration changes in Dynamics 365 Finance & Operations.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, client secrets, certificates, or customer personally identifiable information into chat.
- Use exported security reports or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent role names, duty names, privilege counts, SoD rule configurations, or live environment state.
- Require explicit human approval before recommending any production role assignment change, SoD override, or security configuration mutation.
- Use current official Microsoft Learn documentation for D365 Finance & Operations security behavior.
- Keep remediation least-privilege, reversible, and scoped to the domain in question.
- Production role changes are live-guard gated. Always escalate to a qualified D365 system administrator with environment access before execution.

## Stress checks

- What duty pairs could enable fraud or bypass an internal control?
- What system administrator or super-user assignments exist without documented justification?
- What SoD override is in place without a compensating detective control?
- What role change breaks an existing compliance posture or audit trail?
- What rollback path exists if a role assignment removes required access for a business process?
- What audit evidence is missing that regulators or internal auditors would expect?

## Evidence labels

Use `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live D365 role configuration, SoD rule set, or override history.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Assigning or removing security roles in the production environment
- Approving or denying SoD conflict overrides in production
- Creating, modifying, or deleting SoD rules in production
- Modifying system administrator role membership in any environment
- Changing legal entity scoping on role assignments
