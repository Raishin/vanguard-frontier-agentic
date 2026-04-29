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

## Recommended scope

Bind this role to one stack family and one environment.

## CloudFormation-focused minimum

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "CloudFormationReadAndPreview",
      "Effect": "Allow",
      "Action": [
        "cloudformation:ValidateTemplate",
        "cloudformation:GetTemplate",
        "cloudformation:GetTemplateSummary",
        "cloudformation:DescribeStacks",
        "cloudformation:DescribeStackEvents",
        "cloudformation:DescribeStackResources",
        "cloudformation:DescribeChangeSet",
        "cloudformation:ListChangeSets",
        "cloudformation:CreateChangeSet",
        "cloudformation:DeleteChangeSet",
        "cloudformation:ExecuteChangeSet",
        "cloudformation:DetectStackDrift",
        "cloudformation:DetectStackResourceDrift",
        "cloudformation:DescribeStackDriftDetectionStatus",
        "cloudformation:BatchDescribeTypeConfigurations"
      ],
      "Resource": [
        "arn:aws:cloudformation:<REGION>:<ACCOUNT_ID>:stack/<STACK_NAME>/*",
        "arn:aws:cloudformation:<REGION>:<ACCOUNT_ID>:changeSet/*/*"
      ]
    },
    {
      "Sid": "CallerIdentity",
      "Effect": "Allow",
      "Action": "sts:GetCallerIdentity",
      "Resource": "*"
    }
  ]
}
```

## Important catch

Drift detection also needs read permissions for the underlying resource types in the stack, for example `ec2:DescribeInstances` when EC2 resources exist.

## Optional only if needed

- narrow `iam:PassRole` to one CloudFormation service role ARN

## Do not add

- broad `cloudformation:*`
- broad `iam:PassRole`
- unrelated mutate permissions on resources outside the named stack family
