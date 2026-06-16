# Identity Zero Trust Domain Guide

Use this reference for Microsoft Entra identity posture, Conditional Access design failure modes, PIM configuration, access review cadence, safe workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> We have MFA turned on, so our identity is Zero Trust.

Wrong. Zero Trust identity is about verifying every access request with all available signals, enforcing least privilege via JIT/JEA, and assuming breach by minimizing admin blast radius. MFA is necessary but insufficient — it can be bypassed by token theft, adversary-in-the-middle phishing, MFA fatigue, or legacy authentication protocols that skip MFA entirely.

Common bad assumptions:

- Security defaults provide equivalent protection to Conditional Access.
- Per-user MFA enforcement is as effective as Conditional Access-enforced MFA.
- Making admins eligible in PIM is enough without access reviews or activation requirements.
- Excluding a few VIPs or service accounts from Conditional Access policies is acceptable with no compensating controls.
- Blocking legacy authentication will break everything and can be deferred indefinitely.
- A single Global Administrator account is acceptable if it has a strong password and MFA.
- Guest access reviews are optional if external users are "trusted partners."

## Identity Zero Trust failure modes

- **Persistent admin assignments**: Standing Global Administrator or other privileged roles outside PIM create a permanent high-value target. A single compromised admin credential grants full tenant control.
- **Legacy authentication not blocked**: SMTP, POP3, IMAP, Basic Auth, and older Office clients bypass Conditional Access and MFA. Credential stuffing and password spray attacks exploit these.
- **Broad CA exclusions**: Excluding break-glass accounts, service accounts, or VIP users without monitoring and compensating controls creates undetected bypass paths.
- **MFA fatigue**: Push-notification MFA without number matching or additional context is vulnerable to fatigue attacks. Phishing-resistant MFA (FIDO2, certificate-based auth) is required for privileged accounts.
- **No risk-based CA**: Without Microsoft Entra ID Protection risk-based policies, compromised credentials may authenticate successfully across sessions before detection.
- **Stale guest access**: B2B guest accounts that never went through an access review accumulate over time. Compromised guest accounts can pivot to internal resources.
- **No break-glass monitoring**: Emergency access accounts excluded from CA policies are invisible to normal sign-in monitoring. Unauthorized use goes undetected without specific alerts.
- **PIM without access reviews**: Converting roles to eligible status without periodic access reviews means stale eligibility accumulates — a privileged role the employee no longer needs is still one activation away.

## Minimum safe workflow

1. Identify all Global Administrators and privileged role holders; use PIM Discovery and Insights to find permanent assignments outside PIM.
2. Review Conditional Access policy inventory — baseline coverage, exclusions, report-only vs. enforced, legacy authentication block status.
3. Classify MFA gaps: which user populations, apps, or authentication flows are not covered by CA-enforced MFA.
4. Classify PIM gaps: which privileged roles have standing active assignments instead of eligible; which eligible roles lack activation requirements (MFA, approval, time limit).
5. Review guest and external identity lifecycle — last sign-in dates, access review schedule, cross-tenant access policies.
6. Verify break-glass account hygiene — excluded from CA, password-based, monitored via alerts, reviewed by separate identity team.
7. Recommend smallest safe change: enable Conditional Access in report-only mode first, pilot PIM eligible conversion before full rollout, stage legacy authentication block by protocol.
8. Require approval and rollback plan before any Conditional Access policy moves from report-only to enforced mode.

## Verification targets

- PIM Discovery and Insights report — permanent active privileged role assignments to convert
- Conditional Access policy list — enforcement mode, assignment scope, exclusions, conditions, session controls
- Sign-in logs — legacy authentication protocol usage (filter by client app = Other clients, Exchange ActiveSync)
- Microsoft Entra access review results — privileged roles, guest users, group memberships, application assignments
- Authentication Methods policy — FIDO2, Microsoft Authenticator, certificate-based auth registration coverage
- Break-glass account inventory — excluded from CA, FIDO2 or certificate-based auth, alerts configured, last review date
- Microsoft Entra ID Protection risk detections — risky users, risky sign-ins, risk policy configuration
- Role assignment audit log — unexpected permanent assignments or PIM activations

## When to push back

Push back if the user asks to:

- Disable or weaken MFA for any user population to reduce friction
- Add broad Conditional Access exclusions without time-bounded exceptions and monitoring
- Keep standing Global Administrator assignments outside PIM because "PIM is too complex"
- Defer blocking legacy authentication because of old client concerns without a migration plan
- Approve Conditional Access policy changes directly in enforced mode without report-only validation
- Remove access reviews from PIM configuration to reduce administrative overhead
- Trust guest partner accounts without periodic access review or expiration
- Exclude all service accounts from CA without documenting workload identity alternatives
