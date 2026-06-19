# Safety checklist

Use this reference before any recommendation that changes DLP policies, sensitivity labels, retention policies, eDiscovery holds, Insider Risk Management configuration, or any other Microsoft Purview tenant compliance configuration.

## Non-negotiables

- Never recommend weakening DLP policies, adding broad exclusions, reducing sensitive information type coverage, or switching policies from enforcement mode back to audit mode for convenience, deadline pressure, or VIP exceptions. State this refusal plainly.
- Never recommend releasing an eDiscovery legal hold unless the litigation or investigation is formally closed and the legal team has confirmed in writing. State this escalation requirement plainly.
- Never recommend removing or weakening a preservation lock on a retention policy.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Microsoft Purview compliance portal evidence or Graph API read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent DLP policy enforcement states, sensitivity label taxonomy, retention policy coverage, or eDiscovery hold scope.
- Require explicit user approval before recommending creation or modification of DLP policies, sensitivity label policies, retention policies, eDiscovery holds, or Insider Risk Management policy changes.
- Keep remediation least-privilege, reversible, staged (audit mode before enforcement), and scoped to the requested compliance boundary.
- Treat any DLP policy permanently in audit/test mode without a documented promotion plan as a gap.
- Treat any active litigation trigger without a confirmed eDiscovery legal hold covering all relevant custodians and data sources as critical.

## Stress checks

- What sensitive content type is not covered by any DLP policy or sensitivity label?
- What DLP policy has been in audit mode without an enforcement promotion plan?
- What regulated content type (financial, HR, legal, health) lacks a retention policy?
- What active litigation or regulatory investigation lacks a confirmed eDiscovery hold on all custodians?
- What Insider Risk Management policy template is missing for high-risk user scenarios?
- What Microsoft 365 Copilot or third-party AI app interaction is exposing unlabeled or over-shared sensitive data per DSPM for AI assessments?
- What rollback path exists if a new DLP policy disrupts legitimate business workflows, and has it been validated in audit mode first?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Microsoft Purview DLP policy enforcement state, sensitivity label taxonomy deployment, retention coverage, or eDiscovery hold completeness.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Creating or modifying DLP policies (especially switching from audit to enforcement mode, adding or removing sensitive information types, or adding broad exclusions)
- Releasing or modifying any eDiscovery legal hold on an active case or investigation
- Creating, modifying, or deleting retention policies or retention labels for regulated content
- Modifying or disabling Insider Risk Management policies, removing risk indicators, or changing Adaptive Protection configuration
- Creating or modifying sensitivity label encryption settings or label policies with mandatory labeling
- Changing Audit (Premium) log retention periods or disabling audit logging for any workload
- Any DSPM for AI remediation that removes access, deletes content, or changes sharing permissions
