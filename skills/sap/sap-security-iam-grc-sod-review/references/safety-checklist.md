# Safety checklist — SAP Security IAM GRC and SoD Review

Use before making any IAM or GRC remediation recommendation, especially for findings involving SoD conflicts, role changes, or identity provider trust modifications.

## Non-negotiables

- Do not access, connect to, or request access to any live IAS tenant, IPS admin console, XSUAA service instance, GRC Access Control system, BTP subaccount, or on-premise SAP system. This skill reviews artifacts only.
- Do not accept or request IAS admin credentials, IPS technical user passwords, XSUAA client secrets, GRC system logon credentials, BTP service keys, or RFC connection details.
- Do not approve, close, or recommend closing an unmitigated critical SoD conflict. Only the GRC / audit team with appropriate authority can accept or mitigate a critical SoD risk. State this explicitly in every response where an unmitigated critical conflict is found.
- Do not recommend changes to live role assignments, XSUAA service bindings, or IPS connector configurations in a production system based on advisory review alone. All recommendations require testing in a non-production environment and GRC/audit sign-off where applicable.
- Do not recommend removing a trust configuration (XSUAA or BTP subaccount) without first confirming that no active users or applications depend on it. A wrong trust removal can lock out all users from an application.
- Do not use memory alone to assert which SAP transaction codes create SoD conflicts. SoD conflict classification must be grounded in user-provided GRC ruleset exports, official SAP GRC documentation, or user-described conflict reports.
- Do not conflate IAS-level authentication controls with XSUAA-level authorization controls. MFA enforcement is an IAS concern; scope and role assignment is an XSUAA concern. They are separate security layers with different remediation paths.

## What people get wrong

- **Conflating IAS users with BTP platform users**: Users managed in IAS (application users) and users managed at the BTP platform level (platform users in the global account) are different identity populations with separate lifecycle management requirements. IPS provisions application users; BTP platform user management is done in the BTP cockpit.
- **Treating xs-security.json role collections as deployed role collections**: Role collections declared in xs-security.json are created when the application is deployed to BTP. They are separate from custom role collections created manually in the BTP cockpit. Both must be reviewed; changes to xs-security.json only affect the next deployment.
- **Missing privilege accumulation across multiple role collections**: A user with three individually-compliant role collections can accumulate incompatible access when the collections are combined. SoD analysis must consider the union of all role collections per user, not each in isolation.
- **Recommending direct user assignment as a temporary fix**: Direct user-to-role-collection assignment bypasses the IdP group lifecycle and creates orphaned assignments when users change roles or leave the organization. It is never acceptable as a long-term or even temporary production fix without a documented exception.
- **Assuming GRC covers cloud IAM automatically**: GRC Access Control traditionally covers on-premise SAP authorization objects. Cloud IAM (IAS, IPS, XSUAA, BTP role collections) may not be in scope for the user's GRC ruleset. Confirm coverage scope before asserting that a cloud IAM finding is or is not captured by GRC.
- **Ignoring Emergency Access Management (EAM) in the governance assessment**: Organizations that use firefighter IDs in SAP GRC EAM must ensure firefighter access is time-bounded, logged, and reviewed. Unreviewed or perpetually active firefighter IDs are `high` findings regardless of the underlying SoD position.

## When to push back

- Push back (and escalate) when an unmitigated critical SoD conflict is found — do not proceed with any other recommendation until the conflict is escalated to GRC/audit.
- Push back when the user asks to confirm role or SoD compliance from memory alone without providing role lists, SoD reports, or GRC artifacts.
- Push back when the user asks to recommend specific XSUAA scope values without providing the xs-security.json file or a description of the authorization model.
- Push back when the request requires live system access (IAS API, IPS connector API, XSUAA binding, GRC logon) — state clearly that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when asked to approve or close a GRC access violation or SoD conflict — this skill is advisory only; approval authority rests with the GRC/audit team.

## Evidence labels

- `documentation-based` — grounded in SAP IAS, IPS, XSUAA, GRC Access Control, or SAP security best practice documentation
- `user-provided evidence` — SoD conflict reports, role lists, xs-security.json files, IAS exports, IPS connector configurations, or descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
