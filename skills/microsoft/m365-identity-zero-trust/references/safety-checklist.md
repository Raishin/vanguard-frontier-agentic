# Safety checklist

Use this reference before any recommendation that changes Conditional Access policies, MFA requirements, PIM configuration, role assignments, guest access policies, or any other Microsoft Entra tenant configuration.

## Non-negotiables

- Never recommend weakening MFA coverage, adding broad Conditional Access exclusions, or disabling risk-based policies for convenience, deadline pressure, or VIP exceptions. State this refusal plainly.
- Never ask users to paste secrets, admin credentials, tenant IDs, client secrets, certificates, private keys, or customer data into chat.
- Use read-only Microsoft Entra admin evidence or Graph API read evidence for live state when available; otherwise use repository evidence, sanitized user evidence, or official documentation and label the evidence level.
- Do not invent Conditional Access policy states, PIM configuration, role assignment counts, or MFA enforcement coverage.
- Require explicit user approval before recommending creation or modification of Conditional Access policies, PIM role assignments, access review configuration, or break-glass account changes.
- Keep remediation least-privilege, reversible, staged (report mode before enforcement), and scoped to the requested role or policy boundary.
- Treat any standing Global Administrator assignment outside PIM as critical until converted to eligible.
- Treat any Conditional Access policy with broad exclusions (all admins excluded, no MFA for legacy apps) as high risk until compensating controls are documented.

## Stress checks

- What identity path can an attacker follow from initial compromise to Global Administrator?
- What MFA or Conditional Access gap allows password spray, phishing, or credential stuffing to succeed?
- What standing privileged role assignment widens blast radius if compromised?
- What guest or external account has stale, unreviewed access?
- What break-glass account monitoring gap would hide unauthorized emergency access use?
- What rollback path exists if a new Conditional Access policy locks out users or breaks service accounts?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Microsoft Entra tenant Conditional Access policy state, PIM configuration, or role assignment coverage.

## Escalation triggers

Escalate to live-guard gate before any of the following:

- Creating or modifying Conditional Access policies (especially disabling, adding exclusions, or changing enforcement mode from report-only to enabled)
- Changing PIM eligible or active role assignments for privileged roles
- Modifying MFA registration policies or authentication methods
- Changing external collaboration or B2B cross-tenant access policies
- Modifying break-glass or emergency access account configurations
- Enabling, disabling, or modifying Microsoft Entra ID Protection risk policies
