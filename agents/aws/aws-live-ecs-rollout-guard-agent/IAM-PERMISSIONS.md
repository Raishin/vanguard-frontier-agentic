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

One ECS cluster and one service family.

## Narrow ECS rollout policy

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "ReadServiceState",
      "Effect": "Allow",
      "Action": [
        "ecs:DescribeServices",
        "ecs:DescribeTaskDefinition",
        "ecs:DescribeTasks",
        "ecs:ListTasks"
      ],
      "Resource": "*"
    },
    {
      "Sid": "UpdateOneService",
      "Effect": "Allow",
      "Action": "ecs:UpdateService",
      "Resource": "arn:aws:ecs:<REGION>:<ACCOUNT_ID>:service/<CLUSTER_NAME>/<SERVICE_NAME>"
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

## Important note

Some ECS read APIs are not resource-scoped cleanly, so `Resource: "*"` may still be required for specific `Describe*` calls. That is still far better than `ecs:*`.

## Do not add unless truly needed

- `ecs:DeleteService`
- `ecs:DeregisterTaskDefinition`
- `ecs:RunTask`
- `ecs:StopTask`
- broad `iam:PassRole`
