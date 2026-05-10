# Workflow and output contract

Use this reference only when performing the full IAM policy audit, pre-deployment review, or least-privilege remediation pass.

## Review domains

Check these areas before giving a verdict:

- Policy type, principal identity or group, resource URN scope, action set, and intended operations
- Wildcarded URNs (`urn:v1:eu:resource:*`), over-broad action lists, allow rules that may supersede deny rules unexpectedly
- Missing condition blocks: IP CIDR restrictions, resource tag conditions, expiration dates
- OAuth2 service account scope vs. minimum required permissions
- Identity group membership sprawl and aggregated permission blast radius
- Reversibility of policy changes and rollback path

## Safe workflow

1. **Frame scope**
   - Account / NIC handle and environment:
   - Policy name(s) and principal type (user, group, service account):
   - Business purpose and data classification:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live OVHcloud API or Terraform state evidence if available.
   - Otherwise inspect repository `ovh_iam_policy` resources, sanitized user-provided config, or official OVHcloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What URN scope can expose unintended resources?
   - What action set can escalate privilege or perform destructive operations?
   - What missing condition allows access from untrusted networks or identities?
   - What identity group membership creates over-aggregated permissions?
   - What evidence is missing that prevents a confident verdict?
4. **Recommend the smallest safe action**
   - Prefer narrowest URN prefix, explicit condition blocks, and staged rollout.
   - If the safest action is to stop and gather the actual policy JSON, say that plainly.

## Output contract

Return this structure:

```markdown
# OVHcloud IAM Policy Review: <policy name or scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Checks or API calls:
- Expected result:
## Residual risk
- <risk or explicit none>
```
