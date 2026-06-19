# Dataverse Security and DLP Domain Guide

Use this reference for Power Platform DLP policy failure modes, Dataverse privilege model depth, connector governance pitfalls, and pushback criteria.

## What people get wrong

The lazy story is:

> Block all connectors in the default environment and the tenant is secure.

Wrong. DLP covers connector data flows but does not control Dataverse record access, column exposure, or maker ability to create new environments (if unrestricted). Governance requires both DLP discipline and Dataverse security model discipline.

Common bad assumptions:

- The default environment is automatically secured by a tenant policy — it requires explicit DLP hardening and environment routing.
- Organization-scope read on a Dataverse table is acceptable for all users because "they need access to the app."
- Ad-hoc row sharing is a safe substitute for a proper security role design.
- Multiple DLP policies on the same environment simplify management — they fragment the connector space and create hard-to-debug compliance violations.
- Blocking a connector in DLP prevents makers from using HTTP or custom connectors to reach the same endpoint.
- Column-level security (field-level security) compensates for a weak record-level security model — it does not; users must already have record access for column security to apply.
- Removing a user from a team automatically removes their accumulated privileges if the team has security roles.

## DLP failure modes

- Default environment has no tenant-level DLP policy, allowing any connector combination to be used by all licensed users in the organization.
- Custom connectors are unclassified (default non-business) and can be used with any other non-business connectors, including third-party data exfiltration endpoints.
- HTTP connector is not blocked, providing a generic bypass path to arbitrary external endpoints regardless of connector classification.
- Multiple environment-level policies stack on the tenant policy, fragmenting connector availability in ways that makers cannot observe or understand.
- Exception process for DLP changes is informal — admins reclassify connectors on request without blast-radius assessment or documentation.
- No tenant-level policy covers newly created environments, leaving all maker-provisioned environments open until manually assigned.

## Dataverse privilege failure modes

- System Administrator role assigned to too many users, bypassing all record-level and column-level security controls.
- Security roles grant Organization-scope Create, Write, or Delete on sensitive tables (customer data, financial records, HR data) where User-scope would suffice.
- Business unit hierarchy is flat (single root BU) making Parent-Child scope equivalent to Organization scope, defeating the hierarchy-based isolation intent.
- Owner teams accumulate security roles over time without periodic review — effective user access grows silently as team membership changes.
- Access teams are used where ownership and accountability for records are required — access teams cannot own records and should not substitute for owner teams with defined roles.
- Ad-hoc sharing of individual rows proliferates to compensate for missing role design — sharing is non-auditable at scale and degrades query performance.
- Column security profiles are assigned to broad groups or teams, making "restricted" columns accessible to most of the organization.
- Microsoft Entra guest accounts are granted Dataverse security roles without confirming they are excluded from sensitive connector access.

## Minimum safe workflow

1. Inventory: enumerate environments, DLP policies in scope, Dataverse security roles, business unit hierarchy, and team structure.
2. Map coverage gaps: which environments have no DLP policy? Which tables have Organization-scope privileges in roles assigned to broad populations?
3. Classify risk: connector exfiltration paths, privilege escalation via System Administrator, cross-BU data leakage, and ad-hoc sharing debt.
4. Recommend smallest safe action: DLP policy changes on test environment first; role cloning with reduced scope before removing existing roles; BU restructuring in a sandbox.
5. Gate production changes: explicit approval, blast-radius assessment, and rollback plan for any production DLP or Dataverse role change.

## Verification targets

- Tenant-level and environment-level DLP policy coverage: connector classification lists, environments in scope, custom connector assignments
- Dataverse security roles: table-level privileges and access levels (None/User/BU/Parent-Child/Organization) for sensitive tables
- Business unit hierarchy depth and whether hierarchy or position hierarchy is used
- Team types (owner vs. access) and security role assignments per team
- System Administrator role membership — minimize to operational necessity
- Column security profiles: which columns are protected, which profiles exist, and which users/teams hold each profile
- Ad-hoc sharing volume: row-level sharing records in audit logs or CoE inventory
- CoE Starter Kit deployment and maker governance signals: environment request coverage, DLP change request workflow active

## When to push back

Push back if the user asks to:

- disable DLP or exempt an environment from all policies to unblock a delivery deadline
- grant Organization-scope privileges on sensitive tables to all licensed users
- classify a custom connector as Business Data without documenting the endpoint and data handled
- leave the HTTP connector unclassified or in the Non-Business group
- rely on app-level access control as the sole substitute for Dataverse security roles
- bulk-share rows with a broad Microsoft Entra group instead of designing a proper security role
- add users directly to the System Administrator role to simplify troubleshooting
- apply production DLP or role changes without a blast-radius assessment and rollback plan
