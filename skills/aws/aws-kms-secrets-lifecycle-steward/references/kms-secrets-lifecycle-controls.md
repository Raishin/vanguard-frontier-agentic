# KMS and Secrets Lifecycle Controls Guide

Use this reference for AWS KMS key policy, grants, aliases, rotation, multi-Region keys, imported key material, key deletion, Secrets Manager rotation, replication, VPC endpoints, resource policies, and application secret consumption.

## What people get wrong

The lazy story is:

> Encryption is enabled and secrets rotate, so crypto posture is fine.

Wrong. KMS and Secrets Manager failures are usually lifecycle failures: bad key policy, grant sprawl, broken rotation consumers, disabled replica keys, undecryptable backups, or irreversible key deletion.

Common bad assumptions:

- IAM policy alone controls KMS access.
- Key aliases are stable security boundaries.
- Automatic key rotation solves secret rotation.
- Secret replication is harmless metadata.
- VPC endpoints make secret retrieval private and authorized.
- Scheduled key deletion is a normal cleanup task.

## KMS/secrets failure modes

- Key policy omits account/root enablement, break-glass, or required service principals.
- Grants survive longer than workload need or permit unintended decrypt/data-key use.
- KMS key disabled/deleted/rotated blocks RDS, EBS, S3, Lambda, backups, or secret replication.
- Rotation Lambda updates the secret but not every consuming application or connection pool.
- Secrets Manager resource policy allows cross-account access without org/source conditions.
- Multi-Region key or replicated secret has asymmetric policy/KMS/VPC endpoint posture.

## Minimum safe workflow

1. Identify protected data, key/secret owners, consuming services, Regions, accounts, and recovery requirements.
2. Review KMS key policy first, then IAM policies, grants, aliases, rotation, multi-Region state, and deletion schedule.
3. Review secret lifecycle: creation, rotation, version staging labels, replication, resource policy, retrieval path, cache, and consumers.
4. Check endpoint and policy conditions: VPC endpoint, principal org, source account/ARN, encryption context, and service principal.
5. Verify recovery: break-glass, backup restore decryptability, replica key availability, and rollback of rotation.
6. Recommend reversible policy/rotation fixes; key deletion, disablement, or secret deletion require explicit approval and impact proof.
7. Separate configuration evidence from successful decrypt/restore/rotation evidence.

## Verification targets

- KMS key policy, IAM policies, grants, aliases, rotation, multi-Region key state, imported material, deletion schedule, and CloudTrail usage
- encryption context, ViaService/source conditions, cross-account principals, and service-linked usage
- Secrets Manager secret policy, KMS key, rotation Lambda, version staging labels, replication Regions, and last rotation status
- VPC endpoint policy, CloudTrail events, application retrieval path, cache TTL, and error handling
- RDS/EBS/S3/Lambda/ECS/EKS/AWS Backup dependencies on the key or secret
- break-glass access, recovery tests, backup restore decryptability, and consumer rotation validation

## When to push back

Push back if the user asks to:

- schedule KMS key deletion without dependency and restore proof
- rotate secrets without consumer readiness and rollback plan
- use broad decrypt permissions or wildcard key policies
- treat aliases as immutable trust anchors
- replicate secrets/keys without Region, KMS, and policy review
- remove CloudTrail/log evidence while fixing access
