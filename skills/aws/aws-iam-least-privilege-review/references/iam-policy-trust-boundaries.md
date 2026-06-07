# IAM Policy and Trust Boundary Guide

Use this reference for AWS IAM identity policies, trust policies, resource policies, permission boundaries, SCPs, session policies, PassRole, federation, OIDC, Access Analyzer findings, and least-privilege remediation.

## What people get wrong

The lazy story is:

> Remove wildcards and the policy becomes least privilege.

Wrong. Least privilege is about actions, resources, conditions, trust, session context, permission boundaries, and how the role is reached. A narrow action on a dangerous resource can still be privilege escalation.

Common bad assumptions:

- Managed policies are safer than inline policies.
- Trust policy principal is enough without conditions.
- `iam:PassRole` is harmless if deployment needs it.
- SCP denies prove workload least privilege.
- Access Analyzer generated policy is complete authorization design.
- OIDC federation is safe if the provider is official.

## IAM failure modes

- Broad trust allows external accounts, services, or web identities without audience, subject, source, or external ID constraints.
- `iam:PassRole`, `sts:AssumeRole`, `iam:CreatePolicyVersion`, or permissions-boundary edits enable escalation.
- Resource policies grant cross-account/public access bypassing identity-only review.
- Conditions use wrong operator, key, or case; deny/allow precedence is misunderstood.
- Session policies, permission boundaries, and SCPs interact differently than expected.
- Break-glass or CI/CD roles accumulate production admin over time.

## Minimum safe workflow

1. Identify principal, resource, action path, caller context, account/Org boundary, and business need.
2. Review effective access path: identity policy, trust policy, resource policy, boundary, SCP, session policy, and service control conditions.
3. Classify escalation risk: PassRole, policy administration, KMS decrypt, network/security group, data exfiltration, secrets, and logging disablement.
4. Reduce access by action, resource ARN, condition, tag, principal, source account/ARN, VPC endpoint, and time/session constraints where supported.
5. Use IAM Access Analyzer, policy validation, and simulation as evidence inputs, not proof of production safety.
6. Provide a minimal diff and explain what it grants, denies, and cannot prove.
7. Require approval before live IAM changes, especially denies, trust changes, and production deploy roles.

## Verification targets

- identity policies, trust policies, resource policies, permission boundaries, SCPs, session policies, and permission sets
- Access Analyzer external access and unused access findings, policy validation, generated policies, and simulate-principal-policy output
- condition keys: ExternalId, SourceArn, SourceAccount, PrincipalOrgID, VPC endpoint, MFA, tags, audience/subject for OIDC
- `iam:PassRole`, `sts:AssumeRole`, policy version/admin actions, KMS decrypt, S3/object access, Secrets Manager, and logging controls
- CloudTrail evidence of actual usage, last accessed data, and role chaining/session name patterns
- rollback path for policy denies, boundary changes, SCPs, and trust policy edits

## When to push back

Push back if the user asks to:

- grant wildcard admin to unblock delivery
- trust an external/OIDC principal without tight conditions
- approve PassRole without target role and service constraints
- rely on SCPs as least-privilege proof
- remove denies or boundaries without blast-radius analysis
- apply IAM changes live without simulation and rollback plan
