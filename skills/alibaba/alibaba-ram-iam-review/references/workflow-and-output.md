# Workflow and output contract

Use this reference only when performing a full RAM IAM review, privilege escalation analysis, or org-level permission boundary assessment.

## RAM IAM review areas to check

- RAM users: active user list, MFA status, console vs. API-only access, last login date, AccessKey count and rotation age
- RAM groups: group membership, attached managed policies, over-privileged group assignments
- RAM roles: trust policy principals (wildcard check), attached permissions, cross-account trust, `sts:AssumeRole` chains
- Managed policies: AdministratorAccess assignments, wildcard action policies (`*`), wildcard resource policies
- STS tokens: token validity configuration, application-level caching, token scope vs. minimum required permissions
- Resource Directory: org tree structure, Control Policy attachments to OUs and member accounts, simulation vs. enforcement status
- Privilege escalation paths: roles that can assume other privileged roles, policies granting `ram:*` or `sts:AssumeRole` broadly

## Safe workflow

1. **Frame scope** — confirm target account/org, review driver, evidence available, and explicit non-goals
2. **Collect evidence** — prefer sanitized RAM exports, IaC, or structured user descriptions; never request actual credentials; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what can escalate privilege? what is AdministratorAccess scope? what Control Policy gaps exist?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path; test Control Policy in simulation before enforcement

## Output contract

Return this structure:

```markdown
# Alibaba Cloud RAM IAM Review: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
