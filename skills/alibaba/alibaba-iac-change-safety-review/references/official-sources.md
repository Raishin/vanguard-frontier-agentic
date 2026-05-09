# Official sources

Use this reference only when you need source grounding for Alibaba Cloud IaC service behavior or the detailed source list.

## Alibaba Cloud documentation

Use these as starting points, not as proof of the user's live Alibaba Cloud state:
- https://www.alibabacloud.com/help/en/resource-orchestration-service/latest/what-is-ros
- https://www.alibabacloud.com/help/en/resource-orchestration-service/latest/detect-stack-drift
- https://www.alibabacloud.com/help/en/resource-orchestration-service/latest/deletion-protection
- https://registry.terraform.io/providers/aliyun/alicloud/latest/docs
- https://www.alibabacloud.com/help/en/resource-management/latest/what-is-resource-management
- https://www.alibabacloud.com/help/en/oss/user-guide/server-side-encryption
- https://www.alibabacloud.com/help/en/kms/latest/overview

## Grounding rule

Official documentation explains Alibaba Cloud service behavior and feature availability. It does not prove the user's current stack state, drift status, account scope, or applied policy. Prefer sanitized terraform plan output or ROS change set preview for current-state claims. Terraform state files are authoritative for deployed resource attributes but must never be shared with credentials or sensitive values present.
