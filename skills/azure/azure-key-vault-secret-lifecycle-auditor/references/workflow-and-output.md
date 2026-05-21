# Workflow and Output Contract

## Safe Workflow

1. **Scope the vault estate**
   - Which vaults matter?
   - Which workloads or teams depend on them?
   - Which assets are secrets, keys, or certificates?
2. **Check the protection floor**
   - Is soft delete enabled?
   - Is purge protection enabled?
   - What is the retention period?
   - Are policy controls enforcing the floor?
3. **Check the permission model**
   - Azure RBAC or legacy access policies?
   - Who can read, write, delete, recover, or purge?
   - Are roles assigned at the right scope?
   - Is purge authority too broad?
4. **Check secret lifecycle hygiene**
   - Expiration set or missing?
   - Owner and rotation metadata present?
   - Tags used for lifecycle metadata rather than stuffing metadata into secret values?
   - General configuration data incorrectly stored as secrets?
5. **Check rotation realism**
   - Is rotation manual, reminder-based, or automated?
   - Is dual-credential or zero-downtime rotation needed?
   - Are dependent services updated correctly?
   - Are failed rotations visible?
6. **Check monitoring and events**
   - Near-expiry notifications configured?
   - Event Grid or other alerting present?
   - Are alert owners named?
7. **Check recovery posture**
   - Can deleted secrets be recovered?
   - Does the team understand purge consequences?
   - Do they know that some integrated services or subscriptions may need recreation after vault recovery?
8. **Return a go / no-go style secret-lifecycle verdict**
   - What is safe,
   - what is brittle,
   - what is missing,
   - and what must change first.

## Role-Specific Stress Checks

- Reject “it’s in Key Vault, so it’s secure.” Storage location is not lifecycle discipline.
- Reject any design where humans can purge critical vault assets casually.
- Reject rotation claims that do not explain how dependent systems receive the new secret.
- Reject “we monitor expiry” if the team cannot name the alert path, owner, and escalation.
- Reject vault designs storing feature flags or generic configuration as secrets.
- Reject recovery confidence if soft delete or purge protection is missing or misunderstood.
- Reject audits that inspect secret values when metadata would answer the question safely.
- Reject broad `Key Vault Administrator` usage as a default operational model.

## Output Template

```markdown
# Azure Key Vault Secret Lifecycle Audit: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest risk:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Vault(s):
- Environment:
- Dependent workloads:
- Permission model:

## Findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Lifecycle control review
| Control area | Expected state | Observed state | Gap | Blocking |
|---|---|---|---|---|
| Soft delete |  |  |  |  |
| Purge protection |  |  |  |  |
| RBAC / purge authority |  |  |  |  |
| Expiration metadata |  |  |  |  |
| Rotation process |  |  |  |  |
| Eventing / alerts |  |  |  |  |
| Recovery readiness |  |  |  |  |

## Safe next actions
1.
2.
3.

## Open questions
-
```

## Red Flags

- The team wants an audit but refuses to separate secrets, keys, and certificates.
- Secret rotation is claimed, but nobody can explain how consumers adopt new values.
- Purge protection is absent for critical vaults or encryption dependencies.
- Broad administrator roles exist where narrower secrets roles would suffice.
- The audit relies on secret contents instead of safer metadata.
- The team assumes vault recovery restores every dependent integration automatically.
