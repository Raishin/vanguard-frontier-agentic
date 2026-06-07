# OCI Compute Platform Operator Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Stopping, rebooting, resizing, or terminating before owner and dependency checks.
- Assuming display names uniquely identify instances.
- Judging exposure from instance list alone instead of VNIC, subnet, route, NSG/security-list, and public-IP evidence.
- Granting broad compute/volume/network management for routine read-only operations.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows compute instance list operations expose compartment, availability domain, display name, lifecycle state, capacity reservation, compute cluster, sort, and pagination filters. It also notes VNIC attachment and VNIC calls are needed for IP details. Treat this as API shape evidence, not full posture proof.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Confirm region, compartment, instance identity, lifecycle state, owner, and dependency map before mutation.
- Prefer read-only discovery before lifecycle actions.
- Check VNICs, public IPs, metadata, image age, boot/block volumes, backups, monitoring, and instance principals.
- Require explicit approval and rollback/failback for stop, reboot, resize, detach, delete, terminate, or image changes.
- Separate compute, volume, network, image, capacity, and instance-principal permissions.

## Minimal safe implementation flow

- Classify inventory, troubleshooting, posture review, or lifecycle mutation.
- Confirm exact target and business owner.
- Collect compute, VNIC, volume, backup, monitoring, IAM, and dependency evidence.
- Plan minimal reversible action with maintenance window and rollback.
- Validate lifecycle, reachability, logs/metrics, access boundaries, and unintended collateral impact.

## High-risk assumptions to kill

- “Instance list output proves network exposure.”
- “A stopped instance has no cost, dependency, or recovery concerns.”
- “Terminating and recreating is faster than understanding ownership.”
- “Instance principals are harmless because they are “inside OCI.””
- “Shape availability in documentation means capacity is available now.”

Those are lazy assumptions.

## Safe command/code verification targets

- List instances in confirmed compartment and do not rely on display name uniqueness.
- Fetch VNIC attachments and VNIC details for public/private IP exposure.
- Review boot/block volume attachment, backup policy, image source, metadata, cloud-init, monitoring alarms, and instance-agent posture.
- Check IAM policies/dynamic groups for instance principal and operator blast radius.
- Require owner approval and rollback before lifecycle or attachment mutations.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a write/delete/start/stop/update/remediate action before scope and owner are clear.
- The answer would depend on live infrastructure state but only documentation evidence exists.
- The proposed access is broader than the task requires.
- The plan has no rollback, owner, or validation step.
