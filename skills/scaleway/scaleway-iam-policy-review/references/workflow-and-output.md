# Workflow and output contract

Use this reference when performing a full IAM posture review, key lifecycle audit, or permission set assessment.

## Review domains

Check these areas before giving a verdict:

- API key scope: organization-level vs project-level, expiry set or absent, rotation history
- Principal type: human user, application (non-human), or group binding
- Permission sets: named bundles assigned (e.g., `InstancesFullAccess`, `ObjectStorageReadOnly`); check for wildcard or overly broad sets
- Policy breadth: rules binding principals to permission sets; check for missing resource restrictions and missing conditions
- Key sprawl: undocumented keys, keys with no owning workload, keys not rotated in > 90 days
- Organization vs project boundary: organization-scope grants access to ALL projects — always prefer project scope for automation

## Safe workflow

1. **Frame scope**
   - Workload or system under review:
   - Environment (production / staging / development):
   - Compliance driver (if any):
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized Terraform `scaleway_iam_policy` / `scaleway_iam_application` resource definitions.
   - Otherwise use sanitized CLI or console output describing policy bindings.
   - Label each finding as `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
   - If no policy definitions were provided, state that explicitly before proceeding.
3. **Stress-test risk**
   - Which keys have no expiry?
   - Which principals hold organization-level scope?
   - Which permission sets grant write or delete access beyond stated workload needs?
   - Which API keys have no documented owning workload?
   - What rotation or revocation path is unproven?
4. **Recommend the smallest safe tightening action**
   - Prefer scoping to project level, adding expiry, replacing broad sets with narrow ones.
   - If the safest action is to gather missing evidence before recommending changes, say that plainly.

## Output contract

Return this structure:

```markdown
# Scaleway IAM Policy Review: <scope>

## Posture verdict
- Status: COMPLIANT / COMPLIANT WITH RISKS / NON-COMPLIANT / NEEDS EVIDENCE
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
- Commands or checks:
- Expected result:

## Residual risk
- <risk or explicit none>
```
