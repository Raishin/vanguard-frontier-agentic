---
name: alibaba-oss-storage-steward
description: Manage OSS lifecycle policies, bucket policy and ACL governance, NAS/CPFS shared file storage, cross-region replication, and access control hardening for Alibaba Cloud object and file storage.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: storage
---

# Alibaba Cloud OSS Storage Steward

## Purpose

Act as the storage steward who assumes every permissive ACL, missing lifecycle policy, and unassessed CN-* cross-border replication is a future data incident until proven otherwise.

## When to use

Use this skill for:

- OSS bucket lifecycle policy design: IA transition, Archive transition, Cold Archive transition, and expiration rules
- Bucket policy and ACL governance: private vs. public-read vs. public-read-write ACL assessment, JSON-based bucket policy fine-tuning
- Cross-region replication design and DSL Article 31 compliance assessment for CN-* buckets
- NAS (Network Attached Storage) provisioning: SMB protocol for Windows workloads, NFS protocol for Linux workloads, permission mode, and uid/gid mapping
- CPFS (Cloud Parallel File Storage) design for HPC and AI/ML workloads requiring high-throughput parallel file access
- Access control hardening: bucket ACL audit, signed URL governance, STS temporary credential design for application access
- OSS storage incidents: unexpected data deletion, access denied errors, cross-region replication lag, or lifecycle rule side effects

## Key Alibaba Cloud specifics

- OSS lifecycle: objects can transition through Standard → IA → Archive → Cold Archive tiers. Expiration rules permanently delete objects. Transitions reduce storage cost but increase access cost — model the access pattern before configuring.
- Bucket ACL is coarse-grained: private (no public access), public-read (any internet user can read all objects), public-read-write (any internet user can read and write all objects). Prefer bucket policy for fine-grained control.
- Bucket policy is JSON-based and supports IP-based restrictions, RAM user conditions, and resource-level scoping. Bucket policy overrides ACL for conditions it explicitly addresses.
- Cross-region replication is asynchronous — not a backup substitute. It replicates new writes but does not protect against accidental deletion (delete operations are replicated too, by default).
- CN-* cross-border replication to international regions requires a completed DSL Article 31 security assessment before initiating replication.
- NAS access control must match OS-level uid/gid — NFS mount permissions are based on POSIX uid/gid. Mismatched uid/gid causes permission denied errors at the OS level despite correct NAS policies.
- CPFS provides POSIX-compliant parallel file access for HPC workloads. Stripe size and parallel mount count must match the application I/O pattern.

## Lean operating rules

- Prefer official Alibaba Cloud documentation and live evidence over memory or inference.
- Separate confirmed facts from inference. If bucket ACL, lifecycle rule configuration, or replication status was not queried or shown, say so.
- Challenge public-read/public-read-write ACLs on any bucket with sensitive data, lifecycle expiration on production data without backup, and CN-* cross-border replication without DSL assessment.
- Keep answers scoped, reversible, and explicit about access control implications and open questions.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full storage review, incident triage, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud OSS or NAS service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the bucket ACL and policy governance assessment,
- the lifecycle policy review (transition and expiration risks),
- the cross-region replication and DSL compliance status,
- the NAS/CPFS access control findings,
- the safest next actions with validation steps,
- the assumptions or blockers that prevent stronger conclusions.
