# Safety checklist — SAP BTP Governance Review

Use before making any governance recommendation, especially for findings that affect trust configuration, global account administration, or entitlement assignments with cost implications.

## Non-negotiables

- Do not access, connect to, or request access to any live BTP global account, subaccount, Cloud Foundry org, or Kyma cluster. This skill reviews artifacts only.
- Do not accept or request BTP service keys, client secrets, OAuth tokens, platform user credentials, global account administrator passwords, or subaccount-level access tokens.
- Do not recommend disabling or removing a trust configuration without first confirming (with the user) that no active users or applications depend on it. A wrong trust removal can immediately lock all users out of a subaccount.
- Do not recommend reducing or removing entitlements without first confirming with the user which service instances are currently consuming that quota. Reducing quota below active consumption terminates running service instances.
- Do not conflate BTP platform roles (global account, directory, subaccount admin) with application-level roles (XSUAA role collections for deployed applications). They are separate authorization hierarchies.
- Do not recommend the default SAP ID Service trust as a long-term production identity strategy. Production BTP subaccounts should use a federated corporate IdP via SAP Identity Authentication Service.
- Do not classify a finding as `critical` without being able to trace the specific unauthorized access path or compliance breach from user-provided evidence or documentation.

## What people get wrong

- **Conflating CF org quotas with BTP entitlements**: Cloud Foundry org-level memory/service instance quotas are set separately from BTP service entitlements. Both must be reviewed independently; reducing BTP entitlements does not automatically reduce CF org quota consumption.
- **Treating directory entitlements as automatic subaccount distribution**: Entitlements assigned at directory level are not automatically available to subaccounts — they still require explicit assignment. Unused directory-level entitlements are still a sprawl risk.
- **Assuming all Kyma modules are billed**: Kyma module enablement policy varies by BTP contract type. Never make cost-impact assertions without confirming the user's BTP commercial model.
- **Recommending IdP group mapping without confirming the IdP group structure exists**: Role collection assignment via IdP groups only works if the identity provider is configured to assert the correct group attribute and the group exists. Recommending this without confirming IdP group readiness creates an access lockout risk.
- **Missing the default trust risk**: Every new BTP subaccount has SAP ID Service trust enabled by default. In production subaccounts where a corporate IdP is in use, the default SAP ID Service trust should be reviewed — leaving it enabled allows any SAP ID Service user to attempt authentication.
- **Ignoring emergency access procedures**: Every BTP global account should have a documented emergency access path (emergency admin user not dependent on federated IdP) in case the corporate IdP is unavailable. Absence of this is a `high` governance risk.

## When to push back

- Push back when the user asks to confirm a governance finding from memory alone without providing any account structure artifact.
- Push back when the user asks to recommend specific quota numbers without providing current consumption data.
- Push back when the user asks to disable trust configurations without first mapping which users and applications currently rely on that trust.
- Push back when a request requires live BTP API access — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.

## Evidence labels

- `documentation-based` — grounded in SAP BTP Help Portal account model, entitlements, environments, role collections, or trust configuration docs
- `user-provided evidence` — BTP cockpit exports, architecture documents, role collection lists, or entitlement summaries provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
