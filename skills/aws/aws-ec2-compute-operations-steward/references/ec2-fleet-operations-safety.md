# EC2 Fleet Operations Safety Guide

Use this reference for EC2, Auto Scaling groups, Launch Templates, AMIs, Systems Manager, Patch Manager, Session Manager, EBS volumes/snapshots, instance refresh, lifecycle hooks, health checks, and fleet reliability reviews.

## What people get wrong

The lazy story is:

> EC2 is legacy; patch it, refresh it, or replace instances.

Wrong. EC2 fleets hide state, pets, bootstrap drift, manual access paths, patch gaps, EBS coupling, and Auto Scaling rollout risk. Treat every host operation as potentially stateful until proven otherwise.

Common bad assumptions:

- Instance refresh is always safer than manual replacement.
- Latest AMI means compliant and compatible.
- SSM managed node status proves patch posture.
- Session Manager removes all SSH/bastion risk.
- EBS snapshots prove application-consistent recovery.
- Auto Scaling health checks match customer health.

## EC2 operations failure modes

- Launch Template update changes IAM, user data, security groups, block devices, or AMI unexpectedly.
- Instance refresh drains capacity without lifecycle hooks, warmup, health checks, or rollback target.
- Patch Manager compliance misses unmanaged nodes, maintenance windows, or reboot requirements.
- Session Manager lacks logging, KMS, VPC endpoint, or least-privilege controls.
- EBS volume performance, burst balance, attachment, encryption, or snapshot consistency is ignored.
- Pets/manual changes create drift from AMI/bootstrap expectations.

## Minimum safe workflow

1. Identify fleet: instances, Auto Scaling groups, Launch Templates, AMIs, subnets, load balancers, and stateful dependencies.
2. Review access/management: SSM managed node status, Session Manager, IAM instance profile, patch baseline, and logging.
3. Review rollout safety: instance refresh settings, lifecycle hooks, health checks, desired/min/max capacity, warm pools, and rollback AMI/template version.
4. Check storage/recovery: EBS volumes, snapshots, encryption, application consistency, and attachment behavior.
5. Review observability: EC2 status checks, CloudWatch agent, logs, alarms, load balancer target health, and application health.
6. Recommend staged, reversible changes; stop/terminate/reboot/refresh actions require explicit approval.
7. Separate host configuration evidence from live workload health evidence.

## Verification targets

- EC2 instance state, status checks, AMI, user data, IAM instance profile, security groups, tags, and SSM managed node status
- Auto Scaling group desired/min/max capacity, Launch Template version, instance refresh, lifecycle hooks, warmup, health check type, and rollback target
- Systems Manager Patch Manager baseline, compliance, maintenance windows, State Manager associations, Inventory, and Session Manager logging/KMS settings
- EBS volume type, size, IOPS/throughput, burst balance, encryption, snapshots, and application-consistent backup evidence
- load balancer target health, CloudWatch metrics/logs, CloudWatch agent, alarms, and recent change/deployment timeline
- SSH/bastion exposure, Session Manager endpoint policy, VPC endpoints, and break-glass path

## When to push back

Push back if the user asks to:

- terminate/reboot/refresh instances without state and capacity proof
- roll forward to latest AMI without compatibility and rollback target
- ignore unmanaged nodes in patch reports
- disable health checks or lifecycle hooks to speed rollout
- treat EBS snapshots as application-consistent without evidence
- broaden SSH/Session Manager access for convenience
