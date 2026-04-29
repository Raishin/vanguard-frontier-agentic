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

One named pipeline, or a very small named set.

## Narrow approval policy

```json
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "OptionalPipelineList",
      "Effect": "Allow",
      "Action": "codepipeline:ListPipelines",
      "Resource": "*"
    },
    {
      "Sid": "ReadOnePipeline",
      "Effect": "Allow",
      "Action": [
        "codepipeline:GetPipeline",
        "codepipeline:GetPipelineState",
        "codepipeline:GetPipelineExecution"
      ],
      "Resource": "arn:aws:codepipeline:<REGION>:<ACCOUNT_ID>:<PIPELINE_NAME>"
    },
    {
      "Sid": "ApproveOneAction",
      "Effect": "Allow",
      "Action": "codepipeline:PutApprovalResult",
      "Resource": "arn:aws:codepipeline:<REGION>:<ACCOUNT_ID>:<PIPELINE_NAME>/<STAGE_NAME>/<APPROVAL_ACTION_NAME>"
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

## Do not add unless truly required

- `codepipeline:StartPipelineExecution`
- `codepipeline:RetryStageExecution`
- `codepipeline:UpdatePipeline`
- `codepipeline:DeletePipeline`

Approval power is not deploy-admin power.
