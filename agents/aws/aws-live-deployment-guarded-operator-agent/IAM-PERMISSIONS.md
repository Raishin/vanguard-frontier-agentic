# Least-privilege IAM guidance

## Identity model

Preferred order:

1. IAM Identity Center or federation
2. short-lived assumed role
3. narrow environment-scoped role
4. IAM user only as a last resort

Minimum common baseline for any live operator:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "CallerIdentity",
      "Effect": "Allow",
      "Action": "sts:GetCallerIdentity",
      "Resource": "*"
    }
  ]
}
```

Do not treat repo write access as a reason to hand out broad AWS permissions.

## Recommended binding

This agent is too generic for direct broad deploy rights.

Best practice:
- give it `sts:GetCallerIdentity`
- give it only scoped `sts:AssumeRole`
- let it jump into one of the narrower live roles below

Example broker policy:

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "CallerIdentity",
      "Effect": "Allow",
      "Action": "sts:GetCallerIdentity",
      "Resource": "*"
    },
    {
      "Sid": "AssumeOnlyNamedGuardedRoles",
      "Effect": "Allow",
      "Action": "sts:AssumeRole",
      "Resource": [
        "arn:aws:iam::<ACCOUNT_ID>:role/aws-live-iac-change-guard-prod",
        "arn:aws:iam::<ACCOUNT_ID>:role/aws-live-pipeline-approval-prod",
        "arn:aws:iam::<ACCOUNT_ID>:role/aws-live-serverless-release-prod",
        "arn:aws:iam::<ACCOUNT_ID>:role/aws-live-ecs-rollout-prod"
      ]
    }
  ]
}
```

## Why

If you give this agent one giant live policy that covers CloudFormation, CodePipeline, Lambda, ECS, and CodeDeploy, you have already failed least privilege.

## Do not add

- `AdministratorAccess`
- broad `iam:PassRole`
- direct wildcard deploy rights across multiple live services
