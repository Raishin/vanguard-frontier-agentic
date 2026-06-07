# Microsoft Entra identity operations

## What people get wrong

- They disable security defaults before replacement Conditional Access coverage exists.
- They make broad Conditional Access exclusions for executives, service accounts, or break-glass without monitoring and compensating controls.
- They grant permanent privileged roles instead of using PIM and just-in-time activation.
- They ignore workload identities, app registrations, credentials, and delegated permissions while focusing only on human users.
- They treat report-only Conditional Access results as proof enforcement is safe for every token path.

## Officially grounded service shape

Microsoft Learn describes security defaults as a basic baseline requiring MFA registration, administrator MFA, protection for privileged Azure Resource Manager access, and blocking of legacy authentication and device code flow. For complex tenants, Conditional Access provides customizable policies. Microsoft guidance also emphasizes PIM for just-in-time privileged roles, MFA for administrators, emergency access accounts, least privilege, backup/recovery, and monitoring of identity governance changes.

## Non-negotiable design rules

1. Keep either security defaults or equivalent Conditional Access coverage active.
2. Require emergency access accounts and monitoring before risky policy changes.
3. Prefer PIM eligible assignments over permanent privileged roles.
4. Minimize exclusions and document owner, reason, expiration, and compensating controls.
5. Block or phase out legacy authentication and risky flows unless a documented exception exists.
6. Govern workload identities with owners, least privilege, credential lifecycle, and reviews.
7. Treat identity configuration changes as high blast-radius mutations.

## Minimal safe implementation flow

1. Identify licensing and whether security defaults or Conditional Access is the active baseline.
2. Inventory MFA, legacy auth, device code flow, admin access, and emergency access posture.
3. Review Conditional Access assignments, targets, conditions, grants, sessions, exclusions, and report-only data.
4. Review privileged roles, PIM settings, activation requirements, alerts, and access reviews.
5. Review app registrations and workload identities for owners, credentials, grants, and lifecycle.
6. Stage changes with pilot scope, report-only, break-glass verification, and rollback.
7. Enforce only after monitoring and support readiness are proven.

## High-risk assumptions to kill

- Disabling security defaults is unsafe unless equivalent Conditional Access coverage, emergency access, monitoring, and support readiness already exist.
- Report-only Conditional Access is impact evidence, not enforcement proof; token paths, exclusions, app dependencies, and break-glass flows still need testing.
- Broad block policies can lock out admins, especially when combined with all-resources scope and weak exclusion planning.
- Workload identities are not covered by user-scoped Conditional Access; service principals, managed identities, and federated credentials need separate review.
- A control may require specific Entra licensing or service entitlement; do not imply availability from documentation alone.

## Safe command/code verification targets

- Inspect policy-as-code or exported Conditional Access JSON for users/workload identities, target resources, conditions, grants, session controls, state, exclusions, and report-only posture.
- Review role assignments and PIM configuration for permanent privileged roles, activation duration, MFA, approval, notifications, and access reviews.
- Check app registrations and service principals for owners, credential type, credential age, federated credentials, API permissions, consent grants, and sign-in/risk logs.
- Verify emergency access accounts are excluded appropriately, monitored, tested, and not used for routine administration.
- Confirm final evidence distinguishes documented Microsoft Learn behavior from sampled tenant evidence and unverified licensing assumptions.

## Safe verification targets

- Security defaults state or equivalent Conditional Access baseline.
- MFA registration and enforcement for users and privileged roles.
- Legacy authentication and device code flow handling.
- Emergency access account count, configuration, and alerting.
- PIM role eligibility, activation duration, approval, MFA, and notifications.
- Workload identity owners, credentials, permissions, and risk signals.

## When to push back

Push back on disabling defaults, enforcing broad Conditional Access without emergency access, permanent Global Administrator assignments, unowned app registrations, blanket exclusions, or identity recommendations based only on screenshots or assumptions.
