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

## Choose one release path

Do not combine direct alias mutation and CodeDeploy management unless your release path genuinely needs both.

## Mode A: alias shift only

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "ReadAndShiftOneFunctionAlias",
      "Effect": "Allow",
      "Action": [
        "lambda:GetFunctionConfiguration",
        "lambda:GetAlias",
        "lambda:ListVersionsByFunction",
        "lambda:UpdateAlias"
      ],
      "Resource": [
        "arn:aws:lambda:<REGION>:<ACCOUNT_ID>:function:<FUNCTION_NAME>",
        "arn:aws:lambda:<REGION>:<ACCOUNT_ID>:function:<FUNCTION_NAME>:<ALIAS_NAME>"
      ]
    },
    {
      "Sid": "ReadNamedAlarms",
      "Effect": "Allow",
      "Action": [
        "cloudwatch:DescribeAlarms",
        "cloudwatch:DescribeAlarmsForMetric"
      ],
      "Resource": "*"
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

## Mode B: CodeDeploy-managed Lambda release

Typical narrow actions:
- `codedeploy:GetApplication`
- `codedeploy:GetDeploymentGroup`
- `codedeploy:GetDeployment`
- `codedeploy:CreateDeployment`
- `codedeploy:StopDeployment` only if abort capability is truly required

Scope those to one deployment application and one deployment group. If revisions come from S3, add only scoped `s3:GetObject` to the exact revision bucket/prefix.

## Do not add blindly

- wildcard `lambda:*`
- unrestricted direct `lambda:UpdateAlias` when CodeDeploy is the actual release path
