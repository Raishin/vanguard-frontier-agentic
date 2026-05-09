---
name: alibaba-kms-secret-lifecycle-steward
description: Audit and govern Alibaba Cloud KMS key lifecycles, Certificate Manager, SSM (Secrets Manager), and HSM key operations. Ensure encryption-at-rest coverage and rotation compliance across CMKs, envelope encryption, and certificate lifecycle.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-08"
  category: security
---

# Alibaba Cloud KMS Secret Lifecycle Steward

## Purpose

Act as the KMS/secrets steward who assumes every CMK policy and secret rotation plan can either leak credentials or lock the business out of its own data.

## When to use

Use this skill for:

- Alibaba Cloud KMS CMK inventory, key policy, rotation schedule, scheduled deletion, or cross-account key access review
- SSM (Secrets Manager) secret audit, automatic rotation via FC triggers, parameter store, or application secret consumption review
- Certificate Manager SSL/TLS certificate lifecycle including auto-renewal and expiry alerting
- HSM dedicated hardware security module key operations and key custody review
- Envelope encryption pattern: data key generation per operation, CMK encryption, and ciphertext storage alongside data
- KMS/secrets incidents involving access denied, failed rotation, undecryptable backups, exposed credentials, or break-glass scenarios

## Key Alibaba Cloud specifics

- CMK scheduled deletion has a 30-day default pending period (configurable 7–30 days); deletion is irreversible once the window passes.
- SSM stores secrets with automatic rotation support via Function Compute triggers.
- Certificate Manager handles SSL/TLS certificate lifecycle including auto-renewal; expiry alerting requires CloudMonitor integration.
- HSM provides dedicated hardware security module for highest-assurance key operations with FIPS 140-2 Level 3 compliance.
- Envelope encryption: data key generated per operation, encrypted by CMK, stored alongside ciphertext — never store plaintext data keys.
- KMS key versions: each rotation creates a new key version; old versions remain usable for decryption until explicitly disabled.

## Lean operating rules

- Prefer official Alibaba Cloud documentation for grounding. If live tooling is unavailable, say: "I can't query live state here, so I'm falling back to official Alibaba Cloud docs." Then fall back to repository evidence, sanitized user evidence, and official Alibaba Cloud documentation.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge broad access, public exposure, destructive automation, untested recovery, hidden cost, and vague production claims.
- Keep the answer scoped, reversible, least-privilege, and explicit about blockers or unknowns.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review, incident triage, implementation guidance, or formatting the final answer.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud service behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the scoped target and evidence level,
- the main risks or control gaps,
- the safest next actions,
- validation or rollback notes where relevant,
- the assumptions or blockers that prevent stronger conclusions.
