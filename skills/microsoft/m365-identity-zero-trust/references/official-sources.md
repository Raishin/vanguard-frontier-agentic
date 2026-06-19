# Official sources

Use this reference only when you need source grounding for Microsoft Entra identity, Conditional Access, PIM, and Zero Trust identity pillar service behavior or the detailed source list.

## Microsoft documentation

Use these as starting points, not as proof of the user's live Microsoft Entra tenant state:

- https://learn.microsoft.com/security/zero-trust/deploy/identity
- https://learn.microsoft.com/entra/identity/conditional-access/plan-conditional-access
- https://learn.microsoft.com/entra/identity/conditional-access/overview
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-configure
- https://learn.microsoft.com/entra/fundamentals/zero-trust-protect-identities
- https://learn.microsoft.com/entra/id-governance/access-reviews-overview
- https://learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-create-roles-and-resource-roles-review
- https://learn.microsoft.com/security/zero-trust/zero-trust-identity-device-access-policies-overview
- https://learn.microsoft.com/entra/identity/conditional-access/concept-conditional-access-policy-common
- https://learn.microsoft.com/entra/identity/role-based-access-control/best-practices

## Grounding rule

Official documentation explains Microsoft Entra and Conditional Access service behavior. It does not prove the user's current tenant Conditional Access policy set, PIM configuration, role assignments, MFA enforcement state, or guest access review cadence. Prefer read-only Microsoft Entra admin evidence, Graph API read output, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Key service facts from official Microsoft Learn documentation:

**Zero Trust identity principles (per learn.microsoft.com/security/zero-trust/deploy/identity):**
- Verify explicitly: evaluate every access request with all available signals — user, device, location, risk, app
- Least privilege: JIT/JEA via PIM, risk-based Conditional Access, role delegation by task
- Assume breach: minimize admin blast radius, segment privileged access, audit privileged role activations

**Conditional Access (per learn.microsoft.com/entra/identity/conditional-access):**
- Real-time Zero Trust policy engine in Microsoft Entra ID
- Signals: user/group, location (named location/country), device state, app, sign-in risk, user risk, authentication strength
- Common baseline policies (Microsoft Entra ID P1): require MFA for admins, require MFA for all users, block legacy authentication
- Risk-based policies (Microsoft Entra ID P2 / Identity Protection): require MFA on medium/high sign-in risk, require password change on high user risk
- Require phishing-resistant MFA (FIDO2/certificate-based) for privileged administrators

**Privileged Identity Management (per learn.microsoft.com/entra/id-governance/privileged-identity-management/pim-configure):**
- Converts standing ("active") privileged role assignments to time-bound JIT ("eligible") assignments
- Requires MFA, approval, justification, and time limit for role activation
- Sends notifications on role activation
- Access reviews for privileged roles — periodic re-attestation by approvers
- PIM Discovery and Insights — identify all permanent admin assignments for conversion
- Supports Microsoft Entra roles and resource roles (subscriptions, resource groups)

**Microsoft Entra ID Governance (access reviews, entitlement management):**
- Access reviews for groups, applications, privileged roles, and guest users
- Entitlement management — time-bound access packages with automatic expiration
- Lifecycle workflows — automate identity lifecycle (joiner/mover/leaver)

**Common failure modes:**
- Standing Global Administrator assignments outside PIM (permanent active admin)
- Broad Conditional Access exclusions for VIPs, service accounts, or legacy applications without compensating controls
- MFA gaps for guest and external users
- Stale guest accounts without access review cadence
- Break-glass accounts without monitoring and review procedures
- Legacy authentication not blocked, enabling password spray / credential stuffing

Review implications:
- Do not approve CA policy designs that weaken MFA or add broad exclusions without time-bounded exceptions and compensating controls.
- PIM eligibility alone does not prove least privilege — verify role scope, activation requirements, and review cadence.
- Documentation cannot prove the user's actual Conditional Access policy state, PIM configuration, or role assignment coverage.
