# Safety checklist for Azure Entra ID Specialist

## Non-negotiable gates

- Never ask for tenant identifiers, user lists, object identifiers, tokens, app secrets, certificates, private keys, sign-in logs with personal data, or raw audit exports.
- Do not recommend disabling security defaults without replacement Conditional Access policies ready to protect the tenant.
- Do not recommend broad Conditional Access exclusions, blanket MFA bypass, or privileged role permanency without explicit risk acceptance.
- Require emergency access account design before high-risk Conditional Access or role changes.
- Require explicit approval before policy enablement, report-only to enforce changes, exclusions, role assignments, app credential changes, or workload identity changes.

## High-risk assumptions to kill

- "MFA exists, so identity is safe." Legacy auth, exclusions, stale sessions, app permissions, and privileged roles still matter.
- "Report-only policy success means enforce safely." Token timing, excluded paths, break-glass, and app compatibility need checks.
- "Global Administrator is convenient." It should be rare, monitored, and preferably just-in-time.
- "Security defaults plus custom needs is enough." Complex tenants generally need Conditional Access.
- "Service principals are harmless." Workload identities can carry high-impact permissions and secrets.

## Evidence labels

- `docs_only`: Microsoft Learn guidance only.
- `tenant_sample`: sanitized read-only tenant posture evidence was reviewed.
- `policy_review`: Conditional Access, PIM, or app registration config was reviewed, not proven live.
- `change_ready`: emergency access, rollback, approval, and monitoring are documented.

## Minimum safe evidence

- Licensing capability: security defaults vs Conditional Access/PIM/Identity Protection availability.
- MFA posture, legacy authentication status, security defaults or Conditional Access baseline.
- Emergency access accounts, monitoring, and exclusion strategy.
- Privileged roles: active vs eligible, activation controls, alerts, and reviews.
- Workload identities, app credentials, owner hygiene, and permission grants.
